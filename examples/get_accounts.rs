pub(crate) use futures::{pin_mut, stream::StreamExt};

use coinbase_v3::{accounts::Account, basic_oauth::OAuthCbClient, client::CbClient, utils};

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let (client_id, client_secret, redirect_url) = utils::get_env_variables();
    let oauth_cb_client = OAuthCbClient::new(&client_id, &client_secret, &redirect_url)
        .add_scope("wallet:accounts:read")
        .authorize_once()
        .await;

    let cb_client = CbClient::new(&oauth_cb_client);
    run_list_get_accounts(&cb_client).await;

    oauth_cb_client.revoke_access().await;
}

pub async fn run_list_get_accounts(cb_client: &CbClient<'_>) {
    let limit = Some(4);
    let accounts_stream = cb_client.list_accounts(limit, None);
    pin_mut!(accounts_stream);

    let mut accounts = Vec::<Account>::new();
    while let Some(account_result) = accounts_stream.next().await {
        let partial_accounts = account_result.unwrap();
        println!("Got {} accounts", partial_accounts.len());
        for account in partial_accounts {
            accounts.push(account);
        }
    }
    println!("Got {} accounts in total.", accounts.len());

    let account = &accounts[0];
    let acc = cb_client.get_account(account.uuid).await.unwrap();
    assert_eq!(account, &acc);
}
