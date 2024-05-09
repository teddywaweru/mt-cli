use crate::{
    error::Mt5Error, parse, Account, HistoricalTickDataRequest, Indicator, Mt5Bridge, Symbol,
    Timeframe,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Map;

/// Trade Object Data for Generating New Trades
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    magic: u32,
    ticket: u32,
    symbol: Symbol,
    volume: f32,
    price: f32,
    sl: f32,
    tp: f32,
    deviation: u32,
    order_type: OrderType,
    order_type_filling: OrderTypeFilling,
    order_type_time: OrderTypeTime,
    expiration: DateTime<Utc>,
    comment: String,
    position: u32,
    position_by: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderTypeFilling {
    SymbolFillingFOK = 0,
    SymbolFillingIOC = 1,
    SymbolFillingBOC = 2,
    // SymbolTradeExecutionInstant,
    // SymbolTradeExecutionRequest,
    // SymbolTradeExecutionMarket,
    // SymbolTradeExecutionExchange,
}

pub mod serde_order_type_filling {
    use crate::OrderTypeFilling;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OrderTypeFilling, D::Error>
    where
        D: Deserializer<'de>,
    {
        let type_filling = u8::deserialize(deserializer)?;
        let type_filling = match type_filling {
            0 => OrderTypeFilling::SymbolFillingFOK,
            1 => OrderTypeFilling::SymbolFillingIOC,
            2 => OrderTypeFilling::SymbolFillingBOC,
            _ => panic!("No matching Type Filling for val provided: \n Value: {type_filling}"),
        };

        Ok(type_filling)
    }
    pub fn serialize<S>(type_filling: &OrderTypeFilling, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let type_filling = match type_filling {
            OrderTypeFilling::SymbolFillingFOK => OrderTypeFilling::SymbolFillingFOK as u8,
            OrderTypeFilling::SymbolFillingIOC => OrderTypeFilling::SymbolFillingIOC as u8,
            OrderTypeFilling::SymbolFillingBOC => OrderTypeFilling::SymbolFillingBOC as u8,
        };

        serializer.serialize_u8(type_filling)
    }
}

impl Default for OrderTypeFilling {
    fn default() -> Self {
        Self::SymbolFillingIOC
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum OrderTypeTime {
    OrderTimeGtc,
    OrderTimeDay,
    OrderTimeSpecified,
    OrderTimeSpecifiedDay,
}

impl Default for OrderTypeTime {
    fn default() -> Self {
        Self::OrderTimeGtc
    }
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
    OrderNanDefault,
}
impl Default for OrderType {
    fn default() -> Self {
        Self::OrderNanDefault
    }
}
impl From<String> for OrderType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "buy" => OrderType::OrderTypeBuy,
            "sell" => OrderType::OrderTypeSell,
            "buylimit" => OrderType::OrderTypeBuyLimit,
            "selllimit" => OrderType::OrderTypeSell,
            "buystop" => OrderType::OrderTypeBuyStop,
            "sellstop" => OrderType::OrderTypeSellStop,
            _ => panic!("Unable to translate the OrderType String to Object: {value}"),
        }
    }
}
impl Default for Order {
    fn default() -> Self {
        Order {
            magic: 123321,
            ticket: 0,
            symbol: Symbol::default(),
            volume: 0.01,
            price: 0.0,
            sl: 500.0,
            tp: 500.0,
            deviation: 0,
            order_type: OrderType::OrderNanDefault,
            order_type_filling: OrderTypeFilling::default(),
            order_type_time: OrderTypeTime::default(),
            expiration: Utc::now(),
            comment: "Default Trade".to_string(),
            position: 0,
            position_by: 0,
        }
    }
}
impl Order {
    pub fn new_order(symbol: Symbol, order_type: OrderType, risk: f32, account: Account) -> Self {
        let risk_amount = account.current_balance * risk;

        // let account = Mt5Bridge::get_account_info();
        let pip_value: f32;
        let volume: f32;
        let sl: f32;
        let tp: f32;
        let pips: u32 = Self::calculate_pips(&symbol);
        let tp_multiplier: f32 = 1.0;
        let sl_multiplier: f32 = 1.5;
        let price: f32;
        let comment: String;
        let magic: u32 = 0;

        // CALCULATING LOT SIZE -> get pip value first
        // get instrument category
        // separate symbol to first and second currencies
        match order_type {
            OrderType::OrderTypeBuy => {
                price = symbol.ask;
                sl = symbol.bid - ((pips * 10) as f32 * symbol.point * sl_multiplier) as f32;
                tp = price + ((pips * 10) as f32 * symbol.point * tp_multiplier) as f32;
                comment = "testing".to_string();
                volume = risk_amount
                    / (symbol.tick_value * 10 as f32 * pips as f32 * sl_multiplier)
                    / 10 as f32;
            }
            OrderType::OrderTypeSell => {
                price = symbol.bid;
                sl = symbol.ask + ((pips * 10) as f32 * symbol.point * sl_multiplier) as f32;
                tp = price - ((pips * 10) as f32 * symbol.point * tp_multiplier) as f32;
                comment = "testing sells".to_string();
                volume = risk_amount
                    / (symbol.tick_value * 10 as f32 * pips as f32 * sl_multiplier)
                    / 10 as f32;
            }
            OrderType::OrderTypeBuyLimit => {
                panic!()
            }
            _ => {
                panic!()
            }
        }

        let mut new_order = Order::default();
        new_order.order_type = order_type;
        new_order.symbol = symbol;
        new_order.price = price;
        new_order.sl = sl;
        new_order.tp = tp;
        new_order.comment = comment;
        new_order.volume = volume;
        new_order.magic = magic;

        new_order
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
            let symbols = Mt5Bridge::get_symbols().symbols;
            let alt_symbol: &Symbol = symbols
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
                .unwrap();

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
        if let Ok(data) = serde_json::from_str::<Order>(data) {
            return data;
        } else {
            println!("Data received on mt5_response for new order: {data}");
            match serde_json::from_str::<Value>(data)
                .unwrap()
                // .get("error_kind")
                // .unwrap()
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
            "TRADE;OPEN;{:#?};{};{};{};{};{};{:.02};{};{},",
            self.order_type as u8,
            self.symbol.name,
            self.price,
            self.sl,
            self.tp,
            self.comment,
            self.volume,
            self.magic,
            self.ticket
        );
        data
    }

    fn calculate_pips(symbol: &Symbol) -> u32 {
        let timeframe = Timeframe::default();
        // 2.2) DATA|SYMBOL|TIMEFRAME|START_DATETIME|END_DATETIME
        // NOTE: datetime has format: D'2015.01.01 00:00'
        //
        let period = 40;
        let timeframe = Timeframe::D1;
        // let start_date = Mt5Bridge::get_historical_tick_data(timeframe.into());
        let end_date = Utc::now();
        let duration = Duration::days(period);
        let start_date = end_date - duration;
        let historical_data =
            Mt5Bridge::get_historical_tick_data(&symbol, timeframe, start_date, end_date);
        let atr = Indicator::get_atr(&historical_data.ticks);
        let atr = atr / symbol.point / 10 as f32;

        atr as u32
        // let historical_data_request =
        // HistoricalTickDataRequest::new(symbol, timeframe, start_date.to_string(), end_date.to_string());
    }
}

#[cfg(test)]
mod trade_tests {
    use super::*;

    #[test]
    fn test_calculate_pips() {
        let timeframe = Timeframe::default();
        // 2.2) DATA|SYMBOL|TIMEFRAME|START_DATETIME|END_DATETIME
        //
        let period = 14;
        let symbol = Symbol::default();
        let end_date = Utc::now();
        let duration = Duration::days(period);
        let start_date = end_date - duration;
        let historical_data_request =
            HistoricalTickDataRequest::new(symbol, timeframe, start_date, end_date);
        let data = historical_data_request.to_mt5_request();
        todo!()

        // let response = Mt5Bridge::get_historical_tick_data(historical_data_request);
        // let tick_data = Mt5Bridge::get_historical_tick_data(timeframe.into());
    }
}
