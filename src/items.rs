use crate::error::{ConnectAPIError, Error};
use crate::{
    client::Client,
    models::{
        item::{FullItem, ItemData, LoginItem, LoginItemBuilder},
        VaultStatus,
    },
    *,
};

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

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn all() {
        let client = get_test_client();
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");

        let (items, _) = items::all(&client, &test_vault_id).await.unwrap();
        dbg!(&items);

        assert!(!items.is_empty());
    }

    #[test]
    async fn add_login_item() {
        let test_vault_id =
            std::env::var("OP_TESTING_VAULT_ID").expect("1Password Vault ID for testing");
        let client = get_test_client();

        let item: FullItem = LoginItemBuilder::new(&test_vault_id)
            .username(&"Bob".to_string())
            .password(&"".to_string())
            .build()
            .unwrap();
        let (new_item, _) = items::add(&client, item).await.unwrap();
        dbg!(&new_item);

        assert_ne!(new_item.id, "foo");
    }
}
