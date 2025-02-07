use std::cell::RefCell;
use std::rc::Weak;
use dsnet::server::App;
use crate::protocol_generated::nexus;

enum NetState {
    PendingLogin,
    Login,
    PendingLogout,
    Logout,
}

pub struct GameUser {
    idx: u64,
    server_app: Weak<RefCell<App>>,
    net_state: NetState,
    actor_idx: u64,
    color: nexus::Color,
}

impl GameUser {
    pub fn new(idx: u64, server_app: Weak<RefCell<App>>) -> Self {
        GameUser {
            idx,
            server_app,
            net_state: NetState::PendingLogin,
            actor_idx: 0,
            color: nexus::Color([0, 0, 0]),
        }
    }

    pub fn on_packet(&mut self, packet_type: u16, payload: Vec<u8>) {
        match nexus::PacketType(packet_type) {
            nexus::PacketType::CG_LOGIN_REQ => {
                let req = flatbuffers::root::<nexus::CGLoginReq>(&payload).unwrap();

            },
            _ => {
                panic!("not allowed packet! {}", packet_type);
            }
        };
    }
}