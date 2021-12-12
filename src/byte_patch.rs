use crate::Hunk;
use crate::Rewrite;

pub struct Patch<'a,T:Hunk> {
    /// Hunk on which this patch is based
    parent: &'a T,
    /// List of patches in sorted order
    patches: Vec<Rewrite>
}

impl<'a,T:Hunk> Patch<'a,T> {
    pub fn new(parent: &'a T) -> Patch<'a,T> {
	Patch{parent, patches:Vec::new()}
    }

    pub fn write_u8(self, offset: usize, data: u8) -> Self {
	unimplemented!()
    }
}
