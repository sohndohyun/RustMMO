use crate::protocol_generated::nexus::*;
use flatbuffers::FlatBufferBuilder;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_vec_u8(data: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

pub fn deref_color(opt_color: Option<&Color>) -> Option<Color> {
    match opt_color {
        Some(color) => Some(*color),
        None => None,
    }
}

pub fn build_gc_login_res(result: ServerCode) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(32);

    let root = GCLoginRes::create(&mut builder, &GCLoginResArgs { result });

    builder.finish(root, None);
    builder.collapse().0 // offset 불필요, 무시 가능
}

pub fn build_gc_join_res(actor_idx: u64, result: ServerCode) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(32);

    let root = GCJoinRes::create(&mut builder, &GCJoinResArgs { actor_idx, result });

    builder.finish(root, None);
    builder.collapse().0 // offset 불필요, 무시 가능
}

pub fn build_gc_spawn_character_noti(
    actor_idx: u64,
    name: Option<String>,
    color: &Option<Color>,
    speed: f32,
    position: &Vec2,
    direction: &Vec2,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(256);

    let name_fb = name.as_ref().map(|n| builder.create_string(n));

    let root = GCSpawnCharacterNoti::create(
        &mut builder,
        &GCSpawnCharacterNotiArgs {
            actor_idx,
            name: name_fb,
            color: color.as_ref(),
            speed,
            position: Some(position),
            direction: Some(direction),
        },
    );

    builder.finish(root, None);
    builder.collapse().0
}

pub fn build_gc_remove_actor_noti(actor_idx: u64) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(16);
    let root = GCRemoveActorNoti::create(&mut builder, &GCRemoveActorNotiArgs { actor_idx });

    builder.finish(root, None);
    builder.collapse().0
}

pub fn build_gc_change_move_direction_noti(
    actor_idx: u64,
    direction: &Vec2,
    position: &Vec2,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(128);

    let root = GCChangeMoveDirectionNoti::create(
        &mut builder,
        &GCChangeMoveDirectionNotiArgs {
            actor_idx,
            direction: Some(direction),
            position: Some(position),
        },
    );

    builder.finish(root, None);
    builder.collapse().0
}
