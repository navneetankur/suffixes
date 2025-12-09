pub trait CastIt: Sized {
    fn u    (self) -> usize;
    fn u8   (self) -> u8   ;
    fn u16  (self) -> u16  ;
    fn u32  (self) -> u32  ;
    fn u64  (self) -> u64  ;
    fn u128 (self) -> u128  ;
    fn usize(self) -> usize;
    fn i8   (self) -> i8   ;
    fn i16  (self) -> i16  ;
    fn i32  (self) -> i32  ;
    fn i64  (self) -> i64  ;
    fn i128 (self) -> i128;
    fn isize(self) -> isize;
    fn f32  (self) -> f32  ;
    fn f64  (self) -> f64  ;
}
macro_rules! cast_these_float {
    ($($t: ident),*) => {
        $(
        #[inline]
        fn $t(self) -> $t {
            let after = self as $t;
            debug_assert_eq!(self, after as Self);
            return after;
        }
        )*
    };
}
macro_rules! cast_these {
    ($($t: ident),*) => {
        $(
        #[inline]
        fn $t(self) -> $t {
            #[cfg(debug_assertions)]
            {
                #[allow(irrefutable_let_patterns)]
                let Ok(rv) = $t::try_from(self) else {
                    panic!("can't cast {self} to {}", stringify!($t));
                };
                return rv;
            }
            #[cfg(not(debug_assertions))]
            {self as $t}
        }
        )*
    };
}
macro_rules! impl_castit {
    ($($t: ty),*) => {
        $(
        impl CastIt for $t {
            #[inline]
            fn u(self) -> usize {
                self.usize()
            }
            cast_these!(u8, u16, u32, u64, u128, usize);
            cast_these!(i8, i16, i32, i64, i128, isize);
            cast_these_float!(f32, f64);
        }
        )*
    };
}
macro_rules! impl_castit_float {
    ($($t: ty),*) => {
        $(
        impl CastIt for $t {
            #[inline]
            fn u(self) -> usize {
                self.usize()
            }
            cast_these_float!(u8, u16, u32, u64, u128, usize);
            cast_these_float!(i8, i16, i32, i64, i128, isize);
            cast_these_float!(f32, f64);
        }
        )*
    };
}
impl_castit!(u8, u16, u32, u64, u128, usize);
impl_castit!(i8, i16, i32, i64, i128, isize);
impl_castit_float!(f32, f64);
