pub mod dsnet;

fn main() {
    _ = dsnet::server::App::new()
        .run();
}
