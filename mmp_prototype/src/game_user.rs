use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::build_packet::*;
use crate::game_server::{GameServer, ToServerApp};
use crate::protocol_generated::nexus::*;
use crate::world::{World, WorldPlayerCharacter};

#[derive(PartialEq)]
enum NetState {
    PendingLogin,
    Login,
    PendingLogout,
}

pub struct GameUser {
    idx: u64,
    name: Option<String>,

    net_state: NetState,
    weak_world: Weak<RefCell<World>>,
    actor: Weak<RefCell<WorldPlayerCharacter>>,

    send_to_server: std::sync::mpsc::Sender<ToServerApp>,
    self_ref: Weak<RefCell<GameUser>>,
}

impl GameUser {
    pub fn new(
        idx: u64,
        send_to_server: std::sync::mpsc::Sender<ToServerApp>,
        weak_world: Weak<RefCell<World>>,
    ) -> Rc<RefCell<Self>> {
        let rc_user = Rc::new(RefCell::new(GameUser {
            idx,
            name: None,
            send_to_server,
            net_state: NetState::PendingLogin,
            weak_world,
            actor: Weak::new(),
            self_ref: Weak::new(),
        }));

        rc_user.borrow_mut().self_ref = Rc::downgrade(&rc_user);

        rc_user
    }

    pub fn on_packet(&mut self, packet_type: u16, payload: Vec<u8>) {
        match PacketType(packet_type) {
            PacketType::CG_LOGIN_REQ => self.on_cg_login_req(payload),
            _ => {
                panic!("not allowed packet! {}", packet_type);
            }
        };
    }

    pub fn send_packet(&self, packet_type: PacketType, payload: Vec<u8>) {
        let _ = self.send_to_server.send(ToServerApp::Send {
            idx: self.idx,
            packet_type: packet_type.0,
            payload,
        });
    }

    pub fn on_disconnect(&mut self) {
        // todo: pending_logout가 되면 world update에서 지워줘야함
        self.net_state = NetState::PendingLogout;
    }

    fn on_cg_login_req(&mut self, data: Vec<u8>) {
        match flatbuffers::root::<CGLoginReq>(&data) {
            Ok(req) => {
                let mut result = ServerCode::FAILED;
                let mut actor_idx: u64 = 0;

                if self.net_state == NetState::PendingLogin {
                    if let Some(world) = self.weak_world.upgrade() {
                        self.actor = world.borrow_mut().spawn_player_character( // spawn character
                            self.name.clone().unwrap(),
                            *req.color().unwrap(),
                            self.self_ref.clone(),
                        );

                        actor_idx = self.actor.upgrade().unwrap().borrow().actor_idx;
                        result = ServerCode::SUCCESS;
                    }
                }

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
