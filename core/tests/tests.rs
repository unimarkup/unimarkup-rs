mod general {
    pub mod metadata;
    pub mod unimarkup;
}

pub mod runner;
pub mod snapshot;

macro_rules! test_fn {
    ($fn: path) => {{
        let try_run = ::std::panic::catch_unwind($fn).map_err(|err| {
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

    let mut tests = runner::test_core_snapshots();
    tests.extend(collect_tests());

    libtest_mimic::run(&args, tests).exit();
}

fn collect_tests() -> impl IntoIterator<Item = libtest_mimic::Trial> {
    [
        test_fn!(general::metadata::create_metadata_from_memory),
        test_fn!(general::unimarkup::compile_empty_content),
    ]
}
