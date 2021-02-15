use super::*;
use std::hash::{Hash, Hasher};

impl Hash for RsDict {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        self.num_ones.hash(state);
        self.num_zeros.hash(state);

        self.sb_classes.hash(state);
        self.sb_indices.hash(state);

        self.large_blocks.hash(state);

        self.select_one_inds.hash(state);
        self.select_zero_inds.hash(state);
        
        self.last_block.hash(state);
    }
}

impl Hash for LastBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
        self.num_ones.hash(state);
        self.num_zeros.hash(state);
    }
}

impl Hash for LargeBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer.hash(state);
        self.rank.hash(state);
    }
}

impl Hash for VarintBuffer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buf.hash(state);
        self.len.hash(state);
    }
}
