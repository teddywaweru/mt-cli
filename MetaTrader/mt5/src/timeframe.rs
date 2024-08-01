use serde::{Deserialize, Serialize};

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

pub mod serde_timeframe {
    use crate::Timeframe;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S>(timeframe: &Timeframe, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timeframe = match timeframe {
            Timeframe::M1 => Timeframe::M1 as u32,
            Timeframe::M10 => Timeframe::M10 as u32,
            Timeframe::M30 => Timeframe::M30 as u32,
            Timeframe::H1 => Timeframe::H1 as u32,
            Timeframe::D1 => Timeframe::D1 as u32,
        };

        serializer.serialize_u32(timeframe)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Timeframe, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timeframe = u32::deserialize(deserializer)?;
        let timeframe = match timeframe {
            1 => Timeframe::M1,
            10 => Timeframe::M10,
            30 => Timeframe::M30,
            16385 => Timeframe::H1,
            16408 => Timeframe::D1,
            _ => panic!("Unable to Parse the timeframe value provided: \n Value: {timeframe}"),
        };
        Ok(timeframe)
    }
}
// fn deserialize_timeframe<'de, D>(deserializer: D) -> Result<Timeframe, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//
//     let timeframe = match s.as_str() {
//         "1" => Timeframe::M1,
//         "10" => Timeframe::M10,
//         "30" => Timeframe::M30,
//         "16385" => Timeframe::H1,
//         "16408" => Timeframe::D1,
//         _ => panic!("Unable to Deserialize Timeframe with value: {s}"),
//     };
//
//     Ok(timeframe)
// }
