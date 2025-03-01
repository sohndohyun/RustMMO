use flatbuffers::FlatBufferBuilder;

use crate::protocol_generated::nexus::*;
use crate::build_packet::build_gc_spawn_character_noti;

pub struct WorldPlayerCharacter {
    pub actor_idx: u64,
    name: Option<String>,
    pub color: Option<Color>,
    speed: f32,
    pub position: Vec2,
    direction: Vec2,
}

impl WorldPlayerCharacter {
    pub fn new(
        actor_idx: u64,
        name: Option<String>,
        color: Option<Color>,
        speed: f32,
        position: Vec2,
        direction: Vec2,
    ) -> Self {
        WorldPlayerCharacter {
            actor_idx,
            name,
            color,
            speed,
            position,
            direction,
        }
    }

    pub fn change_direction(&mut self, direction: Vec2) {
        self.direction.set_x(direction.x());
        self.direction.set_y(direction.y());
    }

    pub fn update_position(&mut self, delta_time: u32) {
        if self.direction.x() == 0. && self.direction.y() == 0. {
            return;
        }

        let delta_time: f32 = delta_time as f32 * 0.001;

        let mut x = self.position.x();
        let mut y = self.position.y();

        x += self.direction.x() * self.speed * delta_time;
        y += self.direction.y() * self.speed * delta_time;

        self.position.set_x(x);
        self.position.set_y(y);
    }

    pub fn into_spawn_noti_vec<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> Vec<u8> {
        build_gc_spawn_character_noti(builder, self.actor_idx, self.name.clone(), &self.color, self.speed, &self.position, &self.direction)
    }
}
