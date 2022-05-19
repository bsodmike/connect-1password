use crate::client::Client;
use crate::connect::Connect;
use crate::error::Error;
use crate::models::{FullItem, LoginItem, LoginItemBuilder, Vault};

// List vaults
// https://developer.1password.com/docs/connect/connect-api-reference#list-vaults

#[tokio::test]
async fn list_vaults() {
    let connect = Connect::new();

    let (vaults, _) = connect.vault().get_list().await.unwrap();
    dbg!(&vaults);

    assert_eq!(vaults[0].name, "Automated".to_string());
}

// Get vault details
// https://developer.1password.com/docs/connect/connect-api-reference#get-vault-details

#[tokio::test]
async fn get_vault_details() {
    let test_vault_id =
        std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
    let connect = Connect::new();

    let (vault, _) = connect.vault().get_details(&test_vault_id).await.unwrap();
    dbg!(&vault);

    assert_eq!(vault.name, "Automated".to_string());
}

#[should_panic]
#[tokio::test]
async fn get_vault_details_not_specified() {
    let connect = Connect::new();

    let (vault, _) = connect.vault().get_details("").await.unwrap();

    assert_eq!(vault.name, "Automated".to_string());
}

#[should_panic]
#[tokio::test]
async fn get_vault_details_invalid_vault() {
    let connect = Connect::new();

    let (_vault, _) = connect.vault().get_details("foo").await.unwrap();
}

// List items
// https://developer.1password.com/docs/connect/connect-api-reference#list-items

#[tokio::test]
async fn list_items() {
    let test_vault_id =
        std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
    let connect = Connect::new();

    let (items, _) = connect.item().get_list(&test_vault_id).await.unwrap();
    dbg!(&items);

    assert!(!items.is_empty());
}

// Add an item

// ItemBuilder
#[tokio::test]
async fn login_item_builder() {
    let test_vault_id =
        std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
    let connect = Connect::new();

    let item: FullItem = LoginItemBuilder::new(&test_vault_id)
        .username(&"Bob".to_string())
        .password(&"".to_string())
        .build();
    let (new_item, _) = connect.item().add(item).await.unwrap();
    dbg!(&new_item);

    assert_ne!(new_item.id, "foo");
}
