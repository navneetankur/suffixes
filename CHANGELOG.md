# Changelog

## 0.5.2

All of the casting guards are `debug_assertions` only. In a release build a cast
is a plain `as`, and always has been. Everything below describes debug builds,
and none of it changes which values a cast accepts.

### Changed

- No guard makes a call any more on the path where it passes. `TrunIt` derived
  its bounds with `exp2`, twice per signed method, and tested integrality with
  `trunc`; both traits called `is_finite`. Each of those is a call in an
  unoptimized build, which is the only build the guards are compiled into, so
  none of them ever folded away. 0.5.1 had removed `exp2` from `CastIt` alone,
  because the two traits carried their own copies of the same bound logic; they
  now share one.

- The bounds are const evaluated rather than derived. `trunc` is gone because a
  truncation lands in `[lo, hi)` exactly when its input lies in `(lo - 1, hi)`,
  which needs no truncation to decide. `is_finite` is gone because the range
  check already rejected the non-finite values: NaN fails every comparison and
  the infinities fail a bound.

- Casting or truncating a NaN or an infinity to an integer now panics with
  `is out of range for` rather than `can't cast` or `can't truncate`. Only the
  wording changed; those values were already rejected. Float to float casts still
  test for NaN, which is not redundant there: NaN converts exactly but never
  compares equal to itself.

- `CastIt::u`, `TrunIt::tu` and `CastFrom::cast_from` are `inline(always)`. They
  only forward, so collapsing them costs no code and saves a stack frame per call
  in an unoptimized build; `cast_from` was costing two before reaching the guard.
  The guarded methods stay `inline` on purpose: always inlining those would stamp
  their cold panic path into every call site of a dependent crate's debug build,
  which measures at roughly 190 bytes of `.text` per cast, and leaves release
  output byte identical.

## 0.5.1

### Changed

- The debug build cast guards no longer call into libm. They derived their
  power of two bounds with `exp2` and tested integrality with `trunc`, both of
  which lower to a libcall in an unoptimized build, and an unoptimized build is
  the only one the guards are compiled into, so the constants were recomputed on
  every cast. The bounds are now const evaluated and integrality is a compare
  against the mantissa threshold plus an `i64` round trip. A cast in a release
  build is still a plain `as`, as it has always been.

## 0.5.0

All of the casting guards are `debug_assertions` only. In a release build a cast
is a plain `as`, and always has been. Everything below describes debug builds.

### Fixed

- Float to integer casts accepted the first out-of-range value and saturated it.
  `MAX as f32` rounds up whenever the target is wider than the float's mantissa,
  so a guard written against it lets that value through, and the `as` cast then
  clamps it. `2147483648.0f32.i32()` returned `2147483647` without panicking.
  Affected `CastIt` for f32 to `u32`/`i32`/`u64`/`i64`/`u128`/`i128`/`usize`/`isize`
  and f64 to `u64`/`i64`/`u128`/`i128`/`usize`/`isize`, and the matching `TrunIt`
  methods.

- Integer to float casts accepted lossy conversions at `MAX`. The check cast the
  float back to compare, and that back cast saturates, cancelling out the
  rounding. `u64::MAX.f64()` lost 11 bits without panicking. Also affected
  `u64::MAX.f32()`, `i64::MAX.f32()`, `i64::MAX.f64()`, `u128::MAX.f32()`,
  `u128::MAX.f64()`, `i128::MAX.f64()` and `u32::MAX.f32()`.

- `CastIt` panicked on NaN in float to float casts. NaN converts exactly but
  never compares equal to itself, so the round trip rejected it. `f32::NAN.f64()`
  panicked.

### Added

- `TrunIt::tu128` and `TrunIt::ti128`, matching the `u128`/`i128` methods
  `CastIt` already had.

### Changed

- `TrunIt` now range checks the truncated value rather than the input, so
  `255.9f32.tu8()` returns `255` and `(-128.9f32).ti8()` returns `-128`. Both
  previously panicked. Truncating first is the point of the trait, and the change
  only ever accepts more; it never returns a different value than before.

- `WrappedErrResult` is re-exported at the crate root. It was previously only
  reachable as `suffixes::wrap::WrappedErrResult`.

### Removed

- The `more-asserts` dependency. The crate now has no dependencies.
