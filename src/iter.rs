use super::*;
use std::ops::{Range, Index};

impl<'a> IntoIterator for &'a RsDict {
    type Item = u64;
    type IntoIter = RsDictIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RsDictIterator::new(self)
    }
}

impl<'a> RsDict {
    /// return an Iterator over the indices of the bits set to one in the RsDict.
    pub fn iter(&'a self) -> RsDictIterator<'a> {
        self.into_iter()
    }

    /// return an Iterator over the indices of the bits set to one in the RsDict which falls in the range `range`.
    pub fn iter_range_values(&'a self, range: Range<u64>) -> RsDictIterator<'a> {
        RsDictIterator::new_range_values(self, range)
    }

    /// return an Iterator over the indices of the bits set to one in the RsDict whichs indices falls in the range `range`.
    pub fn iter_range_indicies(&'a self, range: Range<usize>) -> RsDictIterator<'a> {
        RsDictIterator::new_range_indices(self, range)
    }
} 

pub struct RsDictIterator<'a> {
    father: &'a RsDict,
    current_code: u64,
    ptr: usize,
    index: usize,
    max_index: Option<usize>,
    max_value: Option<u64>,
}

impl<'a> RsDictIterator<'a> {
    pub fn new(father: &RsDict) -> RsDictIterator {
        let class = father.sb_classes[0];
        let code_length = ENUM_CODE_LENGTH[class as usize] as usize;
        let code = father.sb_indices.get(0, code_length);
        let current_code = enum_code::decode(code, class);
        RsDictIterator{
            father:father,
            current_code: current_code,
            ptr: 0,
            index: 0,
            max_index: None,
            max_value: None,
        }
    }

    pub fn new_range_values(father: &RsDict, range: Range<u64>) -> RsDictIterator {
        let class = father.sb_classes[0];
        let code_length = ENUM_CODE_LENGTH[class as usize] as usize;
        let code = father.sb_indices.get(0, code_length);
        let current_code = enum_code::decode(code, class);
        RsDictIterator{
            father:father,
            current_code: current_code,
            ptr: 0,
            index: 0,
            max_index: None,
            max_value: Some(range.end),
        }
    }

    pub fn new_range_indices(father: &RsDict, range: Range<usize>) -> RsDictIterator {
        let class = father.sb_classes[0];
        let code_length = ENUM_CODE_LENGTH[class as usize] as usize;
        let code = father.sb_indices.get(0, code_length);
        let current_code = enum_code::decode(code, class);
        RsDictIterator{
            father:father,
            current_code: current_code,
            ptr: 0,
            index: 0,
            max_index: Some(range.end),
            max_value: None,
        }
    }
}

impl<'a> Iterator for RsDictIterator<'a> {
    type Item = u64;
    /// The iteration code takes inspiration from https://lemire.me/blog/2018/02/21/iterating-over-set-bits-quickly/
    fn next(&mut self) -> Option<Self::Item> {
        // if we have no values left, then read a new u64 chunk from the Rsdict
        if self.current_code == 0 { 
            self.index += 1;
            // find the next not empty word
            let class = loop {
                if let Some(max) = &self.max_index {
                    if self.index > *max {
                        return None;
                    }
                }
                let current_class =  self.father.sb_classes[self.index];
                if current_class != 0 {
                    break current_class;
                }
                self.index += 1;
                if self.index >= self.father.sb_classes.len() {
                    return None;
                }
            };
            // retreive the code
            let code_length = ENUM_CODE_LENGTH[class as usize] as usize;
            let code = self.father.sb_indices.get(self.ptr, code_length);
            self.ptr += code_length;
            
            // decompress the word
            self.current_code = enum_code::decode(code, class);
        }
        
        // get the index of the first one (we are guaranteed to have
        // at least one bit set to 1)
        let t = self.current_code.trailing_zeros();
        // clear it from the current code
        self.current_code ^= 1 << t;
        // compute the result value
        let result = self.index as u64 * 64 + t as u64;

        // check if we are at the end
        if let Some(max) = &self.max_value{
            if result > *max {
                return None;
            }
        }

        Some(result)
    }
}