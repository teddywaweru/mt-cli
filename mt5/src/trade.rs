use crate::{error::Mt5Error, parse, Account, Mt5Bridge, Symbol};
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
    OrderTypeBuy = 0,
    OrderTypeSell = 1,
    OrderTypeBuyLimit = 2,
    OrderTypeSellLimit = 3,
    OrderTypeBuyStop = 4,
    OrderTypeSellStop = 5,
    OrderTypeBuyStopLimit = 6,
    OrderTypeSellStopLimit = 7,
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
        let pip_value: f32;
        let lot_size: f32;
        let stop_loss: f32;
        let pips: u32 = 28;
        let tp_multiplier: f32 = 1.0;
        let sl_multiplier: f32 = 1.5;
        let take_profit: f32;
        let price: f32;
        let comment: String;
        let magic: u32 = 0;
        let ticket: u32 = 0;

        // CALCULATING LOT SIZE -> get pip value first
        // get instrument category
        // separate symbol to first and second currencies
        match order_type {
            OrderType::OrderTypeBuy => {
                price = symbol.bid;
                stop_loss = symbol.bid - (( pips * 10 ) as f32 * symbol.point * sl_multiplier) as f32;
                take_profit = symbol.bid + (( pips * 10 ) as f32 * symbol.point * tp_multiplier) as f32;
                comment = "testing".to_string();
                lot_size = risk_amount /(symbol.tick_value * 10 as f32 * pips as f32 * sl_multiplier);
                println!(
                    "Symbol: {symbol:#?}\nLot_size: {lot_size}\n  \n take_profit: {take_profit},\n
                     stop_loss: {stop_loss}\n risk_amount: {risk_amount}",
                );
            }
            OrderType::OrderTypeSell => {
                stop_loss = symbol.ask + (pips as f32 * sl_multiplier) as f32;
                take_profit = symbol.ask - (pips as f32 * tp_multiplier) as f32;
                panic!()
            }
            OrderType::OrderTypeBuyLimit => {
            panic!()
            }
            _ => {
                panic!()
            }
        }
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
    #[allow(dead_code)]
    fn calc_pip_value(symbol: &Symbol, account_curr: &str) -> f32 {
        let pip_value: f32;
        let (base_curr, quote_curr) = symbol.name.split_at(3);
        if account_curr == base_curr {
            match quote_curr {
                "JPY" => {
                    pip_value = symbol.bid.recip() * 100 as f32;
                }
                _ => {
                    pip_value = symbol.bid.recip() as f32;
                }
            }
        } else if account_curr == quote_curr {
            pip_value = 1.0;
        } else {
            use crate::ConnectionSockets;
            let sockets = ConnectionSockets::initialize().unwrap();
            let data = "TRADE;GET_SYMBOLS";
            println!("quote_curr: {quote_curr}");
            let alt_symbol = Mt5Bridge::get_symbols()
                .symbols
                .iter()
                .filter(|symbol| {
                    let (alt_base, alt_quote) = symbol.name.split_at(3);
                    if alt_base == account_curr || alt_quote == account_curr {
                        true
                    } else {
                        false
                    }
                })
                .filter(|symbol| {
                    let (alt_base, alt_quote) = symbol.name.split_at(3);
                    if alt_base == quote_curr || alt_quote == quote_curr {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<&Symbol>>()
                .first()
                .unwrap()
                .to_owned()
                .to_owned();

            if &alt_symbol.name[0..3] == account_curr {
                pip_value = 1 as f32 / alt_symbol.bid;
            } else {
                pip_value = 1 as f32 * alt_symbol.bid;
            }
        }

        pip_value
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
    pub fn generate_request(self) -> String {
        let data = format!(
            "TRADE;OPEN;{:#?};{};{:.04};{:.04};{};{:.02};{:.02};{};{},",
            self.order_type as u8,
            self.symbol.name,
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
