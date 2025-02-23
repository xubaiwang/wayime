use im::Im;
use wayland_client::Connection;

mod engine;
mod im;

fn main() {
    // 初始化日誌輸出
    env_logger::init();

    // 連接 wayland
    let conn = Connection::connect_to_env().unwrap();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    // 初始化輸入法
    let mut im = Im::new();
    let display = conn.display();
    display.get_registry(&qh, ());

    // 循環
    loop {
        event_queue.blocking_dispatch(&mut im).unwrap();
    }
}
