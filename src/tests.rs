use std::mem::size_of;

use crate::ByteHunk;
use crate::Ref;

// ===========================================================
// Example Data type
// ===========================================================

struct Point {
    x: i32,
    y: i32
}

fn read_point<'a>(hunk: &ByteHunk<'a>) -> Point {
    let x = hunk.read_i32();
    let y = hunk.skip(size_of::<i32>()).read_i32();
    Point{x,y}
}

#[test]
pub fn test_01() {
    let bs : [u8;8] = [1,0,0,0, 2,0,0,0];
    let hunk = ByteHunk::new(&bs,0);
    let r = Ref::new(hunk,read_point);
    let pt : Point = r.get();
    assert_eq!(pt.x,1);
    assert_eq!(pt.y,2);
}
