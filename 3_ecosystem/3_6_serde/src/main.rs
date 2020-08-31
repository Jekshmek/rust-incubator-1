use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};
use core::fmt;
use serde::de::{Error, Visitor};
use serde::export::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublicTariff<'a> {
    id: u64,
    price: u64,
    #[serde(deserialize_with = "de_duration", serialize_with = "se_duration")]
    duration: Duration,
    description: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PrivateTariff<'a> {
    client_price: u64,
    #[serde(deserialize_with = "de_duration", serialize_with = "se_duration")]
    duration: Duration,
    description: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Gift<'a> {
    id: u64,
    price: u64,
    description: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Debug {
    #[serde(deserialize_with = "de_duration", serialize_with = "se_duration")]
    duration: Duration,
    #[serde(deserialize_with = "de_datetime", serialize_with = "se_datetime")]
    at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Stream<'a> {
    user_id: &'a str,
    is_private: bool,
    settings: u64,
    shard_url: &'a str,
    public_tariff: PublicTariff<'a>,
    private_tariff: PrivateTariff<'a>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Request<'a> {
    #[serde(rename = "type")]
    type_: &'a str,
    stream: Stream<'a>,
    gifts: Vec<Gift<'a>>,
    debug: Debug,
}

fn se_datetime<S>(d: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = DateTime::<Utc>::from_utc(*d, Utc);
    s.serialize_str(dt.to_rfc3339().as_str())
}

fn de_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DateTimeVisitor)
}

struct DateTimeVisitor;

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("datetime formatted RFC 3339/ISO 8601")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        NaiveDateTime::parse_from_str(v, "%+").map_err(Error::custom)
    }
}

fn se_duration<S>(d: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let res: String = match d.as_millis() {
        x if x < 1000 => x.to_string() + "ms",
        x if x < 60_000 => (x / 1000).to_string() + "s",
        x if x < 3_600_000 => (x / 60_000).to_string() + "m",
        x => (x / 3_600_000).to_string() + "h",
    };

    s.serialize_str(&res)
}

fn de_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DurationVisitor)
}

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a duration string like \"1h\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let discard_last1 = |s: &str| s[..s.len() - 1].parse::<u64>().map_err(de::Error::custom);

        let discard_last2 = |s: &str| s[..s.len() - 2].parse::<u64>().map_err(de::Error::custom);

        match v {
            v if v.ends_with("ms") => discard_last2(v).map(Duration::from_millis),
            v if v.ends_with('s') => discard_last2(v).map(Duration::from_secs),
            v if v.ends_with('m') => discard_last1(v).map(|m| Duration::from_secs(m * 60)),
            v if v.ends_with('h') => discard_last1(v).map(|m| Duration::from_secs(m * 3600)),
            _ => Err(de::Error::custom("Expected 'm', 'h' or 'ms'")),
        }
    }
}

fn main() {
    let mut path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    path += "\\3_ecosystem\\3_6_serde\\request.json";

    let json = std::fs::read_to_string(path).unwrap();
    let request: Request = serde_json::from_str(&json).unwrap();

    println!("{}", serde_yaml::to_string(&request).unwrap());
    println!("{}", toml::to_string(&request).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    struct Dur {
        #[serde(deserialize_with = "de_duration", serialize_with = "se_duration")]
        duration: Duration,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    struct Dt {
        #[serde(deserialize_with = "de_datetime", serialize_with = "se_datetime")]
        datetime: NaiveDateTime,
    }

    #[test]
    fn duration_de_se() {
        let mut str = r#"{"duration":"1m"}"#;
        let mut d: Dur = serde_json::from_str(str).unwrap();
        assert_eq!(
            Dur {
                duration: Duration::from_secs(60)
            },
            d
        );
        assert_eq!(str, serde_json::to_string(&d).unwrap());

        str = r#"{"duration":"3h"}"#;
        d = serde_json::from_str(str).unwrap();
        assert_eq!(
            Dur {
                duration: Duration::from_secs(3 * 3600)
            },
            d
        );
        assert_eq!(str, serde_json::to_string(&d).unwrap());

        str = r#"{"duration":"300ms"}"#;
        d = serde_json::from_str(str).unwrap();
        assert_eq!(
            Dur {
                duration: Duration::from_millis(300)
            },
            d
        );
        assert_eq!(str, serde_json::to_string(&d).unwrap());
    }

    #[test]
    fn datetime_de_se() {
        let str = r#"{"datetime":"2019-06-28T08:35:46+00:00"}"#;
        let dt: Dt = serde_json::from_str(str).unwrap();

        assert_eq!(
            Dt {
                datetime: NaiveDate::from_ymd(2019, 06, 28).and_hms(8, 35, 46),
            },
            dt
        );

        assert_eq!(str, serde_json::to_string(&dt).unwrap());
    }
}
