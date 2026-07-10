// The cast guards are compiled out in release builds.
#![cfg(debug_assertions)]
use crate::{TrunIt, CastIt};
macro_rules! f64panic {
    ($($t: ident),*) => {
        $(
        #[should_panic]
        #[test]
        fn $t() {
            let three = 3.3f64;
            three.$t();
        }
        )*
    };
}
f64panic!(u8, u16, u32, u64, usize);
f64panic!(i8, i16, i32, i64, isize);

#[should_panic]
#[test]
fn trun_negative() {
    let three = -3.0f64;
    three.tu8();
}
mod castfrom {
use crate::castfrom::CastFrom;
macro_rules! cast_from {
    ($($t: ident),*) => {
        $(
        #[should_panic]
        #[test]
        fn $t() {
            let three = 3.3f64;
            $t::cast_from(three);
        }
        )*
    };
}
cast_from!(u8, u16, u32, u64, usize);
cast_from!(i8, i16, i32, i64, isize);
}
