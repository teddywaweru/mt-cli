// requesting for:
// Existing trades
// Tick values
// Account Info
use crate::sockets::ConnectionSockets;
use crate::{Account, HistoricalTickData, InstantRates, OrderType, Symbol, Symbols, Trade};
//TRADES
pub struct Mt5Bridge {
    sockets: ConnectionSockets,
}

// Get information from mt5
impl Mt5Bridge {
    pub fn get_symbol_info(&self, check_symbol: &str) -> Symbol {
        // TODO only get single symbol, instead of array
        match self.get_symbols() {
            Ok(symbols) => {
                for symbol in symbols.symbols.iter() {
                    if check_symbol == symbol.name {
                        return symbol.clone();
                    }
                }
                let mut available_symbols = "".to_string();
                let _ = symbols
                    .symbols
                    .iter()
                    .map(|symbol| available_symbols.push_str(&symbol.name));
                print!("Available symbols: {:#?}", available_symbols);
                panic!(
                    "Symbol provided does not exist in list of possible trade symbols\n
                       Provided symbol: {check_symbol}. \n
                       Available Symbols: \n
                       {available_symbols}"
                )
            }
            Err(e) => {
                panic!("Unable to get symbols from MT5. \n Error: {e}");
            }
        }
    }
    fn get_symbols(&self) -> Result<Symbols, Box<dyn std::error::Error>> {
        let data = "TRADE;GET_SYMBOLS";
        let response = self.sockets.request(data, 1).receive();

        let response = Symbols::parse_mt5_response(&response);
        Ok(response)
    }
}

// Execute Operations
impl Mt5Bridge {}

// Collect Data Reports
impl Mt5Bridge {
    // Initialize connection sockets
    pub fn init() -> Self {
        let sockets = ConnectionSockets::initialize().unwrap();
        Mt5Bridge { sockets }
    }

    pub fn get_existing_trades(self) -> Result<String, Box<dyn std::error::Error>> {
        let data = "TRADE;GET_OPEN_TRADES";
        let response = self.sockets.request(data, 0).receive();

        Ok(response)
    }
    pub fn get_instant_rates(&self, symbol: &str) -> String {
        InstantRates::get(&symbol)
    }
    pub fn get_historical_tick_data(&self, timeframe: u32) -> String {
        HistoricalTickData::get(timeframe)
    }
    pub fn generate_trade(
        &self,
        symbol: &str,
        risk: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let account = self.get_account_info()?;
        let symbol = self.get_symbol_info(symbol);
        if symbol.sector != "Currency" {
            panic!(
                "Unable to generate trades for symbols that are not in the Currency sector. \n
                  Received symbol: {symbol:#?}"
            );
        }
        println!("Symbol Received from mt5: {:#?}", symbol);
        let order_type = OrderType::OrderTypeBuy;
        let new_trade: Trade = Trade::new_trade(symbol, order_type, risk, account);
        let request = new_trade.generate_request();

        let response = self.sockets.request(&request, 0).receive();
        let response = Trade::from_mt5_response(&response);

        println!("Response back on OPEN_TRADE:\n {:#?}", response);

        Ok(())
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
    pub fn get_account_info(&self) -> Result<Account, Box<dyn std::error::Error>> {
        println!("Gathering account information");
        let data = "TRADE;GET_ACCOUNT_INFO";

        let response = self.sockets.request(data, 1).receive();

        let response = Account::parse_mt5_response(&response)?;

        println!("Response back from MT5 on account request: {:#?}", response);

        Ok(response)
    }
    pub fn get_indicator_data(&self, data: &str) -> String {
        todo!()
    }
}
// pub fn get_existing_trades() -> Result<String, Box<dyn std::error::Error>> {
//     let data = "TRADE;GET_OPEN_TRADES";
//     let sockets = ConnectionSockets::init_and_connect()?;
//     let response = sockets.request(data, 0)?.receive()?;
//
//     Ok(response)
// }
