use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    println!("Implement me!");
}

fn parse_regex(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
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
            let (sign, ..) = parse_regex(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse_regex(input);
            assert_eq!(width, expected);
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
            let (_, _, precision) = parse_regex(input);
            assert_eq!(precision, expected);
        }
    }
}
