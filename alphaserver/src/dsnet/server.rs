use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::dsnet::packet::{check_and_pop_packet, from_ring_to_vec, push_message_with_header};

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

pub struct App {
    str_addr: String,
    on_update_cb: fn(&mut App, u32), // u32는 deltatime (전에 콜된 시점에서 지금까지의 시간)
    on_accept_cb: fn(&mut App, u128),
    on_receive_cb: fn(&mut App, u128, u16, Vec<u8>),
    on_disconnect_cb: fn(&mut App, u128),
    sessions: HashMap<u128, Session>,
}

impl Default for App {
    fn default() -> Self {
        fn default_on_update(_: &mut App, _: u32) {
            println!("Default on_update callback triggered.");
        }

        fn default_on_accept(_: &mut App, _: u128) {
            println!("Default on_accept callback triggered.");
        }

        fn default_on_receive(_: &mut App, _: u128, _: u16, _: Vec<u8>) {
            println!("Default on_receive callback triggered.");
        }

        fn default_on_disconnect(_: &mut App, _: u128) {
            println!("Default on_disconnect callback triggered.");
        }

        App {
            str_addr: "127.0.0.1:8080".to_string(),
            on_update_cb: default_on_update,
            on_accept_cb: default_on_accept,
            on_receive_cb: default_on_receive,
            on_disconnect_cb: default_on_disconnect,
            sessions: HashMap::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        // Create the runtime
        let rt = Runtime::new().unwrap();

        // Spawn the root task
        rt.block_on(async {
            let (to_main_tx, to_main_rx) = mpsc::unbounded_channel();
            let str_addr = self.str_addr.clone();

            tokio::spawn(async move {
                App::accept_process(str_addr, to_main_tx).await;
            });

            self.main_process(to_main_rx).await;
        })
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
                    while let Some((packet_type, message)) = check_and_pop_packet(&mut ring_buf){
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
        let mut cached_buf = Vec::with_capacity(1024);
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
                let send_buf = from_ring_to_vec(&ring_buf, &mut cached_buf);
                match wh.write(&send_buf).await {
                    Ok(send_size) => {
                        ring_buf.drain(0..send_size);
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

    async fn main_process(&mut self, mut to_main_rx: UnboundedReceiver<NetEvent>) {
        let mut start = Instant::now();

        loop {
            // `try_recv`로 메시지를 비동기적으로 확인
            match to_main_rx.try_recv() {
                Ok(net_event) => match net_event {
                    NetEvent::Accept { idx, to_send_tx } => self.on_accept(idx, to_send_tx),
                    NetEvent::Receive { idx, packet_type, message } => self.on_receive(idx, packet_type, message),
                    NetEvent::Disconnect { idx } => self.on_disconnect(idx),
                },
                Err(try_recv_err) => match try_recv_err {
                    mpsc::error::TryRecvError::Empty => (),
                    mpsc::error::TryRecvError::Disconnected => break,
                },
            }

            // 업데이트 콜백 호출
            let delta_time = start.elapsed().as_millis() as u32;
            (self.on_update_cb)(self, delta_time);

            // 새로운 시작 시간으로 갱신
            start = Instant::now();

            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }

    fn on_accept(&mut self, idx: u128, to_send_tx: UnboundedSender<(u16, Vec<u8>)>) {
        self.sessions.insert(idx, Session::new(idx, to_send_tx));

        (self.on_accept_cb)(self, idx);
    }

    fn on_receive(&mut self, idx: u128, packet_type: u16, message: Vec<u8>) {
        (self.on_receive_cb)(self, idx, packet_type, message);
    }

    fn on_disconnect(&mut self, idx: u128) {
        self.sessions.remove(&idx);
        (self.on_disconnect_cb)(self, idx);
    }

    pub fn set_str_addr(&mut self, str_addr: &str) -> Result<(), String> {
        if str_addr.parse::<std::net::SocketAddr>().is_ok() {
            self.str_addr = str_addr.to_string();
            Ok(())
        } else {
            Err("Invalid address format".to_string())
        }
    }

    pub fn set_on_update(&mut self, on_update_cb: fn(&mut App, u32)) {
        self.on_update_cb = on_update_cb;
    }

    pub fn set_on_accept(&mut self, on_accept_cb: fn(&mut App, u128)) {
        self.on_accept_cb = on_accept_cb;
    }

    pub fn set_on_receive(&mut self, on_receive_cb: fn(&mut App, u128, u16, Vec<u8>)) {
        self.on_receive_cb = on_receive_cb;
    }

    pub fn set_on_disconnect(&mut self, on_disconnect_cb: fn(&mut App, u128)) {
        self.on_disconnect_cb = on_disconnect_cb;
    }
}
