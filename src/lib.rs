#![forbid(unsafe_code)]
#![deny(unreachable_pub, private_in_public, unstable_features)]
#![warn(rust_2018_idioms, future_incompatible, nonstandard_style)]

pub mod client;
pub mod connect;
pub mod error;
pub mod vault;

#[cfg(test)]
mod tests;
