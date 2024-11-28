use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use crate::structs::{TradeResponse, TickerResponse};

pub async fn connect_to_binance(symbol: &str) {
    let url = format!("wss://stream.binance.com:9443/ws/{}@trade", symbol.to_lowercase());

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to WebSocket!");

    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            let trade: TradeResponse = serde_json::from_str(&text).expect("Error deserializando trade");
            println!("Nuevo Trade: {:#?}", trade);
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

}

#[allow(dead_code)]
pub async fn connect_to_ticker(symbol: &str) {
    let url = format!("wss://stream.binance.com:9443/ws/{}@ticker", symbol.to_lowercase());

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to WebSocket!");

    let (mut write, mut read) = ws_stream.split();

    // Esperamos mensajes de ticker
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            let ticker: TickerResponse = serde_json::from_str(&text).expect("Error deserializando ticker");
            println!("Ticker: {:#?}", ticker);
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
