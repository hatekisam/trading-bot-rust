use serde::Deserialize;

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


