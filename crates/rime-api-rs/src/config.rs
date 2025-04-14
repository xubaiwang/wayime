use std::{
    ffi::{CStr, CString, NulError},
    ptr::null_mut,
};

use librime_sys::RimeConfig;

use crate::{rime_api_call, struct_impl_managed, Rime};

struct_impl_managed!(Config, config_close);

impl<'a> Config<'a> {
    pub fn new(api: &'a Rime) -> Self {
        let raw = RimeConfig { ptr: null_mut() };
        Self::from_raw(api, raw)
    }
}

/// Getters.
impl<'a> Config<'a> {
    pub fn get_item_c(&mut self, key: &CStr) -> Config {
        let mut raw = RimeConfig { ptr: null_mut() };
        rime_api_call!(
            self.api.raw(),
            config_get_item,
            self.raw_mut(),
            key.as_ptr(),
            &mut raw
        );
        Config::from_raw(&self.api, raw)
    }

    pub fn get_item(&mut self, key: impl Into<Vec<u8>>) -> Result<Config, NulError> {
        let key = CString::new(key)?;
        Ok(self.get_item_c(&key))
    }

    pub fn list_size_c(&mut self, key: &CStr) -> usize {
        rime_api_call!(
            self.api.raw(),
            config_list_size,
            self.raw_mut(),
            key.as_ptr()
        )
    }

    pub fn list_size(&mut self, key: impl Into<Vec<u8>>) -> Result<usize, NulError> {
        let key = CString::new(key)?;
        Ok(self.list_size_c(&key))
    }

    // TODO: config_begin_list
}

/// Setters.
impl<'a> Config<'a> {
    pub fn set_bool_c(&mut self, key: &CStr, value: bool) -> bool {
        rime_api_call!(
            self.api.raw(),
            config_set_bool,
            self.raw_mut(),
            key.as_ptr(),
            value as i32
        ) != 0
    }

    pub fn set_bool(&mut self, key: impl Into<Vec<u8>>, value: bool) -> Result<bool, NulError> {
        let key = CString::new(key)?;
        Ok(self.set_bool_c(&key, value))
    }

    pub fn set_int_c(&mut self, key: &CStr, value: i32) -> bool {
        rime_api_call!(
            self.api.raw(),
            config_set_int,
            self.raw_mut(),
            key.as_ptr(),
            value
        ) != 0
    }

    pub fn set_int(&mut self, key: impl Into<Vec<u8>>, value: i32) -> Result<bool, NulError> {
        let key = CString::new(key)?;
        Ok(self.set_int_c(&key, value))
    }

    pub fn set_double_c(&mut self, key: &CStr, value: f64) -> bool {
        rime_api_call!(
            self.api.raw(),
            config_set_double,
            self.raw_mut(),
            key.as_ptr(),
            value
        ) != 0
    }

    pub fn set_double(&mut self, key: impl Into<Vec<u8>>, value: f64) -> Result<bool, NulError> {
        let key = CString::new(key)?;
        Ok(self.set_double_c(&key, value))
    }

    pub fn set_string_c(&mut self, key: &CStr, value: &CStr) -> bool {
        rime_api_call!(
            self.api.raw(),
            config_set_string,
            self.raw_mut(),
            key.as_ptr(),
            value.as_ptr()
        ) != 0
    }

    pub fn set_string(
        &mut self,
        key: impl Into<Vec<u8>>,
        value: impl Into<Vec<u8>>,
    ) -> Result<bool, NulError> {
        let key = CString::new(key)?;
        let value = CString::new(value)?;
        Ok(self.set_string_c(&key, &value))
    }

    pub fn set_item_c(&mut self, key: &CStr, value: &mut Config) -> bool {
        rime_api_call!(
            self.api.raw(),
            config_set_item,
            self.raw_mut(),
            key.as_ptr(),
            value.raw_mut()
        ) != 0
    }

    pub fn set_item(
        &mut self,
        key: impl Into<Vec<u8>>,
        value: &mut Config,
    ) -> Result<bool, NulError> {
        let key = CString::new(key)?;
        Ok(self.set_item_c(&key, value))
    }

    pub fn clear_c(&mut self, key: &CStr) -> bool {
        rime_api_call!(self.api.raw(), config_clear, self.raw_mut(), key.as_ptr()) != 0
    }

    pub fn clear(&mut self, key: impl Into<Vec<u8>>) -> Result<bool, NulError> {
        let key = CString::new(key)?;
        Ok(self.clear_c(&key))
    }

    pub fn create_list_c(&mut self, key: &CStr) -> bool {
        rime_api_call!(
            self.api.raw(),
            config_create_list,
            self.raw_mut(),
            key.as_ptr()
        ) != 0
    }

    pub fn create_list(&mut self, key: impl Into<Vec<u8>>) -> Result<bool, NulError> {
        let key = CString::new(key)?;
        Ok(self.create_list_c(&key))
    }

    pub fn create_map_c(&mut self, key: &CStr) -> bool {
        rime_api_call!(
            self.api.raw(),
            config_create_map,
            self.raw_mut(),
            key.as_ptr()
        ) != 0
    }

    pub fn create_map(&mut self, key: impl Into<Vec<u8>>) -> Result<bool, NulError> {
        let key = CString::new(key)?;
        Ok(self.create_map_c(&key))
    }
}
