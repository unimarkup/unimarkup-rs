mod lexer;
mod parser;
mod snapshot;

use libtest_mimic::Arguments;

// pub(crate) use snapshot::*;

fn get_indent(input: &str, offs: u32) -> usize {
    input[0..offs as usize]
        .bytes()
        .rev()
        .position(|byte| byte == b'\n')
        .unwrap_or(offs as usize)
}

fn main() {
    let args = Arguments::from_args();
    let lexer_tests = lexer::collect_snapshot_tests();
    let _parser_tests = parser::collect_snapshot_tests();

    let tests = lexer_tests
        .into_iter()
        .chain(_parser_tests)
        .collect::<Vec<_>>();

    libtest_mimic::run(&args, tests).exit();
}
