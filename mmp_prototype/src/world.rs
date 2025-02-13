use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use crate::build_packet::*;
use crate::game_user::GameUser;
use crate::protocol_generated::nexus::*;

pub struct WorldPlayerCharacter {
    pub actor_idx: u64,
    name: String,
    color: Color,
    speed: f32,
    position: Vec2,
    direction: Vec2,
    weak_game_user: Weak<RefCell<GameUser>>,
}

impl WorldPlayerCharacter {
    fn new(
        actor_idx: u64,
        name: String,
        color: Color,
        position: Vec2,
        direction: Vec2,
        weak_game_user: Weak<RefCell<GameUser>>,
    ) -> Self {
        WorldPlayerCharacter {
            actor_idx,
            name,
            color,
            speed: 10.,
            position,
            direction,
            weak_game_user,
        }
    }

    pub fn update(&mut self) {

    }

    fn send_packet(&self, packet_type: PacketType, payload: Vec<u8>) {
        if let Some(game_user) = self.weak_game_user.upgrade() {
            game_user.borrow().send_packet(packet_type, payload);
        }
    }
}

pub struct World {
    counter: u64,
    pub delta_time: u32,
    pub player_characters: HashMap<u64, Rc<RefCell<WorldPlayerCharacter>>>,
}

impl World {
    pub fn new() -> Self {
        World {
            counter: 0,
            delta_time: 0,
            player_characters: HashMap::new(),
        }
    }

    pub fn update(&mut self) {

    }

    pub fn spawn_player_character(
        &mut self,
        name: String,
        color: Color,
        weak_game_user: Weak<RefCell<GameUser>>,
    ) -> Weak<RefCell<WorldPlayerCharacter>> {
        let actor_idx = self.counter;
        let position = Vec2::new(0., 0.);
        let direction = Vec2::new(1., 0.);


        let player_character = Rc::new(RefCell::new(WorldPlayerCharacter::new(
            actor_idx,
            name,
            color,
            position,
            direction,
            weak_game_user,
        )));

        let pc = player_character.borrow();
        self.broadcast(
            PacketType::GC_SPAWN_ACTOR_NOTI,
            build_gc_spawn_actor_noti(
                pc.actor_idx,
                &pc.name,
                &pc.color,
                pc.speed,
                &pc.position,
                &pc.direction,
            ),
        );

        self.player_characters.insert(actor_idx, player_character.clone());
        self.counter += 1;

        Rc::downgrade(&player_character)
    }

    pub fn broadcast(&self, packet_type: PacketType, payload: Vec<u8>) {
        for pc in self.player_characters.values() {
            pc.borrow().send_packet(packet_type, payload.to_vec());
        }
    }
}
