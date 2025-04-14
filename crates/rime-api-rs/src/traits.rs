use std::ffi::{CString, NulError};

use bon::bon;
use librime_sys::{rime_struct, rime_traits_t};

pub struct Traits {
    raw: rime_traits_t,
    _resources: Vec<CString>,
}

#[bon]
impl Traits {
    #[builder]
    pub fn new(
        shared_data_dir: Option<&str>,
        user_data_dir: Option<&str>,
        distribution_name: Option<&str>,
        distribution_code_name: Option<&str>,
        distribution_version: Option<&str>,
        app_name: Option<&str>,
        // TODO: modules
        // modules: Option<Vec<&str>>,
        min_log_level: Option<LogLevel>,
        log_dir: Option<&str>,
        prebuilt_data_dir: Option<&str>,
        staging_dir: Option<&str>,
    ) -> Result<Self, NulError> {
        // initialize
        rime_struct!(raw: rime_traits_t);
        let mut resources = Vec::new();

        macro_rules! set_string_field {
            ($field:ident) => {
                if let Some(s) = $field {
                    let c = CString::new(s)?;
                    raw.$field = c.as_ptr();
                    resources.push(c);
                }
            };
        }

        set_string_field!(shared_data_dir);
        set_string_field!(user_data_dir);
        set_string_field!(distribution_name);
        set_string_field!(distribution_code_name);
        set_string_field!(distribution_version);
        set_string_field!(app_name);

        if let Some(level) = min_log_level {
            raw.min_log_level = level as i32;
        }

        set_string_field!(log_dir);
        set_string_field!(prebuilt_data_dir);
        set_string_field!(staging_dir);

        Ok(Self {
            raw,
            _resources: resources,
        })
    }
}

impl Traits {
    pub fn raw_mut(&mut self) -> &mut rime_traits_t {
        &mut self.raw
    }
}

pub enum LogLevel {
    Info = 0,
    Warning = 1,
    Error = 2,
    Fatal = 3,
}
