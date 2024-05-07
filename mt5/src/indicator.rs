use crate::OHLC;

pub struct Indicator;

impl Indicator {
    pub fn get_atr(data: &Vec<OHLC>) -> f32 {
        //caclulate average TR for all data, store the last one to use later
        // Calculate ATR as ( Prev_ATR(n-1) + last_ATR ) / n
        //
        if data.len() < 10 {panic!("Unable to calculate ATR for data ranges less than 10")}
        let mut true_ranges: Vec<f32> = vec![];
        for (idx, tick) in data[data.len() - 10..].iter().enumerate() {
            if idx == 0 {
                continue;
            };

            let tr = f32::max(
                tick.high - tick.low,
                (tick.high - data[idx - 1].close).abs(),
            );
            let tr = f32::max(tr, (tick.low - data[idx - 1].close).abs());
            true_ranges.push(tr);
        }
        let true_range: f32 = true_ranges.iter().sum::<f32>() / 10 as f32;

        let atr = (true_range * (9 - 1) as f32 + true_range)
            / 10 as f32;

        atr
    }
}
