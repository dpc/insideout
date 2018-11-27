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

/// Wrap items of an iterator inside-out
///
/// Ideally, this would be a same trait as `InsideOut`,
/// but because of potential trait impl conflicts, it's
/// different.
pub trait InsideOutIter {
    type Output;
    fn inside_out_iter(self) -> Self::Output;
}

impl<I, T> InsideOutIter for I
where
    I: Iterator<Item = T>,
    T: InsideOut,
{
    type Output = InsideOutIterImpl<I>;

    fn inside_out_iter(self) -> Self::Output {
        InsideOutIterImpl(self)
    }
}

/// An iterator that performs `InsideOut` on elements of enclosed iterator
///
/// Use `inside_out_iter()` to create.
///
/// ```
/// use insideout::InsideOutIter;
///
/// let i = vec![Ok(Some(1)), Err(2), Ok(None), Err(3)].into_iter();
///
/// let inside_outed :Vec<_> = i.inside_out_iter().collect();
///
/// assert_eq!(inside_outed,[Some(Ok(1)), Some(Err(2)), None, Some(Err(3))]);
///
/// ```
///
pub struct InsideOutIterImpl<I>(I);

impl<I, T> Iterator for InsideOutIterImpl<I>
where
    I: Iterator<Item = T>,
    T: InsideOut,
{
    type Item = <T as InsideOut>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(InsideOut::inside_out)
    }
}
