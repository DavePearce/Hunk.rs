use crate::Hunk;

pub struct Patch<'a> {
    /// Hunk on which this patch is based
    parent: &'a dyn Hunk
}

impl<'a> Patch<'a> {
    pub fn new(parent: &'a dyn Hunk) -> Patch<'a> {
	Patch{parent}
    }
}

impl<'a> Hunk for Patch<'a> {
    fn len(&self) -> usize {
	self.parent.len()
    }

    fn read_u8(&self, offset: usize) -> u8 {
	self.read_u8(offset)
    }

    fn write_u8(&self, offset: usize, data: u8) -> Patch {
	self.write_u8(offset,data)
    }    
}
