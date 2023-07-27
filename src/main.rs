mod endpoints;
mod types;
mod utils;

use crate::{
    endpoints::{account::account_info, balance::get_balance},
    utils::auth::auth_with_monzo,
};
use clap::{Parser, Subcommand};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::env;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Returns account information
    AccountInfo,
    /// Returns account ballance information
    Balance,
}

#[tokio::main]
async fn main() {
    let mc = new_magic_crypt!("magickey", 256);

    if fs::read_to_string("access_key.txt").is_err() {
        let token = auth_with_monzo().await.unwrap();
        fs::write(
            "access_key.txt",
            mc.encrypt_str_to_base64(token.access_token),
        )
        .expect("Unable to write file");
    } else {
        let decrypted_access_key = mc
            .decrypt_base64_to_string(fs::read_to_string("access_key.txt").unwrap())
            .unwrap();
        env::set_var("ACCESS_TOKEN", decrypted_access_key)
    }

    if env::var("ACCOUNT_ID").is_err() {
        let account = account_info(env::var("ACCESS_TOKEN").unwrap())
            .await
            .unwrap();

        let account_id = &account.accounts.first().unwrap().id;
        env::set_var("ACCOUNT_ID", account_id);
    }

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::AccountInfo) => {
            let account = account_info(env::var("ACCESS_TOKEN").unwrap())
                .await
                .unwrap();

            let account_info = account.accounts.first().unwrap();

            println!("{}", account_info);
        }
        Some(Commands::Balance) => {
            let balance = get_balance(
                env::var("ACCESS_TOKEN").unwrap(),
                env::var("ACCOUNT_ID").unwrap(),
            )
            .await
            .unwrap();

            println!("{}", balance);
        }
        None => {}
    }
}
