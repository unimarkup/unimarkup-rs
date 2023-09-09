use super::Snapshot;
use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_parser::elements::atomic::Paragraph;

impl AsSnapshot for Snapshot<&Paragraph> {
    fn as_snapshot(&self) -> String {
        let content: String = self
            .content
            .iter()
            .map(|inline| inline.as_string())
            .collect();

        let content = content.trim_end();

        let is_multiline = content.lines().count() > 1;

        if is_multiline {
            let content: String = content.lines().map(|line| format!("\t{line}\n")).collect();
            format!("Paragraph(\n{content}\n)")
        } else {
            format!("Paragraph({content})")
        }
    }
}
