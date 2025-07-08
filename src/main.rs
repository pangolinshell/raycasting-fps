mod server;

use server::Instance;
use std::thread;
use multiplayer_fps_v3::client::running::run_client;

fn main() {
    let instance = Instance::new(5000, 60);
    let _ = instance.run();
    loop {
        let _ = run_client("127.0.0.1:5000",8080,String::from("Gustave"));
        let _ = run_client("127.0.0.1:5000",8081,String::from("Timmy"));
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
