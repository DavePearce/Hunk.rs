mod byte_hunk;
mod byte_patch;
mod byte_rewrite;
mod byte_region;
mod hunk_ref;

pub use crate::byte_hunk::ByteHunk;
pub use crate::byte_patch::Patch;
pub use crate::byte_rewrite::Rewrite;
pub use crate::byte_region::Region;
pub use crate::hunk_ref::Ref;

// Stuf goes here

// ===========================================================
// Hunk
// ===========================================================

pub trait Hunk {
    /// Determine length of this hunk
    fn len(&self) -> usize;

    /// Read a byte from this hunk
    fn read_u8(&self,offset: usize) -> u8; 
}

pub fn from_bytes<'a>(bytes: &'a [u8]) -> ByteHunk<'a> {
    ByteHunk::new(bytes)
}
