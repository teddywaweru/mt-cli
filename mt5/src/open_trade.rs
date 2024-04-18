use crate::parse;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenTrade {
    #[serde(skip_deserializing)]
    pub trade_id: String,
    magic: i32,
    symbol: String,
    lot_size: f32,
    trade_type: i32,
    open_price: f32,
    open_time: String,
    stop_loss: f32,
    take_profit: f32,
    pnl: f32,
    comment: String,
}
// TODO
//  organize?, calculate
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
        let data = parse::remove_action(&data);
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
