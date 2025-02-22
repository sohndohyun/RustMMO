mod build_packet;
mod protocol_generated;

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::Duration,
};

use crate::build_packet::*;
use dsnet::client::{App, Callback};
use flatbuffers::FlatBufferBuilder;
use protocol_generated::nexus::{Color, GCJoinRes, GCLoginRes, PacketType, ServerCode, Vec2};
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};
use tokio::{task::JoinSet, time::sleep};

const AGENT_COUNT: u32 = 500;
const FRAME_TIME: Duration = Duration::from_millis(8);

fn hash_vec_u8(data: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

fn random_direction(rng: &mut SmallRng) -> Vec2 {
    let x = rng.random_range(-1..=1);
    let y = rng.random_range(-1..=1);

    let x = x as f32;
    let y = y as f32;

    let distance = f32::sqrt( x * x + y * y);
    if distance == 0. {
        Vec2::new(x, y)
    } else {
        Vec2::new(x / distance, y / distance)
    }

}

#[tokio::main]
async fn main() {
    let mut rng = rand::rng();

    let handles: JoinSet<_> = (0..AGENT_COUNT)
        .map(|index| {
            let small_rng = SmallRng::from_rng(&mut rng);
            tokio::spawn(async move { virtual_agent(index, small_rng).await })
        })
        .collect();

    // 모든 태스크가 종료될 때까지 대기
    handles.join_all().await;
}

async fn virtual_agent(index: u32, mut rng: SmallRng) {
    let mut builder = FlatBufferBuilder::new();
    loop {
        let mut app = App::create(String::from("127.0.0.1:1234")).await.unwrap();
        let name: String = format!("virtual_agent {}", index);
        let user_idx: u64 = hash_vec_u8(name.as_bytes());
        let color = Color::new(
            (rng.next_u32() % 256) as u8,
            (rng.next_u32() % 256) as u8,
            (rng.next_u32() % 256) as u8,
            0,
        );

        let mut actor_idx: Option<u64> = None;
        let mut current_direction = Vec2::new(0., 0.);

        _ = app.send_message(
            PacketType::CG_LOGIN_REQ.0,
            build_cg_login_req(&mut builder, user_idx, name.clone()),
        );

        loop {
            let mut flag = true;
            loop {
                match app.get_callback() {
                    Callback::Receive {
                        packet_type,
                        message,
                    } => match PacketType(packet_type) {
                        PacketType::GC_LOGIN_RES => {
                            on_gc_login_res(
                                &mut app,
                                &mut builder,
                                message,
                                user_idx,
                                name.clone(),
                                color,
                            );
                        }
                        PacketType::GC_JOIN_RES => {
                            actor_idx = on_gc_join_res(&mut app, &mut builder, message, color);
                            //println!("joined!")
                        }
                        PacketType::GC_SPAWN_ACTOR_NOTI => {
                            //println!("spawn spawn!");
                        }
                        PacketType::GC_REMOVE_ACTOR_NOTI => {
                            //println!("remove remove!");
                        }
                        PacketType::GC_CHANGE_MOVE_DIRECTION_NOTI => {
                            //println!("move move!");
                        }
                        _ => return,
                    },
                    Callback::Disconnect => {
                        flag = false;
                        break;
                    },
                    Callback::Empty => break,
                    Callback::Close => {flag = false; break},
                }
            }

            if flag == false {
                break;
            }

            if actor_idx.is_none() {
                sleep(FRAME_TIME).await;
            } else {
                // 여기부터는 로그인이 된 상태임. 마음대로 하자.
                let random_p = rng.next_u32() % 10000;
                if random_p > 1 {
                    let new_direction = random_direction(&mut rng);
                    if new_direction == current_direction {
                        let _ = app.send_message(
                            PacketType::CG_CHANGE_MOVE_DIRECTION_NOTI.0,
                            build_cg_change_move_direction_noti(&mut builder, random_direction(&mut rng)),
                        );
                        current_direction = new_direction;
                    }
                } else {
                    _ = app.disconnect();
                }

                let random_duration: u64 = rng.random_range(100..1000);
                
                sleep(Duration::from_millis(random_duration)).await;
            }
        }
    }
}

fn on_gc_login_res<'a>(
    app: &mut App,
    builder: &mut FlatBufferBuilder<'a>,
    message: Vec<u8>,
    user_idx: u64,
    name: String,
    color: Color,
) {
    match flatbuffers::root::<GCLoginRes>(&message) {
        Ok(res) => {
            if res.result() == ServerCode::FAILED {
                _ = app.send_message(
                    PacketType::CG_LOGIN_REQ.0,
                    build_cg_login_req(builder, user_idx, name),
                );
            } else {
                _ = app.send_message(PacketType::CG_JOIN_REQ.0, build_cg_join_req(builder, color))
            }
        }
        Err(e) => eprintln!("invalid_flatbuffer on_gc_login_res {:?}", e),
    }
}

fn on_gc_join_res<'a>(
    app: &mut App,
    builder: &mut FlatBufferBuilder<'a>,
    message: Vec<u8>,
    color: Color,
) -> Option<u64> {
    match flatbuffers::root::<GCJoinRes>(&message) {
        Ok(res) => {
            if res.result() == ServerCode::FAILED {
                _ = app.send_message(PacketType::CG_JOIN_REQ.0, build_cg_join_req(builder, color))
            } else {
                return Some(res.actor_idx());
            }
        }
        Err(e) => eprintln!("invalid_flatbuffer on_gc_login_res {:?}", e),
    }

    None
}
