use xkbcommon::xkb::KEYSYM_NO_FLAGS;

fn main() {
    let key1 = xkbcommon::xkb::Keysym::XF86_Keyboard;
    dbg!(key1.name());

    let key2 = xkbcommon::xkb::keysym_from_name("XF86XK_Keyboard", KEYSYM_NO_FLAGS);
    dbg!(key2);

    let key3 = xkbcommon::xkb::Keysym::Q;
    dbg!(key3.name());

    let key4 = xkbcommon::xkb::keysym_from_name("Q", KEYSYM_NO_FLAGS);
    dbg!(key4);

    let key3 = xkbcommon::xkb::Keysym::Shift_L;
    dbg!(key3.name());

    let key4 = xkbcommon::xkb::keysym_from_name("Shift_L", KEYSYM_NO_FLAGS);
    dbg!(key4);
}
