//! Vault

use crate::{
    client::Client,
    error::{Error, VaultError},
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use log::debug;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct VaultData {
    /// The UUID of the vault.
    pub id: String,
    /// The name of the vault.
    pub name: String,
    /// The description of the vault.
    pub description: Option<String>,
    /// The version of the vault metadata.
    pub attribute_version: u32,
    /// The version of the vault contents.
    pub content_version: u32,
    /// The type of vault.
    pub r#type: String,
    /// Date and time when the vault was created.
    pub created_at: Option<DateTime<Utc>>,
    /// Date and time when the vault or its contents were last changed.
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct Vault {
    client: Client,
}

impl Vault {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_vaults(
        &self,
    ) -> Result<(Vec<VaultData>, serde_json::Value), crate::error::Error> {
        let params = vec![("", "")];

        let result = match self
            .client
            .send_request::<Vec<VaultData>>(crate::client::GET, "v1/vaults", &params)
            .await
        {
            Ok(value) => value,
            Err(err) => {
                let op_error = crate::error::process_vault_error(err.to_string())?;

                let message = "Invalid bearer token";
                if err.to_string().contains(message) {
                    let status = VaultStatus {
                        status: op_error.status_code.unwrap_or_default(),
                    };

                    return Err(Error::new_vault_error(VaultError::new(
                        status.into(),
                        message.to_string(),
                    )));
                }

                return Err(Error::new_internal_error().with(err));
            }
        };

        Ok(result)
    }

    pub async fn get_vault(
        &self,
        id: &str,
    ) -> Result<(VaultData, serde_json::Value), crate::error::Error> {
        let params = vec![("", "")];
        let path = format!("v1/vaults/{}", id);

        let result = match self
            .client
            .send_request::<VaultData>(crate::client::GET, &path, &params)
            .await
        {
            Ok(value) => value,
            Err(err) => {
                let op_error = crate::error::process_vault_error(err.to_string())?;

                let mut message = "Invalid bearer token";
                if err.to_string().contains(message) {
                    let status = VaultStatus {
                        status: op_error.status_code.unwrap_or_default(),
                    };

                    return Err(Error::new_vault_error(VaultError::new(
                        status.into(),
                        message.to_string(),
                    )));
                }

                message = "Invalid Vault UUID";
                if err.to_string().contains(message) {
                    let status = VaultStatus {
                        status: op_error.status_code.unwrap_or_default(),
                    };

                    return Err(Error::new_vault_error(VaultError::new(
                        status.into(),
                        message.to_string(),
                    )));
                }

                return Err(Error::new_internal_error().with(err));
            }
        };

        Ok(result)
    }
}

struct VaultStatus {
    status: u16,
}

impl Into<StatusCode> for VaultStatus {
    fn into(self) -> StatusCode {
        StatusCode::try_from(self.status).unwrap()
    }
}
