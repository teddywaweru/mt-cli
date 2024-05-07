use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Timeframe {
    M1 = 1,
    M10 = 10,
    M30 = 30,
    H1 = 16385,
    D1 = 16408,
}

impl Default for Timeframe {
    fn default() -> Self {
        Self::D1
    }
}

