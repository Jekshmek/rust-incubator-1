use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    println!("Implement me!");
}

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while};
    use nom::character::complete::{alpha1, anychar, char, one_of};
    use nom::combinator::{opt, peek};
    use nom::error::ErrorKind;
    use nom::sequence::preceded;
    use nom::IResult;

    /// Matches `.?`
    fn fill(i: &str) -> IResult<&str, Option<char>> {
        opt(anychar)(i)
    }

    /// Matches `[<>^]?`
    fn align(i: &str) -> IResult<&str, Option<char>> {
        opt(one_of("<>^"))(i)
    }

    /// Matches `[+-]?`
    fn sign(i: &str) -> IResult<&str, Option<char>> {
        opt(one_of("+-"))(i)
    }

    /// Matches `#?`
    fn hash(i: &str) -> IResult<&str, Option<char>> {
        opt(char('#'))(i)
    }

    /// Matches `0?`
    fn zero(i: &str) -> IResult<&str, Option<char>> {
        opt(char('0'))(i)
    }

    /// Matches `[a-zA-Z0-9_]?`
    fn is_identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    /// Peeks `_[a-zA-Z0-9_]`
    fn is_underscore_then_ident_char(i: &str) -> IResult<&str, ()> {
        if i.len() < 2 {
            return Err(nom::Err::Error((i, ErrorKind::IsNot)));
        }

        if i.starts_with('_') && is_identifier_char(i.chars().nth(1).unwrap()) {
            Ok((i, ()))
        } else {
            Err(nom::Err::Error((i, ErrorKind::IsNot)))
        }
    }

    /// Matches `(([a-zA-Z][a-zA-Z0-9_]*)|(_[a-zA-Z0-9_]+))`
    fn identifier(i: &str) -> IResult<&str, &str> {
        // Matches [a-zA-Z][a-zA-Z0-9_]*
        let alpha = preceded(peek(alpha1), take_while(is_identifier_char));

        // Matches _[a-zA-Z0-9_]+
        let underscore = preceded(
            is_underscore_then_ident_char,
            take_while(is_identifier_char),
        );

        alt((alpha, underscore))(i)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fill_test() {
            assert_eq!(fill("abc"), Ok(("bc", Some('a'))));
            assert_eq!(fill(""), Ok(("", None)));
        }

        #[test]
        fn align_test() {
            assert_eq!(align("<>ab"), Ok((">ab", Some('<'))));
            assert_eq!(align("ab"), Ok(("ab", None)));
        }

        #[test]
        fn sign_test() {
            assert_eq!(sign("+-ab"), Ok(("-ab", Some('+'))));
            assert_eq!(sign("ab"), Ok(("ab", None)));
        }

        #[test]
        fn hash_test() {
            assert_eq!(hash("##ab"), Ok(("#ab", Some('#'))));
            assert_eq!(hash("ab"), Ok(("ab", None)));
        }

        #[test]
        fn zero_test() {
            assert_eq!(zero("00ab"), Ok(("0ab", Some('0'))));
            assert_eq!(zero("ab"), Ok(("ab", None)));
        }

        #[test]
        fn identifier_test() {
            assert_eq!(identifier("ident_0"), Ok(("", "ident_0")));
            assert_eq!(identifier("_ident_0*"), Ok(("*", "_ident_0")));
            assert_eq!(identifier("__"), Ok(("", "__")));
            assert_eq!(
                identifier("_"),
                Err(nom::Err::Error(("_", ErrorKind::IsNot)))
            );
        }
    }
}

fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(.?[<>^])?(?P<sign>[+-])?#?0?(?P<width>(((0|[1-9][0-9]*)|([A-Za-z]\w*))\$)|(0|[1-9][0-9]*))?(\.(?P<precision>((((0|[1-9][0-9]*)|([A-Za-z]\w*))\$)|(0|[1-9][0-9]*))|\*))?(([A-Za-z]\w*)|\?)?").unwrap()
    });

    if let Some(names) = RE.captures(input) {
        let sign = names.name("sign").map(|m| match m {
            m if m.as_str() == "+" => Sign::Plus,
            m if m.as_str() == "-" => Sign::Minus,
            _ => unreachable!(),
        });

        let precision = names.name("precision").and_then(|m| match m {
            m if m.as_str() == "*" => Some(Precision::Asterisk),
            m if !m.as_str().ends_with('$') => {
                m.as_str().parse::<usize>().map(Precision::Integer).ok()
            }
            m if m.as_str().ends_with('$') => {
                let without_last_char = &m.as_str()[..m.as_str().len() - 1];
                without_last_char
                    .parse::<usize>()
                    .map(Precision::Argument)
                    .ok()
            }
            _ => unreachable!(),
        });

        let width = names.name("width").and_then(|m| match m {
            m if !m.as_str().ends_with('$') => m.as_str().parse::<usize>().ok(),
            m if m.as_str().ends_with('$') => {
                let without_last_char = &m.as_str()[..m.as_str().len() - 1];
                without_last_char.parse::<usize>().ok()
            }
            _ => unreachable!(),
        });

        return (sign, width, precision);
    }

    (None, None, None)
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in &[
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse(input);
            assert_eq!(width, *expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input);
            assert_eq!(precision, expected);
        }
    }
}
