use dsnet::server;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use crate::game_user::GameUser;

pub struct GameServer {
    app: Rc<RefCell<server::App>>,
    // clients; hashset?
    users: HashMap<u64, GameUser>, // worlds
                                       //  - worlds가 clients index를 vector로 들고있겠죠?
}

impl GameServer {
    pub fn new() -> Rc<RefCell<Self>> {
        let game_server = Rc::new(RefCell::new(GameServer {
            app: Rc::new(RefCell::new(server::App::new())),
            users: HashMap::new(),
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server.borrow_mut().app.borrow_mut().set_update_callback(Box::new(move |delta_time| {
            if let Some(game_server) = weak_game_server.upgrade() {
                game_server.borrow_mut().update(delta_time);
            }
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server.borrow_mut().app.borrow_mut().set_on_accept_callback(Box::new(move |idx| {
            if let Some(game_server) = weak_game_server.upgrade() {
                game_server.borrow_mut().on_accept(idx);
            }
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server.borrow_mut().app.borrow_mut().set_on_receive_callback(Box::new(move |idx, packet_type, payload| {
            if let Some(game_server) = weak_game_server.upgrade() {
                game_server.borrow_mut().on_receive(idx, packet_type, payload);
            }
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server.borrow_mut().app.borrow_mut().set_on_disconnect_callback(Box::new(move |idx| {
            if let Some(game_server) = weak_game_server.upgrade() {
                game_server.borrow_mut().on_disconnect(idx);
            }
        }));

        game_server
    }

    fn update(&mut self, _delta_time: u32) {

    }

    fn on_accept(&mut self, idx: u64) {
        println!("on_accept callback triggered. (idx: {})", idx);

        self.users.insert(idx, GameUser::new(idx, Rc::downgrade(&self.app)));
    }

    fn on_receive(&mut self, idx: u64, packet_type: u16, payload: Vec<u8>) {
        if let Some(user) = self.users.get_mut(&idx) {
            user.on_packet(packet_type, payload);
        }
    }

    fn on_disconnect(&mut self, idx: u64) {
        println!("on_disconnect callback triggered. (idx: {})", idx);

        // start disconnect...
        
    }

    pub async fn run(&mut self) {
        self.app.borrow_mut().run("127.0.0.1:1234".into()).await
    }
}