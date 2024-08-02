use convert_case::{Case, Converter};

pub fn convert_case(s: &str, from_case: Case, to_case: Case) -> String {
    Converter::new()
        .from_case(from_case)
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
}
