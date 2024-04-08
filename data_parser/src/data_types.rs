#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct OHLC {
    open: f32,
    close: f32,
    high: f32,
    low: f32,
}
impl Default for OHLC {
    fn default() -> Self {
        OHLC {
            open: 0.0,
            close: 0.0,
            high: 0.0,
            low: 0.0,
        }
    }
}
impl OHLC {
    fn new(data: String) {
        todo!()
    }
}
// Get data for a single Tick
#[derive(Debug, Serialize, Deserialize)]
pub struct TickData {
    // instrument: String,
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
            // instrument: todo!(),
            time: todo!(),
            spread: todo!(),
            real_volume: todo!(),
            tick_volume: todo!(),
            // timeframe: todo!(),
            // instrument: todo!(),
        }
    }
}
impl TickData {
    pub fn new(timeframe: String, instrument: String) -> Self {
        TickData {
            ohlc: OHLC::default(),
            // timeframe,
            // instrument,
            time: todo!(),
            spread: todo!(),
            real_volume: todo!(),
            tick_volume: todo!(),
            // instrument: todo!(),
        }
    }
    fn get_indicator_values(&self) -> Self {
        // Make a call to MT4 and get the values for the indicators that are wanted.
        // Only requires the timeframe and instrument to calculate
        // and time
        todo!()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenTrade {
    #[serde(skip_deserializing)]
    pub trade_id: String,
    _magic: i32,
    _symbol: String,
    _lots: f32,
    _type: i32,
    _open_price: f32,
    _open_time: String,
    _SL: f32,
    _TP: f32,
    _pnl: f32,
    _comment: String,
}
// TODO
//  organize?, calculate
#[derive(Debug)]
pub struct OpenTrades {
    pub trades: Vec<OpenTrade>,
}
impl Default for OpenTrades {
    fn default() -> Self {
        OpenTrades { trades: vec![] }
    }
}
impl OpenTrades {
    pub fn parse_mt5(data: String) -> Self {
        todo!()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstantRates {
    _symbol: String,
    current_time: String,
    _bid: f32,
    _ask: f32,
}
impl From<String> for OpenTrades {
    fn from(value: String) -> Self {
        OpenTrades { trades: todo!() }
    }
}
impl InstantRates {
    pub fn get(instrument: &str) -> Self {
        // TODO Generate the string using data constructs...
        let request_instant_rate: String = format!("TRADE;INSTANT_RATE;_symbol;{instrument}");
        let response = "".to_owned();

        // let response: String = ConnectionSockets::init_and_connect()
        //     .expect("Unable to connect to the connection sockets")
        //     .request(&request_instant_rate, 0)
        //     .expect("Unable to fulfill the request")
        //     .receive()
        //     .expect("No messages to display at the moment");
        InstantRates::from(response)
    }
}
impl From<String> for InstantRates {
    fn from(value: String) -> Self {
        println!("{}", value);
        let instant_rates = serde_json::from_str::<InstantRates>(&value);
        match instant_rates {
            Ok(instant_rates) => instant_rates,
            Err(e) => panic!("{}", e),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTickData {
    // #[serde]
    _data: Vec<TickData>,
    // duration: u32,
}

impl HistoricalTickData {
    pub fn get(duration: u32) -> Self {
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
        HistoricalTickData::from(response)
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
