use std::io::ErrorKind;

use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
pub struct Mt5Error {
    #[serde(skip_deserializing)]
    magic: u32,
    #[serde(skip_deserializing)]
    ticket: u32,
    #[serde(skip_deserializing)]
    open_time: String,
    #[serde(skip_deserializing)]
    close_lots: f32,
    #[serde(skip_deserializing)]
    open_price: String,
    #[serde(skip_deserializing)]
    sl_attempted: f32,
    #[serde(skip_deserializing)]
    tp_attempted: f32,
    #[serde(skip_deserializing)]
    close_price: f32,
    response: String,
    response_value: String,
}

impl Mt5Error {
    fn from_mt5_response(data: String) -> Self {
        todo!()
    }
    pub fn check_if_mt5_error(data: &str) {
        use serde_json::{Map, Value};
        let x: Map<String, Value> = serde_json::from_str(data).unwrap();
        println!(
            "Value of action: {}, value of response_value: {}",
            x["action"], x["response_value"]
        );
        match serde_json::from_str::<Map<String, Value>>(data) {
            Ok(_) => {}
            Err(error) => {
                let x: Self = serde_json::from_str(data).unwrap();
                // if x["action"] == ""

                panic!("Message from ");
            }
        }
    }
    fn new() -> Self {
        Self {
            magic: 23,
            ticket: 234,
            open_time: "23423".to_string(),
            close_lots: 234.0,
            open_price: "234.0".to_string(),
            sl_attempted: 234.0,
            tp_attempted: 234.0,
            close_price: 234.0,
            response: "werw".to_string(),
            response_value: "werow".to_string(),
        }
    }
}
