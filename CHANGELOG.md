# Changelog

## 0.5.3

### Changed

- `TrunIt` no longer calls `trunc` in its guards, which was the last libcall
  left in them. The guard never needed the truncated value, only the question of
  whether it lands in range, and that is answerable from the input: `trunc(x)`
  is in `[lo, hi)` exactly when `x` is in `(lo - 1, hi)`. A truncating guard now
  makes no call at all on the path where it passes.

- The `is_finite` check went with it. NaN fails every comparison and the
  infinities fail a bound, so the range check already rejected them. They now
  panic with `is out of range for` rather than `can't truncate`.

- `CastIt` drops its `is_finite` checks for the same reason, so every guard in
  the crate is now free of calls on the path where it passes. Casting a NaN or an
  infinity to an integer panics with `is out of range for` rather than
  `can't cast`. Float to float casts still test for NaN, which is not redundant
  there: NaN converts exactly but never compares equal to itself.

- `CastIt::u`, `TrunIt::tu` and `CastFrom::cast_from` are `inline(always)`. They
  only forward, so collapsing them costs no code and saves a stack frame per call
  in an unoptimized build. The guarded methods stay `inline` on purpose: always
  inlining those would stamp their cold panic path into every call site of a
  dependent crate's debug build, which measures at roughly 190 bytes of `.text`
  per cast.

## 0.5.2

### Changed

- `TrunIt` no longer derives its range bounds with `exp2`, which is a libcall in
  an unoptimized build and so was recomputed on every truncation. The signed
  methods paid it twice, once per bound. This is the same fix 0.5.1 made to
  `CastIt`; the two traits had duplicated the bound logic. `TrunIt` still calls
  `trunc`, because there the truncated value is what gets range checked rather
  than being a test for integrality.

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
