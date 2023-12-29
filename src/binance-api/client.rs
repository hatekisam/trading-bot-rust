use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SymbolInfo {
    symbol: String,
    bidPrice: String,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfo {
    symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Deserialize)]
pub struct NewOrderResponse {
    symbol: String,
    orderId: u64,
    origQty: String,
    executedQty: String,
}

pub struct BinanceApiClient {
    api_key: String,
    api_secret: String,
}

impl BinanceApiClient {
    async fn get_spot_best_bid(api_key: &str, api_secret: &str, symbol: &str) -> Result<String, reqwest::Error> {
        let url = format!("https://api.binance.com/api/v3/ticker/bookTicker?symbol={}", symbol);
    
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
    
    async fn place_spot_limit_order(api_key: &str, api_secret: &str, symbol: &str, qty: &str, price: &str) -> Result<NewOrderResponse, reqwest::Error> {
        let url = "https://api.binance.com/api/v3/order";
        let params: HashMap<&str, &str> = [
            ("symbol", symbol),
            ("side", "BUY"),
            ("type", "LIMIT"),
            ("timeInForce", "GTC"),
            ("quantity", qty),
            ("price", price),
            ("timestamp", &chrono::Utc::now().timestamp_millis().to_string()),
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

    async fn place_futures_market_order(api_key: &str, api_secret: &str, symbol: &str, qty: &str) -> Result<NewOrderResponse, reqwest::Error> {
        let url = "https://fapi.binance.com/fapi/v1/order";
        let params: HashMap<&str, &str> = [
            ("symbol", symbol),
            ("side", "SELL"),
            ("type", "MARKET"),
            ("quantity", qty),
            ("timestamp", &chrono::Utc::now().timestamp_millis().to_string()),
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
