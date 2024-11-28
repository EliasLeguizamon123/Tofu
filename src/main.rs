// src/main.rs

mod structs;
mod webhook;

use tokio;

#[tokio::main]
async fn main() {
    // Suscripción a BTC/USDT y ETH/USDT
    let btc_symbol = "btcusdt";
    let xrp_symbol = "xrpusdt";

    // Lanza las tareas de WebSocket
    tokio::spawn(async move {
        webhook::connect_to_binance(btc_symbol).await;
    });

    tokio::spawn(async move {
        webhook::connect_to_binance(xrp_symbol).await;
    });

    // Mantén el proceso activo indefinidamente
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
