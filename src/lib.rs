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

pub mod client;
pub mod connect;
pub mod error;
pub mod models;

#[cfg(test)]
mod tests;
