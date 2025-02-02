use dsnet::server;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

struct GameUser {}

struct GameServer {
    app: server::App,
    // clients; hashset?
    sessions: HashMap<u128, GameUser>, // worlds
                                       //  - worlds가 clients index를 vector로 들고있겠죠?
}

impl GameServer {
    pub fn new() -> Rc<RefCell<Self>> {
        let game_server = Rc::new(RefCell::new(GameServer {
            app: server::App::new(),
            sessions: HashMap::new(),
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server.borrow_mut().app.set_update_callback(Box::new(move |delta_time| {
            if let Some(game_server) = weak_game_server.upgrade() {
                game_server.borrow_mut().update(delta_time);
            }
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server.borrow_mut().app.set_on_accept_callback(Box::new(move |idx| {
            if let Some(game_server) = weak_game_server.upgrade() {
                game_server.borrow_mut().on_accept(idx);
            }
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server.borrow_mut().app.set_on_receive_callback(Box::new(move |idx, packet_type, payload| {
            if let Some(game_server) = weak_game_server.upgrade() {
                game_server.borrow_mut().on_receive(idx, packet_type, payload);
            }
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server.borrow_mut().app.set_on_disconnect_callback(Box::new(move |idx| {
            if let Some(game_server) = weak_game_server.upgrade() {
                game_server.borrow_mut().on_disconnect(idx);
            }
        }));

        game_server
    }

    fn update(&mut self, _delta_time: u32) {}

    fn on_accept(&mut self, idx: u128) {
        println!("on_accept callback triggered. (idx: {})", idx);
    }

    fn on_receive(&mut self, idx: u128, packet_type: u16, payload: Vec<u8>) {
        println!("on_receive callback triggered. (idx: {})", idx);
    }

    fn on_disconnect(&mut self, idx: u128) {
        println!("on_disconnect callback triggered. (idx: {})", idx);
    }

    pub async fn run(&mut self) {
        self.app.run("127.0.0.1:1234".into()).await
    }
}

#[tokio::main]
async fn main() {
    let game_server = GameServer::new();
    game_server.borrow_mut().run().await;
}
