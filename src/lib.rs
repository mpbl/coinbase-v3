//! This crates implements Rust bindings for Coinbase Advanced Trade API (v3).
//!
//! This library mostly provides data bindings for the json responses and
//! the GET/POST functions defined in the
//! [API Reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference).
//! Coinbase's Advanced Trade API description can be found
//! [there](https://docs.cloud.coinbase.com/advanced-trade-api/docs/welcome).
//!
//! In addition this crates provides:
//!   - A client based on [reqwest](https://docs.rs/reqwest/latest/reqwest/)
//!   expecting a Oauth2 token provider.
//!   - A basic OAuth2 token provider based on [oauth2](https://docs.rs/oauth2/4.4.1/oauth2/).
//!
//! Notes:
//!   - The OAuth2 token provider is basic and it may be replaced
//!   by a fancier one implementing the [`basic_oauth::AccessTokenProvider`] trait.
//!   - In particular, it is not taking care of the Access Token by periodically
//!   sending the Refresh Token to the server.
//!
//!  ## Warning
//!
//! Use these bindings at your own risk. You may want to review the source code
//! before.
//!
//! # Usage
//!
//! Most if not all API calls have examples attached to them. They can be found
//! on the [github repo](https://github.com/mpbl/coinbase-v3/tree/main/examples),
//! following more or less the structure of Coinbase's Advanced API documentation
//! (accounts, products, orders, fees), splitted between GET and POST request to
//! avoid creating orders you may regret.
//!
//! ## Example: Getting a single product
//!
//! ```no_run
//! # use coinbase_v3::{basic_oauth, client, utils};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Set up the Oauth2 Client -- can be a different one
//!     let (client_id, client_secret, redirect_url) = utils::get_env_variables();
//!     let oauth_cb_client = basic_oauth::OAuthCbClient::new(&client_id, &client_secret, &redirect_url)
//!         .add_scope("wallet:user:read")
//!         .authorize_once()
//!         .await;
//!
//!     // Create the client
//!     let cb_client = client::CbClient::new(&oauth_cb_client);
//!
//!     // Make the request for the product of interest
//!     let product_id = "OGN-BTC";
//!     let product = cb_client.get_product(product_id).await.unwrap();
//!
//!     // Use the result in some fashion
//!     //....
//!
//!     // You may want to revoke the token access for increased security
//!     // by default it should have a lifetime of 2 hours.
//!     oauth_cb_client.revoke_access().await;
//! }
//!```
//!
//! ## Example: Getting a list of accounts
//!
//! Some API calls allow for pagination, for instance when listing accounts.
//!
//! ```no_run
//! use coinbase_v3::{accounts, basic_oauth, client, utils};
//! pub(crate) use futures::{pin_mut, stream::StreamExt};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Same as above
//!     let (client_id, client_secret, redirect_url) = utils::get_env_variables();
//!     let oauth_cb_client = basic_oauth::OAuthCbClient::new(&client_id, &client_secret, &redirect_url)
//!         .add_scope("wallet:accounts:read")
//!         .authorize_once()
//!         .await;
//!     let cb_client = client::CbClient::new(&oauth_cb_client);
//!
//!     // Request to list accounts
//!     let limit = Some(4); // only 4 accounts at a time to better demonstrate pagination
//!     let accounts_stream = cb_client.list_accounts(limit, None);
//!     pin_mut!(accounts_stream);
//!
//!     // Here we store all accounts in a Vector, but it is not mandatory.
//!     let mut accounts = Vec::<accounts::Account>::new();
//!
//!     // Next step: iterate, 4 accounts at a time until there are no more.
//!     while let Some(account_result) = accounts_stream.next().await {
//!         let mut partial_accounts = account_result.unwrap();
//!         println!("Got {} accounts", partial_accounts.len()); // Should give you 4 or less
//!         // store the retrieved accounts, or do whatever.
//!         accounts.append(&mut partial_accounts);
//!     }
//!     
//!     println!("Got {} accounts in total.", accounts.len());
//!
//!     // Same
//!     oauth_cb_client.revoke_access().await;
//! }

// ================ Libary modules ============================================
pub mod accounts;
pub mod basic_oauth;
pub mod client;
pub mod error;
pub mod fees;
pub mod orders;
pub mod products;
pub mod scopes;
pub mod utils;

// ================ Libary wide variables =====================================
/// Base URL for Coinbase's v3 API.
pub const MAIN_URL: &str = "https://api.coinbase.com/api/v3";

/// Ensure DateTime is exclusively referring to UTC.
pub type DateTime = chrono::DateTime<chrono::Utc>;
