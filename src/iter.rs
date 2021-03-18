use super::*;
use std::ops::Range;

impl<'a> IntoIterator for &'a RsDict {
    type Item = u64;
    type IntoIter = RsDictIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RsDictIterator::new(self)
    }
}

impl<'a> RsDict {
    /// Return an iterator over all the indices of the bits set to one
    /// which are inside the provided range.
    pub fn iter_in_range(&'a self, range: Range<u64>) -> Option<RsDictIterator<'a>> {
        RsDictIterator::new_in_range(self, range)
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
    max: Option<u64>,
}

impl<'a> RsDictIterator<'a> {
    /// Create a structure that iter over all the indices of the bits set to one.
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
            max: None,
        }
    }

    /// Create a structure that iter over all the indices of the bits set to one
    /// which are inside the provided range.
    pub fn new_in_range(father: &RsDict, range: Range<u64>) -> Option<RsDictIterator> {
        let pos = range.start;

        // if the start value is bigger than all the rest, return an empty iterator
        if pos >= father.len() as u64 {
            return None;
        }

        let prefix_num_ones = father.num_ones - father.last_block.num_ones;
        // if the start bit is in the last block, clear the code accordingly
        if pos >= father.last_block_ind() {
            // Get the current code
            let mut code = father.last_block.bits;
            // Clear the bits
            code = clear_lower_bits(code, pos - father.last_block_ind());
            // Return the iterator
            return Some(
                RsDictIterator{
                    father:father,
                    current_code: code,
                    ptr: 0, // no need to initialize, it will never be used
                    index: father.last_block_ind() as usize,
                    max_index: father.sb_classes.len(),
                    max: Some(range.end),
                }
            );
        }

        // Start with the rank from our position's large block.
        let lblock = pos / LARGE_BLOCK_SIZE;
        let LargeBlock {
            mut pointer,
            mut rank,
        } = father.large_blocks[lblock as usize];

        // Add in the ranks (i.e. the classes) per small block up to our
        // position's small block.
        let sblock_start = (lblock * SMALL_BLOCK_PER_LARGE_BLOCK) as usize;
        let sblock = (pos / SMALL_BLOCK_SIZE) as usize;
        // Scan the small blocks from the start of the large block
        // to the current small block to compute the pointer in the enumerative
        // codes array.
        let (class_sum, length_sum) =
            rank_acceleration::scan_block(&father.sb_classes, sblock_start, sblock);
        // Update the pointer and the rank with the sum of the values,
        rank += class_sum;
        pointer += length_sum;
        // Get the class of the current block
        let sb_class = father.sb_classes[sblock];
        // Read the code
        let mut  code = father.read_sb_index(pointer, ENUM_CODE_LENGTH[sb_class as usize]);
        // filter the lower bits
        code = clear_lower_bits(code, pos - rank);
        // Create the iterator
        Some(
            RsDictIterator{
                father:father,
                current_code: code,
                ptr: pointer as usize, // no need to initialize, it will never be used
                index: sblock,
                max_index: father.sb_classes.len(),
                max: Some(range.end),
            }
        )
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
                self.ptr += code_length;
                let code = self.father.sb_indices.get(self.ptr, code_length);
                break enum_code::decode(code, class);
            };
        }
        
        // get the index of the first one (we are guaranteed to have
        // at least one bit set to 1)
        let t = self.current_code.trailing_zeros();
        
        // clear it from the current code
        self.current_code = clear_lowest_bit_set(self.current_code);

        // compute the result value
        let result = self.index as u64 * SMALL_BLOCK_SIZE + t as u64;

        // Check if we exceeds the max value
        if let Some(_max) = &self.max {
            if result >= *_max {
                return None;
            }
        }

        Some(result)
    }
}

#[inline(always)]
/// Clear the lowest set bit.
fn clear_lowest_bit_set(x: u64) -> u64 {
    // if possible, use the fast instruction
    #[cfg(target_feature="bmi1")]
    {
        return unsafe{core::arch::x86_64::_blsr_u64(x)};
    }
    // Otherwise fall down to the generic implementation
    #[cfg(not(target_feature="bmi1"))]
    {
        x & (x - 1)
    }
}

#[inline(always)]
/// Clear the lowest num bits.
fn clear_lower_bits(code: u64, num:u64) -> u64 {
    code & u64::MAX.wrapping_shl(num as u32)
}