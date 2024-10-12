use crate::lexer::token::Token;
use serde::Serialize;

pub use insta::{assert_snapshot, Settings};

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
    pub fn with_fn_any<I, F>(name: &str, input: &'a I, mut func: F) -> SnapTestRunner<'a, ()>
    where
        I: AsRef<str>,
        F: FnMut(&I) -> String,
    {
        let snapshot = func(input);

        SnapTestRunner {
            info: None,
            desc: None,
            input: Some(input.as_ref()),
            name: name.into(),
            sub_path: None,
            snapshot,
        }
    }

    pub fn with_fn<S, PF>(name: &str, input: &'a S, mut parser: PF) -> SnapTestRunner<'a, ()>
    where
        S: AsRef<[Token<'a>]>,
        PF: for<'s, 'i> FnMut(&'s [Token<'i>]) -> String,
    {
        let snapshot = parser(input.as_ref());

        SnapTestRunner {
            info: None,
            desc: None,
            input: Token::flatten(input.as_ref()),
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

/// Runs the [`SnapTestRunner`] and writes the snapshot in the given output path.
///
/// # Arguments
///
/// * *snap_test* - The [`SnapTestRunner`] to run.
/// * *path* - The path to write the snapshot to.
///
/// [SnapTestRunner]: self::SnapTestRunner
#[macro_export]
macro_rules! run_snap_test {
    ($snap_test:expr, $path:expr) => {
        let snap_test: $crate::test_runner::snap_test_runner::SnapTestRunner<_> = $snap_test;

        let mut settings = $crate::test_runner::snap_test_runner::Settings::clone_current();

        let path = $path;

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
            $crate::test_runner::snap_test_runner::assert_snapshot!(
                snap_test.name.as_str(),
                snap_content
            );
        })
    };
}
