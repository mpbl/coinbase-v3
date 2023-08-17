//! OAuth2 related functionalities

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::BasicClient, revocation::StandardRevocableToken, AccessToken, AuthUrl,
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RefreshToken, RevocationUrl,
    Scope, TokenResponse, TokenUrl,
};
use url::Url;

use crate::scopes::VALID_SCOPES;

const AUTH_URL_STR: &str = "https://www.coinbase.com/oauth/authorize";
const TOKEN_URL_STR: &str = "https://www.coinbase.com/oauth/token";
const REVOKE_URL_STR: &str = "https://api.coinbase.com/oauth/revoke";

/// Trait to implement for any class proviging authentication functionalities to the client.
///
/// For instance:
/// ```no_run
/// # use coinbase_v3::basic_oauth;
/// # use coinbase_v3::client;
/// # let client_with_access_token_provider_trait = basic_oauth::OAuthCbClient::new("", "", "");
/// let cb_client = client::CbClient::new(&client_with_access_token_provider_trait);
/// ```
/// the `oauth_cb_client` should implement this trait.
pub trait AccessTokenProvider {
    /// Should return a valid [`oauth2::AccessToken()`](https://docs.rs/oauth2/latest/oauth2/struct.AccessToken.html).
    fn access_token(&self) -> AccessToken;
}

/// Returning the access token stored by the OAuthCbClient.
///
/// Note that the token might be expired and invalid.
impl AccessTokenProvider for OAuthCbClient {
    fn access_token(&self) -> AccessToken {
        self.access_token.clone().unwrap()
    }
}

fn set_oauth_cb_urls() -> (AuthUrl, TokenUrl, RevocationUrl) {
    let auth_url =
        AuthUrl::new(AUTH_URL_STR.to_string()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(TOKEN_URL_STR.to_string()).expect("Invalid token endpoint URL");
    let revoke_url =
        RevocationUrl::new(REVOKE_URL_STR.to_string()).expect("Invalid revocation endpoint URL");

    (auth_url, token_url, revoke_url)
}

/// A simple client to manage OAuth2 access tokens and permissions
pub struct OAuthCbClient {
    client: BasicClient,
    access_token: Option<AccessToken>,
    refresh_token: Option<RefreshToken>,
    scopes: HashSet<Scope>,
}

impl OAuthCbClient {
    /// Instantiate a new OAuthCbClient
    ///
    /// ```no_run
    /// # use coinbase_v3::basic_oauth::OAuthCbClient;
    /// let client_id = "my_secret_client_id_provided_by_coinbase";
    /// let client_secret = "my_client_secret_provided_by_coinbase";
    /// let redirect_url = "http://localhost:3001";
    /// let oauth_cb_client = OAuthCbClient::new(client_id, client_secret, redirect_url);
    /// ```
    ///
    /// - `client_id` and `client secret` are given to you by the API service provider. Store them
    /// in a safe place. For instance hardcodding them in the source code is a bad idea.
    /// - `redirect_url` is the url you will be asked to access to authenticate. Make sure it is
    /// accessible to you.
    pub fn new(client_id: &str, client_secret: &str, redirect_url: &str) -> Self {
        let client_id = ClientId::new(client_id.to_string());
        let client_secret = ClientSecret::new(client_secret.to_string());
        let redirect_url =
            RedirectUrl::new(redirect_url.to_string()).expect("Invalid redirect_url");

        let (auth_url, token_url, revoke_url) = set_oauth_cb_urls();

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url)
            .set_revocation_uri(revoke_url);

        Self {
            client,
            access_token: None,
            refresh_token: None,
            scopes: HashSet::new(),
        }
    }

    /// AccessToken are only valid for predifnied scopes.
    ///
    /// To add one scope, for instance
    /// ```no_run
    /// # use coinbase_v3::basic_oauth::OAuthCbClient;
    /// # let oauth_cb_client = OAuthCbClient::new("", "", "");
    /// oauth_cb_client.add_scope("wallet:transactions:read");
    /// ```
    /// It can be called multiple times to add mutliple scopes.
    /// Refer to Coinbase's documentation for adding the appropriate scopes.
    /// As it can be confusing, you may refer to the examples of the current package
    /// to find out which ones are needed.
    pub fn add_scope(mut self, scope_description: &str) -> Self {
        assert!(VALID_SCOPES.contains(&scope_description));

        self.scopes
            .insert(Scope::new(scope_description.to_string()));

        self
    }

    /// Get Tokens from the issuing authority. Returns once it has stored them. Otherwise crashes.
    ///
    /// ```no_run
    /// # use coinbase_v3::basic_oauth::OAuthCbClient;
    /// # use tokio_test;
    /// # tokio_test::block_on(async {
    /// # let oauth_cb_client = OAuthCbClient::new("", "", "");
    /// oauth_cb_client.add_scope("wallet:transactions:read")
    ///             .authorize_once().await;
    /// # });
    /// ```
    ///
    /// *Once*, because it does not instantiate a mechanism to renew tokens.
    /// So after 2 hours, the tokens will be invalid.
    pub async fn authorize_once(mut self: Self) -> Self {
        let redirect_url = self.client.redirect_url().unwrap();
        let scheme = redirect_url.url().scheme().to_string();
        let host = redirect_url.url().host().unwrap().to_string();
        let port = redirect_url.url().port().unwrap();

        let listener_address = host.to_string() + ":" + &port.to_string();

        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .url();

        println!(
            "\nOpen this URL in your browser:\n{}\n\n",
            authorize_url.to_string()
        );

        let listener = TcpListener::bind(listener_address).unwrap();
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let code;
                let state;
                {
                    let mut reader = BufReader::new(&stream);

                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&(scheme + "://" + &host + redirect_url)).unwrap();

                    let code_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "code"
                        })
                        .unwrap();

                    let (_, value) = code_pair;
                    code = AuthorizationCode::new(value.into_owned());

                    let state_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "state"
                        })
                        .unwrap();

                    let (_, value) = state_pair;
                    state = CsrfToken::new(value.into_owned());
                }

                let message = "Go back to your terminal :)";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes()).unwrap();
                assert!(state.secret() == csrf_state.secret());

                // Exchange the code with a token.
                let token_response = self
                    .client
                    .exchange_code(code)
                    .request_async(async_http_client)
                    .await;

                let token_response = token_response.unwrap();
                if let Some(tok) = token_response.refresh_token() {
                    self.refresh_token = Some(tok.clone());
                }
                self.access_token = Some(token_response.access_token().clone());

                break;
            }
        }
        self
    }

    /// Revoke the obtained token
    ///
    /// Just to make sure no one can use it afterwards.
    /// Note that without calling this function, Coinbase tokens normally expire after 2 hours.
    pub async fn revoke_access(&self) {
        let token_to_revoke: StandardRevocableToken = match self.refresh_token.as_ref() {
            Some(token) => token.into(),
            None => self.access_token.as_ref().unwrap().into(),
        };

        self.client
            .revoke_token(token_to_revoke)
            .unwrap()
            .request_async(async_http_client)
            .await
            .expect("Failed to revoke token");

        println!("=============== ACCESS REVOKED =================");
    }
}

// impl Drop for OauthCbClient {
//     fn drop(&mut self) {
//         tokio::spawn(self.revoke_access());
//         println!("oauth cb client dropped.");
//     }
// }
