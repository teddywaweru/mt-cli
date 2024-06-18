use crate::{parse, serde_order_type_filling, OrderTypeFilling};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub sector: String,
    pub spread: u32,
    pub point: f32,
    pub bid: f32,
    pub ask: f32,
    pub tick_value: f32,
    #[serde(with = "serde_order_type_filling")]
    pub type_filling: OrderTypeFilling,
}
impl Default for Symbol {
    fn default() -> Self {
        let name = "EURUSD_default".to_string();
        let sector = "Default Category".to_string();
        let spread = 3;
        let point = 5.33;
        let bid = 234.23;
        let ask = 234.2;
        let tick_value = 33.3;
        let type_filling = OrderTypeFilling::default();
        Symbol {
            name,
            sector,
            spread,
            point,
            bid,
            ask,
            tick_value,
            type_filling,
        }
    }
}
impl Symbol {
    pub fn parse_mt5_response(data: &str) -> Self {
        let data = parse::sanitize_mt5_response(&data);

        let symbol = match serde_json::from_str(&data) {
            Ok(symbol) => symbol,
            Err(e) => {
                panic!("Unable to parse string to Symbol object. \n Received String: \n {data} \n Error: {e}")
            }
        };
        println!("Symbol Received from mt5: {:#?}", symbol);

        symbol
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbols {
    pub symbols: Vec<Symbol>,
}

impl Default for Symbols {
    fn default() -> Self {
        Symbols {
            symbols: vec![Symbol::default(); 5],
        }
    }
}
impl Symbols {
    fn new() -> Self {
        Symbols { symbols: vec![] }
    }
    pub fn parse_mt5_response(data: &str) -> Self {
        let data = parse::sanitize_mt5_response(&data);
        let mut data: Symbols = match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(e) => {
                panic!("Unable to parse string to Symbols object. \n Received String: \n {data} \n Error: {e}")
            }
        };

        data.order_symbols()
    }

    fn order_symbols(&mut self) -> Self {
        let ordered_symbols = Symbols::new();

        self.symbols.sort_by_key(|symbol| symbol.name.clone());

        std::mem::take(self)
    }
}

#[cfg(test)]
mod SymbolTests {
    use super::*;

    #[test]
    fn get_symbol_info_test() {}
}
