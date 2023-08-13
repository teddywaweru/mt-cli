use crate::tick::InstantRates;
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
        _ => panic!("There is no matching action to the provided data"),
    };
    Ok(())
}
