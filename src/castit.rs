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
// Widening to a float does not saturate, but casting back to check it does, and
// at `Self::MAX` that saturation cancels out the rounding: u64::MAX becomes
// 2f64.powi(64), which clamps straight back to u64::MAX. So `after` is bounded
// before the round trip, against the same exclusive power of two used below.
macro_rules! cast_int_to_float {
    ($s: ty; $($t: ident),*) => {
        $(
        #[inline]
        fn $t(self) -> $t {
            let after = self as $t;
            #[cfg(debug_assertions)]
            {
                // Shifting into an integer and widening keeps this const-evaluable,
                // where `exp2` would be a libm call in the debug builds this block
                // is compiled into. `2f32.powi(128)` overflows to `inf`, which the
                // `is_finite` check rejects, same as the exclusive bound would.
                const HI: $t = if <$s>::MIN == 0 {
                    (1u128 << (<$s>::BITS - 1)) as $t * 2.0
                } else {
                    (1u128 << (<$s>::BITS - 1)) as $t
                };
                const LO: $t = if <$s>::MIN == 0 { 0.0 } else { -HI };
                // `Range::contains` would be a generic call in an unoptimized build.
                #[allow(clippy::manual_range_contains)]
                {
                    assert!(
                        after.is_finite() && after >= LO && after < HI && after as $s == self,
                        "can't cast {self} to {} without loss", stringify!($t),
                    );
                }
            }
            return after;
        }
        )*
    };
}
macro_rules! cast_float_to_float {
    ($($t: ident),*) => {
        $(
        #[inline]
        fn $t(self) -> $t {
            let after = self as $t;
            // NaN converts exactly but never compares equal to itself.
            debug_assert!(
                self.is_nan() || self == after as Self,
                "can't cast {self} to {} without loss", stringify!($t),
            );
            return after;
        }
        )*
    };
}
// A float to int `as` cast saturates. When the target's MAX is not representable
// in Self, that saturation cancels out the rounding: 2f32.powi(32) clamps to
// u32::MAX, which converts straight back to 2f32.powi(32). So the range is
// checked against the exclusive power-of-two bound, which is always exact,
// rather than against `MAX as Self`.
macro_rules! cast_float_to_int {
    (@one $f: ty, $t: ident, $lo: expr, $hi: expr) => {
        #[inline]
        fn $t(self) -> $t {
            #[cfg(debug_assertions)]
            {
                const LO: $f = $lo;
                const HI: $f = $hi;
                assert!(self.is_finite(), "can't cast {self} to {}", stringify!($t));
                // `Range::contains` would be a generic call in an unoptimized build.
                #[allow(clippy::manual_range_contains)]
                {
                    assert!(
                        self >= LO && self < HI,
                        "{self} is out of range for {}", stringify!($t),
                    );
                }
                // Above this the exponent alone makes every value integral. Below
                // it every value fits exactly in `i64`, so the round trip is
                // loss-free and holds only for integers. Neither branch calls out
                // to libm the way `trunc` does, nor to compiler-rt the way a round
                // trip through a 128 bit `$t` would.
                const FRACTIONABLE: $f = (1u64 << (<$f>::MANTISSA_DIGITS - 1)) as $f;
                assert!(
                    self <= -FRACTIONABLE || self >= FRACTIONABLE
                        || self as i64 as $f == self,
                    "can't cast {self} to {} without loss", stringify!($t),
                );
            }
            return self as $t;
        }
    };
    ($f: ty; unsigned: $($t: ident),*) => {
        $( cast_float_to_int!(@one $f, $t, 0.0, (1u128 << (<$t>::BITS - 1)) as $f * 2.0); )*
    };
    ($f: ty; signed: $($t: ident),*) => {
        $( cast_float_to_int!(
            @one $f, $t,
            -((1u128 << (<$t>::BITS - 1)) as $f),
            (1u128 << (<$t>::BITS - 1)) as $f
        ); )*
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
            cast_int_to_float!($t; f32, f64);
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
            cast_float_to_int!($t; unsigned: u8, u16, u32, u64, u128, usize);
            cast_float_to_int!($t; signed: i8, i16, i32, i64, i128, isize);
            cast_float_to_float!(f32, f64);
        }
        )*
    };
}
impl_castit!(u8, u16, u32, u64, u128, usize);
impl_castit!(i8, i16, i32, i64, i128, isize);
impl_castit_float!(f32, f64);
