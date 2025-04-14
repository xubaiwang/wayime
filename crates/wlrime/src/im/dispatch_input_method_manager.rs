use wayland_client::delegate_noop;
use wayland_protocols_misc::zwp_input_method_v2::client::zwp_input_method_manager_v2::ZwpInputMethodManagerV2;

delegate_noop!(super::Im: ignore ZwpInputMethodManagerV2);
