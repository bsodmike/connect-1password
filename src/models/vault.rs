use crate::{client::Client, error::Error};
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

pub struct VaultStatus {
    pub(crate) status: u16,
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
