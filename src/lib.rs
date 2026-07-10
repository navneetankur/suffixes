#![allow(clippy::needless_return)]
#![doc = include_str!("../README.md")]
pub mod wrap;
pub mod castit;
pub mod castfrom;
pub mod trunit;

pub use trunit::TrunIt;
pub use castit::CastIt;
pub use castfrom::CastFrom;
pub use crate::wrap::{WrappedOption, WrappedResult, WrappedErrResult};
#[cfg(test)]
mod test {
    use crate::{WrappedOption, WrappedResult};

    mod boundary;
    mod exhaustive;
    mod f32panic;
    mod f64panic;
    mod oracle;
    mod simpletests;

    #[test]
    fn test_some() {
        assert_eq!(Some(3), 3.some());
    }
    #[test]
    fn test_ok() {
        let three = 3u8;
        assert_eq!(Result::<_,()>::Ok(three), three.ok());
    }
}
