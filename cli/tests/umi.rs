use std::iter::zip;
use std::path::PathBuf;

use unimarkup_core::{
    commons::config::Config,
    inline::element::{Inline, InlineElement},
    parser::{document::Document, elements::blocks::Block},
    render::umi::Umi,
    Unimarkup,
};

fn compile_um(config: Config) -> Option<Unimarkup> {
    let source = std::fs::read_to_string(&config.input).ok()?;

    Some(Unimarkup::parse(&source, config))
}

fn equals_inlines_output(input: &Vec<Inline>, output: &Vec<Inline>) -> bool {
    assert_eq!(
        input.len(),
        output.len(),
        "Parsed Inlines does not have the same number of elements"
    );

    for (in_elem, out_elem) in zip(input.iter(), output.iter()) {
        assert_eq!(
            in_elem.as_unimarkup(),
            out_elem.as_unimarkup(),
            "Inline contains wrong content"
        )
    }
    true
}

fn equals_blocks_output(input: &Vec<Block>, output: &Vec<Block>) -> bool {
    assert_eq!(
        input.len(),
        output.len(),
        "Parsed Blocks does not have the same length as the input"
    );

    for (in_elem, out_elem) in zip(input.iter(), output.iter()) {
        assert_eq!(
            in_elem.variant_str(),
            out_elem.variant_str(),
            "Blocks did not match up at Index"
        );
        let block_in = in_elem.clone();
        let block_out = out_elem.clone();
        match (block_in, block_out) {
            (Block::Heading(block_in), Block::Heading(block_out)) => {
                assert_eq!(block_in.id, block_out.id, "Heading ids do not match!");
                assert_eq!(
                    block_in.level, block_out.level,
                    "Heading Levels do not match!"
                );
                assert!(equals_inlines_output(&block_in.content, &block_out.content));
                // assert_eq!(
                //     block_in.attributes, block_out.attributes,
                //     "Heading Attributes do not match!"
                // );
            }
            (Block::Paragraph(block_in), Block::Paragraph(block_out)) => {
                assert!(equals_inlines_output(&block_in.content, &block_out.content));
            }
            (Block::VerbatimBlock(block_in), Block::VerbatimBlock(block_out)) => {
                assert_eq!(
                    block_in.content, block_out.content,
                    "Verbatim Content does not match"
                );
                assert_eq!(
                    block_in.data_lang, block_out.data_lang,
                    "Verbatim Data_Lang does not match"
                );
                // assert_eq!(
                //     block_in.attributes, block_out.attributes,
                //     "Verbatim Attributes do not match"
                // );
                assert_eq!(
                    block_in.implicit_closed, block_out.implicit_closed,
                    "Verbatim Implicit_Closed does not match"
                );
                assert_eq!(
                    block_in.tick_len, block_out.tick_len,
                    "Verbatim Tick-Len does not match"
                );
            }
            (Block::BulletList(block_in), Block::BulletList(block_out)) => {
                assert_eq!(
                    block_in.entries.len(),
                    block_out.entries.len(),
                    "Bullet List entry count does not match"
                );

                for (in_entry, out_entry) in zip(block_in.entries.iter(), block_out.entries.iter())
                {
                    assert_eq!(
                        in_entry.keyword, out_entry.keyword,
                        "Bullet List Entry Keyword does not match"
                    );
                    assert!(equals_inlines_output(&in_entry.heading, &out_entry.heading));
                    assert!(equals_blocks_output(&in_entry.body, &out_entry.body));
                }
            }
            _ => return false,
        }
    }

    true
}

fn equals_umi_output(input: &Document, output: &Document) -> bool {
    assert_eq!(
        input.config, output.config,
        "Parsed UMI Config differs from original Config"
    );

    equals_blocks_output(&input.blocks, &output.blocks)
}

#[test]
fn umi_supported() {
    let mut config = Config::default();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    path.push("tests/test_files/supported.um");
    config.input = path;

    let um = compile_um(config).unwrap();
    let mut umi = um.render_umi().unwrap();
    let workbook = umi.create_workbook();

    let looped_doc = &Umi::create_um(workbook.to_string().as_str(), &mut workbook.config).unwrap();
    let input = um.get_document();

    assert!(
        equals_umi_output(input, looped_doc),
        "Output does not equal the Input"
    );
}
