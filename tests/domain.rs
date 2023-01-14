

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};

    use zero::domain::{SubscriberName, SubscriberEmail};

    

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    } 

    #[test]
    fn a_name_longer_than_256_grapheme_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }


    #[test]
    fn whitespace_only_names_are_rejected () {
        let name = "  ".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn an_empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn a_name_containing_an_invalid_character_is_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Matthew Castrillon-Madrigal".to_string();
        assert_ok!(SubscriberName::parse(name));
    }


}
