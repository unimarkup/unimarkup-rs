use super::Snapshot;
use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_inline::element::InlineElement;
use unimarkup_parser::elements::atomic::Paragraph;

impl AsSnapshot for Snapshot<&Paragraph> {
    fn as_snapshot(&self) -> String {
        let content: String = self
            .content
            .iter()
            .fold(String::default(), |mut s, inline| {
                s.push_str(&inline.to_plain_string());
                s
            });

        let content = content.trim_end();

        let is_multiline = content.lines().count() > 1;

        if is_multiline {
            let content: String = content
                .lines()
                .fold(String::new(), |s, line| s + "\t" + line + "\n");
            format!("Paragraph(\n{content}\n)")
        } else {
            format!("Paragraph({content})")
        }
    }
}
