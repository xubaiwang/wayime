use std::{
    cell::Cell,
    os::fd::{AsFd, OwnedFd},
    time::{SystemTime, UNIX_EPOCH},
};

use log::info;
use wayland_client::{
    protocol::wl_keyboard::{KeyState, KeymapFormat},
    Connection, Dispatch, QueueHandle, WEnum,
};
use wayland_protocols_misc::zwp_input_method_v2::client::zwp_input_method_keyboard_grab_v2::{
    Event, ZwpInputMethodKeyboardGrabV2,
};
use xkbcommon::xkb::{
    self,
    ffi::{XKB_STATE_LAYOUT_EFFECTIVE, XKB_STATE_MODS_EFFECTIVE},
    KeyDirection, Keycode, Keysym, KEYMAP_COMPILE_NO_FLAGS, KEYMAP_FORMAT_TEXT_V1,
    KEYMAP_FORMAT_USE_ORIGINAL,
};

use super::Im;

/// 處理鍵盤抓取事件
impl Dispatch<ZwpInputMethodKeyboardGrabV2, ()> for Im {
    fn event(
        im: &mut Self,
        _: &ZwpInputMethodKeyboardGrabV2,
        event: <ZwpInputMethodKeyboardGrabV2 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
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
        info!("Handle keymap, format: {format:?}, fd: {fd:?}, size: {size}");
        let format = format.into_result().expect("invalid format enum");
        // 更新 keyboard 鍵盤
        self.virtual_keyboard
            .as_ref()
            .unwrap()
            .keymap(format.into(), fd.as_fd(), size);
        // 設置 XKB keymap 和狀態
        let xkb_keymap = unsafe {
            xkb::Keymap::new_from_fd(
                &self.context,
                fd,
                size as usize,
                match format {
                    KeymapFormat::NoKeymap => KEYMAP_FORMAT_USE_ORIGINAL,
                    KeymapFormat::XkbV1 => KEYMAP_FORMAT_TEXT_V1,
                    _ => unreachable!(),
                },
                KEYMAP_COMPILE_NO_FLAGS,
            )
        }
        .unwrap()
        .unwrap();
        self.state = Some(xkb::State::new(&xkb_keymap));
    }

    /// 處理按鍵事件。
    fn handle_key(&mut self, _serial: u32, _time: u32, key: u32, key_state: WEnum<KeyState>) {
        let state = self.state.as_ref().unwrap();
        // xkb 轉換
        let keycode = Keycode::new(key + 8);
        let keysym = state.key_get_one_sym(keycode);
        info!("Handle key: {:?}", keysym);
        // 獲取 key state
        let key_state = key_state.into_result().expect("unrecognized key state");
        let pressed = key_state == KeyState::Pressed;
        // TODO: 處理 repeat
        self.handle_key_further(keycode, keysym, pressed);
    }

    /// 進一步處理
    fn handle_key_further(&mut self, keycode: Keycode, keysym: Keysym, pressed: bool) {
        let state = self.state.as_mut().unwrap();
        // 更新 state
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
            // 發送按鍵信息到 Rime
            handled = self.engine.key(
                keysym,
                state.serialize_mods(XKB_STATE_MODS_EFFECTIVE | XKB_STATE_LAYOUT_EFFECTIVE),
            );
        }
        // bypass 模式
        if !handled && self.engine.is_bypass() {
            // 直接原樣寫入文本
            let keyboard = self.virtual_keyboard.as_ref().unwrap();
            keyboard.key(
                time_ms(),
                keycode.raw() - 8,
                if pressed {
                    KeyState::Pressed
                } else {
                    KeyState::Released
                } as u32,
            );
        } else {
            self.update_preedit_panel();
            if let Some(commit) = self.engine.get_commit() {
                self.commit_string(commit);
            }
            self.input_method.as_ref().unwrap().commit(self.serial);
        }
    }

    /// 處理修飾鍵。
    fn handle_modifier(
        &mut self,
        _serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        // 更新 XKB 狀態
        self.state.as_mut().unwrap().update_mask(
            mods_depressed,
            mods_latched,
            mods_locked,
            0,
            0,
            group,
        );
        // 更新鍵盤修飾符
        self.virtual_keyboard.as_ref().unwrap().modifiers(
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        );
    }

    /// 處理重複。
    fn handle_repeat(&mut self, _rate: i32, _delay: i32) {
        // TODO: handle repeat
    }

    /// 是否應該更改狀態。
    fn should_toggle(records: &[Cell<Option<Keysym>>; 2], key: Keysym, pressed: bool) -> bool {
        records[1].set(records[0].get());
        records[0].set(Some(key));
        return pressed == false
            && records[0].get() == Some(Keysym::XF86_Keyboard)
            && records[1].get() == Some(Keysym::XF86_Keyboard);
    }

    /// 更新預編輯文本面板。
    fn update_preedit_panel(&self) {
        let mut buf = String::new();

        // 從 Rime 獲取預編輯文本
        let preedit = self.engine.preedit();
        if let Some(text) = preedit.text {
            buf.push_str(&text);
        }

        // 從 Rime 獲取候選詞
        let cand = self.engine.candidate();
        for i in 0..cand.num_candidates {
            // 編號或者高亮
            if i == cand.highlighted_candidate_index {
                buf.push('⁺');
            } else {
                buf.push_str(&map_digits(i));
            }
            // 候選詞
            buf.push_str(&self.engine.candidate_get(i).unwrap_or("".to_string()));
        }

        // 發送設置請求
        self.set_preedit_string(buf, preedit.start, preedit.end);
    }

    /// 設置预编辑文本。
    fn set_preedit_string(&self, text: String, start: i32, end: i32) {
        info!("Set preedit string: {}", text);
        self.input_method
            .as_ref()
            .unwrap()
            .set_preedit_string(text, start, end);
    }

    /// 提交文本。
    fn commit_string(&self, commit: String) {
        info!("Commit string: {}", commit);
        self.input_method.as_ref().unwrap().commit_string(commit);
    }
}

/// 獲取毫秒時間。
fn time_ms() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32
}

/// 映射數位。
fn map_digits(number: i32) -> String {
    number
        .to_string()
        .chars()
        .map(|c| match c {
            '0' => '¹',
            '1' => '²',
            '2' => '³',
            '3' => '⁴',
            '4' => '⁵',
            '5' => '⁶',
            '6' => '⁷',
            '7' => '⁸',
            '8' => '⁹',
            '9' => '⁰',
            '-' => '⁻',
            c => c,
        })
        .collect::<String>()
}
