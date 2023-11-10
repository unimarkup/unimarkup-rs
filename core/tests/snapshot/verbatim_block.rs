use super::Snapshot;
use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_parser::elements::enclosed::VerbatimBlock;

impl AsSnapshot for Snapshot<&VerbatimBlock> {
    fn as_snapshot(&self) -> String {
        let content = &self.content;

        let is_multiline = content.lines().count() > 1;

        if is_multiline {
            let content: String = content
                .lines()
                .fold(String::new(), |s, line| s + "\t" + line + "\n");
            format!("VerbatimBlock(\n{content}\n)")
        } else {
            format!("VerbatimBlock({content})")
        }
    }
}
