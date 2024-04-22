use serde::{Serialize,Deserialize};
use crate::parse;
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub sector: String,
    pub spread: u32,
    pub point: f32,
    pub bid: f32,
    pub ask: f32,
    pub tick_value: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbols {
    pub symbols: Vec<Symbol>
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
         Symbol {
            name,
            sector,
            spread,
            point,
            bid,
            ask,
            tick_value,
        }
        
    }
}

impl Default for Symbols {
    fn default() -> Self {
        Symbols {
            symbols: vec![Symbol::default()]
        }
        
    }
}
impl Symbols {
    pub fn parse_mt5_response(data: &str) -> Self {
        let data = data.replace("'", "\"");
        let data = parse::remove_action(&data);
        let data = match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(e) => {
                panic!("Unable to parse string to Symbols object. \n Received String: \n {data} \n Error: {e}")
            }
        };

        data
    }
}

#[cfg(test)]
mod SymbolTests {
    use super::*;

    #[test]
    fn get_symbol_info_test() {}
}
