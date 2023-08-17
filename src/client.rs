//! Client with all the calls to Coinbase Advanced API

use std::collections::HashMap;

use async_stream::try_stream;
use futures::stream::Stream;
use reqwest;
use uritemplate::UriTemplate;
use uuid::Uuid;

use crate::accounts::{Account, AccountResponse, AccountsResponse};
use crate::basic_oauth::AccessTokenProvider;
use crate::error::{CbError, CbRequestError};
use crate::fees;
use crate::orders::{
    CancelOrderResponse, CancelOrdersResponse, CreateOrderResponse, FillsResponse, Order,
    OrdersResponse,
};
use crate::products::{
    Candle, CandlesResponse, ContractExpiryType, Granularity, MarketTrades, Pricebook,
    PricebookResponse, PricebooksResponse, Product, ProductType, ProductsResponse,
};
use crate::MAIN_URL;
use crate::{orders, DateTime};

/// Client structure performing http requests to Coinbase Advanced API
pub struct CbClient<'a> {
    https_client: reqwest::Client,
    // It is the responsability of the token provider to give a valid one.
    access_token_provider: &'a (dyn AccessTokenProvider + 'a),
}

type Result<T> = std::result::Result<T, CbError>;

impl<'a> CbClient<'a> {
    /// Instantiate a new client.
    ///
    /// The client is relies on an external OAuth2 Token provider. The external provider is
    /// responsible for the validity of the Access Token.
    ///
    /// Example
    ///
    /// ```no_run
    /// # use coinbase_v3::basic_oauth;
    /// # use coinbase_v3::client;
    /// # use coinbase_v3::utils;
    /// # let (client_id, client_secret, redirect_url) = utils::get_env_variables();
    /// // Create / get a provider implementing the [AccessTokenProvider](`basic_oauth::AccessTokenProvider`) trait.
    /// let oauth_cb_client = basic_oauth::OAuthCbClient::new(&client_id, &client_secret, &redirect_url);
    /// // Instantiate the client
    /// let cb_client = client::CbClient::new(&oauth_cb_client);
    /// ```
    pub fn new(oauth_cb_client: &'a (dyn AccessTokenProvider + 'a)) -> Self {
        CbClient {
            https_client: reqwest::Client::new(),
            access_token_provider: oauth_cb_client,
        }
    }

    async fn get<T>(&self, request_url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self
            .https_client
            .get(request_url)
            .bearer_auth(self.access_token_provider.access_token().secret())
            .send()
            .await?;

        Self::unpack_response(response).await
    }

    async fn post<T, U>(&self, request_url: &str, object: &T) -> Result<U>
    where
        T: serde::ser::Serialize,
        U: serde::de::DeserializeOwned,
    {
        let response = self
            .https_client
            .post(request_url)
            .json(object)
            .bearer_auth(self.access_token_provider.access_token().secret())
            .send()
            .await?;

        Self::unpack_response(response).await
    }

    async fn unpack_response<T>(response: reqwest::Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let text_content = response.text().await?;
        println!("{:#?}", text_content);

        match serde_json::from_str::<T>(&text_content) {
            Ok(result) => Ok(result),
            Err(err) => match serde_json::from_str::<CbRequestError>(&text_content) {
                Ok(cb_err) => Err(CbError::Coinbase(cb_err)),
                Err(_) => Err(CbError::Serde(err)),
            },
        }
    }

    /// List all accounts and return a stream of account batches
    ///
    /// `limit` elements per batches, starting from `cursor`.
    /// `cursor` should be None in most cases.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccounts)
    pub fn list_accounts<'b>(
        &'b self,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> impl Stream<Item = Result<Vec<Account>>> + 'b {
        try_stream! {
            let uri = Self::get_list_accounts_uri(limit, cursor);
            let mut accounts_response: AccountsResponse = self.get(&uri).await?;
            yield accounts_response.accounts;

            while accounts_response.has_next {
                let cursor = Some(accounts_response.cursor.clone());
                let uri = Self::get_list_accounts_uri(limit, cursor);
                accounts_response= self.get(&uri).await?;
                yield accounts_response.accounts;
            }
        }
    }

    fn get_list_accounts_uri(limit: Option<i32>, cursor: Option<String>) -> String {
        let args = QueryArgs::new()
            .add_optional_scalar_arg("limit", &limit)
            .add_optional_scalar_arg("cursor", &cursor);
        let uri_string = MAIN_URL.to_string() + "/brokerage/accounts{?query*}";
        let uri = UriTemplate::new(&uri_string)
            .set("query", args.get())
            .build();
        uri
    }

    /// Get a Single Account by id.
    ///
    /// A list of valid ids can be retrieve using [list_accounts()](`crate::client::CbClient::list_accounts`)
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccount)
    pub async fn get_account(&self, account_uuid: Uuid) -> Result<Account> {
        let uri_string = MAIN_URL.to_string() + "/brokerage/accounts/{uuid}";
        let uri = UriTemplate::new(&uri_string)
            .set("uuid", account_uuid.to_string())
            .build();
        let account_response: AccountResponse = self.get(&uri).await?;
        Ok(account_response.account)
    }

    /// Get the best bid/ask for all products. A subset of all products can be returned instead by using the product_ids input.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getbestbidask)
    pub async fn get_best_bid_ask(
        &self,
        product_ids: &Option<Vec<&str>>,
    ) -> Result<Vec<Pricebook>> {
        let args = QueryArgs::new().add_optional_vec_args("product_ids", product_ids);
        let uri_string = MAIN_URL.to_string() + "/brokerage/best_bid_ask{?query*}";
        let uri = UriTemplate::new(&uri_string)
            .set("query", args.get())
            .build();
        let pricebooks_response: PricebooksResponse = self.get(&uri).await?;
        Ok(pricebooks_response.pricebooks)
    }

    /// Get a list of bids/asks for a single product. The amount of detail shown can be customized with the limit parameter.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproductbook)
    pub async fn get_product_book(
        &self,
        product_id: &str,
        limit: Option<i32>,
    ) -> Result<Pricebook> {
        let args = QueryArgs::new()
            .add_mandatory_arg("product_id", &product_id)
            .add_optional_scalar_arg("limit", &limit);
        let uri_string = MAIN_URL.to_string() + "/brokerage/product_book/{?query*}";
        let uri = UriTemplate::new(&uri_string)
            .set("query", args.get())
            .build();
        let pricebook_response: PricebookResponse = self.get(&uri).await?;

        Ok(pricebook_response.pricebook)
    }

    /// Get a list of the available currency pairs for trading.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproducts)
    pub async fn list_products(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        product_type: Option<ProductType>,
        product_ids: &Option<Vec<&str>>,
        contract_expiry_type: Option<ContractExpiryType>,
    ) -> Result<Vec<Product>> {
        let args = QueryArgs::new()
            .add_optional_scalar_arg("limit", &limit)
            .add_optional_scalar_arg("offset", &offset)
            .add_optional_scalar_arg("product_type", &product_type)
            .add_optional_vec_args("product_ids", product_ids)
            .add_optional_scalar_arg("contract_expiry_type", &contract_expiry_type);
        let uri_string = MAIN_URL.to_string() + "/brokerage/products{?query*}";
        let uri = UriTemplate::new(&uri_string)
            .set("query", args.get())
            .build();
        let products_response: ProductsResponse = self.get(&uri).await?;

        Ok(products_response.products)
    }

    /// Get information on a single product by product ID.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproduct)
    pub async fn get_product(&self, product_id: &str) -> Result<Product> {
        let uri_string = MAIN_URL.to_string() + "/brokerage/products/{product_id}";
        let uri = UriTemplate::new(&uri_string)
            .set("product_id", product_id.to_string())
            .build();
        let product: Product = self.get(&uri).await?;
        Ok(product)
    }

    /// Get rates for a single product by product ID, grouped in buckets.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getcandles)
    pub async fn get_product_candles(
        &self,
        product_id: &str,
        start: &DateTime,
        end: &DateTime,
        granularity: Granularity,
    ) -> Result<Vec<Candle>> {
        let uri_string = MAIN_URL.to_string() + "/brokerage/products/{product_id}/candles?start={start}&end={end}&granularity={granularity}";
        let uri = UriTemplate::new(&uri_string)
            .set("product_id", product_id.to_string())
            .set("start", start.timestamp().to_string())
            .set("end", end.timestamp().to_string())
            .set("granularity", granularity.to_string())
            .build();
        let candles_response: CandlesResponse = self.get(&uri).await?;
        Ok(candles_response.candles)
    }

    /// Get snapshot information, by product ID, about the last trades (ticks), best bid/ask, and 24h volume.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getmarkettrades)
    pub async fn get_market_trades(&self, product_id: &str, limit: i32) -> Result<MarketTrades> {
        let uri_string =
            MAIN_URL.to_string() + "/brokerage/products/{product_id}/ticker?limit={limit}";
        let uri = UriTemplate::new(&uri_string)
            .set("product_id", product_id.to_string())
            .set("limit", limit.to_string())
            .build();
        let market_trades: MarketTrades = self.get(&uri).await?;
        Ok(market_trades)
    }

    /// Get a list of orders filtered by optional query parameters (product_id, order_status, etc).
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gethistoricalorders)
    pub fn list_orders<'b>(
        &'b self,
        product_id: Option<String>,
        order_status: Option<Vec<orders::Status>>,
        limit: Option<i32>,
        start_date: Option<DateTime>,
        end_date: Option<DateTime>,
        deprecated_user_native_currency: Option<String>,
        order_type: Option<orders::OrderType>,
        order_side: Option<crate::products::Side>,
        cursor: Option<String>,
        product_type: Option<ProductType>,
        order_placement_source: Option<orders::OrderPlacementSource>,
        contract_expiry_type: Option<ContractExpiryType>,
    ) -> impl Stream<Item = Result<Vec<Order>>> + 'b {
        try_stream! {
            let uri = Self::get_list_orders_uri(
                &product_id,&order_status, &limit, &start_date, &end_date,
                &deprecated_user_native_currency, &order_type, &order_side,
                &cursor, &product_type, &order_placement_source, &contract_expiry_type);

            let mut orders_response: OrdersResponse = self.get(&uri).await?;
            yield orders_response.orders;

            while orders_response.has_next {
                let cursor = Some(orders_response.cursor.clone());
                let uri = Self::get_list_orders_uri(
                    &product_id, &order_status, &limit, &start_date, &end_date,
                    &deprecated_user_native_currency, &order_type, &order_side,
                    &cursor, &product_type, &order_placement_source, &contract_expiry_type);
                orders_response= self.get(&uri).await?;
                yield orders_response.orders;
            }
        }
    }

    ///
    /// [Coinbase API reference]()
    fn get_list_orders_uri(
        product_id: &Option<String>,
        order_status: &Option<Vec<orders::Status>>,
        limit: &Option<i32>,
        start_date: &Option<DateTime>,
        end_date: &Option<DateTime>,
        deprecated_user_native_currency: &Option<String>,
        order_type: &Option<orders::OrderType>,
        order_side: &Option<crate::products::Side>,
        cursor: &Option<String>,
        product_type: &Option<ProductType>,
        order_placement_source: &Option<orders::OrderPlacementSource>,
        contract_expiry_type: &Option<ContractExpiryType>,
    ) -> String {
        let args = QueryArgs::new()
            .add_optional_scalar_arg("product_id", product_id)
            .add_optional_vec_args("order_status", order_status)
            .add_optional_scalar_arg("limit", limit)
            .add_optional_datetime_arg("start_date", start_date) // "2021-05-31T09:59:59Z" RFC3339 ?
            .add_optional_datetime_arg("end_date", end_date)
            .add_optional_scalar_arg(
                "deprecated_user_native_currency",
                deprecated_user_native_currency,
            )
            .add_optional_scalar_arg("order_type", order_type)
            .add_optional_scalar_arg("order_side", order_side)
            .add_optional_scalar_arg("cursor", cursor)
            .add_optional_scalar_arg("product_type", product_type)
            .add_optional_scalar_arg("order_placement_source", order_placement_source)
            .add_optional_scalar_arg("contract_expirty_type", contract_expiry_type);

        let uri_string = MAIN_URL.to_string() + "/brokerage/orders/historical/batch{?query*}";
        let uri = UriTemplate::new(&uri_string)
            .set("query", args.get())
            .build();
        uri
    }

    /// Get a list of fills filtered by optional query parameters (product_id, order_id, etc).
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getfills)
    pub fn list_fills<'b>(
        &'b self,
        order_id: Option<String>,
        product_id: Option<String>,
        start_sequence_timestamp: Option<DateTime>,
        end_sequence_timestamp: Option<DateTime>,
        limit: Option<i64>, // CB inconsistency: why i64 instead of i32 as all the others?
        cursor: Option<String>,
    ) -> impl Stream<Item = Result<Vec<orders::Fill>>> + 'b {
        try_stream! {
            let uri = Self::get_list_fills_uri(&order_id, &product_id, &start_sequence_timestamp, &end_sequence_timestamp, &limit, &cursor);

            let mut fills_response: FillsResponse = self.get(&uri).await?;
            yield fills_response.fills;

            while fills_response.cursor != "" {  // NO `has_next`; inconsistency from CB's api?
                let cursor = Some(fills_response.cursor.clone());
                let uri = Self::get_list_fills_uri(&order_id, &product_id, &start_sequence_timestamp, &end_sequence_timestamp, &limit, &cursor);
                fills_response= self.get(&uri).await?;
                yield fills_response.fills;
            }
        }
    }

    fn get_list_fills_uri(
        order_id: &Option<String>,
        product_id: &Option<String>,
        start_sequence_timestamp: &Option<DateTime>,
        end_sequence_timestamp: &Option<DateTime>,
        limit: &Option<i64>, // CB inconsistency: why i64 instead of i32 as all the others?
        cursor: &Option<String>,
    ) -> String {
        let args = QueryArgs::new()
            .add_optional_scalar_arg("order_id", order_id)
            .add_optional_scalar_arg("product_id", product_id)
            .add_optional_scalar_arg("start_sequence_timestamp", start_sequence_timestamp)
            .add_optional_scalar_arg("end_sequence_timestamp", end_sequence_timestamp)
            .add_optional_scalar_arg("limit", limit)
            .add_optional_scalar_arg("cursor", cursor);
        let uri_string = MAIN_URL.to_string() + "/brokerage/orders/historical/fills{?query*}";
        let uri = UriTemplate::new(&uri_string)
            .set("query", args.get())
            .build();
        uri
    }

    /// Get a single order by order ID.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gethistoricalorder)
    pub async fn get_order(&self, order_id: &str) -> Result<Order> {
        let uri_string = MAIN_URL.to_string() + "/brokerage/orders/historical/{order_id}";
        let uri = UriTemplate::new(&uri_string)
            .set("order_id", order_id.to_string())
            .build();
        let order_response: orders::OrderResponse = self.get(&uri).await?;
        Ok(order_response.order)
    }

    /// Get a summary of transactions with fee tiers, total volume, and fees.
    ///
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gettransactionsummary)
    pub async fn get_transactions_summary(
        &self,
        start_date: Option<DateTime>,
        end_date: Option<DateTime>,
        user_native_currency: Option<String>,
        product_type: Option<ProductType>,
        contract_expiry_type: Option<ContractExpiryType>,
    ) -> Result<fees::TransactionsSummary> {
        let args = QueryArgs::new()
            .add_optional_datetime_arg("start_date", &start_date)
            .add_optional_datetime_arg("end_date", &end_date)
            .add_optional_scalar_arg("user_native_currency", &user_native_currency)
            .add_optional_scalar_arg("product_type", &product_type)
            .add_optional_scalar_arg("contract_expiry_type", &contract_expiry_type);
        let uri_string = MAIN_URL.to_string() + "/brokerage/transaction_summary{?query*}";
        let uri = UriTemplate::new(&uri_string)
            .set("query", args.get())
            .build();

        let transaction_summary: fees::TransactionsSummary = self.get(&uri).await?;
        Ok(transaction_summary)
    }

    /// Create an order with a specified product_id (asset-pair), side (buy/sell), etc.
    ///
    /// !Warning! Using to this function might results in a financial loss.
    ///  
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder)
    pub async fn create_order(&self, order: &orders::OrderToSend) -> Result<CreateOrderResponse> {
        let uri = MAIN_URL.to_string() + "/brokerage/orders";
        self.post(&uri, order).await
    }

    /// Initiate cancel requests for one or more orders.
    ///
    /// /// !Warning! Using to this function might results in a financial loss.
    ///  
    /// [Coinbase API reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_cancelorders)
    pub async fn cancel_order(&self, order_ids: &Vec<String>) -> Result<Vec<CancelOrderResponse>> {
        let mut m = HashMap::<&str, &Vec<String>>::new();
        m.insert("order_ids", order_ids);

        let uri = MAIN_URL.to_string() + "/brokerage/orders/batch_cancel";
        let response = self
            .post::<HashMap<&str, &Vec<String>>, CancelOrdersResponse>(&uri, &m)
            .await?;
        Ok(response.results)
    }
}

/// Store date for passing them to a UriTemplate builder
struct QueryArgs {
    data: Vec<(String, String)>,
}

impl QueryArgs {
    fn new() -> Self {
        QueryArgs {
            data: Vec::<(String, String)>::new(),
        }
    }

    fn get(&self) -> Vec<(String, String)> {
        self.data.clone()
    }

    fn add_mandatory_arg<T: ToString>(mut self, key: &str, value: &T) -> Self {
        self.data.push((key.to_string(), value.to_string()));
        self
    }

    fn add_optional_scalar_arg<T: ToString>(mut self, key: &str, value: &Option<T>) -> Self {
        if let Some(x) = value {
            self.data.push((key.to_string(), x.to_string()));
        }
        self
    }

    fn add_optional_datetime_arg(mut self, key: &str, value: &Option<DateTime>) -> Self {
        if let Some(x) = value {
            self.data.push((
                key.to_string(),
                x.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            ));
        }
        self
    }

    fn add_optional_vec_args<T: ToString>(mut self, key: &str, values: &Option<Vec<T>>) -> Self {
        if let Some(xs) = values {
            self.data.append(
                &mut xs
                    .iter()
                    .map(|x| (key.to_string(), x.to_string()))
                    .collect::<Vec<(String, String)>>(),
            );
        }
        self
    }
}
