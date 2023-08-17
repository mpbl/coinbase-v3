//! Utility functions

use dotenvy::dotenv;
use std::env;

/// Get client_id, client_secret and redirect_url from environment variables
///
/// Only used in examples and provided for conveninence to access the ENV variables stored in ${HOME}/.env
/// - `CB_OAUTH_CLIENT_ID`
/// - `CB_OAUTH_CLIENT_SECRET`
/// - `CB_OAUTH_REDIRECT_URL`
pub fn get_env_variables() -> (String, String, String) {
    dotenv().expect(".env file not found");

    let client_id = env::var("CB_OAUTH_CLIENT_ID")
        .expect("Missing the CB_OAUTH_CLIENT_ID environment variable.");
    let client_secret = env::var("CB_OAUTH_CLIENT_SECRET")
        .expect("Missing the CB_OAUTH_CLIENT_SECRET environment variable.");
    let redirect_url = env::var("CB_OAUTH_REDIRECT_URL")
        .expect("Missing the CB_OAUTH_REDIRECT_URL environment variable");

    (client_id, client_secret, redirect_url)
}
