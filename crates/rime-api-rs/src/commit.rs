use crate::{impl_getters, struct_impl_managed};

struct_impl_managed!(Commit);

impl_getters! {
    Commit,
    /// 獲取提交文本。
    text: str,
}
