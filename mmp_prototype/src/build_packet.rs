use crate::protocol_generated::nexus::*;
use flatbuffers::FlatBufferBuilder;

pub fn build_gc_login_res(actor_idx: u64, result: ServerCode) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(64);
    
    let root = GCLoginRes::create(&mut builder, &GCLoginResArgs { actor_idx, result });

    builder.finish(root, None);
    builder.collapse().0 // offset 불필요, 무시 가능
}

pub fn build_gc_spawn_actor_noti(actor_idx: u64, color: &Color, speed: f32, position: &Vec2, direction: &Vec2) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(128);

    let root = GCSpawnActorNoti::create(
        &mut builder,
        &GCSpawnActorNotiArgs {
            actor_idx,
            color: Some(color),
            speed,
            position: Some(position),
            direction: Some(direction),
        },
    );

    builder.finish(root, None);
    builder.collapse().0
}

pub fn build_gc_change_move_direction_res(result: ServerCode) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(32);

    let root = GCChangeMoveDirectionRes::create(&mut builder, &GCChangeMoveDirectionResArgs { result });

    builder.finish(root, None);
    builder.collapse().0
}

pub fn build_gc_change_actor_direction_noti(
    actor_idx: u64,
    direction: &Vec2,
    position: &Vec2,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::with_capacity(128);

    let root = GCChangeActorDirectionNoti::create(
        &mut builder,
        &GCChangeActorDirectionNotiArgs {
            actor_idx,
            direction: Some(direction),
            position: Some(position),
        },
    );

    builder.finish(root, None);
    builder.collapse().0
}
