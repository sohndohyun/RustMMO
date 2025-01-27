use dsnet::server::App;
extern crate flatbuffers;

mod protocol_generated;
pub use protocol_generated::nexus;

#[tokio::main]
async fn main() {
    App::new("127.0.0.1:1234:1234".into())
        .set_on_update(on_update)
        .set_on_accept(on_accept)
        .set_on_receive(on_receive)
        .set_on_disconnect(on_disconnect)
        .run().await;
}

fn on_update(_: &mut App, _: u32) {
    // println!("Default on_update callback triggered.");
}

fn on_accept(_: &mut App, _: u128) {
    println!("Default on_accept callback triggered.");
}

fn on_receive(app: &mut App, idx: u128, packet_type: u16, message: Vec<u8>) {
    println!("Default on_receive callback triggered.");
    _ = app.send_message(idx, packet_type, message);
}

fn on_disconnect(_: &mut App,_: u128) {
    println!("Default on_disconnect callback triggered.");
}
