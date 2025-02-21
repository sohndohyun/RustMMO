use crate::protocol_generated::nexus::*;
use flatbuffers::FlatBufferBuilder;

pub fn build_cg_login_req<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    user_idx: u64,
    name: String,
) -> Vec<u8> {
    builder.reset();
    let name_offset = builder.create_string(&name);
    let root = CGLoginReq::create(
        builder,
        &CGLoginReqArgs {
            user_idx,
            name: Some(name_offset),
        },
    );

    builder.finish(root, None);
    builder.finished_data().to_vec()
}

pub fn build_cg_join_req<'a>(builder: &mut FlatBufferBuilder<'a>, color: Color) -> Vec<u8> {
    builder.reset();
    let root = CGJoinReq::create(
        builder,
        &CGJoinReqArgs {
            color: Some(&color),
        },
    );

    builder.finish(root, None);
    builder.finished_data().to_vec()
}

pub fn build_cg_change_move_direction_noti<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    direction: Vec2,
) -> Vec<u8> {
    builder.reset();
    let root = CGChangeMoveDirectionNoti::create(
        builder,
        &CGChangeMoveDirectionNotiArgs { direction: Some(&direction) },
    );
    
    builder.finish(root, None);
    builder.finished_data().to_vec()
}
