use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::protocol_generated::nexus::*;
use crate::single_channel::mpsc::{Receiver, Sender};

pub enum WorldRequest {
    JOIN {
        hash_key: u64,
        user_idx: u64,
        name: Option<String>,
        color: Option<Color>,
        sender: Sender<Rc<RefCell<WorldNotify>>>,
    },
    CHANGE_DIRECTION {
        user_idx: u64,
        direction: Vec2,
    },
    LEAVE {
        user_idx: u64,
    },
}

pub enum WorldNotify {
    CURRENT_WORLD_INFO { hash_key:u64, characters: Vec<WorldPlayerCharacter> },
    SOMEONE_JOIN {},
}

pub struct WorldPlayerCharacter {
    actor_idx: u64,
    user_idx: u64,
    name: Option<String>,
    color: Option<Color>,
    speed: f32,
    position: Vec2,
    direction: Vec2,
}

impl WorldPlayerCharacter {
    fn new(
        actor_idx: u64,
        user_idx: u64,
        name: Option<String>,
        color: Option<Color>,
        speed: f32,
        position: Vec2,
        direction: Vec2,
    ) -> Self {
        WorldPlayerCharacter {
            actor_idx,
            user_idx,
            name,
            color,
            speed,
            position,
            direction,
        }
    }

    pub fn update(&mut self) {}
}

pub struct World {
    counter: u64,
    delta_time: u32,
    pub actors: HashMap<u64, WorldPlayerCharacter>,

    request_receiver: Receiver<WorldRequest>,
    response_sender: HashMap<u64, Sender<Rc<RefCell<WorldNotify>>>>,
}

impl World {
    pub fn new(receiver: Receiver<WorldRequest>) -> Self {
        World {
            counter: 1,
            delta_time: 0,
            actors: HashMap::new(),
            request_receiver: receiver,
            response_sender: HashMap::new(),
        }
    }

    pub fn run_command(&mut self) {
        while let Ok(command) = self.request_receiver.try_receive() {
            match command {
                WorldRequest::JOIN {
                    hash_key,
                    user_idx,
                    name,
                    color,
                    sender,
                } => self.command_join(hash_key, user_idx, name, color, sender),
                WorldRequest::CHANGE_DIRECTION {
                    user_idx,
                    direction,
                } => todo!(),
                WorldRequest::LEAVE { user_idx } => todo!(),
            }
        }
    }

    pub fn set_delta_time(&mut self, delta_time: u32) {
        self.delta_time = delta_time;
    }

    fn command_join(
        &mut self,
        hash_key: u64,
        user_idx: u64,
        name: Option<String>,
        color: Option<Color>,
        sender: Sender<Rc<Cell<WorldNotify>>>,
    ) {
        if !self.actors.contains_key(&user_idx) {
            let position = Vec2::new(0., 0.);
            let direction = Vec2::new(1., 0.);
            let speed: f32 = 100.;

        }
    }

    fn broadcast_response(&mut self, response: WorldResponse) {
        let rc_response = Rc::new(Cell::new(response));
        for sender in self.response_sender.values_mut() {
            sender.send(rc_response.clone());
        }
    }
}
