
pub const EXPECTED_MSG: &str = "actual(left) != expected(right)";

#[allow(non_snake_case)]
mod ast {
  mod bold_italic;
  mod escaping;
  mod mixed_nested;
  mod mixed;
  mod offseted;
  mod substitutions;
  mod whitespaces;
}

#[allow(non_snake_case)]
mod tokenizer {
  mod accent;
  mod asterisk;
  mod backslash;
  mod mixed_nested;
  mod mixed;
  mod text_group;
  mod whitespaces;
}
