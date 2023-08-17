use futures::{pin_mut, stream::StreamExt};

use coinbase_v3::{
    basic_oauth::OAuthCbClient, client::CbClient, orders, products, utils, DateTime,
};

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let (client_id, client_secret, redirect_url) = utils::get_env_variables();
    let oauth_cb_client = OAuthCbClient::new(&client_id, &client_secret, &redirect_url)
        .add_scope("wallet:transactions:read") // NOT wallet:orders:read as CB's doc says.
        .authorize_once()
        .await;

    let cb_client = CbClient::new(&oauth_cb_client);
    run_list_orders(&cb_client).await;
    run_list_fills(&cb_client).await;

    oauth_cb_client.revoke_access().await;
}

pub async fn run_list_orders(cb_client: &CbClient<'_>) {
    let product_id: Option<String> = None;
    let order_status: Option<Vec<orders::Status>> = None;
    let limit: Option<i32> = Some(10);
    let start_date: Option<DateTime> = None;
    let end_date: Option<DateTime> = None;
    let deprecated_user_native_currency: Option<String> = None;
    let order_type: Option<orders::OrderType> = None;
    let order_side: Option<orders::OrderSide> = Some(orders::OrderSide::Buy);
    let cursor: Option<String> = None;
    let product_type: Option<products::ProductType> = Some(products::ProductType::Spot);
    let order_placement_source: Option<orders::OrderPlacementSource> = None;
    let contract_expiry_type: Option<products::ContractExpiryType> = None;

    let orders_stream = cb_client.list_orders(
        product_id,
        order_status,
        limit,
        start_date,
        end_date,
        deprecated_user_native_currency,
        order_type,
        order_side,
        cursor,
        product_type,
        order_placement_source,
        contract_expiry_type,
    );
    pin_mut!(orders_stream);

    let mut orders = Vec::<orders::Order>::new();
    while let Some(orders_result) = orders_stream.next().await {
        let mut partial_orders = orders_result.unwrap();
        println!("Got {} orders", partial_orders.len());
        orders.append(&mut partial_orders);

        if partial_orders.len() > 30 {
            break;
        }
    }
    println!("Got {} orders in total.", orders.len());

    let order = cb_client.get_order(&orders[0].order_id).await.unwrap();
    assert_eq!(orders[0], order);
}

pub async fn run_list_fills(cb_client: &CbClient<'_>) {
    let limit = Some(10);
    let fills_stream = cb_client.list_fills(None, None, None, None, limit, None);
    pin_mut!(fills_stream);

    let mut fills = Vec::<orders::Fill>::new();
    while let Some(fills_result) = fills_stream.next().await {
        let mut partial_fills = fills_result.unwrap();
        println!("Got {} fills", partial_fills.len());
        fills.append(&mut partial_fills);

        if fills.len() > 30 {
            break;
        }
    }
    println!("\nFills\n------------\n{:#?}", fills.len());
}
