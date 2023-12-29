use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SymbolInfo {
    symbol: String,
    bidPrice: String,
}

#[derive(Debug, Deserialize)]
struct ExchangeInfo {
    symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Deserialize)]
struct NewOrderResponse {
    symbol: String,
    orderId: u64,
    origQty: String,
    executedQty: String,
}
