use hunk::Region;

#[test]
pub fn test_01() {
    let r = Region::new(1,2);
    assert_eq!(r.offset,1);
    assert_eq!(r.length,2);    
}

#[test]
pub fn test_02() {
    let r1 = Region::new(1,2);
    let r2 = Region::new(1,2);    
    assert_eq!(r1,r2);
}

#[test]
pub fn test_03() {
    let r1 = Region::new(1,2);
    let r2 = Region::new(2,3);    
    assert_ne!(r1,r2);
}

#[test]
pub fn test_04() {
    let r1 = Region::new(1,2);
    let r2 = Region::new(4,3);    
    assert!(r1 < r2);
}

#[test]
pub fn test_05() {
    let r1 = Region::new(1,2);
    let r2 = Region::new(3,3);    
    assert!(r1 < r2);
}


#[test]
pub fn test_06() {
    let r1 = Region::new(3,3);
    let r2 = Region::new(1,2);    
    assert!(r1 > r2);
}
