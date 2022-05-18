use crate::client::Client;
use crate::connect::Connect;
use crate::vault::Vault;

#[tokio::test]
async fn get_vaults() {
    let connect = Connect::new();

    let (response, _) = connect.vault().get_vaults().await.unwrap();
    dbg!(&response);

    assert_eq!(response[0].name, "Automated".to_string());
}
