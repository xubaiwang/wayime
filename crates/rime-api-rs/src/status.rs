use crate::{impl_getters, struct_impl_managed};

struct_impl_managed!(Status);

impl_getters! {
    Status,
    /// 獲取輸入法編號。
    schema_id: str,
    /// 獲取輸入法名稱。
    schema_name: str,
    /// 是否禁用。
    is_disabled: bool,
    /// 是否使用組合模式。
    is_composing: bool,
    /// 是否使用 ASCII 模式。
    is_ascii_mode: bool,
    /// 是否使用全角字符。
    is_full_shape: bool,
    /// 是否使用簡體漢字。
    is_simplified: bool,
    /// 是否使用傳統漢字。
    is_traditional: bool,
    /// 是否使用 ASCII 標點。
    is_ascii_punct: bool,
}
