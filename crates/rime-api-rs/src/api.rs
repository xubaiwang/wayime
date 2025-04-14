use std::{
    ffi::{c_void, CStr, CString, NulError},
    ptr::NonNull,
    str::Utf8Error,
};

use librime_sys::{rime_api_t, rime_get_api};

use crate::{ptr_to_cstr, rime_api_call, Config, SchemaList, Session, Traits};

pub struct Rime(NonNull<rime_api_t>);

impl Rime {
    pub fn new() -> Option<Self> {
        NonNull::new(unsafe { rime_get_api() }).map(Self)
    }

    pub fn from_raw(raw: NonNull<rime_api_t>) -> Self {
        Self(raw)
    }

    pub fn raw(&self) -> NonNull<rime_api_t> {
        self.0
    }
}

/// Setup.
impl Rime {
    pub fn setup(&self, traits: &mut Traits) {
        rime_api_call!(self.0, setup, traits.raw_mut());
    }
}

/// Notification.
impl Rime {
    pub fn set_notification_handler_c<F>(&self, handle: F)
    where
        F: FnMut(usize, &CStr, &CStr),
    {
        rime_api_call!(
            self.0,
            set_notification_handler,
            Some(wrapper::<F>),
            Box::into_raw(Box::new(handle)).cast()
        );

        unsafe extern "C" fn wrapper<F>(
            context: *mut c_void,
            session_id: usize,
            message_type: *const i8,
            message_value: *const i8,
        ) where
            F: FnMut(usize, &CStr, &CStr),
        {
            let handle = &mut *context.cast::<F>();
            let message_type = CStr::from_ptr(message_type);
            let message_value = CStr::from_ptr(message_value);
            handle(session_id, message_type, message_value);
        }
    }

    pub fn set_notification_handler<F>(&self, mut handle: F)
    where
        F: FnMut(usize, &str, &str),
    {
        self.set_notification_handler_c(|session_id, message_type, message_value| {
            // 這裏假設 librime 側不會出錯，故直接 unwrap
            let message_type = message_type.to_str().unwrap();
            let message_value = message_value.to_str().unwrap();
            handle(session_id, message_type, message_value)
        });
    }
}

/// Entry.
impl Rime {
    pub fn initialize(&self, traits: &mut Traits) {
        rime_api_call!(self.0, initialize, traits.raw_mut());
    }

    pub fn start_maintenance(&self, full_check: bool) -> bool {
        rime_api_call!(self.0, start_maintenance, full_check as i32) != 0
    }

    pub fn is_maintenance_mode(&self) -> bool {
        rime_api_call!(self.0, is_maintenance_mode) != 0
    }

    pub fn join_maintenance_thread(&self) {
        rime_api_call!(self.0, join_maintenance_thread);
    }
}

/// Exit.
impl Drop for Rime {
    fn drop(&mut self) {
        rime_api_call!(self.0, finalize);
    }
}

/// Deployment.
impl Rime {
    pub fn deployer_initialize(&self, traits: &mut Traits) {
        rime_api_call!(self.0, deployer_initialize, traits.raw_mut());
    }

    pub fn prebuild(&self) -> bool {
        rime_api_call!(self.0, prebuild) != 0
    }

    pub fn deploy(&self) -> bool {
        rime_api_call!(self.0, deploy) != 0
    }

    pub fn deploy_schema_c(&self, schema_file: &CStr) -> bool {
        rime_api_call!(self.0, deploy_schema, schema_file.as_ptr()) != 0
    }

    pub fn deploy_schema(&self, schema_file: impl Into<Vec<u8>>) -> Result<bool, NulError> {
        let schema_file = CString::new(schema_file)?;
        Ok(self.deploy_schema_c(&schema_file))
    }

    pub fn deploy_config_file_c(&self, file_name: &CStr, version_key: &CStr) -> bool {
        rime_api_call!(
            self.0,
            deploy_config_file,
            file_name.as_ptr(),
            version_key.as_ptr()
        ) != 0
    }

    pub fn deploy_config_file(
        &self,
        file_name: impl Into<Vec<u8>>,
        version_key: impl Into<Vec<u8>>,
    ) -> Result<bool, NulError> {
        let file_name = CString::new(file_name)?;
        let version_key = CString::new(version_key)?;
        Ok(self.deploy_config_file_c(&file_name, &version_key))
    }

    pub fn sync_user_data(&self) -> bool {
        rime_api_call!(self.0, sync_user_data) != 0
    }
}

/// Session management.
impl Rime {
    pub fn create_session(&self) -> Session {
        let id = rime_api_call!(self.0, create_session);
        Session::from_id(self, id)
    }

    pub fn cleanup_stale_sessions(&self) {
        rime_api_call!(self.0, cleanup_stale_sessions);
    }

    pub fn cleanup_all_sessions(&self) {
        rime_api_call!(self.0, cleanup_all_sessions);
    }
}

impl Rime {
    pub fn get_schema_list(&self, schema_list: &mut SchemaList) -> bool {
        rime_api_call!(self.raw(), get_schema_list, schema_list.raw_mut()) != 0
    }
}

/// Runtime options.
impl Rime {
    pub fn schema_open_c(&self, schema_id: &CStr, config: &mut Config) -> bool {
        rime_api_call!(
            self.raw(),
            schema_open,
            schema_id.as_ptr(),
            config.raw_mut()
        ) != 0
    }

    pub fn schema_open(
        &self,
        schema_id: impl Into<Vec<u8>>,
        config: &mut Config,
    ) -> Result<bool, NulError> {
        let schema_id = CString::new(schema_id)?;
        Ok(self.schema_open_c(&schema_id, config))
    }

    pub fn config_open_c(&self, config_id: &CStr, config: &mut Config) -> bool {
        rime_api_call!(
            self.raw(),
            config_open,
            config_id.as_ptr(),
            config.raw_mut()
        ) != 0
    }

    pub fn config_open(
        &self,
        config_id: impl Into<Vec<u8>>,
        config: &mut Config,
    ) -> Result<bool, NulError> {
        let config_id = CString::new(config_id)?;
        Ok(self.config_open_c(&config_id, config))
    }
}

/// Configuration.
impl Rime {
    // TODO: config_get_{bool,int,double,string,cstring}
    // TODO: config_update_signature
    // TODO: config_begin_,ap
    // TODO: config_next
    // TODO: config_end
}

/// 模塊。
impl Rime {
    // TODO: register_module
    // TODO: find_module

    pub fn run_task_c(&self, task_name: &CStr) -> bool {
        rime_api_call!(self.raw(), run_task, task_name.as_ptr()) != 0
    }

    pub fn run_task(&self, task_name: impl Into<Vec<u8>>) -> Result<bool, NulError> {
        let task_name = CString::new(task_name)?;
        Ok(self.run_task_c(&task_name))
    }
}

impl Rime {
    /// 獲取用戶名。
    pub fn get_user_id_c(&self) -> Option<&CStr> {
        let ptr = rime_api_call!(self.raw(), get_user_id);
        ptr_to_cstr!(ptr)
    }

    /// 獲取用戶名。
    pub fn get_user_id(&self) -> Option<&str> {
        // 假設無 Utf8Error
        self.get_user_id_c().map(CStr::to_str).map(Result::unwrap)
    }

    pub fn get_user_data_sync_dir(&self, buf: &mut [i8]) {
        rime_api_call!(
            self.raw(),
            get_user_data_sync_dir,
            buf.as_mut_ptr(),
            buf.len()
        )
    }
}

impl Rime {
    // TODO: config_init
    // TODO: config_load_string
    // TODO: config_set_{bool,int,double,string}
    // TODO: config_get_item
    // TODO: config_set_item
    // TODO: config_clear
    // TODO: config_create_list
    // TODO: config_create_map
    // TODO: config_list_size
    // TODO: config_begin_list
    // TODO: user_config_open
}

impl Rime {
    pub fn get_version_c(&self) -> Option<&CStr> {
        let ptr = rime_api_call!(self.raw(), get_version);
        ptr_to_cstr!(ptr)
    }

    pub fn get_version(&self) -> Option<Result<&str, Utf8Error>> {
        self.get_version_c().map(CStr::to_str)
    }
}

macro_rules! impl_get_dir_s {
    ($($name:ident),* $(,)?) => {
        impl Rime {
            $(
                impl_get_dir_s!(@each $name);
            )*
        }
    };
    // 每個路徑
    (@each $name:ident) => {
        paste::paste! {
            pub fn [<get_ $name _dir_s>](&self, dir: &mut [i8]) {
                rime_api_call!(
                    self.raw(),
                    [<get_ $name _dir_s>],
                    dir.as_mut_ptr(),
                    dir.len()
                )
            }
        }
    };
}

impl_get_dir_s! {
    shared_data,
    user_data,
    prebuilt_data,
    staging,
    sync,
}

// TODO: RIME_MODULE_INITIALIZER
// TODO: RIME_REGISTER_MODULE
// TODO: RIME_REGISTER_CUSTOM_MODULE
