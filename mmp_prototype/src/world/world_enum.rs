use std::cell::RefCell;
use std::rc::Rc;

use crate::protocol_generated::nexus::*;
use crate::single_channel::mpsc::Sender;

pub enum WorldRequest {
    Join {
        hash_key: u64,
        user_idx: u64,
        name: Option<String>,
        color: Option<Color>,
        sender: Sender<Rc<RefCell<WorldNotify>>>,
    },
    ChangeDirection {
        user_idx: u64,
        direction: Vec2,
    },
    Leave {
        user_idx: u64,
    },
}

pub enum WorldNotify {
    CurrentWorldInfo { hash_key:u64, actor_idx: u64, characters: Vec<Vec<u8>> },
    SomeoneJoin { character: Vec<u8> },
}
