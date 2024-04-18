// requesting for:
// Existing trades
// Tick values
// Account Info
use crate::sockets::ConnectionSockets;
use mt5::{calc, instant_rates::InstantRates, tick::HistoricalTickData, trade::Trade};

//TRADES

pub struct Mt5Bridge {
sockets: ConnectionSockets

}
impl Mt5Bridge {
    pub fn init () -> Self {
        let sockets = ConnectionSockets::default();
        Mt5Bridge { sockets  }
    }
    fn connect(&self) -> Self {
    let sockets = ConnectionSockets::init_and_connect().unwrap();
    Mt5Bridge{
        sockets
    }

    }
    fn disconnect (&self) {
        self.sockets.disconnect().unwrap();

    }
pub fn get_existing_trades(&self) -> Result<String, Box<dyn std::error::Error>> {
    let data = "TRADE;GET_OPEN_TRADES";
    // let mt5_bridge = Mt5Bridge::connect();
    self.connect();
    let response = self.sockets.request(data, 0)?.receive()?;
    // let response = sockets.request(data, 0)?.receive()?;
    self.disconnect();

    Ok(response)
}
pub fn get_instant_rates(&self, instrument: &str) -> String {
    InstantRates::get(&instrument)
}
pub fn get_historical_tick_data(&self, timeframe: u32) -> String {
    HistoricalTickData::get(timeframe)
}
pub fn generate_trade(&self, symbol: &str, risk: f32) -> Result<(), Box<dyn std::error::Error>> {
    let new_trade: Trade = calc::new_trade(symbol, risk);
    let request = new_trade.generate_request();

    let sockets = ConnectionSockets::init_and_connect()?;
    let response = sockets.request(&request, 0)?.receive()?;
    let response = Trade::from_mt5_response(&response);

    println!("Response back on OPEN_TRADE:\n {:#?}", response);
    sockets.disconnect();

    Ok(())
}
}
// pub fn get_existing_trades() -> Result<String, Box<dyn std::error::Error>> {
//     let data = "TRADE;GET_OPEN_TRADES";
//     let sockets = ConnectionSockets::init_and_connect()?;
//     let response = sockets.request(data, 0)?.receive()?;
//     sockets.disconnect();
//
//     Ok(response)
// }

fn modify_trade() {
    todo!()
}
fn close_trade() {
    todo!()
}

//TICK DATA
fn get_tick_data() {
    todo!()
}
fn get_ohlc_data() {
    todo!()
}

//ACCOUNT
fn get_account_info() {
    todo!()
}
