//! Models

use crate::{
    client::Client,
    error::{ConnectAPIError, Error},
};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    pub async fn get_list(
        &self,
    ) -> Result<(Vec<VaultData>, serde_json::Value), crate::error::Error> {
        let params = vec![("", "")];

        let result = match self
            .client
            .send_request::<Vec<VaultData>>(crate::client::GET, "v1/vaults", &params, None)
            .await
        {
            Ok(value) => value,
            Err(err) => {
                let op_error = crate::error::process_connect_error_response(err.to_string())?;

                let message = "Invalid bearer token";
                if err.to_string().contains(message) {
                    let status = VaultStatus {
                        status: op_error.status_code.unwrap_or_default(),
                    };

                    return Err(Error::new_connect_error(ConnectAPIError::new(
                        status.into(),
                        message.to_string(),
                    )));
                }

                return Err(Error::new_internal_error().with(err));
            }
        };

        Ok(result)
    }

    pub async fn get_details(
        &self,
        id: &str,
    ) -> Result<(VaultData, serde_json::Value), crate::error::Error> {
        let params = vec![("", "")];
        let path = format!("v1/vaults/{}", id);

        let result = match self
            .client
            .send_request::<VaultData>(crate::client::GET, &path, &params, None)
            .await
        {
            Ok(value) => value,
            Err(err) => {
                let op_error = crate::error::process_connect_error_response(err.to_string())?;

                let mut message = "Invalid bearer token";
                if err.to_string().contains(message) {
                    let status = VaultStatus {
                        status: op_error.status_code.unwrap_or_default(),
                    };

                    return Err(Error::new_connect_error(ConnectAPIError::new(
                        status.into(),
                        message.to_string(),
                    )));
                }

                message = "Invalid Vault UUID";
                if err.to_string().contains(message) {
                    let status = VaultStatus {
                        status: op_error.status_code.unwrap_or_default(),
                    };

                    return Err(Error::new_connect_error(ConnectAPIError::new(
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct ItemData {
    /// The UUID of the item.
    pub id: String,
    /// The title of the item.
    pub title: String,
    /// An object containing an id property whose value is the UUID of the vault the item is in.
    pub vault: VaultID,
    /// The category of the item.
    pub category: Option<String>,
    /// Vector of URL objects containing URLs for the item.
    pub urls: Option<Vec<UrlObject>>,
    /// Whether the item is marked as a favourite.
    pub favorite: Option<bool>,
    /// A vector of strings of the tags assigned to the item.
    pub tags: Option<Vec<String>>,
    /// The state of the item.
    pub state: Option<String>,
    /// Date and time when the vault was created.
    pub created_at: Option<DateTime<Utc>>,
    /// Date and time when the vault or its contents were last changed.
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct VaultID {
    /// The UUID of the vault.
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct UrlObject {
    /// The address.
    pub url: String,
    /// Whether this is the primary URL for the item.
    pub primary: bool,
}

pub struct Item {
    client: Client,
}

impl Item {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_list(
        &self,
        id: &str,
    ) -> Result<(Vec<ItemData>, serde_json::Value), crate::error::Error> {
        let params = vec![("", "")];
        let path = format!("v1/vaults/{}/items", id);

        let result = match self
            .client
            .send_request::<Vec<ItemData>>(crate::client::GET, &path, &params, None)
            .await
        {
            Ok(value) => value,
            Err(err) => {
                let op_error = crate::error::process_connect_error_response(err.to_string())?;

                let message = "Invalid bearer token";
                if err.to_string().contains(message) {
                    let status = VaultStatus {
                        status: op_error.status_code.unwrap_or_default(),
                    };

                    return Err(Error::new_connect_error(ConnectAPIError::new(
                        status.into(),
                        message.to_string(),
                    )));
                }

                return Err(Error::new_internal_error().with(err));
            }
        };

        Ok(result)
    }

    pub async fn add(
        &self,
        id: &str,
        item: FullItem,
    ) -> Result<(ItemData, serde_json::Value), crate::error::Error> {
        let params = vec![("", "")];
        let path = format!("v1/vaults/{}/items", id);

        let body = Some(serde_json::to_string(&item)?);
        let result = match self
            .client
            .send_request::<ItemData>(crate::client::POST, &path, &params, body)
            .await
        {
            Ok(value) => value,
            Err(err) => {
                let op_error = crate::error::process_connect_error_response(err.to_string())?;

                let message = "Invalid bearer token";
                if err.to_string().contains(message) {
                    let status = VaultStatus {
                        status: op_error.status_code.unwrap_or_default(),
                    };

                    return Err(Error::new_connect_error(ConnectAPIError::new(
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

#[derive(Debug, Serialize, Clone)]
pub struct ItemField {
    /// An object containing the UUID of a section in the item.
    pub section: Option<SectionID>,
    /// Use `purpose` for the username, password, and notes fields.
    pub purpose: Option<String>,
    /// Use `type' for all other fields
    pub r#type: Option<String>,
    /// The value to save for the field. You can specify a `generate` field instead of `value` to create a password or other random information for the value.
    pub value: Option<String>,
    /// Generate a password and save in the value for the field. By default, the password is a 32-characters long, made up of letters, numbers, and symbols. To customize the password, include a `recipe` field.
    pub generate: Option<bool>,
    // FIXME the GeneratorRecipe needs to be added
    // pub recipe
    /// Some optional text
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SectionObject {
    /// The UUID of the section.
    pub id: String,
    /// Some optional text
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SectionID {
    /// The UUID of the section.
    pub id: String,
}

impl SectionID {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }
}

pub struct ItemSection {}

#[derive(Debug, Serialize)]
pub struct FullItem {
    /// The title of the item.
    pub title: Option<String>,
    /// An object containing an id property whose value is the UUID of the vault the item is in.
    pub vault: VaultID,
    /// The category of the item.
    pub category: Option<String>,
    /// Vector of URL objects containing URLs for the item.
    pub urls: Option<Vec<UrlObject>>,
    /// Whether the item is marked as a favourite.
    pub favorite: bool,
    /// A vector of strings of the tags assigned to the item.
    pub tags: Option<Vec<String>>,
    /// A vector of Field objects of the fields to include with the item.
    pub fields: Vec<ItemField>,
    /// A vector of Section objects of the sections to include with the item.
    pub sections: Vec<SectionObject>,
}

pub trait LoginItem {
    fn username(&mut self, username: &str) -> &Self;
    fn password(&mut self, password: &str) -> &Self;
}

#[derive(Debug)]
pub struct LoginItemBuilder {
    /// The title of the item.
    pub title: Option<String>,
    /// An object containing an id property whose value is the UUID of the vault the item is in.
    pub vault: VaultID,
    /// The category of the item.
    pub category: Option<String>,
    /// Vector of URL objects containing URLs for the item.
    pub urls: Option<Vec<UrlObject>>,
    /// Whether the item is marked as a favourite.
    pub favorite: bool,
    /// A vector of strings of the tags assigned to the item.
    pub tags: Option<Vec<String>>,
    /// A vector of Field objects of the fields to include with the item.
    pub fields: Vec<ItemField>,
    /// A vector of Section objects of the sections to include with the item.
    pub sections: Vec<SectionObject>,
}

impl LoginItemBuilder {
    pub fn new(vault_id: &str) -> Self {
        let vault = VaultID {
            id: vault_id.to_string(),
        };

        Self {
            vault,
            title: None,
            category: Some("LOGIN".to_string()),
            favorite: false,
            urls: None,
            tags: None,
            fields: vec![],
            sections: vec![],
        }
    }

    pub fn build(&self) -> FullItem {
        FullItem {
            category: self.category.clone(),
            favorite: self.favorite,
            fields: self.fields.clone(),
            sections: self.sections.clone(),
            tags: self.tags.clone(),
            title: self.title.clone(),
            urls: self.urls.clone(),
            vault: self.vault.clone(),
        }
    }
}

impl LoginItem for LoginItemBuilder {
    fn username(&mut self, username: &str) -> &Self {
        let field: ItemField = ItemField {
            value: Some(username.to_string()),
            purpose: Some("USERNAME".to_string()),
            generate: None,
            label: None,
            r#type: None,
            section: None,
        };

        self.fields.push(field);
        self
    }
    fn password(&mut self, password: &str) -> &Self {
        let field: ItemField = ItemField {
            value: if password.is_empty() {
                None
            } else {
                Some(password.to_string())
            },
            purpose: Some("PASSWORD".to_string()),
            generate: if password.is_empty() {
                Some(true)
            } else {
                None
            },
            label: None,
            r#type: None,
            section: None,
        };

        self.fields.push(field);
        self
    }
}
