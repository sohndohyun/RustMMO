use dsnet::server::{App, Callback, Session};
use std::collections::HashMap;
use std::time::Instant;

use crate::game_user::GameUser;
use crate::world::{World, WorldRequest};
use crate::single_channel::mpsc::Sender;

pub struct GameServer {
    // clients; hashset?
    users: HashMap<u64, GameUser>,

    // 원래는 여러개일수도
    world: World,
}

impl GameServer {
    pub fn run(str_addr: String) {
        let mut app = App::run(str_addr);

        let (sender, receiver) = crate::single_channel::mpsc::channel();

        let mut server = GameServer {
            users: HashMap::new(),
            world: World::new(receiver),
        };

        let mut start = Instant::now();
        loop {
            loop {
                match app.get_callback() {
                    Callback::Accept { idx, session } => server.on_accept(idx, session, sender.clone()),
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

            server.check_world_notify();
            server.remove_session();
        }
    }

    fn update(&mut self, delta_time: u32) {
        self.world.update(delta_time);
        self.world.run_command();
    }

    fn check_world_notify(&mut self) {
        for user in self.users.values_mut() {
            user.check_world_notify();
        }
    }

    fn remove_session(&mut self) {
        self.users.retain(|_, user| !user.pending_logout());
    }

    fn on_accept(&mut self, idx: u64, session: Session, sender: Sender<WorldRequest>) {
        self.users.insert(
            idx,
            GameUser::new(idx, session, sender),
        );
    }

    fn on_receive(&mut self, idx: u64, packet_type: u16, payload: Vec<u8>) {
        if let Some(user) = self.users.get_mut(&idx) {
            user.on_receive(packet_type, payload);
        }
    }

    fn on_disconnect(&mut self, idx: u64) {
        if let Some(user) = self.users.get_mut(&idx) {
            user.on_disconnect();
        }
    }
}
