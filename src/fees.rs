//! Structures & Enums representing Coinbase's fee structures

use bigdecimal::BigDecimal;
use serde_derive::Deserialize;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

/// Structure representing Coinbase's fee tier
#[derive(Deserialize, Debug)]
pub struct FeeTier {
    /// Pricing tier for user, determined by notional (USD) volume.
    /// usd_from, usd_to uses comma to separate thousands -- keep as String; serde to BiDecimal will
    /// fail
    pub pricing_tier: String,
    /// Lower bound (inclusive) of pricing tier in notional volume.
    pub usd_from: String,
    /// Upper bound (exclusive) of pricing tier in notional volume.
    pub usd_to: String,
    /// Taker fee rate, applied if the order takes liquidity.
    pub taker_fee_rate: BigDecimal,
    /// Maker fee rate, applied if the order creates liquidity.
    pub maker_fee_rate: BigDecimal,
}

/// Structure representing Coinbase's margin rate.
#[derive(Deserialize, Debug)]
pub struct MarginRate {
    /// String representation allows for unlimited precision.
    pub value: String,
}

/// Enum representing the possible types of goods and service tax
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GoodsAndServicesTaxType {
    Inclusive,
    Exclusive,
}

/// Structure representing Coinbase's good and services tax structure.
#[derive(Deserialize, Debug)]
pub struct GoodsAndServicesTax {
    pub rate: String,
    pub r#type: GoodsAndServicesTaxType,
}

/// Structure representing Coinbase's transaction summary, that is the fees according to the fee tier
#[derive(Deserialize, Debug)]
pub struct TransactionsSummary {
    /// Total volume across assets, denoted in USD.
    pub total_volume: f64,
    /// Total fees across assets, denoted in USD.
    pub total_fees: f64,
    pub fee_tier: FeeTier,
    pub margin_rate: Option<MarginRate>,
    pub goods_and_services_tax: Option<GoodsAndServicesTax>,
    /// Advanced Trade volume (non-inclusive of Pro) across assets, denoted in USD.
    pub advanced_trade_only_volume: f64,
    /// Advanced Trade fees (non-inclusive of Pro) across assets, denoted in USD.
    pub advanced_trade_only_fees: f64,
    /// Coinbase Pro volume across assets, denoted in USD.
    pub coinbase_pro_volume: f64,
    /// Coinbase Pro fees across assets, denoted in USD.
    pub coinbase_pro_fees: f64,
}

//=========== TESTS ===========================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goods_and_services_tax_type_deserialize() {
        let input = r##""INCLUSIVE""##;
        let result: GoodsAndServicesTaxType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, GoodsAndServicesTaxType::Inclusive);

        let input = r##""EXCLUSIVE""##;
        let result: GoodsAndServicesTaxType = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result, GoodsAndServicesTaxType::Exclusive);
    }

    #[test]
    fn test_goods_and_services_tax_type_serialize() {
        let expected = r##""INCLUSIVE""##;
        assert_eq!(
            expected,
            serde_json::to_string(&GoodsAndServicesTaxType::Inclusive).unwrap()
        );
    }

    #[test]
    fn test_transaction_summary_deserialize() {
        let input = r##"{
            "total_volume": 1000,
            "total_fees": 25,
            "fee_tier": {
                "pricing_tier": "<$10k",
                "usd_from": "0",
                "usd_to": "10,000",
                "taker_fee_rate": "0.0010",
                "maker_fee_rate": "0.0020"
            },
            "margin_rate": {
                "value": "string"
            },
            "goods_and_services_tax": {
                "rate": "string",
                "type": "INCLUSIVE"
            },
            "advanced_trade_only_volume": 1000,
            "advanced_trade_only_fees": 25,
            "coinbase_pro_volume": 1000,
            "coinbase_pro_fees": 25
        }"##;
        let result: TransactionsSummary = serde_json::from_slice(input.as_bytes()).unwrap();
        assert_eq!(result.total_volume, 1000.0);
    }
}
