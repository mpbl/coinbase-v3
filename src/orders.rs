//! Structures, Enums & helper functions for Coinbase's order related structures

use anyhow::anyhow;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use crate::products::ProductType;
use crate::products::Side; // Move to order? might make more sense...
use crate::DateTime;

/// Structure representing Coinbase's order configuration structure
///
/// `Option` can nornally be not `None` for one of the item.
/// It is not an enum to be able to deserialize the response
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct OrderConfiguration {
    pub market_market_ioc: Option<Market>,
    pub limit_limit_gtc: Option<Limit>,
    pub limit_limit_gtd: Option<Limit>,
    pub stop_limit_stop_limit_gtc: Option<StopLimit>,
    pub stop_limit_stop_limit_gtd: Option<StopLimit>,
}

/// Structure representing Coinbase's Market order structure
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Market {
    /// Amount of quote currency to spend on order. Required for BUY orders.
    pub quote_size: Option<BigDecimal>,
    /// Amount of base currency to spend on order. Required for SELL orders
    pub base_size: Option<BigDecimal>,
}

/// Structure representing Coinbase's limit order structure
///
/// end_time is only used for gtd orders, not gtc
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Limit {
    /// Amount of base currency to spend on order
    pub base_size: BigDecimal,
    /// Ceiling price for which the order should get filled
    pub limit_price: BigDecimal,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: Option<DateTime>,
    /// Post only limit order
    pub post_only: Option<bool>,
}

/// Enum representing the possible direction of the stop order.
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StopDirection {
    UnknownStopDirection,
    StopDirectionStopUp,
    StopDirectionStopDown,
}

/// Structure representing Coinbase's stop-limit order structure
///
/// end_time is only used for gtd orders, not gtc
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct StopLimit {
    /// Amount of base currency to spend on order
    pub base_size: BigDecimal,
    /// Ceiling price for which the order should get filled
    pub limit_price: BigDecimal,
    /// Price at which the order should trigger - if stop direction is Up,
    /// then the order will trigger when the last trade price goes above this,
    /// otherwise order will trigger when last trade price goes below this price.
    pub stop_price: BigDecimal,
    pub stop_direction: StopDirection,
    pub end_time: Option<DateTime>,
}

/// Enum representing the possible status values of an order
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Open,
    Filled,
    Cancelled,
    Expired,
    Failed,
    UnknownOrderStatus,
}

/// Enum representing the possible values for the time in force of an order
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    UnknownTimeInForce,
    GoodUntilDateTime,
    GoodUntilCancelled,
    ImmediateOrCancel,
    FillOrKill,
}

/// Enum representing the possible values for the trigger status of an order
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TriggerStatus {
    UnknownTriggerStatus,
    InvalidOrderType,
    StopPending,
    StopTriggered,
}

/// Enum representing the possible values for type of order
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    UnknownOrderType,
    Market,
    Limit,
    Stop,
    #[serde(rename = "STOP_LIMIT")]
    StopLimitOrderType,
}

/// Enum representing the possible values for the reject reason
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RejectReason {
    RejectReasonUnspecified,
}

/// Enum representing the possible values for the source of the order placed
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderPlacementSource {
    RetailSimple,
    RetailAdvanced,
}

/// Structure representing an order response
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Order {
    /// The unique id for this order
    pub order_id: String,
    /// The product this order was created for e.g. 'BTC-USD'
    pub product_id: String,
    /// The id of the User owning this Order
    pub user_id: String,
    pub order_configuration: OrderConfiguration,
    /// Possible values: [UNKNOWN_ORDER_SIDE, BUY, SELL]
    pub side: Side,
    /// Client specified ID of order.
    pub client_order_id: String,
    /// Possible values: [OPEN, FILLED, CANCELLED, EXPIRED, FAILED, UNKNOWN_ORDER_STATUS]
    pub status: Status,
    /// Possible values: [UNKNOWN_TIME_IN_FORCE, GOOD_UNTIL_DATE_TIME, GOOD_UNTIL_CANCELLED, IMMEDIATE_OR_CANCEL, FILL_OR_KILL]
    pub time_in_force: TimeInForce,
    /// Timestamp for when the order was created
    pub created_time: DateTime,
    /// The percent of total order amount that has been filled
    pub completion_percentage: String,
    /// The portion (in base currency) of total order amount that has been filled
    pub filled_size: String,
    /// The average of all prices of fills for this order
    pub average_filled_price: String,
    /// Commission amount
    pub fee: String,
    /// Number of fills that have been posted for this order
    pub number_of_fills: String,
    /// The portion (in quote current) of total order amount that has been filled
    pub filled_value: String,
    /// Whether a cancel request has been initiated for the order, and not yet completed
    pub pending_cancel: bool,
    /// Whether the order was placed with quote currency
    pub size_in_quote: bool,
    /// The total fees for the order
    pub total_fees: String,
    /// Whether the order size includes fees
    pub size_inclusive_of_fees: bool,
    /// derived field: filled_value + total_fees for buy orders and filled_value - total_fees for sell orders.
    pub total_value_after_fees: String,
    /// Possible values: [UNKNOWN_TRIGGER_STATUS, INVALID_ORDER_TYPE, STOP_PENDING, STOP_TRIGGERED]
    pub trigger_status: TriggerStatus,
    /// Possible values: [UNKNOWN_ORDER_TYPE, MARKET, LIMIT, STOP, STOP_LIMIT]
    pub order_type: OrderType,
    /// Possible values: REJECT_REASON_UNSPECIFIED
    pub reject_reason: RejectReason,
    // True if the order is fully filled, false otherwise.
    pub settled: bool,
    /// Possible values: [SPOT, FUTURE]
    pub product_type: ProductType,
    /// Message stating why the order was rejected.
    pub reject_message: Option<String>,
    /// Message stating why the order was canceled.
    pub cancel_message: Option<String>,
    /// Possible values: [RETAIL_SIMPLE, RETAIL_ADVANCED]
    pub order_placement_source: OrderPlacementSource,
    // The remaining hold amount (holdAmount - holdAmountReleased). [value is 0 if holdReleased is true]
    pub outstanding_hold_amount: String,
    /// True if order is of liquidation type.
    pub is_liquidation: bool,
}

#[doc(hidden)]
/// Structure representing Coinbase's wrapped response for a single order
#[derive(Deserialize, Debug)]
pub struct OrderResponse {
    pub order: Order,
}

pub type OrderSide = crate::products::Side;
pub type TradeType = crate::products::TradeType;

#[doc(hidden)]
/// Structure representing Coinbase's wrapper response for multiple orders
#[derive(Deserialize, Debug)]
pub struct OrdersResponse {
    pub orders: Vec<Order>,
    pub sequence: String,
    pub has_next: bool,
    pub cursor: String,
}

/// Structure representing CB's response to a fill request
#[derive(Deserialize, Debug)]
pub struct Fill {
    /// Unique identifier for the fill.
    pub entry_id: String,
    /// ID of the fill -- unique for all `FILL` trade_types but not unique for adjusted fills.
    pub trade_id: String,
    /// ID of the order the fill belongs to.
    pub order_id: String,
    /// Time at which this fill was completed.
    pub trade_time: DateTime,
    /// String denoting what type of fill this is. Regular fills have the value `FILL`. Adjusted fills have possible values `REVERSAL`, `CORRECTION`, `SYNTHETIC`.
    pub trade_type: TradeType,
    /// Price the fill was posted at.
    pub price: String,
    /// Amount of order that was transacted at this fill.
    pub size: String,
    /// Fee amount for fill.
    pub commission: String,
    /// The product this order was created for.
    pub product_id: String,
    /// Time at which this fill was posted.
    pub sequence_timestamp: DateTime,
    /// Possible values: [UNKNOWN_LIQUIDITY_INDICATOR, MAKER, TAKER]
    pub liquidity_indicator: LiquidityIndicator,
    /// Whether the order was placed with quote currency.
    pub size_in_quote: bool,
    /// User that placed the order the fill belongs to.
    pub user_id: String,
    /// Possible values: [UNKNOWN_ORDER_SIDE, BUY, SELL]
    pub side: OrderSide,
}

/// Enum representing the possible values for the liquidity indicator
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LiquidityIndicator {
    UnknownLiquidityIndicator,
    Maker,
    Taker,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct FillsResponse {
    pub fills: Vec<Fill>,
    pub cursor: String,
    // CB Bug? why no `has_next`?
}

/// Structure to fill to create a new request to be sent to CB
#[derive(Serialize, Debug)]
pub struct OrderToSend {
    /// Client set unique uuid for this order
    client_order_id: String,
    /// The product this order was created for e.g. 'BTC-USD'
    product_id: String,
    /// Possible values: [UNKNOWN_ORDER_SIDE, BUY, SELL]
    side: OrderSide,
    order_configuration: OrderConfiguration,
}

/// Enum representing the possible values for failure to create an order
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreateOrderFailureReason {
    UnknownFailureReason,
    UnsupportedOrderConfiguration,
    InvalidSide,
    InvalidProductId,
    InvalidSizePrecision,
    InvalidPricePrecision,
    InsufficientFund,
    InvalidLedgerBalance,
    OrderEntryDisabled,
    IneligiblePair,
    InvalidLimitPricePostOnly,
    InvalidLimitPrice,
    InvalidNoLiquidity,
    InvalidRequest,
    CommanderRejectedNewOrder,
    InsufficientFunds,
}

/// Enum representing the possible values for failure to preview create an order
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PreviewCreateOrderFailureReason {
    UnknownPreviewFailureReason,
    PreviewMissingCommissionRate,
    PreviewInvalidSide,
    PreviewInvalidOrderConfig,
    PreviewInvalidProductId,
    PreviewInvalidSizePrecision,
    PreviewInvalidPricePrecision,
    PreviewMissingProductPriceBook,
    PreviewInvalidLedgerBalance,
    PreviewInsufficientLedgerBalance,
    PreviewInvalidLimitPricePostOnly,
    PreviewInvalidLimitPrice,
    PreviewInvalidNoLiquidity,
    PreviewInsufficientFund,
    PreviewInvalidCommissionConfiguration,
    PreviewInvalidStopPrice,
    PreviewInvalidBaseSizeTooLarge,
    PreviewInvalidBaseSizeTooSmall,
    PreviewInvalidQuoteSizePrecision,
    PreviewInvalidQuoteSizeTooLarge,
    PreviewInvalidPriceTooLarge,
    PreviewInvalidQuoteSizeTooSmall,
    PreviewInsufficientFundsForFutures,
    PreviewBreachedPriceLimit,
    PreviewBreachedAccountPositionLimit,
    PreviewBreachedCompanyPositionLimit,
    PreviewInvalidMarginHealth,
    PreviewRiskProxyFailure,
    PreviewUntradableFcmAccountStatus,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct OrderSuccessResponse {
    pub order_id: String,
    pub product_id: String,
    pub side: OrderSide,
    pub client_order_id: String,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct OrderErrorResponse {
    pub error: CreateOrderFailureReason,
    pub message: String,
    pub error_details: String,
    pub preview_failure_reason: PreviewCreateOrderFailureReason,
    pub new_order_failure_reason: CreateOrderFailureReason,
}

/// Structure representing CB's response to a create order request
#[derive(Deserialize, Debug)]
pub struct CreateOrderResponse {
    /// Whether the order was created.
    pub success: bool,
    pub failure_reason: CreateOrderFailureReason,
    /// The ID of the order created
    pub order_id: String,
    pub success_response: Option<OrderSuccessResponse>,
    pub error_response: Option<OrderErrorResponse>,
    pub order_configuration: OrderConfiguration,
}

/// Enum representating the possible values for CB failing to cancel an order
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelOrderFailureReason {
    UnknownCancelFailureReason,
    InvalidCancelRequest,
    UnknownCancelOrder,
    CommanderRejectedCancelOrder,
    DuplicateCancelRequest,
}

/// Structure representing CB's response to a cancel order request
#[derive(Deserialize, Debug)]
pub struct CancelOrderResponse {
    /// Whether the order was cancelled
    pub success: bool,
    pub failure_reason: Option<CancelOrderFailureReason>,
    /// The ID of the order cancelled
    pub order_id: String,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct CancelOrdersResponse {
    pub results: Vec<CancelOrderResponse>,
}

/// Create a MARKET order
///
/// `side` (Buy or Sell) `product_id` for an amount of `order_size`
///
/// `order_size is quote size for Buy and base_size for SELL
///
/// returns an [`OrderToSend`] struct filled with relevant values. Does not make the actual order.
pub fn create_market_order(
    product_id: &str,
    side: OrderSide,
    order_size: f64,
) -> Result<OrderToSend> {
    let client_order_id = uuid::Uuid::new_v4().to_string();

    let mut base_size = None;
    let mut quote_size = None;

    let order_size = f64_to_valid_bigdecimal(order_size)?;

    match side {
        OrderSide::Buy => quote_size = Some(order_size),
        OrderSide::Sell => base_size = Some(order_size),
        _ => {
            return Err(anyhow!(
                "Orders' side should be Buy or Sell . Got: {:?}",
                side
            ));
        }
    }

    let order = OrderToSend {
        client_order_id,
        product_id: product_id.to_string(),
        side,
        order_configuration: OrderConfiguration {
            market_market_ioc: Some(Market {
                base_size,
                quote_size,
            }),
            limit_limit_gtc: None,
            limit_limit_gtd: None,
            stop_limit_stop_limit_gtc: None,
            stop_limit_stop_limit_gtd: None,
        },
    };
    Ok(order)
}

/// Create a LIMIT Good-Til-Canceled order
///
/// `side` (Buy or Sell) `product_id` for an amount of `base_size` at a price of `limit_price`
///
/// returns an [`OrderToSend`] struct filled with relevant values. Does not make the actual order.
pub fn create_limit_order_good_til_canceled(
    product_id: &str,
    side: OrderSide,
    base_size: f64,
    limit_price: f64,
    post_only: bool,
) -> Result<OrderToSend> {
    let client_order_id = uuid::Uuid::new_v4().to_string();
    anyhow::ensure!(
        side == OrderSide::Buy || side == OrderSide::Sell,
        "Orders' side should be Buy or Sell . Got: {:?}",
        side
    );
    let base_size = f64_to_valid_bigdecimal(base_size)?;
    let limit_price = f64_to_valid_bigdecimal(limit_price)?;

    let order = OrderToSend {
        client_order_id,
        product_id: product_id.to_string(),
        side,
        order_configuration: OrderConfiguration {
            market_market_ioc: None,
            limit_limit_gtc: Some(Limit {
                base_size,
                limit_price,
                end_time: None,
                post_only: Some(post_only),
            }),
            limit_limit_gtd: None,
            stop_limit_stop_limit_gtc: None,
            stop_limit_stop_limit_gtd: None,
        },
    };
    Ok(order)
}

/// Create a LIMIT Good-Til-Date order
///
/// `side` (Buy or Sell) `product_id` for an amount of `base_size` at a price of `limit_price`
///
/// returns an [`OrderToSend`] struct filled with relevant values. Does not make the actual order.
pub fn create_limit_order_good_til_date(
    product_id: &str,
    side: OrderSide,
    base_size: f64,
    limit_price: f64,
    end_time: DateTime,
    post_only: bool,
) -> Result<OrderToSend> {
    let client_order_id = uuid::Uuid::new_v4().to_string();
    anyhow::ensure!(
        side == OrderSide::Buy || side == OrderSide::Sell,
        "Orders' side should be Buy or Sell . Got: {:?}",
        side
    );
    let size = f64_to_valid_bigdecimal(base_size)?;
    let price = f64_to_valid_bigdecimal(limit_price)?;

    let order = OrderToSend {
        client_order_id,
        product_id: product_id.to_string(),
        side,
        order_configuration: OrderConfiguration {
            market_market_ioc: None,
            limit_limit_gtc: None,
            limit_limit_gtd: Some(Limit {
                base_size: size,
                limit_price: price,
                end_time: Some(end_time),
                post_only: Some(post_only),
            }),
            stop_limit_stop_limit_gtc: None,
            stop_limit_stop_limit_gtd: None,
        },
    };
    Ok(order)
}

/// Create a STOP-LIMIT Good-Til-Canceled order
///
/// `side` (Buy or Sell) `product_id` for an amount of `base_size` at a price of `limit_price`
///
/// returns an [`OrderToSend`] struct filled with relevant values. Does not make the actual order.
pub fn create_stop_limit_order_good_til_canceled(
    product_id: &str,
    side: OrderSide,
    base_size: f64,
    limit_price: f64,
    stop_price: f64,
    stop_direction: StopDirection,
) -> Result<OrderToSend> {
    let client_order_id = uuid::Uuid::new_v4().to_string();
    anyhow::ensure!(
        side == OrderSide::Buy || side == OrderSide::Sell,
        "Orders' side should be Buy or Sell . Got: {:?}",
        side
    );
    let base_size = f64_to_valid_bigdecimal(base_size)?;
    let limit_price = f64_to_valid_bigdecimal(limit_price)?;
    let stop_price = f64_to_valid_bigdecimal(stop_price)?;

    let order = OrderToSend {
        client_order_id,
        product_id: product_id.to_string(),
        side,
        order_configuration: OrderConfiguration {
            market_market_ioc: None,
            limit_limit_gtc: None,
            limit_limit_gtd: None,
            stop_limit_stop_limit_gtc: Some(StopLimit {
                base_size,
                limit_price,
                stop_price,
                end_time: None,
                stop_direction,
            }),
            stop_limit_stop_limit_gtd: None,
        },
    };
    Ok(order)
}

/// Create a STOP-LIMIT Good-Til-Date order
///
/// `side` (Buy or Sell) `product_id` for an amount of `base_size` at a price of `limit_price`
///
/// returns an [`OrderToSend`] struct filled with relevant values. Does not make the actual order.
pub fn create_stop_limit_order_good_til_date(
    product_id: &str,
    side: OrderSide,
    base_size: f64,
    limit_price: f64,
    stop_price: f64,
    end_time: DateTime,
    stop_direction: StopDirection,
) -> Result<OrderToSend> {
    let client_order_id = uuid::Uuid::new_v4().to_string();
    anyhow::ensure!(
        side == OrderSide::Buy || side == OrderSide::Sell,
        "Orders' side should be Buy or Sell . Got: {:?}",
        side
    );
    let base_size = f64_to_valid_bigdecimal(base_size)?;
    let limit_price = f64_to_valid_bigdecimal(limit_price)?;
    let stop_price = f64_to_valid_bigdecimal(stop_price)?;

    let order = OrderToSend {
        client_order_id,
        product_id: product_id.to_string(),
        side,
        order_configuration: OrderConfiguration {
            market_market_ioc: None,
            limit_limit_gtc: None,
            limit_limit_gtd: None,
            stop_limit_stop_limit_gtc: None,
            stop_limit_stop_limit_gtd: Some(StopLimit {
                base_size,
                limit_price,
                stop_price,
                end_time: Some(end_time),
                stop_direction,
            }),
        },
    };
    Ok(order)
}

/// Converting a f64 to a Result<BigDecimal> instead of an Option<BigDecimal>
///
/// Useful for instance when creating an order and failure is preferred to a non-relevant value.
fn f64_to_valid_bigdecimal(x: f64) -> Result<BigDecimal> {
    FromPrimitive::from_f64(x).ok_or(anyhow!("Could not convert {} to BigDecimal", x))
}

//=========== TESTS ===========================================================

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_order_deserialize() {
        let input = r##"{
            "order": {
                "order_id": "0000-000000-000000",
                "product_id": "BTC-USD",
                "user_id": "2222-000000-000000",
                "order_configuration": {
                    "market_market_ioc": {
                        "quote_size": "10.00",
                        "base_size": "0.001"
                    },
                    "limit_limit_gtc": {
                        "base_size": "0.001",
                        "limit_price": "10000.00",
                        "post_only": false
                    },
                    "limit_limit_gtd": {
                        "base_size": "0.001",
                        "limit_price": "10000.00",
                        "end_time": "2021-05-31T09:59:59Z",
                        "post_only": false
                    },
                    "stop_limit_stop_limit_gtc": {
                        "base_size": "0.001",
                        "limit_price": "10000.00",
                        "stop_price": "20000.00",
                        "stop_direction": "UNKNOWN_STOP_DIRECTION"
                    },
                    "stop_limit_stop_limit_gtd": {
                        "base_size": "0.001",
                        "limit_price": "10000.00",
                        "stop_price": "20000.00",
                        "end_time": "2021-05-31T09:59:59Z",
                        "stop_direction": "UNKNOWN_STOP_DIRECTION"
                    }
                },
                "side": "UNKNOWN_ORDER_SIDE",
                "client_order_id": "11111-000000-000000",
                "status": "OPEN",
                "time_in_force": "UNKNOWN_TIME_IN_FORCE",
                "created_time": "2021-05-31T09:59:59Z",
                "completion_percentage": "50",
                "filled_size": "0.001",
                "average_filled_price": "50",
                "fee": "string",
                "number_of_fills": "2",
                "filled_value": "10000",
                "pending_cancel": true,
                "size_in_quote": false,
                "total_fees": "5.00",
                "size_inclusive_of_fees": false,
                "total_value_after_fees": "string",
                "trigger_status": "UNKNOWN_TRIGGER_STATUS",
                "order_type": "UNKNOWN_ORDER_TYPE",
                "reject_reason": "REJECT_REASON_UNSPECIFIED",
                "settled": false,
                "product_type": "SPOT",
                "reject_message": "string",
                "cancel_message": "string",
                "order_placement_source": "RETAIL_ADVANCED",
                "outstanding_hold_amount": "string",
                "is_liquidation": false 
            }
        }"##;
        let result: OrderResponse = serde_json::from_slice(input.as_bytes()).unwrap();
        let order = result.order;
        assert_eq!(order.product_id, "BTC-USD".to_string());
        assert!(!order
            .order_configuration
            .limit_limit_gtc
            .unwrap()
            .post_only
            .unwrap());
    }

    #[test]
    fn test_stop_direction_deserialize() {
        let input = r##""UNKNOWN_STOP_DIRECTION""##;
        let result: StopDirection = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, StopDirection::UnknownStopDirection);

        let input = r##""STOP_DIRECTION_STOP_UP""##;
        let result: StopDirection = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, StopDirection::StopDirectionStopUp);

        let input = r##""STOP_DIRECTION_STOP_DOWN""##;
        let result: StopDirection = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, StopDirection::StopDirectionStopDown);
    }

    #[test]
    fn test_stop_direction_serialize() {
        let expected = r##""STOP_DIRECTION_STOP_DOWN""##;
        assert_eq!(
            expected,
            serde_json::to_string(&StopDirection::StopDirectionStopDown).unwrap()
        );
    }

    #[test]
    fn test_status_deserialize() {
        let input = r##""OPEN""##;
        let result: Status = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Status::Open);

        let input = r##""FILLED""##;
        let result: Status = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Status::Filled);

        let input = r##""CANCELLED""##;
        let result: Status = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Status::Cancelled);

        let input = r##""EXPIRED""##;
        let result: Status = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Status::Expired);

        let input = r##""FAILED""##;
        let result: Status = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Status::Failed);

        let input = r##""UNKNOWN_ORDER_STATUS""##;
        let result: Status = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, Status::UnknownOrderStatus);
    }

    #[test]
    fn test_status_serialize() {
        let expected = r##""FILLED""##;
        assert_eq!(expected, serde_json::to_string(&Status::Filled).unwrap());
    }

    #[test]
    fn test_time_in_force_deserialize() {
        let input = r##""UNKNOWN_TIME_IN_FORCE""##;
        let result: TimeInForce = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TimeInForce::UnknownTimeInForce);

        let input = r##""GOOD_UNTIL_DATE_TIME""##;
        let result: TimeInForce = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TimeInForce::GoodUntilDateTime);

        let input = r##""GOOD_UNTIL_CANCELLED""##;
        let result: TimeInForce = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TimeInForce::GoodUntilCancelled);

        let input = r##""IMMEDIATE_OR_CANCEL""##;
        let result: TimeInForce = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TimeInForce::ImmediateOrCancel);

        let input = r##""FILL_OR_KILL""##;
        let result: TimeInForce = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TimeInForce::FillOrKill);
    }

    #[test]
    fn test_time_in_force_serialize() {
        let expected = r##""GOOD_UNTIL_CANCELLED""##;
        assert_eq!(
            expected,
            serde_json::to_string(&TimeInForce::GoodUntilCancelled).unwrap()
        );
    }

    #[test]
    fn test_trigger_status_deserialize() {
        let input = r##""UNKNOWN_TRIGGER_STATUS""##;
        let result: TriggerStatus = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TriggerStatus::UnknownTriggerStatus);

        let input = r##""INVALID_ORDER_TYPE""##;
        let result: TriggerStatus = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TriggerStatus::InvalidOrderType);

        let input = r##""STOP_PENDING""##;
        let result: TriggerStatus = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TriggerStatus::StopPending);

        let input = r##""STOP_TRIGGERED""##;
        let result: TriggerStatus = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, TriggerStatus::StopTriggered);
    }

    #[test]
    fn test_trigger_status_serialize() {
        let expected = r##""INVALID_ORDER_TYPE""##;
        assert_eq!(
            expected,
            serde_json::to_string(&TriggerStatus::InvalidOrderType).unwrap()
        );
    }

    #[test]
    fn test_order_type_deserialize() {
        let input = r##""UNKNOWN_ORDER_TYPE""##;
        let result: OrderType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, OrderType::UnknownOrderType);

        let input = r##""MARKET""##;
        let result: OrderType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, OrderType::Market);

        let input = r##""LIMIT""##;
        let result: OrderType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, OrderType::Limit);

        let input = r##""STOP""##;
        let result: OrderType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, OrderType::Stop);

        let input = r##""STOP_LIMIT""##;
        let result: OrderType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, OrderType::StopLimitOrderType);
    }

    #[test]
    fn test_order_type_serialize() {
        let expected = r##""MARKET""##;
        assert_eq!(expected, serde_json::to_string(&OrderType::Market).unwrap());
    }

    #[test]
    fn test_reject_reason_deserialize() {
        let input = r##""REJECT_REASON_UNSPECIFIED""##;
        let result: RejectReason = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, RejectReason::RejectReasonUnspecified);
    }

    #[test]
    fn test_reject_reason_serialize() {
        let expected = r##""REJECT_REASON_UNSPECIFIED""##;
        assert_eq!(
            expected,
            serde_json::to_string(&RejectReason::RejectReasonUnspecified).unwrap()
        );
    }

    #[test]
    fn test_order_placement_source_deserialize() {
        let input = r##""RETAIL_SIMPLE""##;
        let result: OrderPlacementSource = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, OrderPlacementSource::RetailSimple);

        let input = r##""RETAIL_ADVANCED""##;
        let result: OrderPlacementSource = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, OrderPlacementSource::RetailAdvanced);
    }

    #[test]
    fn test_order_placement_source_serialize() {
        let expected = r##""RETAIL_SIMPLE""##;
        assert_eq!(
            expected,
            serde_json::to_string(&OrderPlacementSource::RetailSimple).unwrap()
        );
    }

    #[test]
    fn test_create_market_order_serialize() {
        let product_id = "BTC-USD";
        let side = OrderSide::Buy;
        let order_size = 0.00001;
        let order = create_market_order(product_id, side, order_size).unwrap();
        let json = serde_json::to_string(&order);
        assert!(json.is_ok());
    }

    #[test]
    fn test_create_limit_order_good_til_canceled_serialize() {
        let product_id = "BTC-USD";
        let side = OrderSide::Buy;
        let base_size = 0.00001;
        let limit_price = 5000.0;
        let post_only = false;
        let order = create_limit_order_good_til_canceled(
            product_id,
            side,
            base_size,
            limit_price,
            post_only,
        )
        .unwrap();
        let json = serde_json::to_string(&order);
        assert!(json.is_ok());
    }

    #[test]
    fn test_create_limit_order_good_til_date_serialize() {
        let product_id = "BTC-USD";
        let side = OrderSide::Buy;
        let base_size = 0.00001;
        let limit_price = 5000.0;
        let end_time = chrono::offset::Utc::now(); // good enough for serde test
        let post_only = false;
        let order = create_limit_order_good_til_date(
            product_id,
            side,
            base_size,
            limit_price,
            end_time,
            post_only,
        )
        .unwrap();
        let json = serde_json::to_string(&order);
        assert!(json.is_ok());
    }

    #[test]
    fn test_create_stop_limit_order_good_til_canceled_serialize() {
        let product_id = "BTC-USD";
        let side = OrderSide::Buy;
        let base_size = 0.00001;
        let limit_price = 5000.0;
        let stop_price = 4000.0;
        let stop_direction = StopDirection::StopDirectionStopUp;
        let order = create_stop_limit_order_good_til_canceled(
            product_id,
            side,
            base_size,
            limit_price,
            stop_price,
            stop_direction,
        )
        .unwrap();
        let json = serde_json::to_string(&order);
        assert!(json.is_ok());
    }

    #[test]
    fn test_create_stop_limit_order_good_til_date_serialize() {
        let product_id = "BTC-USD";
        let side = OrderSide::Buy;
        let base_size = 0.00001;
        let limit_price = 5000.0;
        let stop_price = 4000.0;
        let end_time = chrono::offset::Utc::now(); // good enough for serde test
        let stop_direction = StopDirection::StopDirectionStopUp;
        let order = create_stop_limit_order_good_til_date(
            product_id,
            side,
            base_size,
            limit_price,
            stop_price,
            end_time,
            stop_direction,
        )
        .unwrap();
        let json = serde_json::to_string(&order);
        assert!(json.is_ok());
    }

    #[test]
    fn test_order_response_serde() {
        let input = r##"{
          "success": true,
          "failure_reason": "INVALID_SIDE",
          "order_id": "string",
          "success_response": {
            "order_id": "11111-00000-000000",
            "product_id": "BTC-USD",
            "side": "UNKNOWN_ORDER_SIDE",
            "client_order_id": "0000-00000-000000"
          },
          "error_response": {
            "error": "UNKNOWN_FAILURE_REASON",
            "message": "The order configuration was invalid",
            "error_details": "Market orders cannot be placed with empty order sizes",
            "preview_failure_reason": "UNKNOWN_PREVIEW_FAILURE_REASON",
            "new_order_failure_reason": "UNKNOWN_FAILURE_REASON"
          },
          "order_configuration": {
            "market_market_ioc": {
              "quote_size": "10.00",
              "base_size": "0.001"
            },
            "limit_limit_gtc": {
              "base_size": "0.001",
              "limit_price": "10000.00",
              "post_only": false
            },
            "limit_limit_gtd": {
              "base_size": "0.001",
              "limit_price": "10000.00",
              "end_time": "2021-05-31T09:59:59Z",
              "post_only": false
            },
            "stop_limit_stop_limit_gtc": {
              "base_size": "0.001",
              "limit_price": "10000.00",
              "stop_price": "20000.00",
              "stop_direction": "UNKNOWN_STOP_DIRECTION"
            },
            "stop_limit_stop_limit_gtd": {
              "base_size": 0.001,
              "limit_price": "10000.00",
              "stop_price": "20000.00",
              "end_time": "2021-05-31T09:59:59Z",
              "stop_direction": "UNKNOWN_STOP_DIRECTION"
            }
          }
        }"##;
        let result: CreateOrderResponse = serde_json::from_slice(input.as_bytes()).unwrap();
        assert!(result.success);
        assert!(!result
            .order_configuration
            .limit_limit_gtc
            .unwrap()
            .post_only
            .unwrap());
    }

    #[test]
    fn test_order_response_success_serde() {
        let input = r##"{
            "success": true,
            "failure_reason": "UNKNOWN_FAILURE_REASON",
            "order_id": "11111111-4c82-40e2-980a-222222222222",
            "success_response": {
                "order_id": "11111111-4c82-40e2-980a-222222222222",
                "product_id": "BTC-USDT",
                "side": "BUY",
                "client_order_id": "33333333-74f5-4508-8b9a-222222222222"
            },
            "order_configuration": {
                "limit_limit_gtd": {
                    "base_size": "1.000000000000000",
                    "limit_price": "0.01000000000000000",
                    "end_time": "2023-08-17T04:59:45.166512756Z",
                    "post_only": false
                }
            }
        }"##;
        let result: CreateOrderResponse = serde_json::from_slice(input.as_bytes()).unwrap();
        assert!(result.success);
        assert!(!result
            .order_configuration
            .limit_limit_gtd
            .unwrap()
            .post_only
            .unwrap());
    }

    #[test]
    fn test_cancel_orders_response_serde() {
        let input = r##"{
            "results": [
                {
                    "success":false, 
                    "failure_reason": "UNKNOWN_CANCEL_ORDER", 
                    "order_id": "foo"
                }
            ]
        }"##;
        let results: CancelOrdersResponse = serde_json::from_slice(input.as_bytes()).unwrap();
        let result = &results.results[0];
        assert!(!result.success);
    }
}
