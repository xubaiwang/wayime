use std::marker::PhantomData;

use librime_sys::RimeCandidate;

use crate::{impl_getters, struct_impl_reference, Candidate};

struct_impl_reference!(Menu);

impl_getters! {
    Menu,
    /// 獲取頁面大小。
    page_size: int,
    /// 獲取頁碼。
    page_no: int,
    /// 是否是最後一頁
    is_last_page: bool,
    /// 高亮候選位置。
    highlighted_candidate_index: int,
    /// 候選词數量。
    num_candidates: int,
    /// 選擇鍵。
    select_keys: str,
}

impl<'a> Menu<'a> {
    pub fn candidates(&self) -> CandidateIter {
        CandidateIter {
            raw: self.raw.candidates,
            len: self.raw.num_candidates as isize,
            index: 0,
            _lifetime: PhantomData,
        }
    }
}

pub struct CandidateIter<'a> {
    raw: *mut RimeCandidate,
    len: isize,
    index: isize,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Iterator for CandidateIter<'a> {
    type Item = Option<Candidate<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let ptr = unsafe { self.raw.offset(self.index) };
            let value = unsafe { ptr.as_ref().map(Candidate::from_raw) };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
