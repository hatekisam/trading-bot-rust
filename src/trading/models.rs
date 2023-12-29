

#[derive(Debug)]
pub struct TradeExecution {
    pub symbol: String,
    pub executed_qty: String,
    // Add other relevant fields for trade execution details...
}

#[derive(Debug)]
pub struct TradeSpread {
    pub spread_bps: f64,
    // Add other relevant fields for spread details...
}

