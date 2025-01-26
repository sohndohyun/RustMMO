use std::collections::VecDeque;
use std::io::IoSlice;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::{net::TcpStream, sync::mpsc};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::packet_functions::*;

pub enum Callback {
    Receive {
        packet_type: u16,
        message: Vec<u8>,
    },
    Disconnect,
    None,
    Close,
}

pub struct App {
    to_main_rx: UnboundedReceiver<Callback>,
    to_send_tx: UnboundedSender<(u16, Vec<u8>)>,
    pending_disconnect: bool,
}


impl App {
    pub async fn create(str_addr: String) -> Result<App, Box<dyn std::error::Error>> {
        let socket = TcpStream::connect(str_addr).await?;
        let (rh, wh) = socket.into_split();
        let (to_main_tx, to_main_rx) = mpsc::unbounded_channel();
        let (to_send_tx, to_send_rx) = mpsc::unbounded_channel();

        tokio::spawn(Self::receive_process(rh, to_main_tx));

        // send process begin
        tokio::spawn(Self::send_process(wh, to_send_rx));

        Ok(App {to_main_rx, to_send_tx, pending_disconnect: false})
    }

    pub fn get_callback(&mut self) -> Callback {
        match self.to_main_rx.try_recv() {
            Ok(callback) => callback,
            Err(try_recv_err) => match try_recv_err {
                mpsc::error::TryRecvError::Empty => Callback::None,
                mpsc::error::TryRecvError::Disconnected => Callback::Close,
            },
        }
    }

    pub fn send_message(&mut self, packet_type: u16, message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        if self.pending_disconnect == false {
            self.to_send_tx.send((packet_type, message))?;
            Ok(())
        } else {
            Err(format!("Pending disconnect!").into())
        }
    }

    pub fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.pending_disconnect == true {
            return Ok(());
        }
        self.pending_disconnect = true;
        self.send_message(0, "".into())
    }

    async fn receive_process(mut rh: OwnedReadHalf, to_main_tx: UnboundedSender<Callback>) {
        let mut recv_buf = [0; 1024];
        let mut ring_buf = VecDeque::with_capacity(1024);

        loop {
            match rh.read(&mut recv_buf).await {
                // 정상 종료
                Ok(0) => {
                    if let Err(e) = to_main_tx.send(Callback::Disconnect) {
                        eprintln!("Failed to send NetEvent::Disconnect: {:?}", e);
                    }
                    break;
                }
                // 오류 발생!
                Err(e) => {
                    eprintln!("Error reading: {:?}", e);

                    if let Err(send_err) = to_main_tx.send(Callback::Disconnect) {
                        eprintln!("Failed to send NetEvent::Disconnect: {:?}", send_err);
                    }
                    break;
                }
                // 정상적인 읽기 완료 이 안에서 처리 ㄱㄱ
                Ok(n) => {
                    // TODO: 여기서 메시지 파싱을 해줘야함
                    ring_buf.extend(recv_buf[0..n].iter());
                    while let Some((packet_type, message)) = check_and_pop_packet(&mut ring_buf) {
                        if let Err(e) = to_main_tx.send(Callback::Receive {
                            packet_type,
                            message,
                        }) {
                            eprintln!("Failed to send NetEvent::Receive: {:?}", e);
                        }
                    }
                }
            };
        }
    }

    async fn send_process(
        mut wh: OwnedWriteHalf,
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
                        eprintln!("failed to write: {:?}", e);
                        break;
                    }
                }
            }
        }

        // log...
        println!("Connection send process ended");
    }

}
