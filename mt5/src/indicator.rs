use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct IndicatorData {
    ehler: f32,
    j_tpo: f32,
    forecast: f32,
}
