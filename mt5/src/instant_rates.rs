use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstantRates {
    symbol: String,
    current_time: String,
    bid: f32,
    ask: f32,
}
impl InstantRates {
    pub fn get(symbol: &str) -> String {
        // TODO Generate the string using data constructs...
        let request_instant_rate: String = format!("TRADE;INSTANT_RATE;_symbol;{symbol}");
        let response = "".to_owned();

        // let response: String = ConnectionSockets::init_and_connect()
        //     .expect("Unable to connect to the connection sockets")
        //     .request(&request_instant_rate, 0)
        //     .expect("Unable to fulfill the request")
        //     .receive()
        //     .expect("No messages to display at the moment");
        // InstantRates::from(response);
        format!(
            "Instant Rates Response \n: {:#?}",
            InstantRates::from(response)
        )
    }
}
impl From<String> for InstantRates {
    fn from(value: String) -> Self {
        println!("{}", value);
        let instant_rates = serde_json::from_str::<InstantRates>(&value);
        match instant_rates {
            Ok(instant_rates) => instant_rates,
            Err(e) => panic!("{}", e),
        }
    }
}
