#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod account;
mod error;
mod indicator;
mod instant_rates;
mod ohlc;
mod open_trade;
mod order;
mod parse;
mod symbol;
mod tick;
mod trade;
mod mt5_bridge;
mod sockets;

pub use account::Account;
pub use instant_rates::InstantRates;
pub use symbol::{Symbol, Symbols};
pub use tick::HistoricalTickData;
pub use trade::{ Trade, OrderType };
pub use order::Order;
pub use mt5_bridge::Mt5Bridge;
pub use indicator::IndicatorData;
pub use sockets::ConnectionSockets;

use open_trade::{OpenTrade, OpenTrades};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

enum Actions {
    // GET_CURRENT_RATE("GET_CURRENT_RATE"),
    // GET_INDICATOR_DATA("GET_INDICATOR_DATA"),
    GetCurrentRate(String),
    GetIndicatorData(String),
}
