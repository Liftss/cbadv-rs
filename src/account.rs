//! # Coinbase Advanced Account API
//!
//! `account` gives access to the Account API and the various endpoints associated with it.
//! This allows you to obtain account information either by account UUID or in bulk (all accounts).

use crate::utils::{CBAdvError, Result, Signer};
use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};

/// Represents a Balance for either Available or Held funds.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub value: String,
    pub currency: String,
}

/// Represents an Account received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub uuid: String,
    pub name: String,
    pub currency: String,
    pub available_balance: Balance,
    pub default: bool,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub r#type: String,
    pub ready: bool,
    pub hold: Balance,
}

/// Represents a list of accounts received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListedAccounts {
    pub accounts: Vec<Account>,
    pub has_next: bool,
    pub cursor: String,
    pub size: i32,
}

/// Represents an account response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct AccountResponse {
    pub account: Account,
}

/// Represents parameters that are optional for List Account API request.
#[allow(dead_code)]
#[derive(Serialize, Default, Debug)]
pub struct ListAccountsParams {
    /// Amount to obtain, default 49 maximum is 250.
    pub limit: Option<i32>,
    /// Returns accounts after the cursor provided.
    pub cursor: Option<String>,
}

impl ListAccountsParams {
    /// Converts the object into HTTP request parameters.
    pub fn to_params(&self) -> String {
        let mut params: String = "".to_string();

        params = match &self.limit {
            Some(v) => format!("{}&limit={}", params, v),
            _ => params,
        };

        params = match &self.cursor {
            Some(v) => format!("{}&cursor={}", params, v),
            _ => params,
        };

        match params.is_empty() {
            true => params,
            false => params[1..].to_string(),
        }
    }
}

/// Provides access to the Account API for the service.
pub struct AccountAPI {
    signer: Signer,
}

impl AccountAPI {
    /// Resource for the API.
    const RESOURCE: &str = "/api/v3/brokerage/accounts";

    /// Creates a new instance of the Account API. This grants access to account information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Obtains a single account based on the Account UUID (ex. "XXXX-YYYY-ZZZZ"). This is the most
    /// efficient way to get a single account, however it requires the user to know the UUID.
    ///
    /// # Arguments
    ///
    /// * `account_uuid` - A string the represents the account's UUID.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/accounts/{account_uuid}
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccount>
    pub async fn get(&self, account_uuid: &str) -> Result<Account> {
        let resource = format!("{}/{}", Self::RESOURCE, account_uuid);
        match self.signer.get(&resource, "").await {
            Ok(value) => match value.json::<AccountResponse>().await {
                Ok(resp) => Ok(resp.account),
                Err(_) => Err(CBAdvError::BadParse("account object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains a single account based on the Account ID (ex. "BTC").
    /// This wraps `get_bulk` and recursively makes several additional requests until either the
    /// account is found or there are not more accounts. This is a more expensive call, but more
    /// convient than `get` which requires knowing the UUID already.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests than
    /// normal.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifier for the account, such as BTC or ETH.
    /// * `params` - Optional parameters, should default to None unless you want additional control.
    #[async_recursion]
    pub async fn get_by_id(&self, id: &str, params: Option<ListAccountsParams>) -> Result<Account> {
        let mut params = match params {
            Some(p) => p,
            None => ListAccountsParams::default(),
        };

        match self.get_bulk(&params).await {
            Ok(mut listed) => {
                // Find the index.
                match listed.accounts.iter().position(|r| r.currency == id) {
                    Some(index) => Ok(listed.accounts.swap_remove(index)),
                    None => {
                        // Prevent further requests if no more can be made.
                        if !listed.has_next {
                            return Err(CBAdvError::NotFound("no matching ids".to_string()));
                        }

                        // Make another request to the API for the account.
                        params.cursor = Some(listed.cursor);
                        self.get_by_id(id, Some(params)).await
                    }
                }
            }
            Err(error) => Err(error),
        }
    }

    /// Obtains various accounts from the API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/accounts
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccounts>
    pub async fn get_bulk(&self, params: &ListAccountsParams) -> Result<ListedAccounts> {
        match self.signer.get(Self::RESOURCE, &params.to_params()).await {
            Ok(value) => match value.json::<ListedAccounts>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse("accounts vector".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
