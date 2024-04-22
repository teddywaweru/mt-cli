use crate::ohlc::OHLC;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TickData {
    // symbol: String,
    #[serde(flatten)]
    ohlc: OHLC,
    time: String,
    // timeframe: String,
    spread: i32,
    real_volume: i32,
    tick_volume: i32,
}
impl Default for TickData {
    fn default() -> Self {
        TickData {
            ohlc: todo!(),
            // symbol: todo!(),
            time: todo!(),
            spread: todo!(),
            real_volume: todo!(),
            tick_volume: todo!(),
            // timeframe: todo!(),
            // symbol: todo!(),
        }
    }
}
impl TickData {
    ///Create a New TickData Struct
    pub fn new(timeframe: String, symbol: String) -> Self {
        TickData {
            ohlc: OHLC::default(),
            // timeframe,
            // symbol,
            time: todo!(),
            spread: todo!(),
            real_volume: todo!(),
            tick_volume: todo!(),
            // symbol: todo!(),
        }
    }
    ///Get Requested Tick Data
    pub fn get_indicator_values(&self) -> Self {
        // Make a call to MT4 and get the values for the indicators that are wanted.
        // Only requires the timeframe and symbol to calculate
        // and time
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTickData {
    // #[serde]
    _data: Vec<TickData>,
    // duration: u32,
}
impl HistoricalTickData {
    pub fn get(duration: u32) -> String {
        let request = "HIST;EURUSD;1440;2023.08.01 00:00:00;2023.08.4 00:00:00";
        let flag = 0;
        let response = "".to_owned();
        // let response = ConnectionSockets::init_and_connect()
        //     .unwrap()
        //     .request(request, flag)
        //     .unwrap()
        //     .receive()
        //     .unwrap();
        let historical_data: Vec<TickData>;
        let x = HistoricalTickData::from(response);
        format!("stuff, {:#?}", x)
    }
    fn parse_data(data: String) -> Self {
        todo!()
    }
}
// impl Deserialize
impl From<String> for HistoricalTickData {
    fn from(value: String) -> Self {
        // Optional Implementations:
        // 1. Parse each tick data individually then add them to a Vec
        // 2. nvestigate How to implement Vec parsing for serde_json
        println!("{value}");
        // let parsed_data: serde_json::Map<String, serde_json::Value> =
        // serde_json::from_str(&value).unwrap();
        // let data = parsed_data.get("_data").unwrap().to_owned();
        let val: HistoricalTickData = serde_json::from_str(&value).unwrap();
        val
    }
}
