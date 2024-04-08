#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
pub mod data_types;
// use crate::tick::InstantRates;
use data_types::{InstantRates, OpenTrade, OpenTrades};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

enum Actions {
    // GET_CURRENT_RATE("GET_CURRENT_RATE"),
    // GET_INDICATOR_DATA("GET_INDICATOR_DATA"),
    GetCurrentRate(String),
    GetIndicatorData(String),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IndicatorData {
    ehler: f32,
    j_tpo: f32,
    forecast: f32,
}
fn parse_price_data(data: String) -> Result<(), serde_json::Error> {
    let mut instant_rates: Map<String, Value> = serde_json::from_str(&data)?;
    instant_rates.remove("_action");
    let data = serde_json::to_string(&instant_rates)?;
    let instant_rates: InstantRates = serde_json::from_str(&data)?;
    Ok(())
}
pub fn parse_indicator_data(data: String) -> Result<IndicatorData, serde_json::Error> {
    let mut indicator_data: Map<String, Value> = serde_json::from_str(&data)?;
    indicator_data.remove("_action");
    let indicator_data: String = serde_json::to_string(&indicator_data)?;
    let indicator_data: IndicatorData = serde_json::from_str(&indicator_data)?;
    Ok(indicator_data)
}

pub fn parse_open_trade(
    trade_id: &String,
    trade: &serde_json::Value,
) -> Result<OpenTrade, serde_json::Error> {
    let mut trade: OpenTrade = serde_json::from_value(trade.to_owned())?;
    trade.trade_id = trade_id.to_owned();
    Ok(trade)
}

pub fn parse_open_trades(data: String) -> Result<OpenTrades, serde_json::Error> {
    let mut open_trades: Map<String, Value> = serde_json::from_str(&data)?;
    open_trades.remove("_action");
    let open_trades: String = serde_json::to_string(open_trades.get("_trades").unwrap())?;
    let mapped_trades: Map<String, Value> = serde_json::from_str(&open_trades)?;
    let mut open_trades: OpenTrades = OpenTrades { trades: vec![] };
    for (idx, (trade_id, trade)) in mapped_trades.iter().enumerate() {
        let trade: OpenTrade = parse_open_trade(trade_id, trade)?;
        open_trades.trades.insert(idx, trade);
    }
    Ok(open_trades)
}

pub fn parse_message(data: String) -> Result<(), serde_json::Error> {
    let parsed_data: Map<String, Value> = serde_json::from_str(&data)?;
    let action = parsed_data
        .get("_action")
        .expect("_action key is not in the provided data")
        .as_str()
        .expect("Provided action is not a string value");
    match action {
        "GET_CURRENT_RATE" => {
            parse_price_data(data)?;
        }
        "GET_INDICATOR_DATA" => {
            parse_indicator_data(data)?;
        }
        "OPEN_TRADES" => {
            parse_open_trades(data)?;
        }
        _ => panic!("There is no matching action to the provided data"),
    };
    Ok(())
}

#[cfg(test)]
mod parse_data_tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, BufRead};
    #[test]
    fn parse_open_trade_test() {
        unimplemented!();
    }
    #[test]
    fn parse_open_trades_test() {
        let file = File::open("test_data/current_trades.json").unwrap();
        let reader = io::BufReader::new(file);
        let data = reader
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .join("");
        // for lines in reader.lines() {
        // let line = lines.unwrap();
        let open_trades = parse_open_trades(data).unwrap();
        println!("The lines then {:#?}", open_trades);
        // }
        unimplemented!();
    }
}
