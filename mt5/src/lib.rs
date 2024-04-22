#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub(crate) mod account;
pub mod calc;
mod error;
mod indicator;
pub mod instant_rates;
mod ohlc;
mod open_trade;
pub mod order;
mod parse;
pub mod symbol;
pub mod tick;
pub mod trade;
mod mt5_bridge;
mod sockets;
pub use mt5_bridge::Mt5Bridge;
pub use sockets::ConnectionSockets;

use crate::trade::Trade;
use indicator::IndicatorData;
use instant_rates::InstantRates;
use open_trade::{OpenTrade, OpenTrades};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

enum Actions {
    // GET_CURRENT_RATE("GET_CURRENT_RATE"),
    // GET_INDICATOR_DATA("GET_INDICATOR_DATA"),
    GetCurrentRate(String),
    GetIndicatorData(String),
}
