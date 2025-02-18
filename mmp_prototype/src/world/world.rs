use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;
use std::rc::Rc;

use crate::protocol_generated::nexus::*;
use crate::single_channel::mpsc::{Receiver, Sender};
use crate::world::player_character::WorldPlayerCharacter;
use crate::world::world_enum::{WorldNotify, WorldRequest};

pub struct World {
    counter: u64,
    rnd: ThreadRng,
    actors: HashMap<u64, WorldPlayerCharacter>,

    request_receiver: Receiver<WorldRequest>,
    response_sender: HashMap<u64, Sender<Rc<RefCell<WorldNotify>>>>,
}

impl World {
    pub fn new(receiver: Receiver<WorldRequest>) -> Self {
        World {
            counter: 1,
            actors: HashMap::new(),
            request_receiver: receiver,
            response_sender: HashMap::new(),
            rnd: rand::rng(),
        }
    }

    pub fn run_command(&mut self) {
        while let Ok(command) = self.request_receiver.try_receive() {
            match command {
                WorldRequest::Join {
                    hash_key,
                    user_idx,
                    name,
                    color,
                    sender,
                } => self.command_join(hash_key, user_idx, name, color, sender),
                WorldRequest::ChangeDirection {
                    user_idx,
                    direction,
                } => todo!(),
                WorldRequest::Leave { user_idx } => self.command_leave(user_idx),
            }
        }
    }

    fn alloc_actor_idx(&mut self) -> u64 {
        let temp = self.counter;
        self.counter += 1;
        temp
    }

    pub fn update(&mut self, delta_time: u32) {}

    fn command_join(
        &mut self,
        hash_key: u64,
        user_idx: u64,
        name: Option<String>,
        color: Option<Color>,
        sender: Sender<Rc<RefCell<WorldNotify>>>,
    ) {
        if !self.response_sender.contains_key(&user_idx) {
            return;
        }

        let position = Vec2::new(
            self.rnd.random_range(-100.0..100.0),
            self.rnd.random_range(-100.0..100.0),
        );
        let direction = Vec2::new(0., 0.);
        let speed: f32 = 10.;
        let actor_idx = self.alloc_actor_idx();

        let character =
            WorldPlayerCharacter::new(user_idx, actor_idx, name, color, speed, position, direction);

        self.broadcast_notify(WorldNotify::SomeoneJoin {
            character: character.into_spawn_noti_vec(),
        });
        self.actors.insert(actor_idx, character);

        sender.send(Rc::new(RefCell::new(WorldNotify::CurrentWorldInfo {
            hash_key,
            actor_idx,
            characters: self
                .actors
                .values()
                .map(|actor| actor.into_spawn_noti_vec())
                .collect(),
        })));
        self.response_sender.insert(user_idx, sender);
    }

    fn command_leave(&mut self, user_idx: u64) {
        todo!()
    }

    fn broadcast_notify(&self, notify: WorldNotify) {
        let rc_notify = Rc::new(RefCell::new(notify));
        for sender in self.response_sender.values() {
            sender.send(rc_notify.clone());
        }
    }
}
