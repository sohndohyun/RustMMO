use std::sync::Arc;

use dsnet::server::Session;
use flatbuffers::FlatBufferBuilder;

use crate::protocol_generated::nexus::*;
use crate::single_channel::mpsc::{ReceiveError, Receiver, Sender};
use crate::world::{WorldNotify, WorldRequest};
use crate::{build_packet::*, single_channel};

#[derive(PartialEq)]
enum NetState {
    PendingLogin,
    Login,
    PendingLogout,
}

pub struct GameUser {
    name: Option<String>,

    net_state: NetState,
    command_sender: Sender<WorldRequest>,
    response_receiver: Option<Receiver<WorldNotify>>,

    user_idx: u64,

    session: Session,
    packet_hashes: Vec<u64>,

    builder: FlatBufferBuilder<'static>,
}

impl GameUser {
    pub fn new(session: Session, command_sender: Sender<WorldRequest>) -> Self {
        GameUser {
            name: None,
            net_state: NetState::PendingLogin,
            command_sender,
            response_receiver: None,
            user_idx: 0,
            session,
            packet_hashes: Vec::new(),

            builder: FlatBufferBuilder::with_capacity(1024),
        }
    }

    pub fn on_receive(&mut self, packet_type: u16, data: Vec<u8>) {
        match PacketType(packet_type) {
            PacketType::CG_LOGIN_REQ => self.on_cg_login_req(data),
            PacketType::CG_JOIN_REQ => self.on_cg_join_req(data),
            PacketType::CG_LOGOUT_NOTI => self.disconnect(),
            PacketType::CG_CHANGE_MOVE_DIRECTION_NOTI => {
                self.on_cg_change_move_direction_noti(data)
            }
            _ => {
                panic!("not allowed packet! {}", packet_type);
            }
        };
    }

    pub fn check_world_notify(&mut self) {
        if let Some(mut noti) = self.response_receiver.take() {
            let disconnected = self.process_notifications(&mut noti);

            self.response_receiver = if disconnected { None } else { Some(noti) };
        }
    }

    fn process_notifications(&mut self, noti: &mut Receiver<WorldNotify>) -> bool {
        loop {
            match noti.try_receive() {
                Ok(notify) => match notify {
                    WorldNotify::CurrentWorldInfo {
                        hash_key,
                        actor_idx,
                        characters,
                    } => self.notify_world_info(hash_key, actor_idx, characters),
                    WorldNotify::Broadcast {
                        packet_type,
                        packet,
                    } => self.send_packet(packet_type, packet),
                },
                Err(ReceiveError::Empty) => return false,
                Err(ReceiveError::Disconnected) => return true, // 연결 끊김 처리
            }
        }
    }

    fn send_packet(&self, packet_type: PacketType, payload: Arc<[u8]>) {
        _ = self.session.send_message(packet_type.0, payload);
    }

    fn disconnect(&mut self) {
        self.session.disconnect();
    }

    pub fn on_disconnect(&mut self) {
        self.net_state = NetState::PendingLogout;
        self.command_sender.send(WorldRequest::Leave {
            user_idx: self.user_idx,
        });
    }

    fn on_cg_login_req(&mut self, data: Vec<u8>) {
        match flatbuffers::root::<CGLoginReq>(&data) {
            Ok(req) => {
                let mut result = ServerCode::FAILED;

                if self.net_state == NetState::PendingLogin {
                    self.net_state = NetState::Login;
                    self.user_idx = req.user_idx();

                    if let Some(name) = req.name() {
                        self.name = Some(String::from(name));
                    }

                    result = ServerCode::SUCCESS;
                }

                let packet = build_gc_login_res(&mut self.builder, result);
                self.send_packet(PacketType::GC_LOGIN_RES, packet.into());
            }
            Err(e) => eprintln!("invalid_flatbuffer on_cg_login_req {:?}", e),
        }
    }

    fn on_cg_join_req(&mut self, data: Vec<u8>) {
        match flatbuffers::root::<CGJoinReq>(&data) {
            Ok(req) => {
                if self.response_receiver.is_some() {
                    eprintln!("already joined!");
                    return;
                }

                let hash_key = hash_vec_u8(&data);
                let (sender, receiver) = single_channel::mpsc::channel();
                self.response_receiver = Some(receiver);
                self.packet_hashes.push(hash_key);

                self.command_sender.send(WorldRequest::Join {
                    hash_key,
                    user_idx: self.user_idx,
                    name: self.name.clone(),
                    color: deref_color(req.color()),
                    sender,
                });
            }
            Err(e) => eprintln!("invalid_flatbuffer on_cg_join_req {:?}", e),
        }
    }

    fn on_cg_change_move_direction_noti(&mut self, data: Vec<u8>) {
        match flatbuffers::root::<CGChangeMoveDirectionNoti>(&data) {
            Ok(req) => {
                if self.response_receiver.is_none() {
                    eprintln!("not joined!");
                    return;
                }

                let direction = req.direction();
                if direction.is_none() {
                    eprintln!("no direction!");
                    return;
                }

                self.command_sender.send(WorldRequest::ChangeMoveDirection {
                    user_idx: self.user_idx,
                    direction: *direction.unwrap(),
                });
            }
            Err(e) => eprintln!("invalid_flatbuffer on_cg_change_move_direction_req {:?}", e),
        }
    }

    fn notify_world_info(&mut self, hash_key: u64, actor_idx: u64, characters: Vec<Arc<[u8]>>) {
        for character in characters {
            self.send_packet(PacketType::GC_SPAWN_ACTOR_NOTI, character.clone());
        }

        if let Some(index) = self.packet_hashes.iter().position(|&h| h == hash_key) {
            self.packet_hashes.remove(index);
            let packet = build_gc_join_res(&mut self.builder, actor_idx, ServerCode::SUCCESS);
            self.send_packet(PacketType::GC_JOIN_RES, packet.into());
        }
    }

    pub fn pending_logout(&self) -> bool {
        self.net_state == NetState::PendingLogout
    }
}
