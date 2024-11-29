use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use crate::structs::{TradeResponse, TickerResponse};
use std::thread::sleep;

#[allow(dead_code)]
pub async fn connect_to_binance(symbol: &str) -> Option<TradeResponse> {
    let url = format!("wss://stream.binance.com:9443/ws/{}@trade", symbol.to_lowercase());

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            let trade: TradeResponse = serde_json::from_str(&text).expect("Error deserializando trade");
            return Some(trade);
        }
        sleep(std::time::Duration::from_secs(2));
    }

    None
}

#[allow(dead_code)]
pub async fn connect_to_ticker(symbol: &str) -> Option<TickerResponse> {
    let url = format!("wss://stream.binance.com:9443/ws/{}@ticker", symbol.to_lowercase());

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (_, mut read) = ws_stream.split();

    if let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            let ticker: TickerResponse = serde_json::from_str(&text).expect("Error deserializando ticker");
            return Some(ticker);
        }
    }

    None
}

