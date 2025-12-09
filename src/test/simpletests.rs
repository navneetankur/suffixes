use crate::{trunit::TrunIt, CastFrom, CastIt};
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_u8() {
    let three = 3u8;
    assert_eq!(three.u8(), three as u8);
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);

}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_u16() {
    let three = 3u16;
    assert_eq!(three.u8(), three as u8);
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_u32() {
    let three = 3u32;
    assert_eq!(three.u8(), three as u8);
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_u64() {
    let three = 3u64;
    assert_eq!(three.u8(), three as u8);
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_usize() {
    let three = 3usize;
    assert_eq!(three.u8(), three as u8);
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_i8() {
    let three = 3i8;
    #[allow(clippy::unnecessary_cast)]
    { assert_eq!(three.u8(), three as u8); }
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_i16() {
    let three = 3i16;
    assert_eq!(three.u8(), three as u8);
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_i32() {
    let three = 3i32;
    assert_eq!(three.u8(), three as u8);
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_i64() {
    let three = 3i64;
    assert_eq!(three.u8(), three as u8);
    assert_eq!(three.u16(), three as u16);
    assert_eq!(three.u32(), three as u32);
    assert_eq!(three.u64(), three as u64);
    assert_eq!(three.usize(), three as usize);
    assert_eq!(three.u(), three as usize);
    assert_eq!(three.i8(), three as i8);
    assert_eq!(three.i16(), three as i16);
    assert_eq!(three.i32(), three as i32);
    assert_eq!(three.i64(), three as i64);
    assert_eq!(three.isize(), three as isize);
    assert_eq!(three.f32(), three as f32);
    assert_eq!(three.f64(), three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_isize() {
    let three = 3isize;
    assert_eq!(three.u8()    , three as u8);
    assert_eq!(three.u16()   , three as u16);
    assert_eq!(three.u32()   , three as u32);
    assert_eq!(three.u64()   , three as u64);
    assert_eq!(three.usize() , three as usize);
    assert_eq!(three.u()     , three as usize);
    assert_eq!(three.i8()    , three as i8);
    assert_eq!(three.i16()   , three as i16);
    assert_eq!(three.i32()   , three as i32);
    assert_eq!(three.i64()   , three as i64);
    assert_eq!(three.isize() , three as isize);
    assert_eq!(three.f32()   , three as f32);
    assert_eq!(three.f64()   , three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}

#[test]
#[allow(clippy::unnecessary_cast)]
fn test_f32() {
    let three = 3f32;
    assert_eq!(three.u8()    , three as u8);
    assert_eq!(three.u16()   , three as u16);
    assert_eq!(three.u32()   , three as u32);
    assert_eq!(three.u64()   , three as u64);
    assert_eq!(three.usize() , three as usize);
    assert_eq!(three.u()     , three as usize);
    assert_eq!(three.i8()    , three as i8);
    assert_eq!(three.i16()   , three as i16);
    assert_eq!(three.i32()   , three as i32);
    assert_eq!(three.i64()   , three as i64);
    assert_eq!(three.isize() , three as isize);
    assert_eq!(three.f32()   , three as f32);
    assert_eq!(three.f64()   , three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_f64() {
    let three = 3f64;
    assert_eq!(three.u8()    , three as u8);
    assert_eq!(three.u16()   , three as u16);
    assert_eq!(three.u32()   , three as u32);
    assert_eq!(three.u64()   , three as u64);
    assert_eq!(three.usize() , three as usize);
    assert_eq!(three.u()     , three as usize);
    assert_eq!(three.i8()    , three as i8);
    assert_eq!(three.i16()   , three as i16);
    assert_eq!(three.i32()   , three as i32);
    assert_eq!(three.i64()   , three as i64);
    assert_eq!(three.isize() , three as isize);
    assert_eq!(three.f32()   , three as f32);
    assert_eq!(three.f64()   , three as f64);

    assert_eq!(u8::cast_from(three), three as u8);
    assert_eq!(u16::cast_from(three), three as u16);
    assert_eq!(u32::cast_from(three), three as u32);
    assert_eq!(u64::cast_from(three), three as u64);
    assert_eq!(usize::cast_from(three), three as usize);
    assert_eq!(i8::cast_from(three), three as i8);
    assert_eq!(i16::cast_from(three), three as i16);
    assert_eq!(i32::cast_from(three), three as i32);
    assert_eq!(i64::cast_from(three), three as i64);
    assert_eq!(isize::cast_from(three), three as isize);
    assert_eq!(f32::cast_from(three), three as f32);
    assert_eq!(f64::cast_from(three), three as f64);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_trun_f32() {
    let three = 3.3f32;
    assert_eq!(three.tu8()    , three as u8);
    assert_eq!(three.tu16()   , three as u16);
    assert_eq!(three.tu32()   , three as u32);
    assert_eq!(three.tu64()   , three as u64);
    assert_eq!(three.tusize() , three as usize);
    assert_eq!(three.tu()     , three as usize);
    assert_eq!(three.ti8()    , three as i8);
    assert_eq!(three.ti16()   , three as i16);
    assert_eq!(three.ti32()   , three as i32);
    assert_eq!(three.ti64()   , three as i64);
    assert_eq!(three.tisize() , three as isize);
}
#[test]
#[allow(clippy::unnecessary_cast)]
fn test_trun_f64() {
    let three = 3.3f64;
    assert_eq!(three.tu8()    , three as u8);
    assert_eq!(three.tu16()   , three as u16);
    assert_eq!(three.tu32()   , three as u32);
    assert_eq!(three.tu64()   , three as u64);
    assert_eq!(three.tusize() , three as usize);
    assert_eq!(three.tu()     , three as usize);
    assert_eq!(three.ti8()    , three as i8);
    assert_eq!(three.ti16()   , three as i16);
    assert_eq!(three.ti32()   , three as i32);
    assert_eq!(three.ti64()   , three as i64);
    assert_eq!(three.tisize() , three as isize);
}

#[test]
#[should_panic]
fn test_panic_32() {
    let minus_three = -3_i32;
    minus_three.u32();
}
