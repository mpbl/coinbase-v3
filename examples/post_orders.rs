use coinbase_v3::{basic_oauth::OAuthCbClient, client::CbClient, error::CbError, orders, utils};

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let (client_id, client_secret, redirect_url) = utils::get_env_variables();
    let oauth_cb_client = OAuthCbClient::new(&client_id, &client_secret, &redirect_url)
        .add_scope("wallet:buys:create")
        .authorize_once()
        .await;

    let cb_client = CbClient::new(&oauth_cb_client);
    // run_order_and_cancel(&cb_client).await;

    run_cancel_nonexistent_order(&cb_client).await;

    oauth_cb_client.revoke_access().await;
}

pub async fn run_order_and_cancel(cb_client: &CbClient<'_>) {
    let product_id = "BTC-USDT";
    let side = orders::OrderSide::Buy;
    let base_size = 1.0; // let's buy a BTC
    let limit_price = 0.01; // if it's one cent.
    let end_time = chrono::offset::Utc::now() + chrono::Duration::days(1); // and happen within 1 day
    let post_only = false;

    let order_to_send = orders::create_limit_order_good_til_date(
        product_id,
        side,
        base_size,
        limit_price,
        end_time,
        post_only,
    )
    .unwrap();

    let create_order_response = cb_client.create_order(&order_to_send).await.unwrap();
    println!("{:#?}", create_order_response);

    let order_ids = vec![create_order_response.order_id];
    let cancel_response = cb_client.cancel_order(&order_ids).await.unwrap();
    println!("{:#?}", cancel_response);
}

pub async fn run_cancel_nonexistent_order(cb_client: &CbClient<'_>) {
    let order_ids = vec!["foo".to_string()];
    let response = cb_client.cancel_order(&order_ids).await;

    match response {
        Ok(cancel_order_response) => println!("{:#?}", cancel_order_response),
        Err(err) => match err {
            CbError::Coinbase(e) => println!("Coinbase error: {:#?}", e),
            CbError::Serde(e) => println!("Serde error: {:#?}", e),
            CbError::Http(e) => println!("Http error: {:#?}", e),
        },
    }
}
