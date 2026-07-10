//! Regression tests for the saturating float-to-int boundary.
//!
//! `MAX as f32` rounds up for any target wider than the f32 mantissa, so a guard
//! written against it accepts the first out-of-range value and then silently
//! saturates. Each `should_panic` test below returned a wrong answer before the
//! bound became the exclusive power of two.
use crate::{CastIt, TrunIt};

/// The guards are `debug_assertions` only; a release build casts with a bare `as`.
macro_rules! must_panic {
    ($($name: ident: $e: expr;)*) => {
        $(
        #[test]
        #[should_panic]
        #[cfg(debug_assertions)]
        fn $name() { let _ = $e; }
        )*
    };
}

must_panic! {
    cast_f32_pow2_31_to_i32: 2f32.powi(31).i32();
    cast_f32_pow2_32_to_u32: 2f32.powi(32).u32();
    cast_f32_pow2_63_to_i64: 2f32.powi(63).i64();
    cast_f32_pow2_64_to_u64: 2f32.powi(64).u64();
    cast_f32_pow2_127_to_i128: 2f32.powi(127).i128();
    cast_f64_pow2_63_to_i64: 2f64.powi(63).i64();
    cast_f64_pow2_64_to_u64: 2f64.powi(64).u64();
    cast_f64_pow2_127_to_i128: 2f64.powi(127).i128();
    cast_f64_pow2_128_to_u128: 2f64.powi(128).u128();

    trun_f32_pow2_31_to_i32: 2f32.powi(31).ti32();
    trun_f32_pow2_32_to_u32: 2f32.powi(32).tu32();
    trun_f32_pow2_64_to_u64: 2f32.powi(64).tu64();
    trun_f32_pow2_127_to_i128: 2f32.powi(127).ti128();
    trun_f64_pow2_63_to_i64: 2f64.powi(63).ti64();
    trun_f64_pow2_64_to_u64: 2f64.powi(64).tu64();
    trun_f64_pow2_127_to_i128: 2f64.powi(127).ti128();
    trun_f64_pow2_128_to_u128: 2f64.powi(128).tu128();
    trun_f32_negative_to_u128: (-1f32).tu128();

    // Int to float. `after as Self` saturates, and at Self::MAX that cancels
    // out the rounding, so these lossy conversions used to pass silently.
    cast_u64_max_to_f32: u64::MAX.f32();
    cast_u64_max_to_f64: u64::MAX.f64();
    cast_i64_max_to_f32: i64::MAX.f32();
    cast_i64_max_to_f64: i64::MAX.f64();
    cast_u128_max_to_f32: u128::MAX.f32();
    cast_u128_max_to_f64: u128::MAX.f64();
    cast_i128_max_to_f64: i128::MAX.f64();
    cast_u32_max_to_f32: u32::MAX.f32();

    cast_nan_to_u8: f32::NAN.u8();
    cast_inf_to_u8: f32::INFINITY.u8();
    trun_nan_to_u8: f32::NAN.tu8();
    trun_inf_to_i64: f64::INFINITY.ti64();
    cast_lossy_f64_to_f32: 1e300f64.f32();
    cast_lossy_u64_to_f32: (u64::MAX - 1).f32();
}

/// The largest values that really do fit must still convert.
#[test]
fn exact_bounds_are_accepted() {
    assert_eq!(255f32.u8(), 255u8);
    assert_eq!((-128f32).i8(), -128i8);
    assert_eq!(65535f32.u16(), 65535u16);
    assert_eq!(f64::from(u32::MAX).u32(), u32::MAX);
    assert_eq!((-(2f64.powi(63))).i64(), i64::MIN);
    assert_eq!((-(2f32.powi(31))).i32(), i32::MIN);
    // The f32 just below 2^32 is 2^32 - 256, which fits in a u32.
    assert_eq!(4294967040f32.u32(), 4_294_967_040u32);
    // Every finite f32 fits in a u128: f32::MAX is well under u128::MAX.
    assert_eq!(f32::MAX.u128(), 340_282_346_638_528_859_811_704_183_484_516_925_440u128);
}

/// Int to float conversions that really are exact must still convert. The
/// bound on `after` is only there to stop the back cast from saturating.
#[test]
fn exact_int_to_float_is_accepted() {
    assert_eq!(u32::MAX.f64(), 4294967295f64);
    assert_eq!(i32::MAX.f64(), 2147483647f64);
    assert_eq!(i64::MIN.f64(), -(2f64.powi(63))); // -2^63 is exactly representable
    assert_eq!(i32::MIN.f32(), -(2f32.powi(31)));
    assert_eq!(u8::MAX.f32(), 255f32);
    assert_eq!((1u64 << 53).f64(), 2f64.powi(53)); // last exact integer in an f64
    assert_eq!((1u64 << 24).f32(), 2f32.powi(24)); // last exact integer in an f32
}

/// NaN converts between float widths exactly, but never compares equal to itself.
#[test]
fn nan_survives_float_to_float() {
    assert!(f32::NAN.f64().is_nan());
    assert!(f64::NAN.f32().is_nan());
    assert!(f32::NAN.f32().is_nan());
    assert!(f64::NAN.f64().is_nan());
}

#[test]
fn infinity_survives_float_to_float() {
    assert_eq!(f32::INFINITY.f64(), f64::INFINITY);
    assert_eq!(f64::NEG_INFINITY.f32(), f32::NEG_INFINITY);
}

/// Truncation drops the fraction, so a value whose fraction reaches past `MAX`
/// is still in range once truncated.
#[test]
fn truncation_rounds_toward_zero() {
    assert_eq!(255.9f32.tu8(), 255u8);
    assert_eq!((-128.9f32).ti8(), -128i8);
    assert_eq!((-3.9f64).ti32(), -3i32);
}
