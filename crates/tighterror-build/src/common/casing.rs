use convert_case::{Case, Converter, Pattern};

pub fn convert_case(s: &str, from_case: Case, to_case: Case) -> String {
    Converter::new()
        .from_case(from_case)
        .add_pattern(Pattern::RemoveEmpty)
        .to_case(to_case)
        .convert(s)
}

pub fn is_case(s: &str, case: Case) -> bool {
    convert_case(s, case, case) == s
}

#[cfg(test)]
mod testing {
    use super::*;
    use convert_case::Case::*;

    #[test]
    fn test_convert_case() {
        let cases = &[
            // str_to_convert, from_case, to_case, expected_result
            ("MY_ERR1", UpperSnake, UpperSnake, "MY_ERR1"),
            ("MyErr1", UpperSnake, UpperSnake, "MYERR1"),
            ("MyErr1", UpperCamel, UpperSnake, "MY_ERR_1"),
            ("MY_ERR1", UpperSnake, UpperCamel, "MyErr1"),
            ("MY_ERR", UpperCamel, UpperCamel, "My_err"),
        ];

        for c in cases {
            assert_eq!(convert_case(c.0, c.1, c.2), c.3);
        }
    }

    // Checks that leading, trailing and doubled underscores make a name fail
    // its case. It simulates the names that convert_case 0.6 rejected by
    // dropping such underscores, while later versions keep them.
    #[test]
    fn test_stray_underscores() {
        let cases = &[
            // name, case, is_case, suggested name
            ("MY_ERR1", UpperSnake, true, "MY_ERR1"),
            ("_LEAD", UpperSnake, false, "LEAD"),
            ("TRAIL_", UpperSnake, false, "TRAIL"),
            ("ERR__DOUBLE", UpperSnake, false, "ERR_DOUBLE"),
            ("my_mod2", Snake, true, "my_mod2"),
            ("my__mod", Snake, false, "my_mod"),
            ("_mod", Snake, false, "mod"),
            ("My_err", UpperCamel, true, "My_err"),
        ];

        for c in cases {
            // the name is in its case only without stray underscores
            assert_eq!(is_case(c.0, c.1), c.2, "{}", c.0);
            // the conversion into the same case drops them
            assert_eq!(convert_case(c.0, c.1, c.1), c.3, "{}", c.0);
        }
    }
}
