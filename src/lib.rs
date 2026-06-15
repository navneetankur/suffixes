#![allow(clippy::needless_return)]
pub mod wrap;
pub mod castit;
pub mod castfrom;
pub mod trunit;

pub use trunit::TrunIt;
pub use castit::CastIt;
pub use castfrom::CastFrom;
pub use crate::wrap::{WrappedOption, WrappedResult};
#[cfg(test)]
mod test {
    use crate::{WrappedOption, WrappedResult};

    mod f32panic;
    mod f64panic;
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
