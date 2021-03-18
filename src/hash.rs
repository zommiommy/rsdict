use super::*;
use std::hash::{Hash, Hasher};

impl Hash for RsDict {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iter().for_each(|x| x.hash(state));
    }
}