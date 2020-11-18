use super::*;

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
}

pub struct RsDictIterator<'a> {
    father: &'a RsDict,
    current_code: u64,
    ptr: usize,
    index: usize,
    max_index: usize,
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
            max_index: father.sb_classes.len(),
        }
    }
}

impl<'a> Iterator for RsDictIterator<'a> {
    type Item = u64;
    /// The iteration code takes inspiration from https://lemire.me/blog/2018/02/21/iterating-over-set-bits-quickly/
    fn next(&mut self) -> Option<Self::Item> {
        // if we have no values left, then read a new u64 chunk from the Rsdict
        if self.current_code == 0 { 
            // find the next not empty word
            self.current_code = loop {
                self.index += 1;
                // if its the last block just dump it
                if self.index == self.max_index {
                    break self.father.last_block.bits;
                }
                // if we are over just end the iterator
                if self.index > self.max_index {
                    return None;
                }
                // we are in an valid index so we must decode the code
                let class =  self.father.sb_classes[self.index];
                // we care only about ones so an empty word can be skipped
                if class == 0 {
                    continue;
                }
                // we have ones in the current code so we can decode it
                let code_length = ENUM_CODE_LENGTH[class as usize] as usize;
                let code = self.father.sb_indices.get(self.ptr, code_length);
                self.ptr += code_length;
                break enum_code::decode(code, class);
            };
        }
        
        // get the index of the first one (we are guaranteed to have
        // at least one bit set to 1)
        let t = self.current_code.trailing_zeros();
        // clear it from the current code
        self.current_code ^= 1 << t;
        // compute the result value
        let result = self.index as u64 * 64 + t as u64;

        Some(result)
    }
}