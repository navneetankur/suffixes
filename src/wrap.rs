pub trait WrappedOption {
    fn some(self) -> Option<Self> where Self: core::marker::Sized;
}
pub trait WrappedResult<E> {
    fn ok(self) -> Result<Self, E> where Self: std::marker::Sized;
}
impl<T> WrappedOption for T {
    fn some(self) -> Option<Self> where Self: core::marker::Sized { Some(self) }
}
impl<T, E> WrappedResult<E> for T {
    fn ok(self) -> Result<Self, E> where Self: std::marker::Sized {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{WrappedOption, WrappedResult};
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
}
