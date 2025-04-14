/// Rime api call
#[macro_export]
macro_rules! rime_api_call {
    ($api:expr, $f:ident $(,$args:expr)*) => {
        unsafe {
            (*$api.as_ref()).$f.unwrap()($($args),*)
        }
    };
}

#[macro_export]
macro_rules! struct_impl_managed {
    ($name:ident $(,$drop_fn:ident)?) => {
        paste::paste! {
            pub struct $name<'a> {
                api: &'a crate::Rime,
                raw: librime_sys::[<Rime $name>],
            }

            impl<'a> $name<'a> {
                pub fn from_raw(api: &'a super::Rime, raw: librime_sys::[<Rime $name>]) -> Self {
                    Self { api, raw }
                }

                pub fn raw(&self) -> &librime_sys::[<Rime $name>] {
                    &self.raw
                }

                pub fn raw_mut(&mut self) -> &mut librime_sys::[<Rime $name>] {
                    &mut self.raw
                }
            }
            struct_impl_managed!(@impl_drop $name $($drop_fn)?);
        }
    };
    // custom drop
    (@impl_drop $name:ident $drop_fn:ident) => {
        impl<'a> Drop for $name<'a> {
            fn drop(&mut self) {
                crate::rime_api_call!(self.api.raw(), $drop_fn, self.raw_mut());
            }
        }
    };
    // default drop
    (@impl_drop $name:ident) => {
        paste::paste! {
            impl<'a> Drop for $name<'a> {
                fn drop(&mut self) {
                    crate::rime_api_call!(self.api.raw(), [<free_ $name:snake>], self.raw_mut());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! struct_impl_reference {
    ($name:ident) => {
        paste::paste! {
            pub struct $name<'a> {
                raw: &'a librime_sys::[<Rime $name>],
            }

            impl<'a> $name<'a> {
                pub fn from_raw(raw: &'a librime_sys::[<Rime $name>]) -> Self {
                    Self { raw }
                }

                pub fn raw(&self) -> &librime_sys::[<Rime $name>] {
                    self.raw
                }
            }
        }
    };
}

/// 實現屬性獲取。
#[macro_export]
macro_rules! impl_getters {
    // entry
    (
        $name:ident,
        $(
            $(#[$attr:meta])*
            $field:ident: $type:tt
        ),*
        $(,)?
    ) => {
        impl<'a> $name<'a> {
            $(
                impl_getters!(@dispatch $(#[$attr])* $field: $type);
            )*
        }
    };
    // get str
    (@dispatch $(#[$attr:meta])* $field:ident: str) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$field _c>](&self) -> Option<&std::ffi::CStr> {
                let ptr = self.raw.$field;
                crate::ptr_to_cstr!(ptr)
            }

            $(#[$attr])*
            pub fn $field(&self) -> Option<Result<&str, std::str::Utf8Error>> {
                self.[<$field _c>]().map(std::ffi::CStr::to_str)
            }
        }
    };
    // get int
    (@dispatch $(#[$attr:meta])* $field:ident: int) => {
        $(#[$attr])*
        pub fn $field(&self) -> i32 {
            self.raw.$field
        }
    };
    // get usize
    (@dispatch $(#[$attr:meta])* $field:ident: usize) => {
        $(#[$attr])*
        pub fn $field(&self) -> usize {
            self.raw.$field
        }
    };
    // get bool
    (@dispatch $(#[$attr:meta])* $field:ident: bool) => {
        $(#[$attr])*
        pub fn $field(&self) -> bool {
            self.raw.$field != 0
        }
    };
}

#[macro_export]
macro_rules! ptr_to_cstr {
    ($ptr:expr) => {
        (!$ptr.is_null()).then(|| unsafe { std::ffi::CStr::from_ptr($ptr) })
    };
}
