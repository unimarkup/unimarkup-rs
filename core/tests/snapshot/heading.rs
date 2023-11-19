use super::Snapshot;
use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_inline::element::InlineElement;
use unimarkup_parser::elements::atomic::Heading;

impl AsSnapshot for Snapshot<&Heading> {
    fn as_snapshot(&self) -> String {
        let content: String = self
            .content
            .iter()
            .fold(String::default(), |mut s, inline| {
                s.push_str(&inline.as_unimarkup());
                s
            });

        let content = content.trim_end();
        let variant_name = format!("Heading-{}", self.level);

        let is_multiline = content.lines().count() > 1;

        if is_multiline {
            let content: String = content
                .lines()
                .fold(String::new(), |s, line| s + "\t" + line + "\n");
            format!("{variant_name}(\n{content}\n)")
        } else {
            format!("{variant_name}({content})")
        }
    }
}
