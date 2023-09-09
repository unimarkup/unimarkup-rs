// allows using `__` for better separation in functionnames
#![allow(non_snake_case)]

mod general {
    pub mod metadata;
    pub mod unimarkup;
}

mod blocks;
mod snapshot;

macro_rules! test_fn {
    ($fn: path) => {{
        let try_run = 
        ::std::panic::catch_unwind($fn)
            .map_err(|err| {
                let panic_msg = err
                    .downcast_ref::<&str>()
                    .unwrap_or(&"Panic message not available");

                format!("Test case panicked: {}", panic_msg).into()
            });

        libtest_mimic::Trial::test(stringify!($fn), || try_run)
    }};
}

fn main() {
    let args = libtest_mimic::Arguments::from_args();
    let snap_tests = blocks::test_block_snapshots();

    let mut tests = snap_tests;
    tests.append(&mut collect_tests());

    libtest_mimic::run(&args, tests).exit();
}

fn collect_tests() -> Vec<libtest_mimic::Trial> {
    vec![
        test_fn!(general::metadata::test__metadata__create_from_memory),
        test_fn!(general::unimarkup::compile_empty_content),
    ]
}
