use std::collections::{HashMap, VecDeque};
use std::io::IoSlice;
use std::time::Instant;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::packet_functions::*;

enum NetEvent {
    Accept {
        idx: u128,
        to_send_tx: UnboundedSender<(u16, Vec<u8>)>,
    },

    Receive {
        idx: u128,
        packet_type: u16,
        message: Vec<u8>,
    },

    Disconnect {
        idx: u128,
    },
}

struct Session {
    idx: u128,
    pending_disconnect: bool,
    to_send_tx: UnboundedSender<(u16, Vec<u8>)>,
}

impl Session {
    pub fn new(idx: u128, to_send_tx: UnboundedSender<(u16, Vec<u8>)>) -> Session {
        Session {
            idx,
            pending_disconnect: false,
            to_send_tx,
        }
    }

    pub fn send_message(
        &self,
        packet_type: u16,
        message: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.pending_disconnect == false {
            self.to_send_tx.send((packet_type, message))?;
            Ok(())
        } else {
            Err(format!("Session {} is pending disconnect", self.idx).into())
        }
    }
}

pub trait Object {
    fn update(&mut self, app: &mut App, delta_time: u32);
    fn on_accept(&mut self, app: &mut App, idx: u128);
    fn on_receive(&mut self, app: &mut App, idx: u128, packet_type: u16, payload: Vec<u8>);
    fn on_disconnect(&mut self, app: &mut App, idx: u128);
}

pub struct App {
    str_addr: String,
    sessions: HashMap<u128, Session>,
}

impl App {
    pub fn new(str_addr: String) -> Self {
        App {
            str_addr,
            sessions: HashMap::new(),
        }
    }

    pub async fn run<T: Object>(&mut self, server_object: T) {
        let (to_main_tx, to_main_rx) = mpsc::unbounded_channel();
        let str_addr = self.str_addr.clone();

        tokio::spawn(async move {
            App::accept_process(str_addr, to_main_tx).await;
        });

        self.main_process(server_object, to_main_rx).await;
    }

    pub fn send_message(
        &mut self,
        idx: u128,
        packet_type: u16,
        message: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = self.sessions.get_mut(&idx) {
            session.send_message(packet_type, message)
        } else {
            Err(format!("Session with ID {} not found", idx).into())
        }
    }

    pub fn disconnect(&mut self, idx: u128) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = self.sessions.get_mut(&idx) {
            if session.pending_disconnect == true {
                return Ok(());
            }
            session.pending_disconnect = true;
            session.send_message(0, "".into())
        } else {
            Err(format!("Session with ID {} not found", idx).into())
        }
    }

    async fn accept_process(str_addr: String, to_main_tx: UnboundedSender<NetEvent>) {
        let listener = TcpListener::bind(str_addr).await.unwrap();

        let mut counter: u128 = 0;

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
        idx: u128,
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
                    // TODO: 여기서 메시지 파싱을 해줘야함
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
        idx: u128,
        mut to_send_rx: UnboundedReceiver<(u16, Vec<u8>)>,
    ) {
        let mut ring_buf = VecDeque::with_capacity(1024);
        let mut active = true;

        while active || !ring_buf.is_empty() {
            if ring_buf.is_empty() {
                if let Some((packet_type, message)) = to_send_rx.recv().await {
                    active = push_message_with_header(packet_type, message, &mut ring_buf) > 0;
                } else {
                    active = false;
                }
            }

            while active {
                match to_send_rx.try_recv() {
                    Ok((packet_type, message)) => {
                        active = push_message_with_header(packet_type, message, &mut ring_buf) > 0;
                    }
                    Err(mpsc::error::TryRecvError::Disconnected) => active = false,
                    Err(mpsc::error::TryRecvError::Empty) => break,
                }
            }

            if !ring_buf.is_empty() {
                let (first, second) = ring_buf.as_slices();
                // 슬라이스를 IoSlice로 감싸기
                let slices = [IoSlice::new(first), IoSlice::new(second)];

                match wh.write_vectored(&slices).await {
                    Ok(send_size) => {
                        ring_buf.drain(..send_size);
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

    async fn main_process<T: Object>(&mut self, mut server_object: T, mut to_main_rx: UnboundedReceiver<NetEvent>) {
        let mut start = Instant::now();

        loop {
            // `try_recv`로 메시지를 비동기적으로 확인
            match to_main_rx.try_recv() {
                Ok(net_event) => match net_event {
                    NetEvent::Accept { idx, to_send_tx } => {
                        self.sessions.insert(idx, Session::new(idx, to_send_tx));
                        server_object.on_accept(self, idx);
                    },
                    NetEvent::Receive {
                        idx,
                        packet_type,
                        message,
                    } => server_object.on_receive(self, idx, packet_type, message),
                    NetEvent::Disconnect { idx } => {
                        self.sessions.remove(&idx);
                        server_object.on_disconnect(self, idx);
                    },
                },
                Err(try_recv_err) => match try_recv_err {
                    mpsc::error::TryRecvError::Empty => {
                        // 업데이트 콜백 호출
                        let delta_time = start.elapsed().as_millis() as u32;
                        server_object.update(self, delta_time);
                        start = Instant::now();
                    },
                    mpsc::error::TryRecvError::Disconnected => break,
                },
            }

        }
    }
}
