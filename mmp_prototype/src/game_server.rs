use dsnet::server;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use crate::game_user::GameUser;
use crate::world::World;

pub struct GameServer {
    app: Rc<RefCell<server::App>>,
    // clients; hashset?
    users: HashMap<u64, Rc<RefCell<GameUser>>>,
    self_ref: Weak<RefCell<Self>>,

    // 원래는 여러개일수도
    world: Rc<RefCell<World>>,
}

impl GameServer {
    pub fn new() -> Rc<RefCell<Self>> {
        let game_server = Rc::new(RefCell::new(GameServer {
            app: Rc::new(RefCell::new(server::App::new())),
            users: HashMap::new(),
            self_ref: Weak::new(),
            world: Rc::new(RefCell::new(World::new())),
        }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server
            .borrow_mut()
            .app
            .borrow_mut()
            .set_update_callback(Box::new(move |delta_time| {
                if let Some(game_server) = weak_game_server.upgrade() {
                    game_server.borrow_mut().update(delta_time);
                }
            }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server
            .borrow_mut()
            .app
            .borrow_mut()
            .set_on_accept_callback(Box::new(move |idx| {
                if let Some(game_server) = weak_game_server.upgrade() {
                    game_server.borrow_mut().on_accept(idx);
                }
            }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server
            .borrow_mut()
            .app
            .borrow_mut()
            .set_on_receive_callback(Box::new(move |idx, packet_type, payload| {
                if let Some(game_server) = weak_game_server.upgrade() {
                    game_server
                        .borrow_mut()
                        .on_receive(idx, packet_type, payload);
                }
            }));

        let weak_game_server = Rc::downgrade(&game_server);
        game_server
            .borrow_mut()
            .app
            .borrow_mut()
            .set_on_disconnect_callback(Box::new(move |idx| {
                if let Some(game_server) = weak_game_server.upgrade() {
                    game_server.borrow_mut().on_disconnect(idx);
                }
            }));

        game_server.borrow_mut().world.borrow_mut().set_game_server(Rc::downgrade(&game_server));
        game_server.borrow_mut().self_ref = Rc::downgrade(&game_server);
        game_server
    }

    fn update(&mut self, _delta_time: u32) {}

    fn on_accept(&mut self, idx: u64) {
        if let Some(server) = self.self_ref.upgrade() {
            self.users.insert(idx, GameUser::new(idx, Rc::downgrade(&server), Rc::downgrade(&self.world)));
        }
    }

    fn on_receive(&mut self, idx: u64, packet_type: u16, payload: Vec<u8>) {
        if let Some(user) = self.users.get_mut(&idx) {
            user.borrow_mut().on_packet(packet_type, payload);
        }
    }

    fn on_disconnect(&mut self, idx: u64) {
        if let Some(user) = self.users.get_mut(&idx) {
            user.borrow_mut().on_disconnect();
        }
    }

    pub async fn run(&mut self) {
        self.app.borrow_mut().run("127.0.0.1:1234".into()).await
    }

    pub fn send_packet(&mut self, idx: u64, packet_type: u16, payload: Vec<u8>) {
        self.app
            .borrow_mut()
            .send_message(idx, packet_type, payload)
            .unwrap_or_else(|e| eprintln!("{:?}", e))
    }
}
