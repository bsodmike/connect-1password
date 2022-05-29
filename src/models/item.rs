use std::ascii::AsciiExt;

use crate::error::{CustomError, Error};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Clone)]
pub struct FieldObject {
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

impl SectionObject {
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: Some(label.to_string()),
        }
    }
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
    pub title: String,
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
    pub fields: Vec<FieldObject>,
    /// A vector of Section objects of the sections to include with the item.
    pub sections: Vec<SectionObject>,
}

pub trait DefaultItem {
    fn build(&self) -> Result<FullItem, Box<dyn std::error::Error + Send + Sync>>;
}

pub trait LoginItem {
    fn title(self, username: &str) -> Self;
    fn username(self, username: &str) -> Self;
    fn password(self, password: &str) -> Self;
    fn build(&self) -> Result<FullItem, Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Debug)]
pub struct ItemBuilder {
    /// The title of the item.
    pub title: String,
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
    pub fields: Vec<FieldObject>,
    /// A vector of Section objects of the sections to include with the item.
    pub sections: Vec<SectionObject>,
}

#[derive(Debug)]
pub enum ItemCategory {
    ApiCredential,
    Login,
    Password,
}

impl ItemCategory {
    fn default() -> Self {
        Self::ApiCredential
    }
}

impl Into<String> for ItemCategory {
    fn into(self) -> String {
        let value = match self {
            ItemCategory::ApiCredential => "API_CREDENTIAL",
            ItemCategory::Login => "LOGIN",
            ItemCategory::Password => "PASSWORD",
        };

        value.to_string()
    }
}

impl ItemBuilder {
    pub fn new(vault_id: &str, category: ItemCategory) -> Self {
        let vault = VaultID {
            id: vault_id.to_string(),
        };

        Self {
            vault,
            title: String::default(),
            category: Some(category.into()),
            favorite: false,
            urls: None,
            tags: None,
            fields: vec![],
            sections: vec![],
        }
    }

    // FIXME: This needs testing to ensure the OTP secret is applied correctly
    pub(crate) fn add_otp(mut self, secret: &str) -> Self {
        let section = SectionID::new();
        let section_obj = SectionObject::new(&section.id, "OTP");

        self.sections.push(section_obj);

        let field_object = FieldObject {
            section: Some(section),
            label: None,
            purpose: None,
            r#type: Some("OTP".to_string()),
            generate: Some(true),
            value: Some(secret.to_string()),
        };
        self.fields.push(field_object);

        self
    }
}

impl DefaultItem for ItemBuilder {
    fn build(&self) -> Result<FullItem, Box<dyn std::error::Error + Send + Sync>> {
        Ok(FullItem {
            title: self.title.clone(),
            category: self.category.clone(),
            favorite: self.favorite,
            fields: self.fields.clone(),
            sections: self.sections.clone(),
            tags: self.tags.clone(),
            urls: self.urls.clone(),
            vault: self.vault.clone(),
        })
    }
}

impl LoginItem for ItemBuilder {
    fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    fn username(mut self, username: &str) -> Self {
        let field: FieldObject = FieldObject {
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

    fn password(mut self, password: &str) -> Self {
        let field: FieldObject = FieldObject {
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

    fn build(&self) -> Result<FullItem, Box<dyn std::error::Error + Send + Sync>> {
        if self.title.is_empty() {
            return Err(Box::new(CustomError::new("Title is required")));
        }

        Ok(FullItem {
            title: self.title.clone(),
            category: self.category.clone(),
            favorite: self.favorite,
            fields: self.fields.clone(),
            sections: self.sections.clone(),
            tags: self.tags.clone(),
            urls: self.urls.clone(),
            vault: self.vault.clone(),
        })
    }
}
