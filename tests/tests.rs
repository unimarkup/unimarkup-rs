mod frontend {
    mod frontend_run;
    mod umblock_tests;
    mod heading_tests;
    mod paragraph_tests;
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

mod um_elements {
    mod heading_block;
}

mod backend {
    mod backend_run;
    mod inline_tests;
}
