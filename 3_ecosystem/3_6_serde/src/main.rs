use std::time::Duration;

use core::fmt;
use serde::de::{Error, Visitor};
use serde::export::Formatter;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublicTariff<'a> {
    id: u64,
    price: u64,
    #[serde(deserialize_with = "deserialize_duration")]
    duration: Duration,
    description: &'a str,
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
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
        E: de::Error,
    {
        let discard_last1 = |s: &str| s[..s.len() - 1].parse::<u64>().map_err(de::Error::custom);

        let discard_last2 = |s: &str| s[..s.len() - 2].parse::<u64>().map_err(de::Error::custom);

        match v {
            v if v.ends_with('m') => discard_last1(v).map(|m| Duration::from_secs(m * 60)),
            v if v.ends_with('h') => discard_last1(v).map(|m| Duration::from_secs(m * 3600)),
            v if v.ends_with("ms") => discard_last2(v).map(Duration::from_millis),
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
        #[serde(deserialize_with = "deserialize_duration")]
        duration: Duration,
    }

    #[test]
    fn duration_de() {
        assert_eq!(
            Dur {
                duration: Duration::from_secs(60)
            },
            serde_json::from_str(r#"{"duration":"1m"}"#).unwrap()
        );
        assert_eq!(
            Dur {
                duration: Duration::from_secs(3 * 3600)
            },
            serde_json::from_str(r#"{"duration":"3h"}"#).unwrap()
        );
        assert_eq!(
            Dur {
                duration: Duration::from_millis(300)
            },
            serde_json::from_str(r#"{"duration":"300ms"}"#).unwrap()
        );
    }
}
