use crate::client::Client;
use crate::connect::Connect;
use crate::vault::Vault;

#[tokio::test]
async fn get_vaults() {
    let connect = Connect::new();

    let (vaults, _) = connect.vault().get_vaults().await.unwrap();
    dbg!(&vaults);

    assert_eq!(vaults[0].name, "Automated".to_string());
}
