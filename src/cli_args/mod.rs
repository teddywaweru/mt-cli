use crate::test_algorithm::RunAlgo;
use crate::tick::{HistoricalTickData, InstantRates};
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(author="njuwate", version="0.1.0", about="Trying something that works with zeromq and metatrader4 and Rust", long_about=None)]
/// Some application to do algorithmic trading with
pub struct Args {
    // #[arg(short, long)]
    // do_this: String,

    // #[arg(short, long)]
    // get_indicator_data: bool,
    // #[arg(short, long, default_value_t = 1)]
    // other_thing: i32,
    #[clap(subcommand)]
    pub subcmd: SubArgs,
}
impl Args {
    pub fn run(self) {
        match self.subcmd {
            SubArgs::RunAlgo => {
                let instrument = "EURUSD".to_owned();
                let timeframe = 1440;
                let duration = 240;
                let start = "2020.01.01 00:00:00".to_owned();
                let end = "2023.01.01 00:00:00".to_owned();
                let runalgo_instance =
                    RunAlgo::new(instrument, timeframe, duration, start, end).run();
            }
            SubArgs::GetActiveTrades => todo!(),
            SubArgs::GetAccountInfo => todo!(),
            SubArgs::GetInstantRates { instrument } => {
                let response: InstantRates = InstantRates::get(&instrument);
                println!("Response back: {:?}", response);
            }
            SubArgs::GetHistoricalTickData {
                instrument,
                duration,
                timeframe,
            } => {
                let response = HistoricalTickData::get(timeframe);
                println!("Response back: {:#?}", response);
            }
            SubArgs::GetIndicatorData => {}
            SubArgs::ExecuteInstantTrade { symbol, lot } => todo!(),
            SubArgs::ExecuteOtherTrade { kind } => todo!(),
            SubArgs::OtherThing { password } => todo!(),
            SubArgs::NewOtherThing { username } => todo!(),
        }
    }
}
#[derive(Subcommand)]
pub enum SubArgs {
    // #[arg(short, long, default_value_t = 1)]
    OtherThing {
        password: String,
    },
    NewOtherThing {
        username: String,
    },
    RunAlgo,
    GetActiveTrades,
    GetInstantRates {
        #[arg(short, long, default_value_t = String::from("EURUSD"))]
        instrument: String,
    },
    GetHistoricalTickData {
        #[arg(short, long, default_value_t = String::from("EURUSD"))]
        instrument: String,

        /// Amount of time for the scan, number of bars
        #[arg(short, long, default_value_t = 30)]
        duration: u32,

        #[arg(short, long, default_value_t = 1440)]
        timeframe: u32,
    },
    GetAccountInfo,
    GetIndicatorData,
    ExecuteInstantTrade {
        #[arg(long, default_value_t = String::from("EURUSD"))]
        symbol: String,

        #[arg(long, default_value_t = 1.0)]
        lot: f32,
    },
    ExecuteOtherTrade {
        #[arg(long)]
        kind: OtherTradeKind,
    },
}
#[derive(Clone)]
pub enum OtherTradeKind {
    BuyStop,
    SellStop,
    BuyLimit,
    SellLimit,
}
impl From<String> for OtherTradeKind {
    fn from(value: String) -> Self {
        todo!()
    }
}
