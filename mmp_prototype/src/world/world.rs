use rand::rngs::ThreadRng;
use rand::Rng;
use tokio::sync::broadcast;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::protocol_generated::nexus::*;
use crate::single_channel::mpsc::{Receiver, Sender};
use crate::world::player_character::WorldPlayerCharacter;
use crate::world::world_enum::{WorldNotify, WorldRequest};

pub struct World {
    counter: u64,
    rnd: ThreadRng,

    actors: HashMap<u64, WorldPlayerCharacter>,
    user_actor_idx: HashMap<u64, u64>,

    request_receiver: Receiver<WorldRequest>,
    response_sender: HashMap<u64, Sender<Rc<RefCell<WorldNotify>>>>,
}

impl World {
    pub fn new(receiver: Receiver<WorldRequest>) -> Self {
        World {
            counter: 1,
            rnd: rand::rng(),

            actors: HashMap::new(),
            user_actor_idx: HashMap::new(),
            request_receiver: receiver,
            response_sender: HashMap::new(),
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
                WorldRequest::ChangeMoveDirection {
                    user_idx,
                    direction,
                } => self.command_change_direction(user_idx, direction),
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
        self.user_actor_idx.insert(user_idx, actor_idx);

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

    fn command_change_direction(&mut self, user_idx: u64, direction: Vec2) {
        if let Some(actor_idx) = self.user_actor_idx.get(&user_idx) {
            if let Some(character) = self.actors.get_mut(&actor_idx) {
                character.change_direction(direction);
                let position = character.position;
                self.broadcast_notify(WorldNotify::ChangeMoveDirection { actor_idx: *actor_idx, direction, position });
            }
        }
    }

    fn command_leave(&mut self, user_idx: u64) {
        // 여기선 일단 그냥 없애버릴 거임... 타이머 기능 추가하면 타이머 두고 액터 삭제 또는... 존버해서 유저 재접속하면 이어주는식으로 할거임.
        self.response_sender.remove(&user_idx);

        if let Some(actor_idx) = self.user_actor_idx.get(&user_idx) {
            self.actors.remove(&actor_idx);
            self.broadcast_notify(WorldNotify::RemoveActor { actor_idx: *actor_idx });
        }

        self.user_actor_idx.remove(&user_idx);
    }

    fn broadcast_notify(&self, notify: WorldNotify) {
        let rc_notify = Rc::new(RefCell::new(notify));
        for sender in self.response_sender.values() {
            sender.send(rc_notify.clone());
        }
    }

}
