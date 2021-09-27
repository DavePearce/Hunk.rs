mod byte_hunk;
mod byte_patch;
mod hunk_ref;
mod tests;

use crate::byte_patch::Patch;

// Stuf goes here

// ===========================================================
// Hunk
// ===========================================================

pub trait Hunk {
    /// Determine length of this hunk
    fn len(&self) -> usize;

    /// Read a byte from this hunk
    fn read_u8(&self,offset: usize) -> u8;

    /// Write a byte into this hunk, producing a patched hunk.
    fn write_u8(&self,offset: usize, data: u8) -> Patch; 
}

