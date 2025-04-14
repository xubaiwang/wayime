use std::{
    ffi::{CStr, CString, NulError},
    str::Utf8Error,
};

use librime_sys::{rime_commit_t, rime_context_t, rime_status_t, rime_struct};

use crate::{ptr_to_cstr, rime_api_call};

use super::{commit::Commit, context::Context, status::Status, Rime};

pub struct Session<'a> {
    api: &'a Rime,
    id: usize,
}

impl<'a> Session<'a> {
    pub fn from_id(api: &'a Rime, id: usize) -> Self {
        Self { api, id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn find(&self) -> bool {
        rime_api_call!(self.api.raw(), find_session, self.id) != 0
    }
}

impl<'a> Drop for Session<'a> {
    fn drop(&mut self) {
        rime_api_call!(self.api.raw(), destroy_session, self.id);
    }
}

/// 輸入。
impl<'a> Session<'a> {
    pub fn process_key(&self, keycode: i32, mask: i32) -> bool {
        rime_api_call!(self.api.raw(), process_key, self.id, keycode, mask) != 0
    }

    pub fn commit_composition(&self) -> bool {
        rime_api_call!(self.api.raw(), commit_composition, self.id) != 0
    }

    pub fn clear_composition(&self) {
        rime_api_call!(self.api.raw(), clear_composition, self.id);
    }
}

/// 輸出。
impl<'a> Session<'a> {
    pub fn commit(&self) -> Commit {
        rime_struct!(raw: rime_commit_t);
        rime_api_call!(self.api.raw(), get_commit, self.id, &mut raw);
        Commit::from_raw(&self.api, raw)
    }

    pub fn context(&self) -> Context {
        rime_struct!(raw: rime_context_t);
        rime_api_call!(self.api.raw(), get_context, self.id, &mut raw);
        Context::from_raw(&self.api, raw)
    }

    pub fn status(&self) -> Status {
        rime_struct!(raw: rime_status_t);
        rime_api_call!(self.api.raw(), get_status, self.id, &mut raw);
        Status::from_raw(&self.api, raw)
    }
}

/// 運行時選項。
impl<'a> Session<'a> {
    pub fn set_option_c(&self, option: &CStr, value: bool) {
        rime_api_call!(
            self.api.raw(),
            set_option,
            self.id,
            option.as_ptr(),
            value as i32
        )
    }

    pub fn set_option(&self, option: impl Into<Vec<u8>>, value: bool) -> Result<(), NulError> {
        let option = CString::new(option)?;
        Ok(self.set_option_c(&option, value))
    }

    pub fn get_option_c(&self, option: &CStr) -> bool {
        rime_api_call!(self.api.raw(), get_option, self.id, option.as_ptr()) != 0
    }

    pub fn get_option(&self, option: impl Into<Vec<u8>>) -> Result<bool, NulError> {
        let option = CString::new(option)?;
        Ok(self.get_option_c(&option))
    }

    pub fn set_property_c(&self, prop: &CStr, value: &CStr) {
        rime_api_call!(
            self.api.raw(),
            set_property,
            self.id,
            prop.as_ptr(),
            value.as_ptr()
        )
    }

    pub fn set_property(
        &self,
        prop: impl Into<Vec<u8>>,
        value: impl Into<Vec<u8>>,
    ) -> Result<(), NulError> {
        let prop = CString::new(prop)?;
        let value = CString::new(value)?;
        Ok(self.set_property_c(&prop, &value))
    }

    pub fn get_property_c(&self, prop: &CStr, value: &mut [i8]) -> bool {
        rime_api_call!(
            self.api.raw(),
            get_property,
            self.id,
            prop.as_ptr(),
            value.as_mut_ptr(),
            value.len()
        ) != 0
    }

    pub fn get_property(
        &self,
        prop: impl Into<Vec<u8>>,
        value: &mut [i8],
    ) -> Result<bool, NulError> {
        let prop = CString::new(prop)?;
        Ok(self.get_property_c(&prop, value))
    }

    pub fn get_current_schema(&self, schema_id: &mut [i8]) -> bool {
        rime_api_call!(
            self.api.raw(),
            get_current_schema,
            self.id,
            schema_id.as_mut_ptr(),
            schema_id.len()
        ) != 0
    }

    pub fn select_schema_c(&self, schema_id: &CStr) -> bool {
        rime_api_call!(self.api.raw(), select_schema, self.id, schema_id.as_ptr()) != 0
    }

    pub fn select_schema(&self, schema_id: impl Into<Vec<u8>>) -> Result<bool, NulError> {
        let schema_id = CString::new(schema_id)?;
        Ok(self.select_schema_c(&schema_id))
    }
}

/// 測試。
impl<'a> Session<'a> {
    pub fn simulate_key_sequence_c(&self, key_sequence: &CStr) -> bool {
        rime_api_call!(
            self.api.raw(),
            simulate_key_sequence,
            self.id,
            key_sequence.as_ptr()
        ) != 0
    }

    pub fn simulate_key_sequence(
        &self,
        key_sequence: impl Into<Vec<u8>>,
    ) -> Result<bool, NulError> {
        let key_sequence = CString::new(key_sequence)?;
        Ok(self.simulate_key_sequence_c(&key_sequence))
    }
}

impl<'a> Session<'a> {
    pub fn get_input_c(&self) -> Option<&CStr> {
        let ptr = rime_api_call!(self.api.raw(), get_input, self.id);
        ptr_to_cstr!(ptr)
    }

    pub fn get_input(&self) -> Option<Result<&str, Utf8Error>> {
        self.get_input_c().map(CStr::to_str)
    }

    pub fn set_input_c(&self, input: &CStr) -> bool {
        rime_api_call!(self.api.raw(), set_input, self.id, input.as_ptr()) != 0
    }

    pub fn set_input(&self, input: impl Into<Vec<u8>>) -> Result<bool, NulError> {
        let input = CString::new(input)?;
        Ok(self.set_input_c(&input))
    }
}

impl<'a> Session<'a> {
    pub fn get_caret_pos(&self) -> usize {
        rime_api_call!(self.api.raw(), get_caret_pos, self.id)
    }

    pub fn set_caret_pos(&self, caret_pos: usize) {
        rime_api_call!(self.api.raw(), set_caret_pos, self.id, caret_pos)
    }
}

/// 候選和翻頁。
impl<'a> Session<'a> {
    pub fn select_candidate(&self, index: usize) -> bool {
        rime_api_call!(self.api.raw(), select_candidate, self.id, index) != 0
    }

    pub fn select_candidate_on_current_page(&self, index: usize) -> bool {
        rime_api_call!(
            self.api.raw(),
            select_candidate_on_current_page,
            self.id,
            index
        ) != 0
    }

    /// 刪除候選詞。
    pub fn delete_candidate(&self, index: usize) -> bool {
        rime_api_call!(self.api.raw(), delete_candidate, self.id, index) != 0
    }

    /// 刪除當前頁面候選詞。
    pub fn delete_candidate_on_current_page(&self, index: usize) -> bool {
        rime_api_call!(
            self.api.raw(),
            delete_candidate_on_current_page,
            self.id,
            index
        ) != 0
    }

    /// 高亮候選词。
    pub fn highlight_candidate(&self, index: usize) -> bool {
        rime_api_call!(self.api.raw(), highlight_candidate, self.id, index) != 0
    }

    /// 高亮當前頁面候選詞。
    pub fn highlight_candidate_on_current_page(&self, index: usize) -> bool {
        rime_api_call!(
            self.api.raw(),
            highlight_candidate_on_current_page,
            self.id,
            index
        ) != 0
    }

    /// 前後翻頁。
    pub fn change_page(&self, backward: bool) -> bool {
        rime_api_call!(self.api.raw(), change_page, self.id, backward as i32) != 0
    }
}

impl<'a> Session<'a> {
    // TODO: candidate_list_{begin,next,end}
    // TODO: candidate_list_from_index
}

impl<'a> Session<'a> {
    pub fn get_state_label_c(&self, option_name: &CStr, state: bool) -> Option<&CStr> {
        let ptr = rime_api_call!(
            self.api.raw(),
            get_state_label,
            self.id,
            option_name.as_ptr(),
            state as i32
        );
        ptr_to_cstr!(ptr)
    }

    // TODO: get_state_label_abbreviated
}
