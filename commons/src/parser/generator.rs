use super::{Parser, ParserFn};

// Makes it impossible to implement `ParserGenerator` trait outside of this module,
// but still makes it possible to name `ParserGenerator` and use it as a bound.
mod private {
    use super::*;

    pub trait Sealed<P> {}
    impl<'a, P, T> Sealed<P> for T where T: Parser<P> + 'a + 'static {}
}

/// Trait implemented by all Unimarkup elements that can generate a parser function for their
/// content.
pub trait ParserGenerator<P>: private::Sealed<P> {
    /// Generates parser function for the given Unimarkup element.
    fn generate_parser() -> ParserFn<P>;
}

impl<'a, P, T> ParserGenerator<P> for T
where
    T: Parser<P> + 'a + 'static,
{
    // NOTE: we might need some context information for parsers. An option could be to pass
    // some kind of Context struct into generate_parser and use that for whatever we need to.
    fn generate_parser() -> ParserFn<P> {
        |input| T::parse(input)
    }
}
