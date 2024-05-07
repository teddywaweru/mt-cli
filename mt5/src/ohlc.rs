use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct OHLC {
    #[serde(with = "mt5_date_serde")]
    time: DateTime<Utc>,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    tick_volume: f32,
}
impl Default for OHLC {
    fn default() -> Self {
        OHLC {
            time: DateTime::default(),
            open: 0.0,
            close: 0.0,
            high: 0.0,
            low: 0.0,
            tick_volume: 0.0,
        }
    }
}
impl OHLC {
    fn new(data: String) {
        todo!()
    }
    fn from_mt5_response(data: &str) -> Self {
        let data = serde_json::from_str(data).unwrap();

        data
    }
}

mod mt5_date_serde {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y.%m.%d %H:%M";

    pub fn serialize<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt = &format!("{}", datetime.format(FORMAT));
        serializer.serialize_str(dt)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)?
            .and_utc();
        Ok(dt)
    }
}
