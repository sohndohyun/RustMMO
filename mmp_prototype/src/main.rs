mod game_server;
mod protocol_generated;
mod game_user;
mod build_packet;
mod world;
mod single_channel;

#[tokio::main]
async fn main() {
    game_server::GameServer::run(String::from("127.0.0.1:1234"));
}
