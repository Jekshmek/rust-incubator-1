use chrono::naive::NaiveDate;
use chrono::Datelike;
use std::convert::TryInto;

fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26";

struct User(NaiveDate);

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        User(NaiveDate::from_ymd(year, month, day))
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let now = NaiveDate::parse_from_str(NOW, "%Y-%m-%d").unwrap();

        let mut years = now.year() - self.0.year();

        if (now.month() < self.0.month())
            || (now.month() == self.0.month() && now.day() < self.0.day())
        {
            years -= 1;
        }

        years.try_into().unwrap_or(0)
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in &[
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(*y, *m, *d);
            assert_eq!(user.age(), *expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in &[
            ((2032, 6, 25), 0),
            ((2019, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(*y, *m, *d);
            assert_eq!(user.age(), *expected);
        }
    }

    #[test]
    fn is_adult() {
        for ((y, m, d), expected) in &[
            ((1990, 6, 4), true),
            ((2000, 7, 4), true),
            ((0, 1, 1), true),
            ((2010, 1, 1), false),
            ((2019, 6, 25), false),
        ] {
            let user = User::with_birthdate(*y, *m, *d);
            assert_eq!(user.is_adult(), *expected);
        }
    }
}
