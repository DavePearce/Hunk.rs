use crate::Hunk;
use crate::hunk_ref::Ref;

// ===========================================================
// Example Data type
// ===========================================================

struct Point {
    x: i32,
    y: i32
}

fn read_point<'a>(hunk: &dyn Hunk) -> Point {
    let x = hunk.read_u8(0);
    let y = hunk.read_u8(1);
    Point{x:x as i32,y:y as i32}
}

#[test]
pub fn test_01() {
    let hunk = crate::from_bytes(&[1,2]);
    let r = Ref::new(&hunk,read_point);
    let pt : Point = r.get();
    assert_eq!(pt.x,1);
    assert_eq!(pt.y,2);
}
