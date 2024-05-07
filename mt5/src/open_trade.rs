use crate::parse;
use crate::{OrderType, Symbol};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenTrade {
    ticket: u32,
    magic: i32,
    symbol: Symbol,
    lot_size: f32,
    trade_type: OrderType,
    open_price: f32,
    open_time: String,
    stop_loss: f32,
    take_profit: f32,
    pnl: f32,
    comment: String,
}
impl Default for OpenTrade {
    fn default() -> Self {
        Self {
            ticket: 123321,
            magic: 123321,
            symbol: Symbol::default(),
            lot_size: 0.01,
            trade_type: OrderType::OrderNanDefault,
            open_price: 0.0,
            open_time: "Default time".to_string(),
            stop_loss: 500.0,
            take_profit: 500.0,
            pnl: 0.0,
            comment: "Default Comment".to_string(),
        }
    }
}
impl OpenTrade {
    pub fn from_mt5_response(data: &str) -> Self {
        let data = parse::sanitize_mt5_response(&data);
        let data = data.replace("'", "\"");
        serde_json::from_str(&data).expect(&format!(
            "Unable to parse string to MT5 OpenTrades Object:\n Received String:\n {}",
            data
        ))
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenTrades {
    pub trades: Vec<OpenTrade>,
}
impl Default for OpenTrades {
    fn default() -> Self {
        OpenTrades { trades: vec![] }
    }
}
impl OpenTrades {
    pub fn parse_mt5_response(data: &str) -> OpenTrades {
        let data = parse::sanitize_mt5_response(&data);
        serde_json::from_str(&data).expect(&format!(
            "Unable to parse string to MT5 OpenTrades Object:\n Received String:\n {}",
            data
        ))
    }
}
impl From<String> for OpenTrades {
    fn from(value: String) -> Self {
        OpenTrades { trades: todo!() }
    }
}
