use trading_bot::TradingBot;

#[tokio::main]
async fn main() {
    let api_key = "your_api_key";
    let api_secret = "your_api_secret";

    let trading_bot = TradingBot::new(api_key, api_secret);

    // Example: X amount
    let amount_to_trade = 50.0;

    trading_bot.execute_trade(amount_to_trade).await;
}
