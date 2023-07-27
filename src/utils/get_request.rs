pub async fn get_request(
    auth_token: &str,
    url_suffix: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("https://api.monzo.com/{url_suffix}"))
        .bearer_auth(auth_token)
        .send()
        .await;

    resp
}
