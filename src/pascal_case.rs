#[cfg(not(feature = "convert-case"))]
pub(crate) fn to_pascal_case(input: &str) -> Result<String, &'static [u8]> {
    let mut pascal = String::with_capacity(input.len());
    let mut at_word_boundary = true;
    for char in input.chars() {
        if !char.is_ascii() {
            return Err(b"Input contains non-ASCII characters.");
        } else if char.is_ascii_alphanumeric() {
            if at_word_boundary {
                pascal.push(char.to_ascii_uppercase());
                at_word_boundary = false;
            } else if char.is_ascii_uppercase() || char.is_ascii_lowercase() {
                // If not at a word boundary and character is uppercase or lowercase,
                // append it as is.
                pascal.push(char);
            } else if char.is_ascii_digit() {
                // If not at a word boundary and character is digit,
                // append it as is and set at word boundary.
                pascal.push(char);
                at_word_boundary = true;
            } else {
                // If not at a word boundary and character is not alphanumeric,
                // set at word boundary.
                at_word_boundary = true;
            }
        } else {
            // If non-alphanumeric character, set at a word boundary
            at_word_boundary = true;
        }
    }
    Ok(pascal)
}

#[cfg(feature = "convert-case")]
pub(crate) fn to_pascal_case(input: &str) -> Result<String, &'static [u8]> {
    if !input.is_ascii() {
        return Err(b"Input contains non-ASCII characters.");
    }
    Ok(convert_case::Casing::to_case(
        &input.to_string(),
        convert_case::Case::Pascal,
    ))
}

#[cfg(test)]
mod tests {
    use super::to_pascal_case;

    #[test]
    fn basic() {
        assert_eq!(to_pascal_case("foo"), Ok("Foo".to_string()));
    }

    #[test]
    fn empty() {
        assert_eq!(to_pascal_case(""), Ok("".to_string()));
    }

    #[test]
    fn hyphen() {
        assert_eq!(to_pascal_case("foo-bar-baz"), Ok("FooBarBaz".to_string()));
    }

    #[test]
    fn underscore() {
        assert_eq!(to_pascal_case("foo_bar_baz"), Ok("FooBarBaz".to_string()));
    }

    #[test]
    fn special_non_ascii_characters() {
        let result = to_pascal_case("foo-bár");
        let err_message = b"Input contains non-ASCII characters." as &[u8];
        assert_eq!(result, Err(err_message));
    }

    #[test]
    fn lower_followed_by_upper() {
        assert_eq!(to_pascal_case("fooBar"), Ok("FooBar".to_string()));
    }

    #[test]
    fn digit_followed_by_upper() {
        assert_eq!(to_pascal_case("foo5Bar"), Ok("Foo5Bar".to_string()));
    }

    #[test]
    fn upper_followed_by_digit() {
        assert_eq!(to_pascal_case("FoO5bar"), Ok("FoO5Bar".to_string()));
    }

    #[test]
    fn digit_followed_by_lower() {
        assert_eq!(to_pascal_case("foo5bar"), Ok("Foo5Bar".to_string()));
    }
}
