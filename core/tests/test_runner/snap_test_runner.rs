use serde::Serialize;
use unimarkup_core::parser::{
    symbol::{IntoSymbols, Symbol},
    MainParser, ParserGenerator,
};

use crate::test_runner::as_snapshot::AsSnapshot;

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
    pub fn with_parser<PG, S>(name: &str, input: S) -> SnapTestRunner<'a, ()>
    where
        PG: ParserGenerator,
        S: IntoSymbols<'a, Vec<Symbol<'a>>> + Clone + Into<&'a str>,
    {
        let symbols = input.clone().into_symbols();
        let parser_fn = PG::generate_parser();

        let (blocks, rest) = parser_fn(&symbols).unwrap();

        assert_eq!(rest.len(), 0, "Whole input should be parsed");

        SnapTestRunner {
            info: None,
            desc: None,
            input: Some(input.into()),
            name: name.into(),
            sub_path: None,
            snapshot: blocks.as_snapshot(),
        }
    }
}

impl<'a> SnapTestRunner<'a> {
    pub fn with_main_parser<S>(name: &str, input: S) -> SnapTestRunner<'a, ()>
    where
        S: IntoSymbols<'a, Vec<Symbol<'a>>> + Clone + Into<&'a str>,
    {
        let symbols = input.clone().into_symbols();
        let parser = MainParser::default();

        let blocks = parser.parse(&symbols);

        SnapTestRunner {
            info: None,
            desc: None,
            input: Some(input.into()),
            name: name.into(),
            sub_path: None,
            snapshot: blocks.as_snapshot(),
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
    ($snap_test:ident) => {
        let snap_test: $crate::test_runner::snap_test_runner::SnapTestRunner<_> = $snap_test;

        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path("../spec/snapshots/");
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
            let ref_input = format!("---\nWith input:\n\n{}\n", input);
            snap_content.push_str(&ref_input);
        }

        settings.set_prepend_module_to_snapshot(false);

        settings.bind(|| {
            insta::assert_snapshot!(snap_test.name.as_str(), snap_content);
        })
    };
}

#[macro_export]
macro_rules! test_parser_snap {
    ($block_ty:ty, $file_path:literal) => {
        let path = PathBuf::from(file!());
        let mut path: PathBuf = path.strip_prefix("core").unwrap().into();

        path.set_file_name("");
        path.push($file_path);

        let mut sub_path: PathBuf = path
            .components()
            .skip_while(|component| Path::new(component) != Path::new("markup"))
            .skip(1)
            .collect();

        sub_path.set_extension("");

        let input = std::fs::read_to_string(&path).unwrap();
        let test_file: $crate::test_runner::test_file::TestFile =
            serde_yaml::from_str(&input).unwrap();

        for test in &test_file.tests {
            let mut snap_runner =
                SnapTestRunner::with_parser::<$block_ty, &str>(&test.name, &test.input)
                    .with_info(file!());

            if let Some(ref description) = test.description {
                snap_runner = snap_runner.with_description(description);
            }

            let snap_runner =
                snap_runner.with_sub_path(sub_path.to_str().expect("Invalid sub path"));

            // TODO: preamble?

            $crate::run_snap_test!(snap_runner);
        }
    };
    ($file_path:literal) => {
        let path = PathBuf::from(file!());
        let mut path: PathBuf = path.strip_prefix("core").unwrap().into();

        path.set_file_name("");
        path.push($file_path);

        let mut sub_path: PathBuf = path
            .components()
            .skip_while(|component| Path::new(component) != Path::new("markup"))
            .skip(1)
            .collect();

        sub_path.set_extension("");

        let input = std::fs::read_to_string(&path).unwrap();
        let test_file: $crate::test_runner::test_file::TestFile =
            serde_yaml::from_str(&input).unwrap();

        for test in &test_file.tests {
            let mut snap_runner = SnapTestRunner::with_main_parser::<&str>(&test.name, &test.input)
                .with_info(file!());

            if let Some(ref description) = test.description {
                snap_runner = snap_runner.with_description(description);
            }

            let snap_runner =
                snap_runner.with_sub_path(sub_path.to_str().expect("Invalid sub path"));

            // TODO: preamble?

            $crate::run_snap_test!(snap_runner);
        }
    };
}
