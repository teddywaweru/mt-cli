#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod account;
mod error;
mod indicator;
mod instant_rates;
mod mt5_bridge;
mod ohlc;
mod open_trade;
mod order;
mod parse;
mod sockets;
mod symbol;
mod tick;
mod timeframe;


pub use account::Account;
pub use indicator::Indicator;
pub use instant_rates::InstantRates;
pub use mt5_bridge::Mt5Bridge;
// pub use order::Order;
pub use sockets::ConnectionSockets;
pub use symbol::{Symbol, Symbols};
pub use tick::{HistoricalTickDataRequest, HistoricalTickData };
pub use order::{OrderType, Order, OrderTypeFilling, serde_order_type_filling};
pub use timeframe::{serde_timeframe, Timeframe };
pub use ohlc::OHLC;


use open_trade::{OpenTrade, OpenTrades};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

enum Actions {
    // GET_CURRENT_RATE("GET_CURRENT_RATE"),
    // GET_INDICATOR_DATA("GET_INDICATOR_DATA"),
    GetCurrentRate(String),
    GetIndicatorData(String),
}
