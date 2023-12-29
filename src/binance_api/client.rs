use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

use chrono::Utc;

pub struct BinanceApiClient {
    pub api_key: String,
    pub api_secret: String,
}

impl BinanceApiClient {
    pub async fn get_spot_best_bid(
        api_key: &str,
        api_secret: &str,
        symbol: &str,
    ) -> Result<String, reqwest::Error> {
        let url = format!(
            "https://api.binance.com/api/v3/ticker/bookTicker?symbol={}",
            symbol
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("X-MBX-APIKEY", api_key)
            .send()
            .await?;

        let body = response.text().await?;
        let exchange_info: ExchangeInfo = serde_json::from_str(&body)?;

        if let Some(symbol_info) = exchange_info.symbols.iter().find(|s| s.symbol == symbol) {
            Ok(symbol_info.bidPrice.clone())
        } else {
            Err(reqwest::Error::custom("Symbol not found"))
        }
    }

    pub async fn place_spot_limit_order(
        api_key: &str,
        api_secret: &str,
        symbol: &str,
        qty: &str,
        price: &str,
    ) -> Result<NewOrderResponse, reqwest::Error> {
        let url = "https://api.binance.com/api/v3/order";
        let params: HashMap<&str, &str> = [
            ("symbol", symbol),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", qty),
            ("price", price),
            ("timestamp", &Utc::now().timestamp_millis().to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-MBX-APIKEY", api_key)
            .form(&params)
            .send()
            .await?;

        let body = response.text().await?;
        let order_response: NewOrderResponse = serde_json::from_str(&body)?;

        Ok(order_response)
    }

    pub async fn place_futures_market_order(
        api_key: &str,
        api_secret: &str,
        symbol: &str,
        qty: &str,
    ) -> Result<NewOrderResponse, reqwest::Error> {
        let url = "https://fapi.binance.com/fapi/v1/order";
        let params: HashMap<&str, &str> = [
            ("symbol", symbol),
            ("side", "SELL"),
            ("type", "MARKET"),
            ("quantity", qty),
            ("timestamp", &Utc::now().timestamp_millis().to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-MBX-APIKEY", api_key)
            .form(&params)
            .send()
            .await?;

        let body = response.text().await?;
        let order_response: NewOrderResponse = serde_json::from_str(&body)?;

        Ok(order_response)
    }
}
