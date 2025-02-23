use wayland_client::{
    protocol::{
        wl_registry::{Event, WlRegistry},
        wl_seat::WlSeat,
    },
    Dispatch, QueueHandle,
};
use wayland_protocols_misc::{
    zwp_input_method_v2::client::zwp_input_method_manager_v2::ZwpInputMethodManagerV2,
    zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
};

use super::Im;

impl Dispatch<WlRegistry, ()> for Im {
    fn event(
        im: &mut Self,
        proxy: &WlRegistry,
        event: <WlRegistry as wayland_client::Proxy>::Event,
        _: &(),
        _: &wayland_client::Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            Event::Global {
                name,
                interface,
                version,
            } => match &interface[..] {
                // 綁定 wl_seat
                "wl_seat" => {
                    im.seat = Some(proxy.bind::<WlSeat, _, _>(name, version, qh, ()));
                    im.init_input_method(qh);
                    im.init_virtual_keyboard(qh);
                }
                // 綁定 input_method_manager
                "zwp_input_method_manager_v2" => {
                    im.input_method_manager =
                        Some(proxy.bind::<ZwpInputMethodManagerV2, _, _>(name, version, qh, ()));
                    im.init_input_method(qh);
                }
                "zwp_virtual_keyboard_manager_v1" => {
                    im.virtual_keyboard_manager = Some(
                        proxy.bind::<ZwpVirtualKeyboardManagerV1, _, _>(name, version, qh, ()),
                    );
                    im.init_virtual_keyboard(qh);
                }
                // 其他接口不處理
                _ => {}
            },
            Event::GlobalRemove { .. } => {
                // TODO: 處理消失情况
            }
            _ => {}
        }
    }
}

impl Im {
    /// 嘗試初始化 input_method
    fn init_input_method(&mut self, qh: &QueueHandle<Self>) {
        if self.input_method.is_none() {
            if let (Some(manager), Some(seat)) = (&self.input_method_manager, &self.seat) {
                let input_method = manager.get_input_method(seat, qh, ());
                self.input_method_keyboard_grab = Some(input_method.grab_keyboard(qh, ()));
                self.input_method = Some(input_method);
            }
        }
    }

    fn init_virtual_keyboard(&mut self, qh: &QueueHandle<Self>) {
        if self.virtual_keyboard.is_none() {
            if let (Some(manager), Some(seat)) = (&self.virtual_keyboard_manager, &self.seat) {
                let virtual_keyboard = manager.create_virtual_keyboard(seat, qh, ());
                self.virtual_keyboard = Some(virtual_keyboard);
            }
        }
    }
}
