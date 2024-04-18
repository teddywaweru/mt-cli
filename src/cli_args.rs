use crate::mt5_bridge::Mt5Bridge;
use crate::sockets::ConnectionSockets;
use crate::test_algorithm::RunAlgo;
use clap::{Parser, Subcommand};
use mt5::instant_rates::InstantRates;
use mt5::tick::HistoricalTickData;

#[derive(Parser)]
#[command(author="njuwate", version="0.1.0", about="Trying something that works with zeromq and metatrader5 and Rust", long_about=None)]
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
            SubArgs::GetActiveTrades => {
                println!("Getting Active Trades");
                let response = Mt5Bridge::init().get_existing_trades().unwrap();

                // let open_trades = OpenTrades::parse_mt5(response);
                println!("Current Trades:{:#?}", response);
            }
            SubArgs::GetAccountInfo => todo!(),
            SubArgs::GetInstantRates { instrument } => {
                let response = Mt5Bridge::init().get_instant_rates(&instrument);
                println!("Response back: {:?}", response);
            }
            SubArgs::GetHistoricalTickData {
                instrument,
                duration,
                timeframe,
            } => {
                let response = Mt5Bridge::init().get_historical_tick_data(timeframe);
                println!("Response back: {:#?}", response);
            }
            SubArgs::GetIndicatorData => {}
            SubArgs::ExecuteInstantTrade { symbol, lot } => {
                //Requires the following to be provided:
                //Symbol to trade, and percent to risk
                //Trade will be calculated based on ATR value to determine lot, SL, tP
                // Will need to determine if it's going to be an instant or one set for
                // later...

                let symbol = "EURUSD";
                let risk: f32 = 0.02;
                // trade_type: 0,
                // symbol: "EURUSD".to_string(),
                // price: 0.0,
                // stop_loss: 500,
                // take_profit: 500,
                // comment: "Test trade".to_string(),
                // lot_size: 0.01,
                // magic: 123321,
                // ticket: 0,
                let response = Mt5Bridge::init().generate_trade(symbol, risk).unwrap();
            }
            SubArgs::ExecuteOtherTrade { kind } => todo!(),
            SubArgs::OtherThing { password } => todo!(),
            SubArgs::NewOtherThing { username } => todo!(),
            SubArgs::TrackPrices => {
                use crate::sockets::ConnectionSockets;
                let sockets = ConnectionSockets::init_and_connect().unwrap();
                let data = "TRACK_PRICES";
                sockets.request(data, 0).unwrap();

                // Parse Indicator data to save
                let data = sockets.receive().unwrap();
                println!("Here's the data : {:#?}", data);
            }
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
    TrackPrices,
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
