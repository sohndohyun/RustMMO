use crate::protocol_generated::nexus::*;
use flatbuffers::FlatBufferBuilder;

pub fn build_gc_login_res<'a>(
    builder: &'a mut FlatBufferBuilder,
    actor_idx: u64,
    result: ServerCode,
) -> &'a [u8] {
    builder.reset();
    let root = GCLoginRes::create(builder, &GCLoginResArgs { actor_idx, result });

    builder.finish(root, None);
    builder.finished_data()
}

pub fn build_gc_spawn_actor_noti<'a>(
    builder: &'a mut FlatBufferBuilder,
    actor_idx: u64,
    color: Color,
    position: Vec2,
) -> &'a [u8] {
    builder.reset();

    let root = GCSpawnActorNoti::create(
        builder,
        &GCSpawnActorNotiArgs {
            actor_idx,
            color: Some(&color),
            position: Some(&position),
        },
    );

    builder.finish(root, None);
    builder.finished_data()
}

pub fn build_gc_change_move_direction_res<'a>(
    builder: &'a mut FlatBufferBuilder,
    result: ServerCode,
) -> &'a [u8] {
    builder.reset();
    let root = GCChangeMoveDirectionRes::create(builder, &GCChangeMoveDirectionResArgs { result });

    builder.finish(root, None);
    builder.finished_data()
}

pub fn build_gc_change_actor_direction_noti<'a>(
    builder: &'a mut FlatBufferBuilder,
    actor_idx:u64,
    direction:Vec2,
    position:Vec2,
) -> &'a [u8] {
    builder.reset();

    let root = GCChangeActorDirectionNoti::create(
        builder,
        &GCChangeActorDirectionNotiArgs {
            actor_idx,
            direction: Some(&direction),
            position: Some(&position),
        },
    );

    builder.finish(root, None);
    builder.finished_data()
}
