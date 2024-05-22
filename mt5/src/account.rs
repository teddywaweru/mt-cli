use crate::parse;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    account_number: i32,
    name: String,
    current_time: String,
    pub current_balance: f32,
    current_equity: f32,
    leverage: u32,
    pub currency: String,
    free_margin: f32,
    current_profit: f32,
}
// "{'_action': 'GET_ACCOUNT_INFO', 'account_number':61189795, '_data': [{'current_time': '2024.04.19
// 08:56', 'name':'Teddy Waweru Njuguna', 'balance':40862.08000000, 'equity':41827.15000000, 'profit':965.07000000, 'margin_free':41003.31000000,
//  'leverage' :400}]}"

impl Default for Account {
    fn default() -> Self {
        let account_number = 65;
        let name = "njuwate".to_string();
        let current_time = "2024".to_string();
        let current_balance = 3434.233;
        let current_equity = 3234.55;
        let leverage = 500;
        let currency = "USD".to_string();
        let free_margin = 3543.34;
        let current_profit = 333.2;
        Self {
            account_number,
            name,
            current_time,
            current_balance,
            current_equity,
            leverage,
            currency,
            free_margin,
            current_profit,
        }
    }
}
impl Account {
    pub(crate) fn get_balance() -> f32 {
        todo!()
    }
    pub fn parse_mt5_response(data: &str) -> Account{
        let data = parse::sanitize_mt5_response(&data);

        let account = match serde_json::from_str(&data) {
            Ok(account) => account,
            Err(e) => {
                println!("Unable to parse Account Data collected from MT5:\n
                       String Received: {}, \n Error: {}", &data, e);
                println!("Sharing default Account information for pseudo use.");
                let account = Account::default();
                account
            }
        };
        account

    }
}
