use crate::email::EmailString;

mod email {
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::convert::TryFrom;
    use std::ops::Deref;

    pub struct EmailString(String);

    impl EmailString {
        pub fn new(email: String) -> Option<EmailString> {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
            }

            if RE.is_match(&email) {
                return Some(EmailString(email));
            }
            None
        }
    }

    impl Deref for EmailString {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl TryFrom<String> for EmailString {
        type Error = &'static str;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            EmailString::new(value).ok_or("String is not email!")
        }
    }

    impl AsRef<str> for EmailString {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl AsMut<str> for EmailString {
        fn as_mut(&mut self) -> &mut str {
            &mut self.0
        }
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_email() {
        assert_eq!(
            *EmailString::new("test@test.com".into()).unwrap(),
            "test@test.com"
        )
    }

    #[test]
    fn failed_email() {
        assert!(EmailString::new("something".into()).is_none());
    }
}
