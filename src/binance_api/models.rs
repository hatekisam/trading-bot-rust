use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SymbolInfo {
    pub symbol: String,
    pub bidPrice: String,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfo {
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Deserialize)]
pub struct NewOrderResponse {
    pub symbol: String,
    pub orderId: u64,
    pub origQty: String,
    pub executedQty: String,
}


