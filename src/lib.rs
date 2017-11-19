/// Wrap composed types inside-out
///
/// ```
/// use insideout::InsideOut;
///
/// fn main() {
///     let maybe_result: Option<Result<_, ()>> = Some(Ok(1));
///     assert_eq!(maybe_result.inside_out(), Ok(Some(1)));
///     let result_maybe: Result<Option<i16>, &'static str> = Err("foo");
///     assert_eq!(result_maybe.inside_out(), Some(Err("foo")));
///     let result_maybe: Result<Option<_>, &'static str> = Ok(Some(3i16));
///     assert_eq!(result_maybe.inside_out(), Some(Ok(3)));
/// }
/// ```
pub trait InsideOut {
    type Output;
    fn inside_out(self) -> Self::Output;
}

impl<T, E> InsideOut for Option<Result<T, E>> {
    type Output = Result<Option<T>, E>;
    fn inside_out(self) -> Self::Output {
        match self {
            Some(Ok(x)) => Ok(Some(x)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }
}

impl<T, E> InsideOut for Result<Option<T>, E> {
    type Output = Option<Result<T, E>>;
    fn inside_out(self) -> Self::Output {
        match self {
            Ok(o) => o.map(|o| Ok(o)),
            Err(e) => Some(Err(e)),
        }
    }
}
