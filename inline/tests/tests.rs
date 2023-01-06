pub const EXPECTED_MSG: &str = "actual(left) != expected(right)";

#[allow(non_snake_case)]
mod ast {
    mod bold_italic;
    mod escaping;
    mod mixed;
    mod mixed_nested;
    mod offseted;
    mod substitutions;
    mod text_group;
    mod verbatim;
    mod whitespaces;
}

#[allow(non_snake_case)]
mod tokenizer {
    mod accent;
    mod asterisk;
    mod backslash;
    mod mixed;
    mod mixed_nested;
    mod text_group;
    mod whitespaces;
}
