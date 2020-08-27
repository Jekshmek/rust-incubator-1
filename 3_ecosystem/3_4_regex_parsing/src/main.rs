use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    println!("Implement me!");
}

mod parser {
    pub use crate::{Precision, Sign};

    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        character::complete::{alpha1, anychar, char, one_of},
        combinator::{all_consuming, map, opt, peek},
        error::ErrorKind,
        sequence::{delimited, pair, preceded, terminated, tuple},
        IResult,
    };

    /// Matches `[<>^]?`
    fn align(i: &str) -> IResult<&str, char> {
        one_of("<>^")(i)
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

    /// Matches `0|[1-9][0-9]*`
    fn integer(i: &str) -> IResult<&str, usize> {
        // Matches [1-9][0-9]*
        let one = preceded(peek(one_of("123456789")), take_while(is_digit_char));

        alt((tag("0"), one))(i).and_then(|(rest, i)| {
            i.parse::<usize>()
                .map(|i| (rest, i))
                .map_err(|_| nom::Err::Error((rest, ErrorKind::IsNot)))
        })
    }

    /// Matches [`integer`]`$`
    fn parameter(i: &str) -> IResult<&str, usize> {
        terminated(integer, char('$'))(i)
    }

    /// Matches [`parameter`] | [`integer`]
    fn width(i: &str) -> IResult<&str, usize> {
        alt((parameter, integer))(i)
    }

    /// Matches [`width`] | `*`
    fn precision(i: &str) -> IResult<&str, Precision> {
        alt((
            map(parameter, Precision::Argument),
            map(integer, Precision::Integer),
            map(char('*'), |_| Precision::Asterisk),
        ))(i)
    }

    type ParseOutput = (Option<Sign>, Option<usize>, Option<Precision>);

    pub fn parse(i: &str) -> IResult<&str, ParseOutput> {
        let fill = anychar;
        let hash = char('#');
        let zero = char('0');
        let dot = char('.');
        let type_ = alt((identifier, tag("?")));

        all_consuming(terminated(
            tuple((
                delimited(
                    opt(alt((preceded(fill, align), align))),
                    opt(sign),
                    pair(opt(hash), opt(zero)),
                ),
                opt(width),
                opt(preceded(dot, precision)),
            )),
            opt(type_),
        ))(i)
    }

    pub fn parse_without_err(i: &str) -> ParseOutput {
        let parsed = parse(i);
        dbg!(&parsed);
        if let Ok((_, output)) = parsed {
            return output;
        }

        (None, None, None)
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
        fn identifier_test() {
            assert_eq!(identifier("ident_0"), Ok(("", "ident_0")));
            assert_eq!(identifier("_ident_0*"), Ok(("*", "_ident_0")));
            assert_eq!(identifier("__"), Ok(("", "__")));
            assert_eq!(
                identifier("_"),
                Err(nom::Err::Error(("_", ErrorKind::IsNot)))
            );
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
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum Precision {
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

            let (sign, ..) = parser::parse_without_err(input);
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

            let (_, width, _) = parser::parse_without_err(input);
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

            let (_, _, precision) = parser::parse_without_err(input);
            assert_eq!(precision, expected);
        }
    }
}
