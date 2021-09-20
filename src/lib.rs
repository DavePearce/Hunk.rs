// Stuf goes here
pub const EMPTY : ByteHunk<'static> = ByteHunk{ bytes: &[] };

/// Represents a hunk of raw data constructed from an in-memory byte
/// array.
pub struct ByteHunk<'a> {
    pub bytes: &'a [u8]
}

impl<'a> ByteHunk<'a> {
    pub fn new(bytes: &'a [u8]) -> ByteHunk<'a> {
	ByteHunk{bytes}
    }
}

