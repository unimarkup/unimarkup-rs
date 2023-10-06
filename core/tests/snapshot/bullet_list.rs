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

        let content: String = content.lines().map(|line| format!("  {line}\n")).collect();
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

        let entry_heading = if entry_heading.lines().count() > 1 {
            let entry_heading: String = entry_heading
                .lines()
                .map(|line| format!("    {line}\n"))
                .collect();
            format!("  EntryHeading(\n{entry_heading}\n  )")
        } else {
            format!("  EntryHeading({entry_heading})")
        };

        if self.body.is_empty() {
            format!("BulletListEntry(\n{entry_heading}\n)")
        } else {
            let entry_body: String = self
                .body
                .iter()
                .map(|block| Snapshot(block).as_snapshot())
                .collect();

            let entry_body: String = entry_body
                .lines()
                .map(|line| format!("    {line}\n"))
                .collect();
            let entry_body = format!("  EntryBody(\n{entry_body}  )");

            format!("BulletListEntry(\n{entry_heading}\n{entry_body}\n)")
        }
    }
}
