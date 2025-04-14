use crate::{impl_getters, struct_impl_reference};

struct_impl_reference!(Candidate);

impl_getters! {
    Candidate,
    /// 獲取候選文本。
    text: str,
    /// 獲取候選註釋。
    comment: str,
}
