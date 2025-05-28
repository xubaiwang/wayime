use wayland_client::delegate_noop;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;

delegate_noop!(super::Im: ZwpVirtualKeyboardManagerV1);
