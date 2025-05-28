use wayland_client::{delegate_noop, protocol::wl_seat::WlSeat};

delegate_noop!(super::Im: ignore WlSeat);
