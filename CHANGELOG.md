# Changelog

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
