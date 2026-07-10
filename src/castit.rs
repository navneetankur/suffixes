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
                const LO: $t = $crate::bounds::lo!($s, $t);
                const HI: $t = $crate::bounds::hi!($s, $t);
                // No `is_finite` check: widening an integer never produces NaN, and
                // the only overflow to `inf` is `u128`/`i128` into `f32`, where `hi`
                // is `inf` too and `inf < inf` is false. The bound rejects it.
                //
                // `Range::contains` would be a generic call in an unoptimized build.
                #[allow(clippy::manual_range_contains)]
                {
                    assert!(
                        after >= LO && after < HI && after as $s == self,
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
    ($f: ty; $($t: ident),*) => {
        $(
        #[inline]
        fn $t(self) -> $t {
            #[cfg(debug_assertions)]
            {
                const LO: $f = $crate::bounds::lo!($t, $f);
                const HI: $f = $crate::bounds::hi!($t, $f);
                // No `is_finite` check: NaN fails every comparison and the
                // infinities fail a bound, `inf < inf` included, so the range check
                // already rejects them. They report as out of range.
                //
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
            // Pure forwarding, so `inline(always)` collapses the frame without
            // duplicating a guard. The guarded methods stay `inline`: always
            // inlining them would stamp their cold panic path into every call site
            // of every downstream crate's unoptimized build.
            #[inline(always)]
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
            // Pure forwarding, so `inline(always)` collapses the frame without
            // duplicating a guard. The guarded methods stay `inline`: always
            // inlining them would stamp their cold panic path into every call site
            // of every downstream crate's unoptimized build.
            #[inline(always)]
            fn u(self) -> usize {
                self.usize()
            }
            cast_float_to_int!($t; u8, u16, u32, u64, u128, usize);
            cast_float_to_int!($t; i8, i16, i32, i64, i128, isize);
            cast_float_to_float!(f32, f64);
        }
        )*
    };
}
impl_castit!(u8, u16, u32, u64, u128, usize);
impl_castit!(i8, i16, i32, i64, i128, isize);
impl_castit_float!(f32, f64);
