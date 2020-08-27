use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    println!("Implement me!");
}

mod parser {
    use crate::Sign;
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while};
    use nom::character::complete::{alpha1, anychar, char, one_of};
    use nom::combinator::{opt, peek};
    use nom::error::ErrorKind;
    use nom::number::complete::be_u64;
    use nom::sequence::{pair, preceded, terminated};
    use nom::IResult;

    /// Matches `[<>^]?`
    fn align(i: &str) -> IResult<&str, char> {
        one_of("<>^")(i)
    }

    /// Matches `[+-]?`
    fn sign(i: &str) -> IResult<&str, Sign> {
        one_of("+-")(i).map(|(rest, s)| {
            (
                rest,
                match s {
                    '+' => Sign::Plus,
                    '-' => Sign::Minus,
                    _ => unreachable!(),
                },
            )
        })
    }

    fn is_digit_char(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn integer(i: &str) -> IResult<&str, usize> {
        // Matches [1-9][0-9]*
        let one = preceded(peek(one_of("123456789")), take_while(is_digit_char));

        alt((tag("0"), one))(i).and_then(|(rest, i)| {
            i.parse::<usize>()
                .map(|i| (rest, i))
                .map_err(|e| nom::Err::Error((rest, ErrorKind::IsNot)))
        })
    }

    fn parameter(i: &str) -> IResult<&str, usize> {
        terminated(integer, char('$'))(i)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn align_test() {
            assert_eq!(align("<>ab"), Ok((">ab", '<')));
            assert_eq!(align("ab"), Err(nom::Err::Error(("ab", ErrorKind::OneOf))));
        }

        #[test]
        fn sign_test() {
            assert_eq!(sign("+-ab"), Ok(("-ab", Sign::Plus)));
            assert_eq!(sign("-ab"), Ok(("ab", Sign::Minus)));
            assert_eq!(sign("ab"), Err(nom::Err::Error(("ab", ErrorKind::OneOf))));
        }

        #[test]
        fn integer_test() {
            assert_eq!(integer("123"), Ok(("", 123)));
            assert_eq!(integer("01"), Ok(("1", 0)));
            assert_eq!(
                integer("ab"),
                Err(nom::Err::Error(("ab", ErrorKind::OneOf)))
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
