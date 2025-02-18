use std::cell::RefCell;
use std::rc::Rc;

use dsnet::server::Session;

use crate::{build_packet::*, single_channel};
use crate::protocol_generated::nexus::*;
use crate::single_channel::mpsc::{Receiver, Sender, ReceiveError};
use crate::world::{WorldPlayerCharacter, WorldNotify, WorldRequest};

#[derive(PartialEq)]
enum NetState {
    PendingLogin,
    Login,
    PendingLogout,
}

pub struct GameUser {
    session_idx: u64,
    name: Option<String>,

    net_state: NetState,
    command_sender: Sender<WorldRequest>,
    response_receiver: Option<Receiver<Rc<RefCell<WorldNotify>>>>,

    user_idx: u64,

    session: Session,
    packet_hashes: Vec<u64>,
}

impl GameUser {
    pub fn new(idx: u64, session: Session, command_sender: Sender<WorldRequest>) -> Self {
        GameUser {
            session_idx: idx,
            name: None,
            net_state: NetState::PendingLogin,
            command_sender,
            response_receiver: None,
            user_idx: 0,
            session,
            packet_hashes: Vec::new(),
        }
    }

    pub fn on_receive(&mut self, packet_type: u16, data: Vec<u8>) {
        match PacketType(packet_type) {
            PacketType::CG_LOGIN_REQ => self.on_cg_login_req(data),
            PacketType::CG_JOIN_REQ => self.on_cg_join_req(data),
            PacketType::CG_LOGOUT_NOTI => self.disconnect(),
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
    
    fn process_notifications(&mut self, noti: &mut Receiver<Rc<RefCell<WorldNotify>>>) -> bool {
        loop {
            match noti.try_receive() {
                Ok(rc_notify) => self.handle_notification(rc_notify),
                Err(ReceiveError::Empty) => return false,
                Err(ReceiveError::Disconnected) => return true, // 연결 끊김 처리
            }
        }
    }
    
    fn handle_notification(&mut self, rc_notify: Rc<RefCell<WorldNotify>>) {
        match &*rc_notify.borrow() {
            WorldNotify::CurrentWorldInfo { hash_key, actor_idx, characters } => {
                self.notify_world_info(hash_key, actor_idx, characters);
            }
            WorldNotify::SomeoneJoin { character } => {
                self.notify_someone_join(character);
            }
        }
    }
    

    fn send_packet(&self, packet_type: PacketType, payload: Vec<u8>) {
        _ = self.session.send_message(packet_type.0, payload);
    }

    fn disconnect(&mut self) {
        self.session.disconnect();
    }

    pub fn on_disconnect(&mut self) {
        self.net_state = NetState::PendingLogout;
        self.command_sender.send(WorldRequest::Leave { user_idx: self.user_idx });
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

                self.send_packet(
                    PacketType::GC_LOGIN_RES,
                    build_gc_login_res(result),
                );
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

    fn notify_world_info(&mut self, hash_key: &u64, actor_idx: &u64, characters: &Vec<Vec<u8>>) {
        for character in characters {
            self.send_packet(PacketType::GC_SPAWN_ACTOR_NOTI, character.to_vec());
        }

        if let Some(index) = self.packet_hashes.iter().position(|&h| h == *hash_key) {
            self.packet_hashes.remove(index);
            self.send_packet(
                PacketType::GC_JOIN_RES,
                build_gc_join_res(*actor_idx, ServerCode::SUCCESS),
            );
        }
    }

    fn notify_someone_join(&mut self, character: &Vec<u8>) {
        self.send_packet(PacketType::GC_SPAWN_ACTOR_NOTI, character.to_vec());
    }

    pub fn pending_logout(&self) -> bool {
        self.net_state == NetState::PendingLogout
    }
}
