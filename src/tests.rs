use crate::client::Client;
use crate::connect::Connect;
use crate::vault::Vault;

#[tokio::test]
async fn get_vaults_client() {
    let api_key = std::env::var("OP_API_TOKEN").expect("1Password API token expected!");
    let host = "http://localhost:8080";
    let client = Client::new(&api_key, host);

    let params = vec![("", "")];
    let req = client.send_request::<Vec<crate::vault::VaultData>>(
        crate::client::GET,
        "v1/vaults",
        &params,
    );

    let (response, _) = req.await.unwrap();
    dbg!(&response);

    assert_eq!(response[0].name, "Automated".to_string());
}

#[tokio::test]
async fn get_vaults() {
    let connect = Connect::new();
    let client = connect.client();

    let (response, _) = connect.vault().get_vaults().await.unwrap();
    dbg!(&response);

    assert_eq!(response[0].name, "Automated".to_string());
}
