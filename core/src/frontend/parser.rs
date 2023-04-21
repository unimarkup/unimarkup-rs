//! [`parser`](crate::frontend::parser) is the module which implements parsing of the Unimarkup syntax

use crate::elements::types;

/// Generates a valid Unimarkup element id from non-empty string.
///
/// Unimarkup identifier has same restrictions as HTML id attribute:
///
/// 1. contains at least one character
/// 2. does not contain ASCII whitespace
///
/// The generated id preserves the case of the input string. Returns `None` if
/// input is an empty string.
///
/// # Arguments:
///
/// * `input` - non-empty input string. **Note:** input string which
/// consists only of whitespace is considered empty.
///
/// # Examples
///
/// ```rust
/// use unimarkup_core::frontend::parser::generate_id;
///
/// let input = "This is some input string";
/// assert_eq!(generate_id(input).unwrap(), "This-is-some-input-string");
/// ```
pub fn generate_id(input: &str) -> Option<String> {
    if input.trim().is_empty() {
        return None;
    }

    let result = {
        let mut id = String::new();

        for (i, word) in input.split_whitespace().enumerate() {
            if i != 0 {
                id.push(types::ELEMENT_TYPE_DELIMITER);
            }

            id.push_str(word);
        }

        id
    };

    Some(result)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    #[test]
    fn test__generate_id__valid_id() {
        let input = "This is some input";
        let expect = "This-is-some-input";

        assert!(super::generate_id(input).is_some(), "generate_id is none");
        assert_eq!(
            super::generate_id(input).unwrap(),
            expect,
            "generate_id does not return expected id"
        );
    }

    #[test]
    fn test__generate_id__valid_id_with_num() {
        let input = "Th15 15 1npu7 with num6ers1";
        let expect = "Th15-15-1npu7-with-num6ers1";

        assert!(super::generate_id(input).is_some(), "generate_id is none");
        assert_eq!(
            super::generate_id(input).unwrap(),
            expect,
            "generate_id does not return expected id"
        );
    }

    #[test]
    fn test__generate_id__valid_id_many_symbols() {
        let input = "7h1$\t~1d~\t \"c0n741n$\" 'many' $ym6o1$ ~!@#$%%^&^&*()_+}{[]";
        let expect = "7h1$-~1d~-\"c0n741n$\"-'many'-$ym6o1$-~!@#$%%^&^&*()_+}{[]";

        assert!(super::generate_id(input).is_some(), "generate_id is none");
        assert_eq!(
            super::generate_id(input).unwrap(),
            expect,
            "generate_id does not return expected id"
        );
    }

    #[test]
    fn test__generate_id__empty_input() {
        let input = "";

        let id = super::generate_id(input);

        assert!(id.is_none(), "generate_id is some");
    }

    #[test]
    fn test__generate_id__whitespace_only() {
        let input = " ";

        let id = super::generate_id(input);

        assert!(id.is_none(), "generate_id is some");
    }
}
