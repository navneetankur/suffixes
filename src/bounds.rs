//! The range an integer type occupies when viewed as a float, as `[lo, hi)`.
//!
//! `hi` is the first power of two above the type's maximum, not `MAX as $f`.
//! `MAX` is not representable in the float whenever the target is wider than the
//! mantissa, so a guard written against `MAX as $f` rounds the bound up and lets
//! the first out of range value through, which the `as` cast then saturates.
//! The power of two is always exact, so the bound is exact.
//!
//! Both are built by shifting into an integer and widening, which const
//! evaluates. Deriving them with `exp2` would not: `exp2` lowers to a libcall
//! that only folds under the optimizer, and the guards these serve are
//! `debug_assertions` only, so they are compiled into exactly the build where
//! the folding never runs.
//!
//! `u128` in `f32` is the one case where `hi` is not finite: `2^128` overflows
//! to `inf`. That is the right bound. Every finite `f32` is below `2^128`, and
//! `inf` itself is not below `inf`, so an infinite `hi` accepts exactly the
//! finite values, which is what the guards want.
#![cfg(debug_assertions)]

/// The exclusive upper bound of `$t` in `$f`: `2^BITS` unsigned, `2^(BITS-1)`
/// signed. Written as a halved shift and a doubling so that the unsigned case,
/// whose exponent equals `BITS`, does not need a `1 << 128`.
macro_rules! hi {
    ($t: ty, $f: ty) => {
        (1u128 << (if <$t>::MIN == 0 { <$t>::BITS - 1 } else { <$t>::BITS - 2 })) as $f * 2.0
    };
}

/// The inclusive lower bound of `$t` in `$f`. `MIN` is a power of two for the
/// signed types and zero for the unsigned ones, so unlike `MAX` it is always
/// exactly representable and needs no widening.
macro_rules! lo {
    ($t: ty, $f: ty) => {
        if <$t>::MIN == 0 { 0.0 } else { -($crate::bounds::hi!($t, $f)) }
    };
}

pub(crate) use {hi, lo};

#[cfg(test)]
mod tests {
    /// The shift is a stand in for `2^exp`. Check it against `exp2` itself, for
    /// every integer type and both floats. `exp2` is fine here; a test is not a
    /// guard, and nothing in a test is recomputed per cast.
    macro_rules! check {
        ($f: ty; $($t: ty),*) => {$({
            let bits = <$t>::BITS;
            let signed = <$t>::MIN != 0;
            let exp = if signed { bits - 1 } else { bits };
            let want_hi: $f = (exp as $f).exp2();
            let want_lo: $f = if signed { -want_hi } else { 0.0 };

            let got_hi: $f = hi!($t, $f);
            let got_lo: $f = lo!($t, $f);

            assert_eq!(got_hi.to_bits(), want_hi.to_bits(),
                "hi {} in {}: {got_hi} vs {want_hi}", stringify!($t), stringify!($f));
            assert_eq!(got_lo.to_bits(), want_lo.to_bits(),
                "lo {} in {}: {got_lo} vs {want_lo}", stringify!($t), stringify!($f));
        })*};
    }

    #[test]
    fn bounds_match_exp2() {
        check!(f32; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
        check!(f64; u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    }

    #[test]
    fn u128_in_f32_is_infinite_and_admits_every_finite_f32() {
        let hi: f32 = hi!(u128, f32);
        assert!(hi.is_infinite() && hi.is_sign_positive());
        assert!(f32::MAX < hi);
        // `inf < hi` is false, which is how the guards reject it.
        assert!(f32::INFINITY >= hi);
    }

    #[test]
    fn i128_in_f32_is_finite() {
        let hi: f32 = hi!(i128, f32);
        let lo: f32 = lo!(i128, f32);
        assert!(hi.is_finite() && lo.is_finite());
        assert_eq!(hi, 2f32.powi(127));
        assert_eq!(lo, -2f32.powi(127));
    }
}
