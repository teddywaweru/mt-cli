use crate::{error::Mt5Error, parse, Account, Symbol};
use serde::{Deserialize, Serialize};
use serde_json::Map;
/// Trade Object Data for Generating New Trades
#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    order_type: OrderType,
    symbol: Symbol,
    price: f32,
    stop_loss: f32,
    take_profit: f32,
    comment: String,
    lot_size: f32,
    magic: u32,
    ticket: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    OrderTypeBuy,
    OrderTypeSell,
    OrderTypeBuyLimit,
    OrderTypeSellLimit,
    OrderTypeBuyStop,
    OrderTypeSellStop,
    OrderTypeBuyStopLimit,
    OrderTypeSellStopLimit,
}
impl Default for Trade {
    fn default() -> Self {
        Trade {
            order_type: OrderType::OrderTypeBuyLimit,
            symbol: Symbol::default(),
            price: 0.0,
            stop_loss: 500.0,
            take_profit: 500.0,
            comment: "Default Trade".to_string(),
            lot_size: 0.01,
            magic: 123321,
            ticket: 0,
        }
    }
}
impl Trade {
    pub fn new_trade(symbol: Symbol, order_type: OrderType, risk: f32, account: Account) -> Self {
        let risk_amount = account.current_balance * risk;

        // let account = Mt5Bridge::get_account_info();
        let lot_size: f32;
        let stop_loss: f32;
        let pips: u32 = 100;
        let tp_multiplier: f32 = 1.0;
        let sl_multiplier: f32 = 1.5;
        let take_profit: f32;
        let price: f32;
        let comment: String;
        let magic: u32;
        let ticket: u32;

        // CALCULATING LOT SIZE -> get pip value first
        // get instrument category
        // separate symbol to first and second currencies
        match order_type {
            OrderType::OrderTypeBuy => {
                price = symbol.bid;
                stop_loss = symbol.bid - (pips as f32 * sl_multiplier) as f32;
                take_profit = symbol.bid + (pips as f32 * tp_multiplier) as f32;
                comment = "".to_string();
                lot_size = Self::calc_lot_size(&symbol, pips, &account.currency);
                let lot_size = risk_amount * symbol.tick_value * symbol.point;
            }
            OrderType::OrderTypeSell => {
                stop_loss = symbol.ask + (pips as f32 * sl_multiplier) as f32;
                take_profit = symbol.ask - (pips as f32 * tp_multiplier) as f32;
            }
            OrderType::OrderTypeBuyLimit => {}
            _ => {}
        }
        todo!();
        Trade {
            order_type,
            symbol,
            price,
            stop_loss,
            take_profit,
            comment,
            lot_size,
            magic,
            ticket,
        }
    }
    fn calc_lot_size(symbol: &Symbol, pips: u32, account_currency: &str) -> f32 {
        let pip_value: f32;
        let (base_curr, quote_curr) = symbol.name.split_at(2);
        if account_currency == base_curr {
            match quote_curr {
                "JPY" => {
                    pip_value = symbol.bid.recip() * 100 as f32;
                }
                _ => {
                    pip_value = symbol.bid.recip() as f32;
                }
            }
        } else if account_currency == quote_curr {
            pip_value = 1.0;
        } else {
            todo!()
            // use Mt5
            // Mt5Bridge::
        }

        todo!()
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
            "TRADE;OPEN;{:#?};{:#?};{};{};{};{};{};{};{},",
            self.order_type,
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
