use std::{
    any::Any,
    cell::Cell,
    os::fd::{AsFd, OwnedFd},
    time::{SystemTime, UNIX_EPOCH},
};

use wayland_client::{
    protocol::wl_keyboard::{KeyState, KeymapFormat},
    Dispatch, QueueHandle, WEnum,
};
use wayland_protocols_misc::zwp_input_method_v2::client::zwp_input_method_keyboard_grab_v2::{
    Event, ZwpInputMethodKeyboardGrabV2,
};
use xkbcommon::xkb::{
    self,
    ffi::{XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_MODS_EFFECTIVE},
    KeyDirection, Keycode, Keysym, KEYMAP_COMPILE_NO_FLAGS, KEYMAP_FORMAT_TEXT_V1,
};

use super::Im;

impl Dispatch<ZwpInputMethodKeyboardGrabV2, ()> for Im {
    fn event(
        im: &mut Self,
        _: &ZwpInputMethodKeyboardGrabV2,
        event: <ZwpInputMethodKeyboardGrabV2 as wayland_client::Proxy>::Event,
        _: &(),
        conn: &wayland_client::Connection,
        _: &QueueHandle<Self>,
    ) {
        // conn.roundtrip().unwrap();
        match event {
            // 處理 keymap
            Event::Keymap { format, fd, size } => {
                im.handle_keymap(format, fd, size);
            }
            // 處理 key
            Event::Key {
                serial,
                time,
                key,
                state,
            } => {
                im.handle_key(serial, time, key, state);
            }
            // 處理 modifiers
            Event::Modifiers {
                serial,
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
            } => {
                im.handle_modifier(serial, mods_depressed, mods_latched, mods_locked, group);
            }
            // 處理重複
            Event::RepeatInfo { rate, delay } => {
                im.handle_repeat(rate, delay);
            }
            _ => {}
        }
    }
}

impl Im {
    /// 處理 keymap, 創建自己的 xkb_state.
    fn handle_keymap(&mut self, format: WEnum<KeymapFormat>, fd: OwnedFd, size: u32) {
        if let Some(keyboard) = self.virtual_keyboard.as_ref() {
            keyboard.keymap(format.into(), fd.as_fd(), size);
        }
        let xkb_keymap = unsafe {
            xkb::Keymap::new_from_fd(
                &self.context,
                fd,
                size as usize,
                KEYMAP_FORMAT_TEXT_V1,
                KEYMAP_COMPILE_NO_FLAGS,
            )
        }
        .unwrap()
        .unwrap();
        self.state = Some(xkb::State::new(&xkb_keymap));
    }

    /// 處理按鍵事件。
    fn handle_key(&mut self, serial: u32, time: u32, key: u32, key_state: WEnum<KeyState>) {
        let Some(state) = &self.state else { return };
        // xkb 轉換
        let keycode = Keycode::new(key + 8);
        let keysym = state.key_get_one_sym(keycode);
        // 獲取 key state
        let WEnum::Value(key_state) = key_state else {
            return;
        };
        let pressed = key_state == KeyState::Pressed;
        // TODO: 處理 repeat
        self.handle_key_further(keycode, keysym, pressed);
    }

    /// 進一步處理
    fn handle_key_further(&mut self, keycode: Keycode, keysym: Keysym, pressed: bool) {
        // 更新 state
        let Some(state) = self.state.as_mut() else {
            return;
        };
        state.update_key(
            keycode,
            if pressed {
                KeyDirection::Down
            } else {
                KeyDirection::Up
            },
        );
        let mut handled = false;
        // toggle
        if !handled && Self::should_toggle(&self.records, keysym, pressed) {
            self.engine.toggle();
            handled = true;
        }
        // 如果是按下
        if !handled && pressed {
            handled = self.engine.key(
                keysym,
                state.serialize_mods(XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LAYOUT_EFFECTIVE),
            );
        }
        // bypass
        if !handled && self.engine.is_bypass() {
            // 類似 ascii 模式
            // 應該 virtual keyboard 寫入
            let keyboard = self.virtual_keyboard.as_ref().unwrap();
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            keyboard.key(
                time as u32,
                keycode.raw() - 8,
                if pressed {
                    KeyState::Pressed
                } else {
                    KeyState::Released
                } as u32,
            );
        } else {
            self.panel_update();
            if let Some(commit) = self.engine.get_commit() {
                self.send_text(commit);
            } else {
            }
            self.input_method.as_ref().unwrap().commit(self.serial);
        }
    }

    /// 處理修飾。
    fn handle_modifier(
        &mut self,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        let Some(state) = &mut self.state else { return };
        state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);
        if let Some(keyboard) = &self.virtual_keyboard {
            keyboard.modifiers(mods_depressed, mods_latched, mods_locked, group);
        }
    }

    /// 處理重複。
    fn handle_repeat(&mut self, rate: i32, delay: i32) {
        // TODO: handle repeat
    }

    fn should_toggle(records: &[Cell<Option<Keysym>>; 2], key: Keysym, pressed: bool) -> bool {
        records[1].set(records[0].get());
        records[0].set(Some(key));
        return pressed == false
            && records[0].get() == Some(Keysym::Shift_L)
            && records[1].get() == Some(Keysym::Shift_L);
    }

    fn panel_update(&self) {
        let mut buf = String::new();
        let preedit = self.engine.preedit();
        let Some(preedit) = preedit.text else {
            self.send_preedit("".to_string());
            return;
        };
        let cand = self.engine.candidate();
        // preedit
        buf.push_str(&preedit);
        // candidates
        for i in 0..cand.num_candidates {
            let highlighted = i == cand.highlighted_candidate_index;
            buf.push_str(&format!(
                " {}{}{}",
                if highlighted { "[" } else { "" },
                self.engine.candidate_get(i).unwrap_or("".to_string()),
                if highlighted { "]" } else { "" }
            ));
        }
        // send
        self.send_preedit(buf);
    }

    fn send_preedit(&self, text: String) {
        self.input_method
            .as_ref()
            .unwrap()
            .set_preedit_string(text, 0, 0);
    }

    fn send_text(&self, commit: String) {
        self.input_method.as_ref().unwrap().commit_string(commit);
    }
}
