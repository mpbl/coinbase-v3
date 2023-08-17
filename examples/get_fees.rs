use coinbase_v3::{basic_oauth::OAuthCbClient, client::CbClient, utils};

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let (client_id, client_secret, redirect_url) = utils::get_env_variables();
    let oauth_cb_client = OAuthCbClient::new(&client_id, &client_secret, &redirect_url)
        .add_scope("wallet:transactions:read")
        .authorize_once()
        .await;

    let cb_client = CbClient::new(&oauth_cb_client);
    run_get_transactions_summary(&cb_client).await;

    oauth_cb_client.revoke_access().await;
}

pub async fn run_get_transactions_summary(cb_client: &CbClient<'_>) {
    let start_date = None;
    let end_date = None;
    let user_native_currency = None;
    let product_type = None;
    let contract_expiry_type = None;

    let transactions_summary = cb_client
        .get_transactions_summary(
            start_date,
            end_date,
            user_native_currency,
            product_type,
            contract_expiry_type,
        )
        .await
        .unwrap();

    println!("{:#?}", transactions_summary);
}
