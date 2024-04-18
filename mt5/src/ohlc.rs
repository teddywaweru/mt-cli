use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OHLC {
    open: f32,
    close: f32,
    high: f32,
    low: f32,
}
impl Default for OHLC {
    fn default() -> Self {
        OHLC {
            open: 0.0,
            close: 0.0,
            high: 0.0,
            low: 0.0,
        }
    }
}
impl OHLC {
    fn new(data: String) {
        todo!()
    }
}
