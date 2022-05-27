#![forbid(unsafe_code)]
#![deny(unreachable_pub, private_in_public, unstable_features)]
#![warn(rust_2018_idioms, future_incompatible, nonstandard_style)]

//! # connect-1password
//!
//! connect-1password is a Rust SDK for 1Password Connect.
//!
//! ## Features
//!
//! - Based on tokio, hyper and hyper_rustls

mod error;

pub mod client;
pub mod models;
pub mod vaults;

#[cfg(test)]
fn get_test_client() -> client::Client {
    client::Client::default()
}
