use im::Im;
use wayland_client::Connection;

mod engine;
mod im;

fn main() {
    let conn = Connection::connect_to_env().unwrap();

    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qh, ());

    let mut im = Im::new();

    loop {
        event_queue.blocking_dispatch(&mut im).unwrap();
    }
}
