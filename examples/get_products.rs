use chrono;

use coinbase_v3::{
    basic_oauth::OAuthCbClient,
    client::CbClient,
    products::ContractExpiryType,
    products::ProductType,
    products::{Granularity, Pricebook},
    utils,
};

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let (client_id, client_secret, redirect_url) = utils::get_env_variables();
    let oauth_cb_client = OAuthCbClient::new(&client_id, &client_secret, &redirect_url)
        .add_scope("wallet:user:read")
        .authorize_once()
        .await;

    let cb_client = CbClient::new(&oauth_cb_client);

    run_get_bid_ask(&cb_client).await;
    run_get_product_book(&cb_client).await;
    run_list_products(&cb_client).await;
    run_get_product(&cb_client).await;
    run_get_product_candles(&cb_client).await;
    run_get_market_trades(&cb_client).await;

    oauth_cb_client.revoke_access().await;
}

pub async fn run_get_bid_ask(cb_client: &CbClient<'_>) {
    {
        println!("============= All best bid asks =============\n");
        let product_ids = None;
        let pricebooks: Vec<Pricebook> = cb_client.get_best_bid_ask(&product_ids).await.unwrap();
        println!("Found {:#?} pricebooks.", pricebooks.len());
    }

    {
        println!("===== Best bid asks for 2 products ==========\n");
        let product_ids = Some(vec!["OGN-BTC", "WCFG-USD"]);
        let pricebooks: Vec<Pricebook> = cb_client.get_best_bid_ask(&product_ids).await.unwrap();
        println!("{:#?}", pricebooks);
    }
}

pub async fn run_get_product_book(cb_client: &CbClient<'_>) {
    {
        let product_id = "BAT-ETH";
        let limit = None;
        let product_book = cb_client.get_product_book(product_id, limit).await.unwrap();
        println!(
            "Found {} bids and {} asks  lines in the product book",
            product_book.bids.len(),
            product_book.asks.len()
        );
    }

    {
        let product_id = "BAT-ETH";
        let limit = Some(3);
        let product_book = cb_client.get_product_book(product_id, limit).await.unwrap();
        println!("{:#?}", product_book);
    }
}

pub async fn run_list_products(cb_client: &CbClient<'_>) {
    {
        let products = cb_client
            .list_products(Some(1), Some(763), None, &None, None)
            .await
            .unwrap();
        println!("Found {:#?} products.", products.len());
    }

    {
        let products = cb_client
            .list_products(Some(4), Some(4), None, &None, None)
            .await
            .unwrap();
        println!("Found {:#?} products.", products.len());
    }

    {
        let product_type = Some(ProductType::Spot);
        let products = cb_client
            .list_products(None, None, product_type, &None, None)
            .await
            .unwrap();
        println!(
            "Found {:#?} products of type {:#?}.",
            products.len(),
            ProductType::Spot
        );
    }

    {
        let product_ids = Some(vec!["OGN-BTC", "WCFG-USD"]);
        let products = cb_client
            .list_products(None, None, None, &product_ids, None)
            .await
            .unwrap();
        println!("Found {:#?} products from Id.", products.len());
    }

    {
        let products = cb_client
            .list_products(None, None, None, &None, Some(ContractExpiryType::Expiring))
            .await
            .unwrap();
        println!(
            "Found {:#?} products with expiring contracts.",
            products.len()
        );
    }
}

pub async fn run_get_product(cb_client: &CbClient<'_>) {
    let product_id = "OGN-BTC";
    let product = cb_client.get_product(product_id).await.unwrap();
    println!("\n{:#?}\n", product);
}

pub async fn run_get_product_candles(cb_client: &CbClient<'_>) {
    let product_id = "OGN-BTC";
    let naivedatetime_utc = chrono::naive::NaiveDate::from_ymd_opt(2023, 1, 12)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let start = chrono::DateTime::from_utc(naivedatetime_utc, chrono::offset::Utc);
    let end = start
        .clone()
        .checked_add_days(chrono::naive::Days::new(2))
        .unwrap();
    let candles = cb_client
        .get_product_candles(product_id, &start, &end, Granularity::OneDay)
        .await
        .unwrap();
    println!("\n{:#?}\n", candles);
}

pub async fn run_get_market_trades(cb_client: &CbClient<'_>) {
    let product_id = "OGN-BTC";
    let limit = 3;
    let market_trades = cb_client
        .get_market_trades(product_id, limit)
        .await
        .unwrap();
    println!("\n{:#?}\n", market_trades);
}
