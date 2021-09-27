use std::io::Read;

mod bytes;
mod refs;
mod tests;

use refs::Ref;
use bytes::ByteHunk;

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
