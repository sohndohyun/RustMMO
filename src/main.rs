pub mod dsnet;

fn on_update(_: u32) {
    println!("Default on_update callback triggered.");
}

fn on_accept(_: u128) {
    println!("Default on_accept callback triggered.");
}

fn on_receive(_: u128, _: Vec<u8>) {
    println!("Default on_receive callback triggered.");
}

fn on_disconnect(_: u128) {
    println!("Default on_disconnect callback triggered.");
}

fn main() {
    let mut server_app = dsnet::server::App::new();
    if let Err(e) = server_app.set_str_addr("127.0.0.1:8080"){
        eprintln!("{:?}", e);
        return;
    }
    
    server_app.set_on_update(on_update);
    server_app.set_on_accept(on_accept);
    server_app.set_on_receive(on_receive);
    server_app.set_on_disconnect(on_disconnect);

    if let Err(e) = server_app.run() {
        eprintln!("{:?}", e);
        return;
    }
}
