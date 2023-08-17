//! Structures & Enums representing Coinbase's order related structures

use bigdecimal::BigDecimal;
use serde::{Deserialize, Deserializer};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::str::FromStr;

use crate::DateTime;

/// Structure representing Coinbase's response for a pricebook
#[derive(Deserialize, Debug)]
pub struct Pricebook {
    pub product_id: String,
    pub bids: Vec<Bid>,
    pub asks: Vec<Ask>,
    pub time: DateTime,
}

/// Structure representing Coinbase's response for a bid
#[derive(Deserialize, Debug)]
pub struct Bid {
    pub price: BigDecimal,
    pub size: BigDecimal,
}

/// Structure representing Coinbase's response for a ask
#[derive(Deserialize, Debug)]
pub struct Ask {
    pub price: BigDecimal,
    pub size: BigDecimal,
}

/// Structure representing Coinbase's response for a details of a fcm trading session
#[derive(Deserialize, Debug)]
pub struct FcmTradingSessionDetails {
    pub is_session_open: bool,
    pub open_time: DateTime,
    pub close_time: Option<DateTime>,
}

/// Structure representing Coinbase's response for perpetual details
#[derive(Deserialize, Debug)]
pub struct PerpetualDetails {
    pub open_interest: String,
    pub funding_rate: String,
    pub funding_time: Option<DateTime>,
}

/// Structure representing Coinbase's response for details of a future product
#[derive(Deserialize, Debug)]
pub struct FutureProductDetails {
    pub venue: String,
    pub contract_code: String,
    pub contract_expiry: DateTime,
    pub contract_size: String,
    pub contract_root_unit: String,
    /// Descriptive name for the product series, eg "Nano Bitcoin Futures".
    pub group_description: String,
    pub contract_expiry_timezone: String,
    /// Short version of the group_description, eg "Nano BTC".
    pub group_short_description: String,
    /// Possible values: [UNKNOWN_RISK_MANAGEMENT_TYPE, MANAGED_BY_FCM, MANAGED_BY_VENUE]
    pub risk_managed_by: String,
    /// Possible values: [UNKNOWN_CONTRACT_EXPIRY_TYPE, EXPIRING]
    pub contract_expiry_type: String,
    pub perpetual_details: PerpetualDetails,
    pub contract_display_name: String,
}

// Accounting for the fact that when no data are available Coinbase return sometimes null sometimes
// the empty string ""
fn deserialize_bigdecimal_stable<'de, D>(deserializer: D) -> Result<Option<BigDecimal>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|b| {
        if b.is_empty() {
            Ok(None)
        } else {
            match BigDecimal::from_str(&b) {
                Ok(value) => Ok(Some(value)),
                Err(_) => Ok(None),
            }
        }
    })?
}

/// Structure representing Coinbase's response for a product
#[derive(Deserialize, Debug)]
pub struct Product {
    /// The trading pair.
    pub product_id: String,
    #[serde(default, deserialize_with = "deserialize_bigdecimal_stable")]
    /// The current price for the product, in quote currency.
    pub price: Option<BigDecimal>,
    #[serde(default, deserialize_with = "deserialize_bigdecimal_stable")]
    /// The amount the price of the product has changed, in percent, in the last 24 hours.
    pub price_percentage_change_24h: Option<BigDecimal>, // from the doc, there is a % sign at the
    // end of the string. from the response, this never happens. same for the next 2 values.
    /// The trading volume for the product in the last 24 hours.
    #[serde(default, deserialize_with = "deserialize_bigdecimal_stable")]
    pub volume_24h: Option<BigDecimal>,
    /// The percentage amount the volume of the product has changed in the last 24 hours.
    #[serde(default, deserialize_with = "deserialize_bigdecimal_stable")]
    pub volume_percentage_change_24h: Option<BigDecimal>,
    /// Minimum amount base value can be increased or decreased at once.
    pub base_increment: BigDecimal,
    /// Minimum amount quote value can be increased or decreased at once.
    pub quote_increment: BigDecimal,
    /// Minimum size that can be represented of quote currency.
    pub quote_min_size: BigDecimal,
    /// Maximum size that can be represented of quote currency.
    pub quote_max_size: BigDecimal,
    /// Minimum size that can be represented of base currency.
    pub base_min_size: BigDecimal,
    /// Maximum size that can be represented of base currency.
    pub base_max_size: BigDecimal,
    /// Name of the base currency.
    pub base_name: String,
    /// Name of the quote currency.
    pub quote_name: String,
    /// Whether or not the product is on the user's watchlist.
    pub watched: bool,
    /// Whether or not the product is disabled for trading.
    pub is_disabled: bool,
    /// Whether or not the product is 'new'.
    pub new: bool,
    /// Status of the product.
    pub status: String,
    /// Whether or not orders of the product can only be cancelled, not placed or edited.          
    pub cancel_only: bool,
    /// Whether or not orders of the product can only be limit orders, not market orders.
    pub limit_only: bool,
    /// Whether or not orders of the product can only be posted, not cancelled.
    pub post_only: bool,
    /// Whether or not the product is disabled for trading for all market participants.
    pub trading_disabled: bool,
    /// Whether or not the product is in auction mode.
    pub auction_mode: bool,
    /// Possible values: [SPOT, FUTURE]
    pub product_type: String,
    /// Symbol of the quote currency.
    pub quote_currency_id: String,
    /// Symbol of the base currency.
    pub base_currency_id: String,
    /// The current midpoint of the bid-ask spread, in quote currency.
    pub fcm_trading_session_details: Option<FcmTradingSessionDetails>,
    pub mid_market_price: String,
    /// Product id for the corresponding unified book.
    pub alias: String,
    /// Product ids that this product serves as an alias for.
    pub alias_to: Vec<String>,
    /// Symbol of the base display currency.
    pub base_display_symbol: String,
    /// Symbol of the quote display currency.
    pub quote_display_symbol: String,
    /// Whether or not the product is in view only mode.
    pub view_only: bool,
    /// Minimum amount price can be increased or decreased at once.
    pub price_increment: BigDecimal,
    pub future_product_details: Option<FutureProductDetails>,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct ProductsResponse {
    pub products: Vec<Product>,
    pub num_products: i32,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct PricebooksResponse {
    pub pricebooks: Vec<Pricebook>,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct PricebookResponse {
    pub pricebook: Pricebook,
}

/// Enum representing Coinbase's valid product types
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    Spot,
    Future,
}

/// Enum representing Coinbase's valid contract expiry types
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractExpiryType {
    UnknownRiskManagementType,
    Expiring,
}

/// Enum representing Coinbase's valid Granularities (for candles)
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Granularity {
    UnknownGranularity,
    OneMinute,
    FiveMinute,
    FifteenMinute,
    ThirtyMinute,
    OneHour,
    TwoHour,
    SixHour,
    OneDay,
}

/// Structure representing Coinbase's response for a candle
#[derive(Deserialize, Debug)]
pub struct Candle {
    /// Timestamp for bucket start time, in UNIX time.
    pub start: String,
    /// Lowest price during the bucket interval.
    pub low: BigDecimal,
    /// Highest price during the bucket interval.
    pub high: BigDecimal,
    /// Opening price (first trade) in the bucket interval.
    pub open: BigDecimal,
    /// Closing price (last trade) in the bucket interval.
    pub close: BigDecimal,
    /// Volume of trading activity during the bucket interval.
    pub volume: BigDecimal,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct CandlesResponse {
    pub candles: Vec<Candle>,
}

/// Enum representing Coinbase's valid Trade Sides
///
/// Aliased to [`crate::orders::OrderSide`]
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    UnknownOrderSide,
    Buy,
    Sell,
}
/// Structure representing Coinbase's response for a Trade
#[derive(Deserialize, Debug)]
pub struct Trade {
    /// The ID of the trade that was placed.
    pub trade_id: String,
    /// The trading pair.
    pub product_id: String,
    /// The price of the trade, in quote currency.
    pub price: BigDecimal,
    /// The size of the trade, in base currency.
    pub size: BigDecimal,
    /// The time of the trade.
    pub time: DateTime,
    /// Possible values: [UNKNOWN_ORDER_SIDE, BUY, SELL]
    pub side: Side,
    // bid and ask are Strings as CB response returns "" for them...
    /// The best bid for the `product_id`, in quote currency.
    pub bid: Option<String>,
    /// The best ask for the `product_id`, in quote currency.
    pub ask: Option<String>,
}

/// Structure representing Coinbase's response listing multiple Market Trades
#[derive(Deserialize, Debug)]
pub struct MarketTrades {
    pub trades: Vec<Trade>,
    /// The best bid for the `product_id`, in quote currency.
    pub best_bid: BigDecimal,
    /// The best ask for the `product_id`, in quote currency.
    pub best_ask: BigDecimal,
}

/// Enum representing Coinbase's valid Trade types
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeType {
    Fill,
    Reversal,
    Correction,
    Synthetic,
}

//=========== TESTS ===========================================================

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_product_deserialize() {
        let input = r##"{
        "product_id": "BAT-ETH",
        "price": "",
        "volume_24h": "6",
        "volume_percentage_change_24h": "-99.40239043824701",
        "base_increment": "1",
        "quote_increment": "0.00000001",
        "quote_min_size": "0.0003",
        "quote_max_size": "2500",
        "base_min_size": "4.5",
        "base_max_size": "480000",
        "base_name": "Basic Attention Token",
        "quote_name": "Ethereum",
        "watched": false,
        "is_disabled": false,
        "new": false,
        "status": "online",
        "cancel_only": false,
        "limit_only": false,
        "post_only": false,
        "trading_disabled": false,
        "auction_mode": false,
        "product_type": "SPOT",
        "quote_currency_id": "ETH",
        "base_currency_id": "BAT",
        "fcm_trading_session_details": null,
        "mid_market_price": "",
        "alias": "ALIAS",
        "alias_to": ["ALIAS-TO"],
        "base_display_symbol": "BAT",
        "quote_display_symbol": "ETH",
        "view_only": false,
        "price_increment": "0.00000001"
    }"##;
        // "price_percentage_change_24h": "9", -- Removed to test Option

        let product: Product = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(product.product_type, "SPOT".to_string());
    }

    #[test]
    fn test_pricebook_deserialize() {
        let input = r##"{
            "product_id": "QSP-USDT",
            "bids": [{ "price": "0.01251", "size": "7448" }],
            "asks": [{ "price": "0.0127", "size": "2850" }],
            "time": "2023-07-05T05:30:57.651784Z"
        }"##;
        let pricebook: Pricebook = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!("QSP-USDT".to_string(), pricebook.product_id);
        assert_eq!("QSP-USDT".to_string(), pricebook.product_id);
    }

    #[test]
    fn test_product_type_deserialize() {
        let input = r##""SPOT""##;
        let product_type: ProductType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(product_type, ProductType::Spot);

        let input = r##""FUTURE""##;
        let product_type: ProductType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(product_type, ProductType::Future);
    }

    #[test]
    fn test_product_type_serialize() {
        let expected = r##""SPOT""##;
        assert_eq!(expected, serde_json::to_string(&ProductType::Spot).unwrap());

        let expected = r##""FUTURE""##;
        assert_eq!(
            expected,
            serde_json::to_string(&ProductType::Future).unwrap()
        );
    }

    #[test]
    fn test_contract_expiry_type_deserialize() {
        let input = r##""UNKNOWN_RISK_MANAGEMENT_TYPE""##;
        let expiry_type: ContractExpiryType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(expiry_type, ContractExpiryType::UnknownRiskManagementType);

        let input = r##""EXPIRING""##;
        let expiry_type: ContractExpiryType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(expiry_type, ContractExpiryType::Expiring);
    }

    #[test]
    fn test_contract_expiry_type_serialize() {
        let expected = r##""UNKNOWN_RISK_MANAGEMENT_TYPE""##;
        assert_eq!(
            expected,
            serde_json::to_string(&ContractExpiryType::UnknownRiskManagementType).unwrap()
        );

        let expected = r##""EXPIRING""##;
        assert_eq!(
            expected,
            serde_json::to_string(&ContractExpiryType::Expiring).unwrap()
        );
    }

    #[test]
    fn test_fcm_trading_session_details_deserialize() {
        let input = r##"{
            "is_session_open": false,
            "open_time": "2023-07-05T05:30:57.651784Z",
            "close_time": "2023-07-06T05:30:57.651784Z"
        }"##;
        let result: FcmTradingSessionDetails = serde_json::from_slice(input.as_bytes()).unwrap();
        assert!(!result.is_session_open);
    }

    #[test]
    fn test_perpetual_details_deserialize() {
        let input = r##"{
            "open_interest": "1.234",
            "funding_rate": "0.05123",
            "funding_time": "2023-07-06T05:30:57.651784Z"
        }"##;
        let result: PerpetualDetails = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result.open_interest, "1.234");
    }

    #[test]
    fn test_future_product_details_deserialize() {
        let input = r##"{
            "venue": "string",
            "contract_code": "string",
            "contract_expiry": "2023-07-06T05:30:57.651784Z",
            "contract_size": "string",
            "contract_root_unit": "string",
            "group_description": "string",
            "contract_expiry_timezone": "string",
            "group_short_description": "string",
            "risk_managed_by": "UNKNOWN_RISK_MANAGEMENT_TYPE",
            "contract_expiry_type": "UNKNOWN_CONTRACT_EXPIRY_TYPE",
            "perpetual_details": {
                "open_interest": "string",
                "funding_rate": "string",
                "funding_time": "2023-07-06T05:30:57.651784Z"
            },
            "contract_display_name": "string"
        }"##;
        let result: FutureProductDetails = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result.perpetual_details.open_interest, "string".to_string());
    }

    #[test]
    fn test_granularity_deserialize() {
        let input = r##""UNKNOWN_GRANULARITY""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::UnknownGranularity);

        let input = r##""ONE_MINUTE""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::OneMinute);

        let input = r##""FIVE_MINUTE""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::FiveMinute);

        let input = r##""FIFTEEN_MINUTE""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::FifteenMinute);

        let input = r##""THIRTY_MINUTE""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::ThirtyMinute);

        let input = r##""ONE_HOUR""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::OneHour);

        let input = r##""TWO_HOUR""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::TwoHour);

        let input = r##""SIX_HOUR""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::SixHour);

        let input = r##""ONE_DAY""##;
        let granularity: Granularity = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(granularity, Granularity::OneDay);
    }

    #[test]
    fn test_granularity_serialize() {
        let expected = r##""ONE_MINUTE""##;
        assert_eq!(
            expected,
            serde_json::to_string(&Granularity::OneMinute).unwrap()
        );
    }

    #[test]
    fn test_candle_deserialize() {
        let input = r##"{
            "start": "1639508050",
            "low": "140.21",
            "high": "140.21",
            "open": "140.21",
            "close": "140.21",
            "volume": "56437345"
        }"##;
        let result: Candle = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result.start, "1639508050");
    }

    #[test]
    fn test_tradeside_deserialize() {
        let input = r##""UNKNOWN_ORDER_SIDE""##;
        let result: Side = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Side::UnknownOrderSide);

        let input = r##""BUY""##;
        let result: Side = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Side::Buy);

        let input = r##""SELL""##;
        let result: Side = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Side::Sell);
    }

    #[test]
    fn test_tradeside_serialize() {
        let expected = r##""BUY""##;
        assert_eq!(expected, serde_json::to_string(&Side::Buy).unwrap());
    }

    #[test]
    fn test_trade_response_deserialize() {
        // TODO: check what is the actual response from a request; doc seems to be off (no []
        // around trades.
        let input = r##"{
            "trade_id":"796313", 
            "product_id":"OGN-BTC",
            "price":"0.00000318", 
            "size":"1.48", 
            "time":"2023-08-11T21:37:07.361937Z", 
            "side":"BUY", 
            "bid":"",
            "ask":""
        }"##;
        let result: Trade = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result.product_id, "OGN-BTC".to_string());
    }

    #[test]
    fn test_market_trade_response_deserialize() {
        // TODO: check what is the actual response from a request; doc seems to be off (no []
        // around trades.
        let input = r##"{
            "trades":[
                {
                    "trade_id":"796313", 
                    "product_id":"OGN-BTC",
                    "price":"0.00000318", 
                    "size":"1.48", 
                    "time":"2023-08-11T21:37:07.361937Z", 
                    "side":"BUY", 
                    "bid":"",
                    "ask":""
                }, 
                {
                    "trade_id":"796312", 
                    "product_id":"OGN-BTC", 
                    "price":"0.00000318", 
                    "size":"4.58", 
                    "time":"2023-08-11T21:37:07.361937Z", 
                    "side":"BUY", 
                    "bid":"", 
                    "ask":""
                },
                {
                    "trade_id":"796311", 
                    "product_id":"OGN-BTC", 
                    "price":"0.00000318", 
                    "size":"6.06", 
                    "time":"2023-08-11T21:25:56.961648Z", 
                    "side":"BUY", 
                    "bid":"", 
                    "ask":""
                }
            ],
            "best_bid":"0.00000318", 
            "best_ask":"0.0000032"
        }"##;
        let result: MarketTrades = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result.trades[0].product_id, "OGN-BTC".to_string());
    }

    #[test]
    fn test_tradetype_deserialize() {
        let input = r##""FILL""##;
        let result: TradeType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TradeType::Fill);

        let input = r##""REVERSAL""##;
        let result: TradeType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TradeType::Reversal);

        let input = r##""CORRECTION""##;
        let result: TradeType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TradeType::Correction);

        let input = r##""SYNTHETIC""##;
        let result: TradeType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TradeType::Synthetic);
    }

    #[test]
    fn test_tradetype() {
        let expected = r##""FILL""##;
        assert_eq!(expected, serde_json::to_string(&TradeType::Fill).unwrap());
    }
}
