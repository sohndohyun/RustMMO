use crate::protocol_generated::nexus::*;
use crate::build_packet::build_gc_spawn_character_noti;

pub struct WorldPlayerCharacter {
    user_idx: u64,
    actor_idx: u64,
    name: Option<String>,
    color: Option<Color>,
    speed: f32,
    position: Vec2,
    direction: Vec2,
}

impl WorldPlayerCharacter {
    pub fn new(
        user_idx: u64,
        actor_idx: u64,
        name: Option<String>,
        color: Option<Color>,
        speed: f32,
        position: Vec2,
        direction: Vec2,
    ) -> Self {
        WorldPlayerCharacter {
            user_idx,
            actor_idx,
            name,
            color,
            speed,
            position,
            direction,
        }
    }

    pub fn update(&mut self) {}

    pub fn into_spawn_noti_vec(&self) -> Vec<u8> {
        build_gc_spawn_character_noti(self.actor_idx, self.name.clone(), &self.color, self.speed, &self.position, &self.direction)
    }
}
