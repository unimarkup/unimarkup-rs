// //! Implementation of Render trait for Unimarkup inlines

// use unimarkup_render::{html::Html, render::Render};

// use crate::{Inline, InlineContent, TokenDelimiters, TokenKind};

// impl Render for TokenDelimiters {
//     fn render_html(&self) -> Html {
//         let open_tag = match self.open() {
//             TokenKind::Bold => "<strong>",
//             TokenKind::Italic => "<em>",
//             TokenKind::ItalicBold => "<em><strong>",
//             TokenKind::Underline => "<span style='text-decoration: underline;'>",
//             TokenKind::Subscript => "<sub>",
//             TokenKind::UnderlineSubscript => "<sub style='text-decoration: underline'>",
//             TokenKind::Superscript => "<sup>",
//             TokenKind::Overline => "<span style='text-decoration: overline;'>",
//             TokenKind::Strikethrough => "<span style='text-decoration: line-through;'>",
//             TokenKind::Highlight => "<span style='background-color: #ffaaaa;'>",
//             TokenKind::Verbatim => "<pre><code>",
//             TokenKind::Quote => "<span class='quote'>",
//             TokenKind::Math => "<span class='math'>",
//             TokenKind::OpenParens => "(",
//             TokenKind::CloseParens => ")",
//             TokenKind::OpenBracket => "<span>",
//             TokenKind::CloseBracket => "</span>",
//             TokenKind::OpenBrace => "{",
//             TokenKind::CloseBrace => "}",
//             TokenKind::Substitution => "",
//             TokenKind::Newline => "<br />",
//             TokenKind::EndOfLine => "<br />",
//             TokenKind::Whitespace => "&nbsp",
//             TokenKind::Plain => "",
//         };

//         let mut tags = String::from(open_tag);

//         if let Some(close_tag) = self.close() {
//             let close_tag = match close_tag {
//                 TokenKind::Bold => "</strong>",
//                 TokenKind::Italic => "</em>",
//                 TokenKind::ItalicBold => "</strong></em>",
//                 TokenKind::Underline => "</span>",
//                 TokenKind::Subscript => "</sub>",
//                 TokenKind::UnderlineSubscript => "</sub>",
//                 TokenKind::Superscript => "</sup>",
//                 TokenKind::Overline => "</span>",
//                 TokenKind::Strikethrough => "</span>",
//                 TokenKind::Highlight => "</span>",
//                 TokenKind::Verbatim => "</code></pre>",
//                 TokenKind::Quote => "</span>",
//                 TokenKind::Math => "</span>",
//                 TokenKind::OpenParens => "(",
//                 TokenKind::CloseParens => ")",
//                 TokenKind::OpenBracket => "<span>",
//                 TokenKind::CloseBracket => "</span>",
//                 TokenKind::OpenBrace => "{",
//                 TokenKind::CloseBrace => "}",
//                 TokenKind::Substitution => "",
//                 TokenKind::Newline => "<br />",
//                 TokenKind::EndOfLine => "<br />",
//                 TokenKind::Whitespace => "&nbsp",
//                 TokenKind::Plain => "",
//             };

//             tags.push('|');
//             tags.push_str(close_tag);
//         }

//         Html {
//             body: tags,
//             ..Default::default()
//         }
//     }
// }

// impl Render for Inline {
//     fn render_html(&self) -> Html {
//         let mut res = String::new();
//         let tags = self.delimiters().render_html().body;
//         let mut tags = tags.split('|');

//         if let Some(open_tag) = tags.next() {
//             res.push_str(open_tag);
//         }

//         let content = match self.as_ref() {
//             InlineContent::Plain(plain_content) => String::from(plain_content.as_str()),
//             InlineContent::Nested(nested_inlines) => {
//                 let mut content = String::new();

//                 for inline in nested_inlines.iter() {
//                     let html = inline.render_html().body;
//                     content.push_str(&html);
//                 }

//                 content
//             }
//         };

//         res.push_str(&content);

//         if let Some(close_tag) = tags.next() {
//             res.push_str(close_tag);
//         }

//         Html {
//             body: res,
//             ..Default::default()
//         }
//     }
// }
