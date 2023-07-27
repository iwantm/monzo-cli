use crate::types::monzo::Balance;
use crate::utils::get_request::get_request;

pub async fn get_balance(
    auth_token: String,
    account_id: String,
) -> Result<Balance, reqwest::Error> {
    let balance = get_request(&auth_token, &format!("/balance?account_id={account_id}"))
        .await
        .unwrap()
        .json::<Balance>()
        .await;

    balance
}
