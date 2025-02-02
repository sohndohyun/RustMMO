use std::{cell::RefCell, rc::Rc};

use dsnet::server::{self, App};
extern crate flatbuffers;

mod protocol_generated;
pub use protocol_generated::nexus;

struct ChatServer {
    app: server::App,
    indexs: Vec<u128>,
}

impl ChatServer {
    pub fn new() -> Rc<RefCell<Self>> {
        let chat_server = Rc::new(RefCell::new(ChatServer {
            app: server::App::new(),
            indexs: Vec::new(),
        }));

        let weak_chat_server = Rc::downgrade(&chat_server);
        chat_server.borrow_mut().app.set_update_callback(Box::new(move |delta_time| {
            if let Some(game_server) = weak_chat_server.upgrade() {
                game_server.borrow_mut().update(delta_time);
            }
        }));

        let weak_chat_server = Rc::downgrade(&chat_server);
        chat_server.borrow_mut().app.set_on_accept_callback(Box::new(move |idx| {
            if let Some(game_server) = weak_chat_server.upgrade() {
                game_server.borrow_mut().on_accept(idx);
            }
        }));

        let weak_chat_server = Rc::downgrade(&chat_server);
        chat_server.borrow_mut().app.set_on_receive_callback(Box::new(move |idx, packet_type, payload| {
            if let Some(game_server) = weak_chat_server.upgrade() {
                game_server.borrow_mut().on_receive(idx, packet_type, payload);
            }
        }));

        let weak_chat_server = Rc::downgrade(&chat_server);
        chat_server.borrow_mut().app.set_on_disconnect_callback(Box::new(move |idx| {
            if let Some(game_server) = weak_chat_server.upgrade() {
                game_server.borrow_mut().on_disconnect(idx);
            }
        }));

        chat_server
    }

    fn update(&mut self, _idx: u32) {
        
    }

    fn on_accept(&mut self, idx: u128) {
        println!("on_accept callback triggered. (idx: {})", idx);
        self.indexs.push(idx);
    }

    fn on_receive(&mut self, idx: u128, packet_type: u16, payload: Vec<u8>) {
        println!("on_receive callback triggered. (idx: {})", idx);
        for value in &self.indexs {
            _ = self.app.send_message(*value, packet_type, payload.clone());
        }
    }

    fn on_disconnect(&mut self, idx: u128) {
        println!("on_disconnect callback triggered. (idx: {})", idx);
        if let Some(pos) = self.indexs.iter().position(|&x| x == idx) {
            self.indexs.swap_remove(pos); 
        }
    }

    pub async fn run(&mut self) {
        self.app.run("127.0.0.1:1234".into()).await
    }
}

#[tokio::main]
async fn main() {
    let game_server = ChatServer::new();
    game_server.borrow_mut().run().await;
}
