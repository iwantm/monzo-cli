use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
    pub accounts: Vec<AccountInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfo {
    pub id: String,
    closed: bool,
    created: String,
    description: String,
    r#type: String,
    currency: String,
    country_code: String,
    owners: Vec<Owners>,
    account_number: Option<String>,
    sort_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owners {
    user_id: String,
    preferred_name: String,
    preferred_first_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    balance: i32,
    total_balance: i32,
    currency: String,
    spend_today: i32,
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Balance: £{}, Total balance with pots: £{}, Total spent today: £{}",
            f64::from(self.balance as f64 / 100 as f64),
            f64::from(self.total_balance as f64 / 100 as f64),
            f64::from(self.spend_today as f64 / 100 as f64)
        )
    }
}

impl fmt::Display for AccountInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "You: {}, Account Number: {}, Sort Code: {}",
            self.owners.first().as_deref().unwrap(),
            self.account_number.as_deref().unwrap(),
            self.sort_code.as_deref().unwrap()
        )
    }
}

impl fmt::Display for Owners {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.preferred_name)
    }
}
