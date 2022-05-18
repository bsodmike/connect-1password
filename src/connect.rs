//! Connect API

use crate::{client::Client, vault::Vault};
use dotenv::dotenv;

pub struct Connect {
    server_url: String,
    token: String,
    client: Client,
    vault: Vault,
}

impl Connect {
    pub fn new() -> Self {
        let token = std::env::var("OP_API_TOKEN").expect("1Password API token expected!");
        let host = std::env::var("OP_SERVER_URL").expect("1Password Connect server URL expected!");

        // .env to override settings in ENV
        dotenv().ok();
        let client = Client::new(&token, &host);

        Self {
            server_url: host.clone(),
            token: token.clone(),
            client,
            vault: Vault::new(Client::new(&token, &host)),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn vault(&self) -> &Vault {
        &self.vault
    }
}
