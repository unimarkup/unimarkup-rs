#![allow(non_snake_case)]
// allows using `__` for better separation in functionnames

#[allow(non_snake_case)]
mod frontend {
    mod frontend_run;
}

// allows using `__` for better separation in functionnames
#[allow(non_snake_case)]
mod elements {
    mod heading;
    mod inline;
    mod paragraph;
    mod tests_helper;
}

// allows using `__` for better separation in functionnames
#[allow(non_snake_case)]
mod general {
    mod metadata;
    mod unimarkup;
}

mod test_runner;
