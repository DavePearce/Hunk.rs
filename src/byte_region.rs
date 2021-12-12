use std::cmp::{PartialOrd,Ordering};

#[derive(PartialEq,Debug)]
pub struct Region {
    /// Starting point in source hunk of this rewrite.
    pub offset: usize,
    /// Length of source hunk (in bytes) being replaced.
    pub length: usize    
}

impl Region {
    pub fn new(offset: usize, length: usize) -> Self {
	Region{offset,length}
    }
}

impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	unimplemented!("TODO")
    }

    fn lt(&self, other: &Self) -> bool {
	(self.offset + self.length) <= other.offset
    }

    fn gt(&self, other: &Self) -> bool {
	(other.offset + other.length) <= self.offset
    }
}
