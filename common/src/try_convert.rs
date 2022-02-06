/// The fallible equivalent of Substrate's `Convert`. If the conversion fails, `None` is returned.
pub trait TryConvert<From, To> {
    fn try_convert(from: From) -> Option<To>;
}
