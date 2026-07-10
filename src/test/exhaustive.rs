//! Pins `TruncatesInto` against the `trunc` based guard it replaced.
//!
//! The replacement rests on two claims: that `trunc(x) >= lo` is `x > lo - 1.0`,
//! and that where `lo - 1.0` rounds back to `lo` no float lies in between, so
//! `x >= lo` says the same thing. The second claim is the one that can break, so
//! the bounds below cover both regimes: `i8` and `u8` where `lo - 1.0` is exact,
//! and `i32` and `u32` where in `f32` it is not.
//!
//! Every finite `f32` is checked, and the non-finite ones too, since the guard
//! no longer tests `is_finite` separately. Slow, so it is ignored by default.
//! It needs the optimizer to finish in reasonable time and `debug_assertions`
//! to exist at all, and `--release` turns the latter off, so both are asked for:
//!
//! ```text
//! RUSTFLAGS="-C debug-assertions" cargo test --release -- --ignored
//! ```
#![cfg(debug_assertions)]
use crate::trunit::TruncatesInto;

/// What the guard used to compute, `trunc` and all.
fn reference(x: f32, lo: f32, hi: f32) -> bool {
    let truncated = x.trunc();
    truncated >= lo && truncated < hi
}

#[test]
#[ignore = "walks all 2^32 f32 bit patterns; run with --release"]
fn truncates_into_matches_trunc_over_every_f32() {
    // (name, lo, hi) for the targets `f32` reaches, plus the `u128` bound that
    // overflows to an infinite `hi`.
    let bounds: [(&str, f32, f32); 6] = [
        ("u8", 0.0, 256.0),
        ("i8", -128.0, 128.0),
        ("u32", 0.0, 4_294_967_296.0),
        ("i32", -2_147_483_648.0, 2_147_483_648.0),
        ("i128", -170_141_183_460_469_231_731_687_303_715_884_105_728.0,
                  170_141_183_460_469_231_731_687_303_715_884_105_728.0),
        ("u128", 0.0, f32::INFINITY),
    ];

    let mut checked = 0u64;
    for bits in 0..=u32::MAX {
        let x = f32::from_bits(bits);
        for (name, lo, hi) in bounds {
            assert_eq!(
                reference(x, lo, hi),
                x.truncates_into(lo, hi),
                "{name}: x = {x:?} (bits {bits:#010x})",
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 1u64 << 32);
}

/// The same claims for `f64`, where exhaustion is impossible. Walks the
/// neighbourhood of every power of two, which is where the float spacing crosses
/// 1 and the second claim starts carrying the proof.
#[test]
fn truncates_into_matches_trunc_around_f64_powers_of_two() {
    fn reference64(x: f64, lo: f64, hi: f64) -> bool {
        let truncated = x.trunc();
        truncated >= lo && truncated < hi
    }
    let bounds: [(f64, f64); 4] = [
        (0.0, 256.0),
        (-128.0, 128.0),
        (-9_223_372_036_854_775_808.0, 9_223_372_036_854_775_808.0),
        (0.0, 18_446_744_073_709_551_616.0),
    ];
    let mut checked = 0u64;
    for e in -1074i32..=1023 {
        let base = 2f64.powi(e);
        for delta in -4i64..=4 {
            for signed in [base, -base] {
                let x = f64::from_bits((signed.to_bits() as i64).wrapping_add(delta) as u64);
                for (lo, hi) in bounds {
                    assert_eq!(reference64(x, lo, hi), x.truncates_into(lo, hi), "x = {x:?}");
                }
                checked += 1;
            }
        }
    }
    // The half integers either side of each bound, where truncation actually bites.
    for (lo, hi) in bounds {
        for x in [lo - 1.5, lo - 1.0, lo - 0.5, lo, lo + 0.5, hi - 0.5, hi, hi + 0.5] {
            assert_eq!(reference64(x, lo, hi), x.truncates_into(lo, hi), "x = {x:?}");
            checked += 1;
        }
    }
    for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -0.0] {
        for (lo, hi) in bounds {
            assert_eq!(reference64(x, lo, hi), x.truncates_into(lo, hi), "x = {x:?}");
            checked += 1;
        }
    }
    assert!(checked > 19_000);
}
