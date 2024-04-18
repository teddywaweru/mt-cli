use crate::{account::Account, order::Order, symbol::Symbol, trade::Trade};
pub fn new_trade(symbol: &str, risk: f32) -> Trade {
    // Check if this symbol exists...
    // Check amount and if allowed to trade anyway? Fail if market is inactive?
    // Interacts with:
    // The account - to get current balance so we can calculate the risk amount which determines
    // the lot size
    // The tick = to get the currnt price value and to calculate SL and TP
    let risk_amount = Account::get_balance() * risk;
    let pips = Symbol::get_symbol_info(symbol).pips;
    let lot_size = risk_amount * pips as f32;
    todo!()
}
pub fn new_pending_order(symbol: &str, risk: f32, price: f32) -> Order {
    todo!()
}
pub fn this_and_that (sfwe: i32) {

}
