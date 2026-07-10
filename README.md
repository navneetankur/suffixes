# suffixes

Cast primitive types and wrap `Option`/`Result` using a method suffix, so the
target type reads left to right instead of trailing behind an `as`.

```rust
use suffixes::CastIt;

let three = 3;
assert_eq!(three.u8(), 3u8);
assert_eq!(three.f64(), 3.0f64);
// `u()` is shorthand for `usize()`.
assert_eq!(three.u(), 3usize);
```

## Checked casts

`CastIt` is `as` with a guard. Every conversion is checked in debug builds and
panics if it would lose information — out of range, a dropped fraction, or a
float that cannot hold the integer exactly:

```rust,should_panic,no_run
use suffixes::CastIt;
300i32.u8();       // out of range
```

```rust,should_panic,no_run
use suffixes::CastIt;
3.3f32.u8();       // would drop the fraction
```

```rust,should_panic,no_run
use suffixes::CastIt;
16777217i32.f32(); // not representable in an f32
```

In release builds the guards compile out and the conversion is a plain `as`,
with all of the wrapping, saturating and truncating that implies. `CastIt` is
for casts you believe are lossless and want caught in testing when they are not.
It is not a substitute for `try_from` on untrusted input.

`CastFrom` is the same thing from the other side, for when the type you have is
the destination:

```rust
use suffixes::CastFrom;
assert_eq!(u8::cast_from(3i64), 3u8);
```

## Truncating casts

`TrunIt` is for float to integer conversions where dropping the fraction is the
point. It rounds toward zero and only checks that the result is in range:

```rust
use suffixes::TrunIt;
assert_eq!(3.9f32.tu8(), 3u8);
assert_eq!((-3.9f32).ti8(), -3i8);
```

```rust,should_panic,no_run
use suffixes::TrunIt;
(-3.9f32).tu8();   // negative, no unsigned result
```

## Wrapping

```rust
use suffixes::{WrappedOption, WrappedResult, WrappedErrResult};

fn find() -> Option<i32> { 4.some() }
fn parse() -> Result<i32, String> { 4.ok() }
fn fail() -> Result<i32, String> { "bad".to_string().err() }

assert_eq!(find(), Some(4));
assert_eq!(parse(), Ok(4));
assert_eq!(fail(), Err("bad".to_string()));
```

These are handy at the tail of a long expression, where wrapping the whole thing
in `Some(..)` would mean going back to the start to add the opening paren.
