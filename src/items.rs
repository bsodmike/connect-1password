use crate::error::{ConnectAPIError, Error};
use crate::{
    client::Client,
    models::{
        item::{FullItem, ItemBuilder, ItemData, LoginItem},
        VaultStatus,
    },
    *,
};
use serde::{Deserialize, Serialize};

/// Get all items
pub async fn all(
    client: &Client,
    id: &str,
) -> Result<(Vec<ItemData>, serde_json::Value), crate::error::Error> {
    let params = vec![("", "")];
    let path = format!("v1/vaults/{}/items", id);

    let result = match client
        .send_request::<Vec<ItemData>>(crate::client::GET, &path, &params, None)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            let op_error = crate::error::process_connect_error_response(err.to_string())?;

            let message = "Invalid bearer token";
            if err.to_string().contains(message) {
                let status = VaultStatus {
                    status: op_error.status_code.unwrap_or_default(),
                };

                return Err(Error::new_connect_error(ConnectAPIError::new(
                    status.into(),
                    message.to_string(),
                )));
            }

            return Err(Error::new_internal_error().with(err));
        }
    };

    Ok(result)
}

/// Add an item
pub async fn add(
    client: &Client,
    item: FullItem,
) -> Result<(ItemData, serde_json::Value), crate::error::Error> {
    let id = &item.vault.id;

    let params = vec![("", "")];
    let path = format!("v1/vaults/{}/items", id);

    let body = Some(serde_json::to_string(&item)?);
    let result = match client
        .send_request::<ItemData>(crate::client::POST, &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            let op_error = crate::error::process_connect_error_response(err.to_string())?;

            let message = "Invalid bearer token";
            if err.to_string().contains(message) {
                let status = VaultStatus {
                    status: op_error.status_code.unwrap_or_default(),
                };

                return Err(Error::new_connect_error(ConnectAPIError::new(
                    status.into(),
                    message.to_string(),
                )));
            }

            return Err(Error::new_internal_error().with(err));
        }
    };

    Ok(result)
}

#[derive(Debug, Deserialize, PartialEq)]
struct DeleteReturnType {}

/// Delete an item
pub async fn remove(client: &Client, id: &str, item_id: &str) -> Result<(), crate::error::Error> {
    let params = vec![("", "")];
    let path = format!("v1/vaults/{}/items/{}", id, item_id);
    dbg!(&path);

    let body = None;
    let _result = match client
        .send_request::<DeleteReturnType>(crate::client::DELETE, &path, &params, body)
        .await
    {
        Ok(value) => value,
        Err(err) => {
            let op_error = crate::error::process_connect_error_response(err.to_string())?;

            let message = "Invalid bearer token";
            if err.to_string().contains(message) {
                let status = VaultStatus {
                    status: op_error.status_code.unwrap_or_default(),
                };

                return Err(Error::new_connect_error(ConnectAPIError::new(
                    status.into(),
                    message.to_string(),
                )));
            }

            return Err(Error::new_internal_error().with(err));
        }
    };

    Ok(())
}

const SLEEP_DELAY: u64 = 2; // seconds

#[cfg(test)]
mod default {
    use super::SLEEP_DELAY;
    use crate::get_test_client;
    use tokio::test;

    use crate::{
        client::Client,
        items,
        models::{
            item::{DefaultItem, FullItem, ItemBuilder, ItemData},
            VaultStatus,
        },
    };

    #[test]
    async fn all() {
        let client = get_test_client();
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");

        let (items, _) = items::all(&client, &test_vault_id).await.unwrap();
        dbg!(&items);

        assert!(items.is_empty());
    }

    #[test]
    async fn add_item() {
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
        let client = get_test_client();

        let item: FullItem = ItemBuilder::new(&test_vault_id).build().unwrap();
        let (new_item, _) = items::add(&client, item).await.unwrap();
        dbg!(&new_item);

        assert_ne!(new_item.id, "foo");

        tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

        items::remove(&client, &test_vault_id, &new_item.id)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod login_item {
    use super::*;
    use tokio::test;

    #[test]
    async fn add_login_item() {
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
        let client = get_test_client();

        let item: FullItem = ItemBuilder::new(&test_vault_id)
            .title("Test login item")
            .username(&"Bob".to_string())
            .password(&"".to_string())
            .build()
            .unwrap();
        let (new_item, _) = items::add(&client, item).await.unwrap();
        dbg!(&new_item);

        assert_ne!(new_item.id, "foo");

        tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

        items::remove(&client, &test_vault_id, &new_item.id)
            .await
            .unwrap();
    }

    #[test]
    async fn add_login_item_with_otp() {
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
        let client = get_test_client();

        let item: FullItem = ItemBuilder::new(&test_vault_id)
            .title("Test login item")
            .username(&"Bob".to_string())
            .password(&"".to_string())
            .add_otp("replaceme")
            .build()
            .unwrap();
        let (new_item, _) = items::add(&client, item).await.unwrap();
        dbg!(&new_item);

        assert_ne!(new_item.id, "foo");

        tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

        items::remove(&client, &test_vault_id, &new_item.id)
            .await
            .unwrap();
    }

    #[should_panic]
    #[test]
    async fn add_login_item_requires_title() {
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
        let client = get_test_client();

        let item: FullItem = ItemBuilder::new(&test_vault_id)
            .username(&"Bob".to_string())
            .password(&"".to_string())
            .build()
            .unwrap();
        let (_new_item, _) = items::add(&client, item).await.unwrap();
    }

    #[test]
    async fn remove_login_item() {
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
        let client = get_test_client();

        let item: FullItem = ItemBuilder::new(&test_vault_id)
            .title("Test login item, will be removed")
            .username(&"Bob".to_string())
            .password(&"".to_string())
            .build()
            .unwrap();
        let (new_item, _) = items::add(&client, item).await.unwrap();
        dbg!(&new_item);

        tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

        items::remove(&client, &test_vault_id, &new_item.id)
            .await
            .unwrap();

        tokio::time::sleep(std::time::Duration::new(SLEEP_DELAY, 0)).await;

        let (items, _) = items::all(&client, &test_vault_id).await.unwrap();
        assert!(items.is_empty());
    }
}
