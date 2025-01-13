pub mod dsnet;

use crate::dsnet::server::App;

fn on_update(_: &mut App, _: u32) {
    println!("Default on_update callback triggered.");
}

fn on_accept(_: &mut App, _: u128) {
    println!("Default on_accept callback triggered.");
}

fn on_receive(app: &mut App, idx: u128, message: Vec<u8>) {
    _ = app.send_message(idx, message);
}

fn on_disconnect(_: &mut App,_: u128) {
    println!("Default on_disconnect callback triggered.");
}

fn main() {
    let mut server_app = App::new();
    if let Err(e) = server_app.set_str_addr("127.0.0.1:8080"){
        eprintln!("{:?}", e);
        return;
    }
    
    server_app.set_on_update(on_update);
    server_app.set_on_accept(on_accept);
    server_app.set_on_receive(on_receive);
    server_app.set_on_disconnect(on_disconnect);

    server_app.run();
}
