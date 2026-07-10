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

/// Whether truncating `self` toward zero lands in `[lo, hi)`, both of which are
/// integral. Answered without calling `trunc`, which is a libcall in the
/// unoptimized builds these guards are compiled into.
///
/// `trunc(x) < hi` is `x < hi`. Truncation moves a value toward zero by less
/// than one and `hi` is integral, so it cannot carry a value across `hi`.
///
/// `trunc(x) >= lo` is `x > lo - 1.0`, by the same argument in the other
/// direction. Where `lo - 1.0` is not representable it rounds back to `lo`, and
/// then `x >= lo` is the same predicate: at that magnitude consecutive floats
/// are more than 1 apart, so no float lies between `lo - 1` and `lo`. The
/// disjunction spells both cases at once, and the dead half is a compare
/// against a constant.
///
/// NaN fails every comparison and the infinities fail a bound, so this also
/// subsumes the `is_finite` check the guards used to make. They are reported as
/// out of range rather than as not finite.
#[cfg(debug_assertions)]
pub(crate) trait TruncatesInto: Copy {
    fn truncates_into(self, lo: Self, hi: Self) -> bool;
}
#[cfg(debug_assertions)]
macro_rules! impl_truncates_into {
    ($($f: ty),*) => {
        $(
        impl TruncatesInto for $f {
            // Must not become a call in an unoptimized build; `inline(always)`
            // is the one inline hint the `-O0` pipeline still honours.
            #[inline(always)]
            fn truncates_into(self, lo: Self, hi: Self) -> bool {
                (self >= lo || self > lo - 1.0) && self < hi
            }
        }
        )*
    };
}
#[cfg(debug_assertions)]
impl_truncates_into!(f32, f64);

// The truncated value is what the `as` cast produces, so that is what gets range
// checked, against the bounds in `crate::bounds`.
macro_rules! trun_to_int {
    ($f: ty; $($name: ident -> $t: ident),*) => {
        $(
        #[inline]
        fn $name(self) -> $t {
            #[cfg(debug_assertions)]
            {
                const LO: $f = $crate::bounds::lo!($t, $f);
                const HI: $f = $crate::bounds::hi!($t, $f);
                assert!(
                    self.truncates_into(LO, HI),
                    "{self} is out of range for {}", stringify!($t),
                );
            }
            return self as $t;
        }
        )*
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
            trun_to_int!($f;
                tu8 -> u8, tu16 -> u16, tu32 -> u32, tu64 -> u64, tu128 -> u128, tusize -> usize);
            trun_to_int!($f;
                ti8 -> i8, ti16 -> i16, ti32 -> i32, ti64 -> i64, ti128 -> i128, tisize -> isize);
        }
        )*
    };
}
impl_trunit!(f32, f64);
