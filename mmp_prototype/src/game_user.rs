use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::build_packet::*;
use crate::game_server::GameServer;
use crate::protocol_generated::nexus::*;
use crate::world::{World, WorldPlayerCharacter};

#[derive(PartialEq)]
enum NetState {
    PendingLogin,
    Login,
    PendingLogout,
    Logout,
}

pub struct GameUser {
    idx: u64,
    name: Option<String>,
    weak_game_server: Weak<RefCell<GameServer>>,
    net_state: NetState,
    weak_world: Weak<RefCell<World>>,
    actor: Weak<RefCell<WorldPlayerCharacter>>,
    self_ref: Weak<RefCell<Self>>,
}

impl GameUser {
    pub fn new(
        idx: u64,
        weak_game_server: Weak<RefCell<GameServer>>,
        weak_world: Weak<RefCell<World>>,
    ) -> Rc<RefCell<Self>> {
        let game_user = Rc::new(RefCell::new(GameUser {
            idx,
            name: None,
            weak_game_server,
            net_state: NetState::PendingLogin,
            weak_world,
            actor: Weak::new(),
            self_ref: Weak::new(),
        }));

        game_user.borrow_mut().self_ref = Rc::downgrade(&game_user);
        game_user
    }

    pub fn on_packet(&mut self, packet_type: u16, payload: Vec<u8>) {
        match PacketType(packet_type) {
            PacketType::CG_LOGIN_REQ => self.on_cg_login_req(payload),
            _ => {
                panic!("not allowed packet! {}", packet_type);
            }
        };
    }

    pub fn send_packet(&mut self, packet_type: PacketType, payload: Vec<u8>) {
        if let Some(game_server) = self.weak_game_server.upgrade() {
            game_server
                .borrow_mut()
                .send_packet(self.idx, packet_type.0, payload);
        }
    }

    pub fn on_disconnect(&mut self) {
        self.net_state = NetState::PendingLogout;
    }

    fn on_cg_login_req(&mut self, data: Vec<u8>) {
        match flatbuffers::root::<CGLoginReq>(&data) {
            Ok(req) => {
                let mut result = ServerCode::FAILED;

                if self.net_state == NetState::PendingLogin {
                    self.net_state = NetState::Login;
                    self.name = Some(req.name().unwrap().into());

                    if let Some(world) = self.weak_world.upgrade() {
                        world.borrow_mut().spawn_player_character(
                            self.name.clone().unwrap(),
                            *req.color().unwrap(),
                            self.self_ref.clone(),
                        );
                    }
                    // 여기서 world spawn 로직 시작해야함.
                    result = ServerCode::SUCCESS;
                }

                let actor_idx: u64 = 0;

                // send gc_login_res
                self.send_packet(
                    PacketType::GC_LOGIN_RES,
                    build_gc_login_res(actor_idx, result),
                );
            }
            Err(e) => eprintln!("invalid_flatbuffer on_cg_login_req {:?}", e),
        }
    }
}
