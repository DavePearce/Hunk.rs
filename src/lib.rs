use std::io::Read;

mod tests;

type HunkReader<'a,T> = fn(&ByteHunk<'a>)->T;

// Stuf goes here
pub const EMPTY : ByteHunk<'static> = ByteHunk{ bytes: &[], offset: 0 };

// ===========================================================
// Hunk
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

// ===========================================================
// Hunk Ref
// ===========================================================

struct Ref<'a,T> {
    hunk : ByteHunk<'a>,
    // Item Reader
    reader: HunkReader<'a,T>
}

impl<'a,T> Ref<'a,T> {
    pub fn new(hunk: ByteHunk<'a>, reader: HunkReader<'a,T>) -> Ref<'a,T> {
	Ref{hunk,reader}
    }

    pub fn get(&self) -> T {
	(self.reader)(&self.hunk)
    }
}
