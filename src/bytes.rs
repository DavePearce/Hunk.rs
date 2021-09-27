use crate::Hunk;

pub const EMPTY : ByteHunk<'static> = ByteHunk{ bytes: &[], offset: 0 };

// ===========================================================
// Byte Hunk
// ===========================================================

pub struct ByteHunk<'a> {
    // Reference to underlying hunk
    bytes: &'a [u8],
    // Offset within hunk
    offset: usize,    
}

impl<'a> ByteHunk<'a> {
    pub fn new(bytes: &'a [u8],offset: usize) -> ByteHunk<'a> {
	ByteHunk{bytes,offset}
    }

    pub fn skip(&self,n:usize) -> ByteHunk<'a> {
	ByteHunk{bytes:self.bytes,offset:self.offset+n}
    }

    fn read_i32(&self) -> i32 {
	let mut raw : [u8;4] = [0;4];
	raw[0] = self.bytes[self.offset];
	raw[1] = self.bytes[self.offset + 1];
	raw[2] = self.bytes[self.offset + 2];
	raw[3] = self.bytes[self.offset + 3];
	return i32::from_le_bytes(raw);
    }
}

impl<'a> Hunk for ByteHunk<'a> {
    
    fn len(&self) -> usize {
	self.bytes.len()
    }

    fn read_u8(&self, offset: usize) -> u8 {
	self.bytes[offset]
    }
}
