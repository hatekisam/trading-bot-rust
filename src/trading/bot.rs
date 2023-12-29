use crate::binance_api::client::BinanceApiClient;

#[derive(Debug)]
pub struct TradingBot {
    api_client: BinanceApiClient,
    spot_symbol: String,
    futures_symbol: String,
    max_order_size: f64,
    min_order_size: f64,
}

impl TradingBot {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        let api_client = BinanceApiClient {
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(),
        };
        let spot_symbol = "BTCUSDT".to_owned();
        let futures_symbol = "BTCUSDT_PERP".to_owned();
        let max_order_size = 100.0;
        let min_order_size = 25.0;

        TradingBot {
            api_client,
            spot_symbol,
            futures_symbol,
            max_order_size,
            min_order_size,
        }
    }

    pub async fn execute_trade(&self, amount_to_trade: f64) {
        // Get the best bid price from the spot market
        let spot_bid_price = self
            .api_client
            .get_spot_best_bid(&self.spot_symbol)
            .await
            .unwrap();

        // Calculate order quantity based on the amount and spot bid price
        let order_qty = (amount_to_trade / spot_bid_price.parse::<f64>().unwrap()).to_string();

        // Place a limit buy order on the spot market
        let spot_order_response = self
            .api_client
            .place_spot_limit_order(&self.spot_symbol, &order_qty, &spot_bid_price)
            .await
            .unwrap();
        println!("Spot Order Response: {:?}", spot_order_response);
        let futures_trade_price = get_latest_futures_trade_price().await.unwrap();
        let futures_order_response = self
            .api_client
            .place_futures_market_order(&self.futures_symbol, &order_qty)
            .await
            .unwrap();
        println!("Futures Order Response: {:?}", futures_order_response);
        // Assuming `avgPrice` is the correct field name in NewOrderResponse
        let spread = spot_bid_price.parse::<f64>().unwrap()
            - futures_order_response.avg_price.parse::<f64>().unwrap();
        println!("Spread: {} bps", spread);
    }
}
