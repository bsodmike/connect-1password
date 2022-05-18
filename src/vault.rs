use crate::client::Client;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct VaultData {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) content_version: u32,
    pub(super) attribute_version: u32,
    pub(super) r#type: String,
    pub(super) created_at: Option<DateTime<Utc>>,
    pub(super) updated_at: Option<DateTime<Utc>>,
}

pub struct Vault {
    client: Client,
}

impl Vault {
    pub(super) fn new(client: Client) -> Self {
        Self { client }
    }

    pub(super) async fn get_vaults(
        &self,
    ) -> Result<(Vec<VaultData>, serde_json::Value), crate::error::Error> {
        let params = vec![("", "")];

        Ok(self
            .client
            .send_request::<Vec<VaultData>>(crate::client::GET, "v1/vaults", &params)
            .await?)
    }
}
