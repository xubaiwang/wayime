use wayland_client::{Dispatch, QueueHandle};
use wayland_protocols_misc::zwp_input_method_v2::client::zwp_input_method_v2::{
    Event, ZwpInputMethodV2,
};

use super::Im;

impl Dispatch<ZwpInputMethodV2, ()> for Im {
    fn event(
        im: &mut Self,
        proxy: &ZwpInputMethodV2,
        event: <ZwpInputMethodV2 as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        match event {
            Event::Activate => {
                im.handle_reset();
            }
            Event::Deactivate => {
                im.handle_reset();
            }
            Event::SurroundingText {
                text,
                cursor,
                anchor,
            } => {
                // noop
            }
            Event::TextChangeCause { cause } => {
                // noop
            }
            Event::ContentType { hint, purpose } => {
                // noop
            }
            Event::Done => {
                im.handle_done();
            }
            Event::Unavailable => {
                im.handle_reset();
            }
            _ => {}
        }
    }
}

impl Im {
    fn handle_reset(&mut self) {
        self.engine.reset();
    }

    fn handle_done(&mut self) {
        self.serial += 1;
    }
}
