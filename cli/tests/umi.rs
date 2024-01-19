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

    let mut i = 0;
    while i < output.len() {
        assert_eq!(
            input[i].as_unimarkup(),
            output[i].as_unimarkup(),
            "Inline contains wrong content"
        );
        i += 1;
    }
    true
}

fn equals_blocks_output(input: &Vec<Block>, output: &Vec<Block>) -> bool {
    assert_eq!(
        input.len(),
        output.len(),
        "Parsed Blocks does not have the same length as the input"
    );

    let mut i = 0;
    while i < input.len() {
        assert_eq!(
            input[i].variant_str(),
            output[i].variant_str(),
            "Blocks did not match up at Index"
        );
        let block_in = input[i].clone();
        let block_out = output[i].clone();
        match (block_in, block_out) {
            (Block::Heading(block_in), Block::Heading(block_out)) => {
                assert_eq!(block_in.id, block_out.id, "Heading ids do not match!");
                assert_eq!(
                    block_in.level, block_out.level,
                    "Heading Levels do not match!"
                );
                assert!(equals_inlines_output(&block_in.content, &block_out.content));
                assert_eq!(
                    block_in.attributes, block_out.attributes,
                    "Heading Attributes do not match!"
                );
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
                assert_eq!(
                    block_in.attributes, block_out.attributes,
                    "Verbatim Attributes do not match"
                );
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

                let mut j = 0;
                while j < block_in.entries.len() {
                    assert_eq!(
                        block_in.entries[j].keyword, block_out.entries[j].keyword,
                        "Bullet List Entry Keyword does not match"
                    );
                    assert!(equals_inlines_output(
                        &block_in.entries[j].heading,
                        &block_out.entries[j].heading
                    ));
                    assert!(equals_blocks_output(
                        &block_in.entries[j].body,
                        &block_out.entries[j].body
                    ));
                    j += 1;
                }
            }
            _ => return false,
        }

        i += 1;
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

    let looped_doc = &Umi::create_um(workbook.to_string().as_str(), &mut workbook.config)
        .map_err(|_| panic!())
        .unwrap();
    let input = um.get_document();

    assert!(
        equals_umi_output(input, looped_doc),
        "Output does not equal the Input"
    );
}
