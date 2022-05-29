//! Models

pub(crate) mod item;
mod vault;

pub use item::*;
pub use vault::*;

/// This is a wrapper to assist creating instances of `ConnectAPIError`
#[derive(Debug)]
pub struct StatusWrapper {
    pub(crate) status: u16,
}

impl Into<String> for StatusWrapper {
    fn into(self) -> String {
        self.status.to_string()
    }
}
