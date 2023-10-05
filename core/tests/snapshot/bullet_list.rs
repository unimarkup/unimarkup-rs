use super::Snapshot;
use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_parser::elements::indents::{BulletList, BulletListEntry};

impl AsSnapshot for Snapshot<&BulletList> {
    fn as_snapshot(&self) -> String {
        let mut content = String::new();

        for entry in &self.entries {
            content.push_str(&Snapshot(entry).as_snapshot());
            content.push('\n');
        }

        format!("BulletList(\n{content})")
    }
}

impl AsSnapshot for Snapshot<&BulletListEntry> {
    fn as_snapshot(&self) -> String {
        let entry_heading: String = self
            .heading
            .iter()
            .map(|inline| inline.as_string())
            .collect();

        if self.body.is_empty() {
            format!("BulletListEntry(\n{entry_heading}\n)")
        } else {
            let entry_body: String = self
                .body
                .iter()
                .map(|block| Snapshot(block).as_snapshot())
                .collect();

            format!("BulletListEntry(\n{entry_heading}\n\n{entry_body}\n)")
        }
    }
}
