use hunk::{Hunk,Ref,ByteHunk};

// ==================================================
// u8
// ==================================================

#[test]
pub fn test_read_u8_01() {
    let hunk : ByteHunk = hunk::from_bytes(&[1,2]);
    assert_eq!(hunk.read_u8(0),1);
}

#[test]
pub fn test_read_u8_02() {
    let hunk : ByteHunk = hunk::from_bytes(&[1,2]);
    assert_eq!(hunk.read_u8(1),2);
}

// ==================================================
// i8
// ==================================================

// ==================================================
// u16
// ==================================================

#[test]
pub fn test_read_u16_01() {
    let hunk : ByteHunk = hunk::from_bytes(&[1,2]);
    assert_eq!(hunk.read_le_u16(0),513);
}

#[test]
pub fn test_read_u16_02() {
    let hunk : ByteHunk = hunk::from_bytes(&[0,128]);
    assert_eq!(hunk.read_le_u16(0),32768);
}

#[test]
pub fn test_read_u16_03() {
    let hunk : ByteHunk = hunk::from_bytes(&[1,128]);
    assert_eq!(hunk.read_le_u16(0),32769);
}

// ==================================================
// i16
// ==================================================

#[test]
pub fn test_read_i16_01() {
    let hunk : ByteHunk = hunk::from_bytes(&[1,2]);
    assert_eq!(hunk.read_le_i16(0),513);
}

#[test]
pub fn test_read_i16_02() {
    let hunk : ByteHunk = hunk::from_bytes(&[0,128]);
    assert_eq!(hunk.read_le_i16(0),-32768);
}

#[test]
pub fn test_read_i16_03() {
    let hunk : ByteHunk = hunk::from_bytes(&[1,128]);
    assert_eq!(hunk.read_le_i16(0),-32767);
}
