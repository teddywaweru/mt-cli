// requesting for:
// Existing trades
// Tick values
// Account Info
use crate::sockets::ConnectionSockets;

//TRADES
pub fn get_existing_trades() -> Result<String, Box<dyn std::error::Error>> {
    let data = "TRADE;GET_OPEN_TRADES";
    let sockets = ConnectionSockets::init_and_connect()?;
    let response = sockets.request(data, 0)?.receive()?;
    sockets.disconnect();

    Ok(response)
}
fn generate_trade() {
    todo!()
}
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
