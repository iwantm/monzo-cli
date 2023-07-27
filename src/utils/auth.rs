use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use oauth2::basic::BasicClient;

use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse,
    TokenUrl,
};
use reqwest::Url;

use crate::types::auth::AuthCredentials;

pub async fn auth_with_monzo() -> Result<AuthCredentials, &'static str> {
    let client = BasicClient::new(
        ClientId::new(env::var("MONZO_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(env::var("MONZO_SECRET").unwrap())),
        AuthUrl::new("https://auth.monzo.com".to_string()).unwrap(),
        Some(TokenUrl::new("https://api.monzo.com/oauth2/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string()).expect("Redirect"));

    let (auth_url, _csrf_state) = client.authorize_url(CsrfToken::new_random).url();

    println!("Browse to: {}", auth_url);

    let listener = TcpListener::bind("localhost:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());
            }
            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            let token = client
                .set_auth_type(oauth2::AuthType::RequestBody)
                .exchange_code(code)
                .request_async(async_http_client)
                .await;

            match token {
                Ok(token) => {
                    return Ok(AuthCredentials {
                        access_token: token.access_token().secret().into(),
                    })
                }
                Err(_) => {
                    return Err("No credentials");
                }
            };
        }
    }
    panic!()
}
