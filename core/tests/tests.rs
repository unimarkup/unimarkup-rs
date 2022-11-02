#![allow(non_snake_case)]
// allows using `__` for better separation in functionnames

#[allow(non_snake_case)]
mod frontend {
    mod frontend_run;
}

// allows using `__` for better separation in functionnames
#[allow(non_snake_case)]
mod middleend {
    mod content;
    mod macros;
    mod metadata;
    mod resources;
    mod setup;
    pub mod test_setup;
    mod variables;
}

// allows using `__` for better separation in functionnames
#[allow(non_snake_case)]
mod elements {
    mod heading;
    mod inline;
    mod paragraph;
    mod preamble;
    mod tests_helper;
}

// allows using `__` for better separation in functionnames
#[allow(non_snake_case)]
mod general {
    mod metadata;
    mod unimarkup;
}

// allows using `__` for better separation in functionnames
#[allow(non_snake_case)]
mod backend {
    mod backend_run;
}
