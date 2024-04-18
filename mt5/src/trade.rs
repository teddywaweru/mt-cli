use crate::{error::Mt5Error, parse};
use serde::{Deserialize, Serialize};
use serde_json::Map;
/// Trade Object Data for Generating New Trades
#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    trade_type: i32,
    symbol: String,
    price: f32,
    stop_loss: u32,
    take_profit: u32,
    comment: String,
    lot_size: f32,
    magic: u32,
    ticket: u32,
}

impl Default for Trade {
    fn default() -> Self {
        Trade {
            trade_type: 0,
            symbol: "EURUSD".to_string(),
            price: 0.0,
            stop_loss: 500,
            take_profit: 500,
            comment: "Test trade".to_string(),
            lot_size: 0.01,
            magic: 123321,
            ticket: 0,
        }
    }
}
#[derive(Debug)]
enum Mt5ErrorKind {
    Normal { tough: i32 },
    Other { error: Mt5Error },
    Critical,
}
impl Trade {
    pub fn new() -> Self {
        Trade {
            trade_type: 0,
            symbol: "EURUSD".to_string(),
            price: 0.0,
            stop_loss: 500,
            take_profit: 500,
            comment: "Test trade".to_string(),
            lot_size: 0.01,
            magic: 123321,
            ticket: 0,
        }
    }
    pub fn from_mt5_response(data: &str) -> Self {
        use crate::error::Mt5Error;
        use serde_json::Value;
        if let Ok(data) = serde_json::from_str::<Trade>(data) {
            return data;
        } else {
            match serde_json::from_str::<Value>(data)
                .unwrap()
                .get("error_kind")
                .unwrap()
                // .to_owned()
                .to_string()
                .as_str()
            {
                "No Connection" => todo!(),
                "No Connection" => todo!(),
                _ => todo!(),
            }
            if let Ok(data) = serde_json::from_str::<Mt5Error>(data) {
                panic!(
                    "Response from MetaTrader: Error in Execution\n. Response {:#?}",
                    data
                )
            } else {
                panic!(
                    "Unable to parse response to serde_json Value Object. \n Received Response: \n{}",
                    data)
            }
        }
    }
    pub fn generate_request(&self) -> String {
        let data = format!(
            "TRADE;OPEN;{};{};{};{};{};{};{};{};{},",
            self.trade_type,
            self.symbol,
            self.price,
            self.stop_loss,
            self.take_profit,
            self.comment,
            self.lot_size,
            self.magic,
            self.ticket
        );
        data
    }
}
