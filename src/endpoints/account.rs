use crate::types::monzo::Accounts;
use crate::utils::get_request::get_request;

pub async fn account_info(auth_token: String) -> Result<Accounts, reqwest::Error> {
    return get_request(&auth_token, "accounts?account_type=uk_retail")
        .await
        .unwrap()
        .json::<Accounts>()
        .await;
}
