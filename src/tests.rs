use crate::client::Client;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Vault {
    id: String,
    name: String,
    content_version: u32,
    attribute_version: u32,
    r#type: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[tokio::test]
async fn get_vaults() {
    let api_key = std::env::var("OP_API_TOKEN").expect("1Password API token expected!");
    let host = "localhost:8080";
    let client = Client::new(api_key, host.to_string());

    let params = vec![("", "")];
    let req = client.send_request::<Vec<Vault>>(crate::client::GET, "v1/vaults", &params);

    let (response, _) = req.await.unwrap();
    dbg!(&response);

    assert_eq!(response[0].name, "Automated".to_string());
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
