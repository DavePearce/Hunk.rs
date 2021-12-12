use crate::Hunk;
use crate::Patch;

pub const EMPTY : ByteHunk<'static> = ByteHunk{ bytes: &[] };

// ===========================================================
// Byte Hunk
// ===========================================================

pub struct ByteHunk<'a> {
    // Reference to underlying slice
    bytes: &'a [u8],
}

impl<'a> ByteHunk<'a> {
    pub fn new(bytes: &'a [u8]) -> ByteHunk<'a> {
	ByteHunk{bytes}
    }

    pub fn skip(&self,n:usize) -> ByteHunk<'a> {
	ByteHunk{bytes:self.bytes}
    }

    pub fn read_i8(&self, offset: usize) -> i8 {
	self.bytes[offset] as i8
    }

    pub fn read_u8(&self, offset: usize) -> u8 {
	self.bytes[offset]
    }

    pub fn read_le_i16(&self, offset: usize) -> i16 {
	let mut raw : [u8;2] = [0;2];
	raw[0] = self.bytes[offset];
	raw[1] = self.bytes[offset + 1];
	i16::from_le_bytes(raw)
    }
           
    pub fn read_le_u16(&self, offset: usize) -> u16 {
	let mut raw : [u8;2] = [0;2];
	raw[0] = self.bytes[offset];
	raw[1] = self.bytes[offset + 1];
	u16::from_le_bytes(raw)
    }
    
    pub fn read_le_i32(&self, offset: usize) -> i32 {
	let mut raw : [u8;4] = [0;4];
	raw[0] = self.bytes[offset];
	raw[1] = self.bytes[offset + 1];
	raw[2] = self.bytes[offset + 2];
	raw[3] = self.bytes[offset + 3];
	i32::from_le_bytes(raw)
    }

    pub fn read_le_u32(&self, offset: usize) -> u32 {
	let mut raw : [u8;4] = [0;4];
	raw[0] = self.bytes[offset];
	raw[1] = self.bytes[offset + 1];
	raw[2] = self.bytes[offset + 2];
	raw[3] = self.bytes[offset + 3];
	u32::from_le_bytes(raw)
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
