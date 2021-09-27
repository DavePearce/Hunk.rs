use std::io::Read;
use std::mem::size_of;

// trait Layout {
//     /// Item this layout correspons with
//     type Item,
//     /// Calculates the size of this object 
//     fn len(&self) -> usize,
//     // Instantiate item
//     fn instantiate(&self) -> Item
// }

type HunkReader<'a,T> = fn(&Hunk<'a>)->T;

// ===========================================================
// Hunk
// ===========================================================

struct Hunk<'a> {
    // Reference to underlying hunk
    bytes: &'a [u8],
    // Offset within hunk
    offset: usize,    
}

impl<'a> Hunk<'a> {
    pub fn new(bytes: &'a [u8],offset: usize) -> Hunk<'a> {
	Hunk{bytes,offset}
    }

    pub fn skip(&self,n:usize) -> Hunk<'a> {
	Hunk{bytes:self.bytes,offset:self.offset+n}
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
    hunk : Hunk<'a>,
    // Item Reader
    reader: HunkReader<'a,T>
}

impl<'a,T> Ref<'a,T> {
    pub fn new(hunk: Hunk<'a>, reader: HunkReader<'a,T>) -> Ref<'a,T> {
	Ref{hunk,reader}
    }

    pub fn get(&self) -> T {
	(self.reader)(&self.hunk)
    }
}

// ===========================================================
// Example Data type
// ===========================================================

struct Point {
    x: i32,
    y: i32
}

fn read_point<'a>(hunk: &Hunk<'a>) -> Point {
    let x = hunk.read_i32();
    let y = hunk.skip(size_of::<i32>()).read_i32();
    Point{x,y}
}

pub fn main() {
    let bs : [u8;8] = [1,0,0,0, 2,0,0,0];
    let hunk = Hunk::new(&bs,0);
    let r = Ref::new(hunk,read_point);
    let pt : Point = r.get();
    println!("Read {},{}!",pt.x,pt.y);
}
