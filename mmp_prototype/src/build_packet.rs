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

pub fn build_gc_login_res<'a>(builder: &mut FlatBufferBuilder<'a>, result: ServerCode) -> Vec<u8> {
    builder.reset();

    let root = GCLoginRes::create(builder, &GCLoginResArgs { result });

    builder.finish(root, None);
    builder.finished_data().to_vec()
}

pub fn build_gc_join_res<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    actor_idx: u64,
    result: ServerCode,
) -> Vec<u8> {
    builder.reset();

    let root = GCJoinRes::create(builder, &GCJoinResArgs { actor_idx, result });

    builder.finish(root, None);
    builder.finished_data().to_vec()
}

pub fn build_gc_spawn_character_noti<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    actor_idx: u64,
    name: Option<String>,
    color: &Option<Color>,
    speed: f32,
    position: &Vec2,
    direction: &Vec2,
) -> Vec<u8> {
    builder.reset();

    let name_fb = name.as_ref().map(|n| builder.create_string(n));

    let root = GCSpawnCharacterNoti::create(
        builder,
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
    builder.finished_data().to_vec()
}

pub fn build_gc_remove_actor_noti<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    actor_idx: u64,
) -> Vec<u8> {
    builder.reset();
    let root = GCRemoveActorNoti::create(builder, &GCRemoveActorNotiArgs { actor_idx });

    builder.finish(root, None);
    builder.finished_data().to_vec()
}

pub fn build_gc_change_move_direction_noti<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    actor_idx: u64,
    direction: &Vec2,
    position: &Vec2,
) -> Vec<u8> {
    builder.reset();

    let root = GCChangeMoveDirectionNoti::create(
        builder,
        &GCChangeMoveDirectionNotiArgs {
            actor_idx,
            direction: Some(direction),
            position: Some(position),
        },
    );

    builder.finish(root, None);
    builder.finished_data().to_vec()
}
