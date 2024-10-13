use std::fmt::Write;

use unimarkup_commons::test_runner::as_snapshot::AsSnapshot;
use unimarkup_frontend::parser::block::{
    bulletlist::BulletList, heading::Heading, paragraph::Paragraph, verbatim::Verbatim, Block,
};

use crate::snapshot::Snapshot;

impl AsSnapshot for Snapshot<(&str, Block)> {
    fn as_snapshot(&self) -> String {
        let (input, block) = &self.0;

        match block {
            Block::Blankline(span) => {
                format!("Blankline @ ({} -> {})", span.offs, span.offs + span.len)
            }
            Block::Heading(heading) => Snapshot((*input, heading)).as_snapshot(),
            Block::Paragraph(paragraph) => Snapshot((*input, paragraph)).as_snapshot(),
            Block::Verbatim(verbatim) => Snapshot((*input, verbatim)).as_snapshot(),
            Block::BulletList(bullet_list) => Snapshot((*input, bullet_list)).as_snapshot(),
        }
    }
}

impl AsSnapshot for Snapshot<(&str, &Heading)> {
    fn as_snapshot(&self) -> String {
        let (_, heading) = self.0;

        let mut output = String::with_capacity(heading.content.iter().map(|c| c.len()).sum());

        let _ = writeln!(
            &mut output,
            "Heading({}) @ ({} -> {}) {{",
            heading.level,
            heading.span.offs,
            heading.span.offs + heading.span.len
        );

        for line in heading.content.iter().flat_map(|s| s.lines()) {
            let _ = writeln!(&mut output, "  {line}");
        }

        let _ = writeln!(&mut output, "}}");

        output
    }
}

impl AsSnapshot for Snapshot<(&str, &Paragraph)> {
    fn as_snapshot(&self) -> String {
        let (_, paragraph) = self.0;

        let mut output = String::with_capacity(paragraph.content.iter().map(|s| s.len()).sum());

        let _ = writeln!(
            &mut output,
            "Paragraph @ ({} -> {}) {{",
            paragraph.span.offs,
            paragraph.span.offs + paragraph.span.len
        );

        for line in paragraph.content.iter().flat_map(|s| s.lines()) {
            let _ = writeln!(&mut output, "  {line}");
        }

        let _ = writeln!(&mut output, "}}");

        output
    }
}

impl AsSnapshot for Snapshot<(&str, &Verbatim)> {
    fn as_snapshot(&self) -> String {
        todo!()
    }
}

impl AsSnapshot for Snapshot<(&str, &BulletList)> {
    fn as_snapshot(&self) -> String {
        todo!()
    }
}
