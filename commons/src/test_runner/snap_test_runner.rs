use crate::scanner::Symbol;
use serde::Serialize;

#[derive(Debug)]
pub struct SnapTestRunner<'a, I = ()> {
    pub info: Option<I>,
    pub desc: Option<&'a str>,
    pub input: Option<&'a str>,
    pub name: String,
    pub sub_path: Option<&'a str>,
    pub snapshot: String,
}

impl<'a> SnapTestRunner<'a> {
    pub fn with_fn<S, PF>(name: &str, input: &'a S, mut parser: PF) -> SnapTestRunner<'a, ()>
    where
        S: AsRef<[Symbol<'a>]>,
        PF: for<'s> FnMut(&'s [Symbol<'s>]) -> String,
    {
        let snapshot = parser(input.as_ref());

        SnapTestRunner {
            info: None,
            desc: None,
            input: Symbol::flatten(input.as_ref()),
            name: name.into(),
            sub_path: None,
            snapshot,
        }
    }
}

impl<'a, I> SnapTestRunner<'a, I>
where
    I: Serialize,
{
    pub fn with_description(self, description: &'a str) -> Self {
        Self {
            desc: Some(description),
            ..self
        }
    }

    pub fn with_info<Info>(self, info: Info) -> SnapTestRunner<'a, Info>
    where
        Info: Serialize,
    {
        SnapTestRunner {
            info: Some(info),
            desc: self.desc,
            input: self.input,
            name: self.name,
            sub_path: self.sub_path,
            snapshot: self.snapshot,
        }
    }

    pub fn with_sub_path(self, sub_path: &'a str) -> Self {
        Self {
            sub_path: Some(sub_path),
            ..self
        }
    }
}

#[macro_export]
macro_rules! run_snap_test {
    ($snap_test:expr $(, $path:expr)?) => {
        let snap_test: $crate::test_runner::snap_test_runner::SnapTestRunner<_> = $snap_test;

        let mut settings = $crate::test_runner::insta::Settings::clone_current();

        let mut path = std::path::Path::new("../spec/snapshots/");
        $(path = $path;)?

        settings.set_snapshot_path(path);
        settings.set_omit_expression(true);

        if let Some(subfolder) = snap_test.sub_path {
            let curr_path = settings.snapshot_path();
            let new_path = curr_path.join(subfolder);

            settings.set_snapshot_path(new_path);
        }

        if let Some(ref info) = snap_test.info {
            settings.set_info(info);
        }

        if let Some(description) = snap_test.desc {
            settings.set_description(description);
        }

        let mut snap_content = snap_test.snapshot.clone();
        if let Some(ref input) = snap_test.input {
            let ref_input = format!("\n---\nWith input:\n\n{}\n", input);
            snap_content.push_str(&ref_input);
        }

        settings.set_prepend_module_to_snapshot(false);

        settings.bind(|| {
            $crate::test_runner::insta::assert_snapshot!(snap_test.name.as_str(), snap_content);
        })
    };
}

/// Macro for snapshot testing of spec files.
///
/// ## Arguments
///
/// * *file_path* ... A path to the spec file to test, where the path must be relative to the `tests` directory of your crate (e.g. "spec/markup/blocks/paragraph.yml")
/// * *block_ty* ... A specific block element implementing `ParserGenerator` **may** be given as second argument to use the parser of the element for testing
///
/// ## Usage
///
/// ```ignore
/// test_parser_snap!("spec/markup/blocks/paragraph.yml", Paragraph);
/// test_parser_snap!("spec/markup/blocks/paragraph.yml");
/// ```
#[macro_export]
macro_rules! test_parser_snap {
    ($paths:expr, $parser_fn:expr) => {
        let test_content = $crate::test_runner::test_file::get_test_content($paths.0, $paths.1);
        let cfg = $crate::config::Config::default();

        for test in &test_content.test_file.tests {
            let symbols = $crate::scanner::scan_str(&test.input);

            let mut snap_runner = SnapTestRunner::with_fn::<_, _>(&test.name, &symbols, $parser_fn)
                .with_info(format!(
                    "Test '{}-{}' from: {}",
                    &test_content.test_file.name,
                    &test.name,
                    $paths.0.to_string_lossy()
                ));

            if let Some(ref description) = test.description {
                snap_runner = snap_runner.with_description(description);
            }

            let snap_runner = snap_runner
                .with_sub_path(test_content.snap_path.to_str().expect("Invalid sub path"));

            // TODO: preamble?

            $crate::run_snap_test!(snap_runner);
        }
    };
    ($file_path:literal) => {
        let test_content = $crate::test_runner::test_file::get_test_content($file_path);

        for test in &test_content.test_file.tests {
            let mut snap_runner = SnapTestRunner::with_parser::<&str, _>(&test.name, &test.input)
                .with_info(format!(
                    "Test '{}-{}' from: {}",
                    &test_content.test_file.name, &test.name, $file_path
                ));

            if let Some(ref description) = test.description {
                snap_runner = snap_runner.with_description(description);
            }

            let snap_runner = snap_runner
                .with_sub_path(test_content.snap_path.to_str().expect("Invalid sub path"));

            // TODO: preamble?

            $crate::run_snap_test!(snap_runner);
        }
    };
}
