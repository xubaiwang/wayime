use std::marker::PhantomData;

use librime_sys::RimeSchemaListItem;

use crate::{impl_getters, struct_impl_managed, struct_impl_reference};

struct_impl_managed!(SchemaList);
struct_impl_reference!(SchemaListItem);

impl_getters! {
    SchemaList,
    size: usize,
}

impl<'a> SchemaList<'a> {
    pub fn list(&self) -> SchemaListItemIter {
        SchemaListItemIter {
            raw: self.raw.list,
            len: self.raw.size,
            index: 0,
            _lifetime: PhantomData,
        }
    }
}

impl_getters! {
    SchemaListItem,
    schema_id: str,
    name: str,
}

pub struct SchemaListItemIter<'a> {
    raw: *mut RimeSchemaListItem,
    len: usize,
    index: usize,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Iterator for SchemaListItemIter<'a> {
    type Item = Option<SchemaListItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let ptr = unsafe { self.raw.offset(self.index as isize) };
            let value = unsafe { ptr.as_ref().map(SchemaListItem::from_raw) };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
