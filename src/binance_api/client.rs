use anyhow::{anyhow, Result}; // Import anyhow Result
use reqwest;
use std::collections::HashMap;

use chrono::Utc;

use super::models::{ExchangeInfo, NewOrderResponse}; // Import NewOrderResponse

#[derive(Debug)]
pub struct BinanceApiClient {
    pub api_key: String,
    pub api_secret: String,
}

impl BinanceApiClient {
    pub async fn get_spot_best_bid(&self, symbol: &str) -> Result<String> {
        let url = format!(
            "https://api.binance.com/api/v3/ticker/bookTicker?symbol={}",
            symbol
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        let body = response.text().await?;
        let exchange_info: ExchangeInfo = serde_json::from_str(&body)?;

        match exchange_info.symbols.iter().find(|s| s.symbol == symbol) {
            Some(symbol_info) => Ok(symbol_info.bidPrice.clone()),
            None => Err(anyhow!("Symbol not found")),
        }
    }

    pub async fn place_spot_limit_order(
        &self,
        symbol: &str,
        qty: &str,
        price: &str,
    ) -> Result<NewOrderResponse> {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let url = "https://api.binance.com/api/v3/order";
        let params: HashMap<&str, &str> = [
            ("symbol", symbol),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", qty),
            ("price", price),
            ("timestamp", &timestamp),
        ]
        .iter()
        .cloned()
        .collect();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .form(&params)
            .send()
            .await?;

        let body = response.text().await?;
        let order_response: NewOrderResponse = serde_json::from_str(&body)?;

        Ok(order_response)
    }

    pub async fn place_futures_market_order(
        &self,
        symbol: &str,
        qty: &str,
    ) -> Result<NewOrderResponse> {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let url = "https://fapi.binance.com/fapi/v1/order";
        let params: HashMap<&str, &str> = [
            ("symbol", symbol),
            ("side", "SELL"),
            ("type", "MARKET"),
            ("quantity", qty),
            ("timestamp", &timestamp),
        ]
        .iter()
        .cloned()
        .collect();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .form(&params)
            .send()
            .await?;

        let body = response.text().await?;
        let order_response: NewOrderResponse = serde_json::from_str(&body)?;

        Ok(order_response)
    }
}
