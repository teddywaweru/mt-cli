use crate::{Mt5Date, Symbol, Timeframe, OHLC};
use chrono::{prelude::*, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTickData {
    #[serde(deserialize_with = "deserialize_timeframe")]
    timeframe: Timeframe,
    pub ticks: Vec<OHLC>,
}
use serde::Deserializer;
fn deserialize_timeframe<'de, D>(deserializer: D) -> Result<Timeframe, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let timeframe = match s.as_str() {
        "1" => Timeframe::M1,
        "10" => Timeframe::M10,
        "30" => Timeframe::M30,
        "16385" => Timeframe::H1,
        "16408" => Timeframe::D1,
        _ => panic!("Unable to Deserialize Timeframe with value: {s}"),
    };

    Ok(timeframe)
}
impl HistoricalTickData {
    // pub fn get(duration: u32) -> String {
    //     let request = "HIST;EURUSD;1440;2023.08.01 00:00:00;2023.08.4 00:00:00";
    //     let flag = 0;
    //     let response = "".to_owned();
    //     let historical_data: Vec<OHLC>;
    //     let x = HistoricalTickData::from(response);
    //     format!("stuff, {:#?}", x)
    // }
    pub fn from_mt5_response(data: &str) -> Self {
        use crate::parse;
        let data = parse::sanitize_mt5_response(data);
        let data =
            serde_json::from_str(&data).expect(&format!("Unable to parse string: \n {data}"));
        data
    }
}
// impl Deserialize
// impl From<String> for HistoricalTickData {
//     fn from(value: String) -> Self {
//         // Optional Implementations:
//         // 1. Parse each tick data individually then add them to a Vec
//         // 2. nvestigate How to implement Vec parsing for serde_json
//         println!("{value}");
//         // let parsed_data: serde_json::Map<String, serde_json::Value> =
//         // serde_json::from_str(&value).unwrap();
//         // let data = parsed_data.get("_data").unwrap().to_owned();
//         let val: HistoricalTickData = serde_json::from_str(&value).unwrap();
//         val
//     }
// }

pub struct HistoricalTickDataRequest {
    symbol: Symbol,
    timeframe: Timeframe,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

impl Default for HistoricalTickDataRequest {
    fn default() -> Self {
        Self {
            symbol: Symbol::default(),
            timeframe: Timeframe::default(),
            start_date: Utc::now(),
            end_date: Utc::now(),
        }
    }
}
impl HistoricalTickDataRequest {
    pub fn new(
        symbol: Symbol,
        timeframe: Timeframe,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Self {
        Self {
            symbol,
            timeframe,
            start_date,
            end_date,
        }
    }
    pub fn to_mt5_request(self) -> String {
        // 2.2) DATA|SYMBOL|TIMEFRAME|START_DATETIME|END_DATETIME
        // assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2014-11-28 12:00:09"
        // NOTE: datetime has format: D'2015.01.01 00:00'
        let start_date = self.start_date.format("D'%Y.%m.%d %H:%M'");
        let data = format!(
            "TRADE;GET_OPEN_TRADES;{};{},{},{}",
            self.symbol.name, self.timeframe as u8, self.start_date, self.end_date
        );
        println!("The HistoricalTickDataRequest data: {start_date}");
        data
    }
}
