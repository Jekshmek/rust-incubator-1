use std::time::Duration;

use chrono::NaiveDateTime;
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

fn se_datetime<S>(d: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&d.format("%Y-%m-%dT%H:%M:%S%z").to_string())
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
        formatter.write_str("datetime formatted %Y-%m-%dT%H:%M:%S%z")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S%z").map_err(Error::custom)
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
    let tariff: PublicTariff = serde_json::from_str(
        r#"{
            "id": 1,
            "price": 100,
            "duration": "12h",
            "description": "test public tariff"
        }"#,
    )
    .unwrap();

    dbg!(tariff);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    struct Dur {
        #[serde(deserialize_with = "de_duration", serialize_with = "se_duration")]
        duration: Duration,
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
}
