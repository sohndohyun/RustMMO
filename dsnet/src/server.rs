use std::collections::VecDeque;
use std::io::IoSlice;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::packet_functions::*;

pub struct Session {
    idx: u64,
    pending_disconnect: bool,
    to_send_tx: UnboundedSender<(u16, Arc<[u8]>)>,
}

impl Session {
    fn new(idx: u64, to_send_tx: UnboundedSender<(u16, Arc<[u8]>)>) -> Session {
        Session {
            idx,
            pending_disconnect: false,
            to_send_tx,
        }
    }

    pub fn get_idx(&self) -> u64 {
        self.idx
    }
    pub fn is_pending_disconnect(&self) -> bool {
        self.pending_disconnect
    }

    pub fn send_message(
        &self,
        packet_type: u16,
        message: Arc<[u8]>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.pending_disconnect == false {
            self.to_send_tx.send((packet_type, message))?;
            Ok(())
        } else {
            Err(format!("Session {} is pending disconnect", self.idx).into())
        }
    }

    pub fn disconnect(&mut self) {
        if self.pending_disconnect == true {
            return;
        }
        self.pending_disconnect = true;
        _ = self.to_send_tx.send((0, Arc::from([])));
    }
}

pub enum Callback {
    Accept {
        idx: u64,
        session: Session,
    },

    Receive {
        idx: u64,
        packet_type: u16,
        message: Vec<u8>,
    },

    Disconnect {
        idx: u64,
    },

    Empty,
    Close,
}

enum NetEvent {
    Accept {
        idx: u64,
        to_send_tx: UnboundedSender<(u16, Arc<[u8]>)>,
    },

    Receive {
        idx: u64,
        packet_type: u16,
        message: Vec<u8>,
    },

    Disconnect {
        idx: u64,
    },
}

pub struct App {
    to_main_rx: UnboundedReceiver<NetEvent>,
}

impl App {
    pub fn run(str_addr: String) -> App {
        let (to_main_tx, to_main_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            App::accept_process(str_addr, to_main_tx).await;
        });

        App { to_main_rx }
    }

    async fn accept_process(str_addr: String, to_main_tx: UnboundedSender<NetEvent>) {
        let listener = TcpListener::bind(str_addr).await.unwrap();

        let mut counter: u64 = 0;

        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    let (rh, wh) = socket.into_split();

                    // receive process begin
                    tokio::spawn(Self::receive_process(rh, counter, to_main_tx.clone()));

                    let (to_send_tx, to_send_rx) = mpsc::unbounded_channel();
                    // send process begin
                    tokio::spawn(Self::send_process(wh, counter, to_send_rx));

                    // send message to main process
                    to_main_tx
                        .send(NetEvent::Accept {
                            idx: counter,
                            to_send_tx,
                        })
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to send NetEvent::Accept: {:?}", e);
                        });

                    counter += 1;
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    }

    async fn receive_process(
        mut rh: OwnedReadHalf,
        idx: u64,
        to_main_tx: UnboundedSender<NetEvent>,
    ) {
        let mut recv_buf = [0; 1024];
        let mut ring_buf = VecDeque::with_capacity(1024);

        loop {
            match rh.read(&mut recv_buf).await {
                // 정상 종료
                Ok(0) => {
                    if let Err(e) = to_main_tx.send(NetEvent::Disconnect { idx }) {
                        eprintln!("Failed to send NetEvent::Disconnect: {:?}", e);
                    }
                    break;
                }
                // 오류 발생!
                Err(e) => {
                    eprintln!("Error reading from idx {}: {:?}", idx, e);

                    if let Err(send_err) = to_main_tx.send(NetEvent::Disconnect { idx }) {
                        eprintln!("Failed to send NetEvent::Disconnect: {:?}", send_err);
                    }
                    break;
                }
                // 정상적인 읽기 완료 이 안에서 처리 ㄱㄱ
                Ok(n) => {
                    ring_buf.extend(recv_buf[0..n].iter());
                    while let Some((packet_type, message)) = check_and_pop_packet(&mut ring_buf) {
                        if let Err(e) = to_main_tx.send(NetEvent::Receive {
                            idx,
                            packet_type,
                            message,
                        }) {
                            eprintln!("Failed to send NetEvent::Receive: {:?}", e);
                        }
                    }
                }
            };
        }

        // log...
        println!("end receive process!");
    }

    async fn send_process(
        mut wh: OwnedWriteHalf,
        idx: u64,
        mut to_send_rx: UnboundedReceiver<(u16, Arc<[u8]>)>,
    ) {
        let mut slice_queue = VecDeque::with_capacity(16);
        let mut active = true;
        let mut offset: usize = 0;

        while active || !slice_queue.is_empty() {
            if slice_queue.is_empty() {
                if let Some((packet_type, message)) = to_send_rx.recv().await {
                    active = push_message_with_header(packet_type, message, &mut slice_queue) > 0;
                } else {
                    active = false;
                }
            }

            while active {
                match to_send_rx.try_recv() {
                    Ok((packet_type, message)) => {
                        active = push_message_with_header(packet_type, message, &mut slice_queue) > 0;
                    }
                    Err(mpsc::error::TryRecvError::Disconnected) => active = false,
                    Err(mpsc::error::TryRecvError::Empty) => break,
                }
            }

            if !slice_queue.is_empty() {
                // 슬라이스를 IoSlice로 감싸기
                let io_slices = slice_queue_to_io_slice(&slice_queue, offset);

                let write_result = wh.write_vectored(&io_slices).await;

                match write_result {
                    Ok(mut send_size) => {
                        while let Some(slice) = slice_queue.front() {
                            let slice_len = slice.len() - offset;
                            if slice_len <= send_size {
                                send_size -= slice_len;
                                offset = 0;
                                slice_queue.pop_front();
                            }
                        }

                        offset = send_size;
                    }
                    Err(e) => {
                        eprintln!("failed to write to `{}`: {:?}", idx, e);
                        break;
                    }
                }
            }
        }

        // log...
        println!("Connection `{}` send process ended", idx);
    }

    pub fn get_callback(&mut self) -> Callback {
        match self.to_main_rx.try_recv() {
            Ok(net_event) => match net_event {
                NetEvent::Accept { idx, to_send_tx } => {
                    return Callback::Accept {
                        idx,
                        session: Session::new(idx, to_send_tx),
                    }
                }
                NetEvent::Receive {
                    idx,
                    packet_type,
                    message,
                } => {
                    return Callback::Receive {
                        idx,
                        packet_type,
                        message,
                    }
                }
                NetEvent::Disconnect { idx } => return Callback::Disconnect { idx },
            },
            Err(try_recv_err) => match try_recv_err {
                mpsc::error::TryRecvError::Empty => return Callback::Empty,
                mpsc::error::TryRecvError::Disconnected => return Callback::Close,
            },
        }
    }
}
