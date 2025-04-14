use std::{ffi::CStr, marker::PhantomData, str::Utf8Error};

use crate::{impl_getters, ptr_to_cstr, struct_impl_managed};

use super::{Composition, Menu};

struct_impl_managed!(Context);

impl_getters! {
    Context,
    commit_text_preview: str,
}

impl<'a> Context<'a> {
    pub fn menu(&self) -> Menu {
        Menu::from_raw(&self.raw.menu)
    }

    pub fn composition(&self) -> Composition {
        Composition::from_raw(&self.raw.composition)
    }
}

impl<'a> Context<'a> {
    pub fn select_labels_c(&self) -> SelectLabelsCIter {
        SelectLabelsCIter {
            raw: self.raw.select_labels,
            _phatom: PhantomData,
        }
    }

    pub fn select_labels(&self) -> impl Iterator<Item = Option<Result<&str, Utf8Error>>> {
        self.select_labels_c()
            .map(|option| option.map(CStr::to_str))
    }
}

pub struct SelectLabelsCIter<'a> {
    raw: *mut *mut i8,
    _phatom: PhantomData<&'a ()>,
}

impl<'a> Iterator for SelectLabelsCIter<'a> {
    type Item = Option<&'a CStr>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ptr) = unsafe { self.raw.as_ref() } {
            let ptr = *ptr;
            let value = ptr_to_cstr!(ptr);
            self.raw = unsafe { self.raw.offset(1) };
            Some(value)
        } else {
            None
        }
    }
}
