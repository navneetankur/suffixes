//! Cross-checks the cast guards against an oracle that decides exactness in
//! integer space instead of float space.
//!
//! A finite float with no fractional part and `abs() < 2^127` converts to i128
//! with neither saturation nor rounding, so `v as i128` is its exact
//! mathematical value and can be compared against `T::MIN`/`T::MAX` as i128.
//! That path never evaluates `MAX as f32`, so it shares no assumption with the
//! guards it is checking. The boundary tests next door were derived from the
//! same reasoning as the guards; these are not.
#![cfg(debug_assertions)]
use crate::CastIt;

/// 2^127, the largest magnitude that survives `as i128` intact.
const I128_LIMIT: f64 = 170_141_183_460_469_231_731_687_303_715_884_105_728.0;

fn panics(f: impl FnOnce() + std::panic::UnwindSafe) -> bool {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(f).is_err();
    std::panic::set_hook(prev);
    caught
}

/// Deterministic, so a failure reproduces.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }
}

/// Every float within three ULP of every power of two, the half-integers either
/// side, the specials, and a spray of random bit patterns.
fn probe_f32() -> Vec<f32> {
    let mut v = vec![
        0.0, -0.0, 0.5, -0.5, 1.5, -1.5, 255.5, -128.5,
        f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::MAX, f32::MIN, f32::MIN_POSITIVE,
    ];
    for e in 0..=64i32 {
        let p = 2f32.powi(e);
        for d in -3i64..=3 {
            let bits = (i64::from(p.to_bits()) + d) as u32;
            v.push(f32::from_bits(bits));
            v.push(-f32::from_bits(bits));
        }
        v.extend([p - 0.5, p + 0.5, -p - 0.5, -p + 0.5]);
    }
    let mut r = Rng(0x1234_5678);
    v.extend((0..20_000).map(|_| f32::from_bits((r.next() >> 32) as u32)));
    v
}
fn probe_f64() -> Vec<f64> {
    let mut v = vec![
        0.0, -0.0, 0.5, -0.5, 1.5, -1.5,
        f64::NAN, f64::INFINITY, f64::NEG_INFINITY, f64::MAX, f64::MIN, f64::MIN_POSITIVE,
    ];
    for e in 0..=70i32 {
        let p = 2f64.powi(e);
        for d in -3i128..=3 {
            let bits = (i128::from(p.to_bits()) + d) as u64;
            v.push(f64::from_bits(bits));
            v.push(-f64::from_bits(bits));
        }
        v.extend([p - 0.5, p + 0.5, -p - 0.5, -p + 0.5]);
    }
    let mut r = Rng(0xdead_beef);
    v.extend((0..20_000).map(|_| f64::from_bits(r.next())));
    v
}

macro_rules! f2i {
    ($f: ident, $t: ident, $probe: expr) => {{
        for v in $probe {
            let v: $f = v;
            let exact = v.is_finite()
                && v.trunc() == v
                && (v.abs() as f64) < I128_LIMIT
                && {
                    let b = v as i128;
                    b >= <$t>::MIN as i128 && b <= <$t>::MAX as i128
                };
            // The guard must panic exactly when the conversion is not exact.
            if panics(move || { let _ = <$f as CastIt>::$t(v); }) == exact {
                panic!(
                    "{}->{}: v={:e} (bits {:#x}) exact={exact} but guard disagreed",
                    stringify!($f), stringify!($t), v, v.to_bits(),
                );
            }
        }
    }};
}

#[test]
fn float_to_int_matches_oracle() {
    f2i!(f32, u8, probe_f32());
    f2i!(f32, u16, probe_f32());
    f2i!(f32, u32, probe_f32());
    f2i!(f32, u64, probe_f32());
    f2i!(f32, i8, probe_f32());
    f2i!(f32, i16, probe_f32());
    f2i!(f32, i32, probe_f32());
    f2i!(f32, i64, probe_f32());
    f2i!(f64, u8, probe_f64());
    f2i!(f64, u16, probe_f64());
    f2i!(f64, u32, probe_f64());
    f2i!(f64, u64, probe_f64());
    f2i!(f64, i8, probe_f64());
    f2i!(f64, i16, probe_f64());
    f2i!(f64, i32, probe_f64());
    f2i!(f64, i64, probe_f64());
}

/// Powers of two and their neighbours, plus random values, truncated per type.
fn probe_ints() -> Vec<u128> {
    let mut v = vec![0, 1, 2, 3];
    for e in 0..127u32 {
        let p = 1u128 << e;
        v.extend([p.saturating_sub(2), p.saturating_sub(1), p, p + 1, p + 2]);
    }
    v.extend([u128::MAX, u128::MAX - 1, u64::MAX as u128, u64::MAX as u128 - 1]);
    let mut r = Rng(0xcafe_f00d);
    v.extend((0..10_000).map(|_| u128::from(r.next())));
    v
}

macro_rules! i2f {
    ($t: ident, $f: ident) => {{
        for raw in probe_ints() {
            let v = raw as $t;
            let after = v as $f;
            // `after as i128` cannot saturate: |after| <= 2^64 for these types.
            let exact = after.is_finite()
                && (after.abs() as f64) < I128_LIMIT
                && (after as i128) == (v as i128);
            if panics(move || { let _ = <$t as CastIt>::$f(v); }) == exact {
                panic!(
                    "{}->{}: v={v} exact={exact} but guard disagreed",
                    stringify!($t), stringify!($f),
                );
            }
        }
    }};
}

#[test]
fn int_to_float_matches_oracle() {
    i2f!(u8, f32);
    i2f!(u16, f32);
    i2f!(u32, f32);
    i2f!(u32, f64);
    i2f!(u64, f32);
    i2f!(u64, f64);
    i2f!(i8, f32);
    i2f!(i16, f32);
    i2f!(i32, f32);
    i2f!(i32, f64);
    i2f!(i64, f32);
    i2f!(i64, f64);
}
