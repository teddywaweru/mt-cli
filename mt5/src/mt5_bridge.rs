// requesting for:
// Existing trades
// Tick values
// Account Info
use crate::sockets::ConnectionSockets;
use crate::{
    Account, HistoricalTickData, HistoricalTickDataRequest, InstantRates, OpenTrade, Order,
    OrderType, Symbol, Symbols, Timeframe,
};
use chrono::{DateTime, Utc};
//TRADES

pub struct Mt5Bridge {
    sockets: ConnectionSockets,
}

// Get generic information from mt5
impl Mt5Bridge {
    //TICK DATA
    fn get_tick_data() {
        todo!()
    }
    fn get_ohlc_data() {
        todo!()
    }
    pub fn get_symbol_info(symbol: &str) -> Symbol {
        // TODO only get single symbol, instead of array
        // let symbols = Self::get_symbols();
        let data = format!("DATA;GET_SYMBOL_INFO;{}", symbol);
        let response = Self::init().sockets.request(&data, 0).receive();
        let symbol = Symbol::parse_mt5_response(&response);

        symbol
    }
    pub fn get_symbols() -> Symbols {
        let data = "TRADE;GET_SYMBOLS";
        let response = Mt5Bridge::init().sockets.request(data, 1).receive();
        // let response = Symbols::default();
        println!("Message received from mt5_response: {response}");

        let response = Symbols::parse_mt5_response(&response);
        response
    }
}

// Execute Operations
impl Mt5Bridge {
    pub fn generate_order(
        symbol: &str,
        order_type: String,
        risk: f32,
    ) -> OpenTrade{
        let account = Self::get_account_info();
        let symbol = Self::get_symbol_info(symbol);

        //Only processing trades in currency pairs only currently
        if symbol.sector != "Currency" {
            panic!(
                "Unable to generate trades for symbols that are not in the Currency sector. \n
                  Received symbol: {symbol:#?}"
            );
        }

        //Static OrderType currently
        let order_type = OrderType::from(order_type);
        let request = Order::new_order(symbol, order_type, risk, account).generate_request();

        let response = Mt5Bridge::init().sockets.request(&request, 0).receive();
        let response = OpenTrade::from_mt5_response(&response);

        println!("Response back on OPEN_TRADE:\n {:#?}", response);

        response
    }

    fn modify_trade() {
        todo!()
    }
    fn close_trade() {
        todo!()
    }
}

// Collect Data Reports
impl Mt5Bridge {
    // Initialize connection sockets
    fn init() -> Self {
        let sockets = ConnectionSockets::initialize().unwrap();
        Mt5Bridge { sockets }
    }

    pub fn get_existing_trades() -> Result<String, Box<dyn std::error::Error>> {
        let bridge = Self::init();
        let data = "TRADE;GET_OPEN_TRADES";
        let response = bridge.sockets.request(data, 0).receive();

        Ok(response)
    }
    pub fn get_instant_rates(symbol: &str) -> String {
        InstantRates::get(&symbol)
    }
    pub fn get_historical_tick_data(
        symbol: &Symbol,
        timeframe: Timeframe,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> HistoricalTickData {
        // let data = request.to_mt5_request();
        let request = format!(
            "DATA;GET_HISTORICAL_DATA;{};{};{};{}",
            symbol.name,
            timeframe as u32,
            start_date.format("%Y.%m.%d %H:%M"),
            end_date.format("%Y.%m.%d %H:%M")
        );
        let bridge = Self::init();
        let response = bridge.sockets.request(&request, 0).receive();

        let response = HistoricalTickData::from_mt5_response(&response);

        response
    }

    //ACCOUNT
    pub fn get_account_info() -> Account {
        let bridge = Self::init();
        let data = "DATA;GET_ACCOUNT_INFO;";

        let response = bridge.sockets.request(data, 0).receive();

        let account = Account::parse_mt5_response(&response);

        println!("Account Info received from mt5: {:#?}", account);

        account
    }
    pub fn get_indicator_data(data: &str) -> String {
        todo!()
    }

    // pub fn mt5_date_from(date: chrono::DateTime<Utc>) -> String {
    //     let date = std::time::Instant::now();
    //     let date = format!("{:#?}", date);
    //     println!("Here's the date: {date}");
    //     todo!()
    //     // let date: String = date.into();
    //     // String::from(date)
    // }
}
// pub fn get_existing_trades() -> Result<String, Box<dyn std::error::Error>> {
//     let data = "TRADE;GET_OPEN_TRADES";
//     let sockets = ConnectionSockets::init_and_connect()?;
//     let response = sockets.request(data, 0)?.receive()?;
//
//     Ok(response)
// }
