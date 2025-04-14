use crate::{impl_getters, struct_impl_reference};

struct_impl_reference!(Composition);

impl_getters! {
    Composition,
    /// 獲取長度。
    length: int,
    /// 獲取光標位置。
    cursor_pos: int,
    /// 獲取選擇起始。
    sel_start: int,
    /// 獲取選擇結束。
    sel_end: int,
    /// 獲取 preedit.
    preedit: str,
}
