mod frontend {
    mod config_tests;
    mod frontend_run;
    mod heading_tests;
    mod paragraph_tests;
    mod preamble_tests;
    mod umblock_tests;
}

mod middleend {
    mod ir_content;
    mod ir_macros;
    mod ir_metadata;
    mod ir_resources;
    mod ir_setup;
    pub mod ir_test_setup;
    mod ir_variables;
}

mod elements {
    mod heading_block;
    mod metadata;
}

mod backend {
    mod backend_run;
    mod inline_tests;
}