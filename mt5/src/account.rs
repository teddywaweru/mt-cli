use crate::parse;
use serde_json;
use serde::{Serialize, Deserialize};

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

impl Account {
    pub(crate) fn get_balance() -> f32 {
        todo!()
    }
    pub fn parse_mt5_response(data: &str) -> Result<Account, serde_json::Error> {
        let data = data.replace("'", "\"");
        let data = parse::remove_action(&data);
        let data: Account = serde_json::from_str(&data)?;

        Ok(data)
    }
}
