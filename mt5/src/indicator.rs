use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct IndicatorData {
    ehler: f32,
    j_tpo: f32,
    forecast: f32,
}
use crate::sockets::ConnectionSockets;

impl Default for IndicatorData {
    fn default() -> Self {
        IndicatorData {
            ehler: todo!(),
            j_tpo: todo!(),
            forecast: todo!(),
        }
    }
}
impl IndicatorData {
    fn get() -> Result<Self, Box<dyn std::error::Error>> {
        let sockets = ConnectionSockets::init_and_connect()?;
        let data = "TRADE;GET_INDICATOR_DATA";
        sockets.request(data, 0);

        // Parse Indicator data to save
        let data = sockets.receive();
        // let data = self::parse_indicator_data(data);
        todo!()
    }
    fn parse_indicator_data(data: String) -> Self {
        todo!()
    }
    pub fn get_historical_data(
        symbol: &str,
        timeframe: u32,
        duration: u32,
    ) -> Result<Vec<Self>, zmq::Error> {
        for _ in 0..duration {
            IndicatorData::get();
        }
        todo!()
    }
}

