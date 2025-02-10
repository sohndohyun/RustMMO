mod game_server;
mod protocol_generated;
mod game_user;
mod build_packet;
mod world;

#[tokio::main]
async fn main() {
    let game_server = game_server::GameServer::new();
    game_server.borrow_mut().run().await;
}
