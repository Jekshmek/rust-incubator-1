use crate::email::EmailString;
use crate::random_ptr::Random;

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
}

mod random_ptr {
    use rand::random;
    use std::convert::{TryFrom, TryInto};
    use std::ops::{Deref, DerefMut};

    pub struct Random<T>([T; 3]);

    impl<T: Copy> TryFrom<&[T]> for Random<T> {
        type Error = &'static str;

        fn try_from(value: &[T]) -> Result<Self, Self::Error> {
            Ok(value.try_into()?)
        }
    }

    impl<T> From<[T; 3]> for Random<T> {
        fn from(value: [T; 3]) -> Self {
            Random(value)
        }
    }

    impl<T> Deref for Random<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe {
                match random::<f32>() {
                    x if x * 3f32 < 1f32 => self.0.get_unchecked(0),
                    x if x * 3f32 < 2f32 => self.0.get_unchecked(1),
                    _ => self.0.get_unchecked(2),
                }
            }
        }
    }

    impl<T> DerefMut for Random<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                match random::<f32>() {
                    x if x * 3f32 < 1f32 => self.0.get_unchecked_mut(0),
                    x if x * 3f32 < 2f32 => self.0.get_unchecked_mut(1),
                    _ => self.0.get_unchecked_mut(2),
                }
            }
        }
    }
}

fn main() {
    let ptr = Random::from([
        EmailString::new("email1@email.com".into()).unwrap(),
        EmailString::new("email2@email.com".into()).unwrap(),
        EmailString::new("email3@email.com".into()).unwrap(),
    ]);

    println!("{}", **ptr);
}
