#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]
#![deny(private_in_public, unstable_features)]
#![warn(rust_2018_idioms, future_incompatible, nonstandard_style)]

//! # connect-1password
//!
//! connect-1password is a Rust SDK for 1Password Connect.
//!
//! ## Features
//!
//! - Based on tokio, hyper and hyper_rustls

pub mod client;
pub mod error;
pub mod items;
pub mod models;
pub mod vaults;

#[cfg(test)]
fn get_test_client() -> client::Client {
    client::Client::default()
}
