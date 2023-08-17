use coinbase_v3::{basic_oauth::OAuthCbClient, client::CbClient, utils};

pub mod get_accounts;
pub mod get_fees;
pub mod get_orders;
pub mod get_products;

use get_accounts::run_list_get_accounts;
use get_fees::run_get_transactions_summary;
use get_orders::{run_list_fills, run_list_orders};
use get_products::{
    run_get_bid_ask, run_get_market_trades, run_get_product, run_get_product_book,
    run_get_product_candles, run_list_products,
};

#[tokio::main]
async fn main() {
    let (client_id, client_secret, redirect_url) = utils::get_env_variables();
    let oauth_cb_client = OAuthCbClient::new(&client_id, &client_secret, &redirect_url)
        .add_scope("wallet:accounts:read")
        .add_scope("wallet:transactions:read")
        .add_scope("wallet:user:read")
        .authorize_once()
        .await;

    let cb_client = CbClient::new(&oauth_cb_client);

    run_list_get_accounts(&cb_client).await;

    run_list_orders(&cb_client).await;
    run_list_fills(&cb_client).await;

    run_get_bid_ask(&cb_client).await;
    run_get_product_book(&cb_client).await;
    run_list_products(&cb_client).await;
    run_get_product(&cb_client).await;
    run_get_product_candles(&cb_client).await;
    run_get_market_trades(&cb_client).await;
    run_get_transactions_summary(&cb_client).await;

    oauth_cb_client.revoke_access().await;
}
