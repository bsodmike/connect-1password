use crate::client::Client;
use crate::connect::Connect;
use crate::error::Error;
use crate::vault::Vault;

// List vaults
// https://developer.1password.com/docs/connect/connect-api-reference#list-vaults

//#[tokio::test]
//async fn get_vaults() {
//    let connect = Connect::new();
//
//    let (vaults, _) = connect.vault().get_vaults().await.unwrap();
//    dbg!(&vaults);
//
//    assert_eq!(vaults[0].name, "Automated".to_string());
//}

// Get vault details
// https://developer.1password.com/docs/connect/connect-api-reference#get-vault-details

#[tokio::test]
async fn get_vault() {
    let test_vault_id =
        std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
    let connect = Connect::new();

    let (vault, _) = connect.vault().get_vault(&test_vault_id).await.unwrap();
    dbg!(&vault);

    assert_eq!(vault.name, "Automated".to_string());
}

#[should_panic]
#[tokio::test]
async fn get_vault_not_specified() {
    let connect = Connect::new();

    let (vault, _) = connect.vault().get_vault("").await.unwrap();

    assert_eq!(vault.name, "Automated".to_string());
}

#[should_panic]
#[tokio::test]
async fn get_vault_invalid_vault() {
    let connect = Connect::new();

    let (_vault, _) = connect.vault().get_vault("foo").await.unwrap();
}
