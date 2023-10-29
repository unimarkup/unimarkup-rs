use crate::element::Inline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Superscript {
    pub(crate) inner: Vec<Inline>,
}

impl From<Superscript> for Inline {
    fn from(value: Superscript) -> Self {
        Inline::Superscript(value)
    }
}

#[cfg(test)]
mod test {
    use unimarkup_commons::scanner::token::iterator::TokenIterator;

    use crate::{
        element::{plain::Plain, spaces::Whitespace},
        inline_parser::InlineParser,
        tokenize::iterator::InlineTokenIterator,
    };

    use super::*;

    #[test]
    fn parse_superscript() {
        let symbols = unimarkup_commons::scanner::scan_str("^superscript^");
        let mut token_iter = InlineTokenIterator::from(TokenIterator::from(&*symbols));

        let inline = &InlineParser::default().parse(&mut token_iter)[0];

        assert_eq!(
            inline,
            &Superscript {
                inner: vec![Plain {
                    content: "superscript".to_string(),
                }
                .into()],
            }
            .into(),
            "Superscript not correctly parsed."
        )
    }
}
