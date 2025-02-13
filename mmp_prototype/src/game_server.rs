use dsnet::server::{App, Callback};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use crate::game_user::GameUser;
use crate::world::World;

pub enum ToServerApp {
    Send {
        idx: u64,
        packet_type: u16,
        payload: Vec<u8>,
    },
    Disconnect {
        idx: u64,
    },
}

pub struct GameServer {
    // clients; hashset?
    users: HashMap<u64, Rc<RefCell<GameUser>>>,

    // 원래는 여러개일수도
    rc_world: Rc<RefCell<World>>,
}

impl GameServer {
    pub fn run(str_addr: String) {
        let mut app = App::run(str_addr);
        let mut server = GameServer {
            users: HashMap::new(),
            rc_world: Rc::new(RefCell::new(World::new())),
        };

        let (sender, receiver) = std::sync::mpsc::channel();

        let mut start = Instant::now();
        loop {
            loop {
                match app.get_callback() {
                    Callback::Accept { idx } => server.on_accept(idx, sender.clone()),
                    Callback::Receive {
                        idx,
                        packet_type,
                        message,
                    } => server.on_receive(idx, packet_type, message),
                    Callback::Disconnect { idx } => server.on_disconnect(idx),
                    Callback::Empty => break,
                    Callback::Close => return,
                }
            }

            server.update(start.elapsed().as_millis() as u32);
            start = Instant::now();

            while let Ok(cmd) = receiver.try_recv() {
                if let Err(e) = match cmd {
                    ToServerApp::Send {
                        idx,
                        packet_type,
                        payload,
                    } => app.send_message(idx, packet_type, payload),
                    ToServerApp::Disconnect { idx } => app.disconnect(idx),
                } {
                    eprintln!("{}", e);
                    return;
                }
            }
        }
    }

    fn update(&mut self, delta_time: u32) {

        {
            let mut world = self.rc_world.borrow_mut();
            world.delta_time = delta_time;
            world.update();
        }

        for rc_character in self.rc_world.borrow().player_characters.values() {
            rc_character.borrow_mut().update();
        }
    }

    fn on_accept(&mut self, idx: u64, sender: std::sync::mpsc::Sender<ToServerApp>) {
        self.users.insert(
            idx,
            GameUser::new(idx, sender, Rc::downgrade(&self.rc_world)),
        );
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
}
