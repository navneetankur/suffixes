pub trait WrappedOption {
    fn some(self) -> Option<Self> where Self: core::marker::Sized;
}
impl<T> WrappedOption for T {
    fn some(self) -> Option<Self> where Self: core::marker::Sized { Some(self) }
}
pub trait WrappedResult {
    fn ok<E>(self) -> Result<Self, E> where Self: std::marker::Sized;
}
impl<T> WrappedResult for T {
    fn ok<E>(self) -> Result<Self, E> where Self: std::marker::Sized {
        Ok(self)
    }
}
pub trait WrappedErrResult {
    fn err<T>(self) -> Result<T, Self> where Self: std::marker::Sized;
}
impl<E> WrappedErrResult for E {
    fn err<T>(self) -> Result<T, Self> where Self: std::marker::Sized {
        Err(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{WrappedOption, WrappedResult, wrap::WrappedErrResult};
    #[test]
    fn t_wrapped_option() {
        fn takes_option(_: Option<i32>){}
        takes_option(4i32.some())
    }
    #[test]
    fn t_wrapped_result() {
        fn takes_result(_: Result<i32, ()>){}
        takes_result(4i32.ok())
    }
    #[test]
    fn t_wrapped_result2() {
        fn returns_result(i: i32) -> Result<i32, ()>{ i.ok() }
        assert_eq!(Ok(4), returns_result(4))
    }
    #[test]
    fn t_wrapped_err_result() {
        fn takes_result(_: Result<(), i32>){}
        takes_result(4i32.err())
    }
    #[test]
    fn t_wrapped_err_result2() {
        fn returns_result(i: i32) -> Result<(), i32>{ i.err() }
        assert_eq!(Err(4), returns_result(4))
    }
}
