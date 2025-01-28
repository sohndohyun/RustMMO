use dsnet::server::{self, App};
extern crate flatbuffers;

mod protocol_generated;
pub use protocol_generated::nexus;

struct ChatServer {
    indexs: Vec<u128>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            indexs: Vec::new(),
        }
    }
}

impl server::Object for ChatServer {
    fn update(&mut self, _app: &mut App, _idx: u32) {
        
    }

    fn on_accept(&mut self, _app: &mut App, idx: u128) {
        println!("on_accept callback triggered. (idx: {})", idx);
        self.indexs.push(idx);
    }

    fn on_receive(&mut self, app: &mut App, idx: u128, packet_type: u16, payload: Vec<u8>) {
        println!("on_receive callback triggered. (idx: {})", idx);
        for value in &self.indexs {
            _ = app.send_message(*value, packet_type, payload.clone());
        }
    }

    fn on_disconnect(&mut self, _app: &mut App, idx: u128) {
        println!("on_disconnect callback triggered. (idx: {})", idx);
        if let Some(pos) = self.indexs.iter().position(|&x| x == idx) {
            self.indexs.swap_remove(pos); 
        }
    }
}


#[tokio::main]
async fn main() {
    let chat_server = ChatServer::new();
    App::new("127.0.0.1:1234".into()).run(chat_server).await
}

