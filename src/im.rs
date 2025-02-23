use std::cell::Cell;

use wayland_client::protocol::wl_seat::WlSeat;
use wayland_protocols_misc::{
    zwp_input_method_v2::client::{
        zwp_input_method_keyboard_grab_v2::ZwpInputMethodKeyboardGrabV2,
        zwp_input_method_manager_v2::ZwpInputMethodManagerV2,
        zwp_input_method_v2::ZwpInputMethodV2,
    },
    zwp_virtual_keyboard_v1::client::{
        zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
        zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
    },
};
use xkbcommon::xkb::{self, Keysym};

use crate::engine::Engine;

mod dispatch_input_method;
mod dispatch_input_method_keyboard_grab;
mod dispatch_input_method_manager;
mod dispatch_registry;
mod dispatch_seat;
mod dispatch_virtual_keyboard;
mod dispatch_virtual_keyboard_manager;

pub struct Im {
    // rime
    engine: Engine,
    // xkb
    context: xkb::Context,
    state: Option<xkb::State>,
    // wayland core
    seat: Option<WlSeat>,
    // input method
    input_method_manager: Option<ZwpInputMethodManagerV2>,
    input_method: Option<ZwpInputMethodV2>,
    input_method_keyboard_grab: Option<ZwpInputMethodKeyboardGrabV2>,
    // virtual keyboard
    virtual_keyboard_manager: Option<ZwpVirtualKeyboardManagerV1>,
    virtual_keyboard: Option<ZwpVirtualKeyboardV1>,
    // records
    records: [Cell<Option<Keysym>>; 2],
    // serial
    serial: u32,
}

impl Im {
    pub fn new() -> Self {
        let engine = Engine::new();
        let context = xkb::Context::new(0);
        let records = [Cell::new(None), Cell::new(None)];
        let serial = 0;
        let im = Self {
            engine,
            context,
            state: None,
            seat: None,
            input_method_manager: None,
            input_method: None,
            input_method_keyboard_grab: None,
            virtual_keyboard_manager: None,
            virtual_keyboard: None,
            records,
            serial,
        };
        im
    }
}

impl Drop for Im {
    fn drop(&mut self) {
        if let Some(grab) = &self.input_method_keyboard_grab {
            grab.release();
        }
        if let Some(input_method) = &self.input_method {
            input_method.destroy();
        }
        if let Some(input_method_manager) = &self.input_method_manager {
            input_method_manager.destroy();
        }
    }
}
