#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]
#![deny(private_in_public, unstable_features)]
#![warn(rust_2018_idioms, future_incompatible, nonstandard_style)]

//! connect-1password is a Rust SDK for 1Password Connect.
//!
//! # High-level features
//!
//! - Based on [`tokio`], [`hyper`] and [`hyper_rustls`] by default.
//! - [`hyper`] can be replaced using the `HTTPClient` interface.
//!
//! # Examples
//!
//! ```
//! use connect_1password::{
//!     error::Error,
//!     client::{Client, HTTPClient},
//!     models::{
//!         item::{LoginItem, FullItem, ItemBuilder, ItemCategory},
//!     },
//!     vaults,
//!     items,
//! };
//!
//! const SLEEP_DELAY: u64 = 4; // seconds
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::default();
//!
//!     let (vaults, _) = vaults::all(client).await?;
//!     assert!(!vaults.is_empty());
//!
//!     let item: FullItem = ItemBuilder::new(&vaults[0].id, ItemCategory::Login)
//!         .title("Secure server login")
//!         .username(&"Bob".to_string())
//!         .password(&"".to_string())
//!         .build()
//!         .unwrap();
//!
//!     let client = Client::default();
//!     let (new_item, _) = items::add(client, item).await?;
//!     assert_eq!(new_item.title, "Secure server login");
//!
//!     // Just as a clean up measure, we remove the item created in the this example
//!     tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;
//!
//!     let client = Client::default();
//!     items::remove(client, &vaults[0].id, &new_item.id)
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod items;
pub mod models;
pub mod vaults;

#[cfg(test)]
fn get_test_client() -> client::Client {
    client::Client::default()
}
