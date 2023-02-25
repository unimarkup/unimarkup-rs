#![allow(non_snake_case)]
// allows using `__` for better separation in functionnames

#[allow(non_snake_case)]
mod frontend {
    mod frontend_run;
    mod heading_tests;
    mod paragraph_tests;
    mod preamble_tests;
    mod tests_helper;
}

// allows using `__` for better separation in functionnames
#[allow(non_snake_case)]
mod elements {
    mod metadata;
}

// allows using `__` for better separation in functionnames
#[allow(non_snake_case)]
mod backend {
    mod backend_run;
    mod inline_tests;
}
