pub trait TrunIt: Sized {
    fn tu(self) -> usize;
    fn tu8(self) ->  u8;
    fn tu16(self) -> u16;
    fn tu32(self) -> u32;
    fn tu64(self) -> u64;
    fn tu128(self) -> u128;
    fn tusize(self) -> usize;
    fn ti8(self) ->  i8;
    fn ti16(self) -> i16;
    fn ti32(self) -> i32;
    fn ti64(self) -> i64;
    fn ti128(self) -> i128;
    fn tisize(self) -> isize;
}

// The truncated value is what the `as` cast produces, so that is what gets range
// checked. The bound is the exclusive power of two rather than `MAX as Self`,
// because `MAX` is not representable in `Self` for the wider targets: comparing
// against `i32::MAX as f32` would wave through 2f32.powi(31), which then
// saturates to `i32::MAX` instead of being rejected.
macro_rules! trun_to_int {
    (@one $f: ty, $name: ident -> $t: ident, $lo: expr, $hi: expr) => {
        #[inline]
        fn $name(self) -> $t {
            #[cfg(debug_assertions)]
            {
                // Shifting into an integer and widening keeps these const-evaluable,
                // where `exp2` would be a libm call in the debug builds this block
                // is compiled into. `2f32.powi(128)` overflows to `inf`, which the
                // `is_finite` check rejects, same as the exclusive bound would.
                const LO: $f = $lo;
                const HI: $f = $hi;
                assert!(self.is_finite(), "can't truncate {self} to {}", stringify!($t));
                let truncated = self.trunc();
                // `Range::contains` would be a generic call in an unoptimized build.
                #[allow(clippy::manual_range_contains)]
                {
                    assert!(
                        truncated >= LO && truncated < HI,
                        "{self} is out of range for {}", stringify!($t),
                    );
                }
            }
            return self as $t;
        }
    };
    ($f: ty; unsigned: $($name: ident -> $t: ident),*) => {
        $( trun_to_int!(
            @one $f, $name -> $t,
            0.0,
            (1u128 << (<$t>::BITS - 1)) as $f * 2.0
        ); )*
    };
    ($f: ty; signed: $($name: ident -> $t: ident),*) => {
        $( trun_to_int!(
            @one $f, $name -> $t,
            -((1u128 << (<$t>::BITS - 1)) as $f),
            (1u128 << (<$t>::BITS - 1)) as $f
        ); )*
    };
}
macro_rules! impl_trunit {
    ($($f: ty),*) => {
        $(
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        impl TrunIt for $f {
            #[inline]
            fn tu(self) -> usize {
                self.tusize()
            }
            trun_to_int!($f; unsigned:
                tu8 -> u8, tu16 -> u16, tu32 -> u32, tu64 -> u64, tu128 -> u128, tusize -> usize);
            trun_to_int!($f; signed:
                ti8 -> i8, ti16 -> i16, ti32 -> i32, ti64 -> i64, ti128 -> i128, tisize -> isize);
        }
        )*
    };
}
impl_trunit!(f32, f64);
