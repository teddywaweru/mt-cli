// needs a struct that holds details of what the sim is about ie
use crate::indicator::IndicatorData;
use crate::sockets::ConnectionSockets;
use chrono::{DateTime, Local, TimeZone};

pub struct RunAlgo {
    instrument: String,
    timeframe: u32,
    duration: u32,
    start: DateTime<Local>,
    end: DateTime<Local>,
}
impl RunAlgo {
    pub fn new(
        instrument: String,
        timeframe: u32,
        duration: u32,
        start: String,
        end: String,
    ) -> Self {
        // let start: DateTime<Local> = DateTime::naive_local(&self)
        let start: DateTime<Local> = DateTime::parse_from_str(&start, "%Y.%m.%d %H:%M:00")
            .expect("Unable to convert to the string to a DateTime format")
            .into();
        let end: DateTime<Local> = DateTime::parse_from_str(&end, "%Y.%m.%d %H:%M:00")
            .expect("Unable to convert to the string to a DateTime format")
            .into();
        RunAlgo {
            instrument,
            timeframe,
            duration,
            start,
            end,
        }
    }
    /// Take parameters, return a percentage to illustrate the performance
    pub fn run(self) -> Result<u32, Box<dyn std::error::Error>> {
        //develop the parameters that are necessary to initiate the run
        // historical data, indicator data,
        let current_date: DateTime<Local> = Local::now();
        let current_date = self.start.format("%Y.%m.%d %H:%M:00");
        let current_date = self.end.format("%Y.%m.%d %H:%M:00");
        // let hist_data: HistData = HistData::get(&self.instrument, self.timeframe, self.duration)?;
        let hist_indicator_data: Vec<IndicatorData> =
            IndicatorData::get_historical_data(&self.instrument, self.timeframe, self.duration)?;

        todo!()
    }
}
