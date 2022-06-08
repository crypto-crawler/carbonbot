use std::sync::mpsc::Sender;

use super::utils::create_conversion_thread;
use crypto_crawler::Message;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use crypto_ws_client::*;

pub(super) async fn crawl_other(tx: Sender<Message>) {
    let tx = create_conversion_thread(
        "okx".to_string(),
        MessageType::Other,
        MarketType::Unknown,
        tx,
    );
    let commands =
        vec![r#"{"op":"subscribe","args":[{"channel":"instruments","instType":"SPOT"},{"channel":"instruments","instType":"MARGIN"},{"channel":"instruments","instType":"SWAP"},{"channel":"instruments","instType":"FUTURES"},{"channel":"instruments","instType":"OPTION"},{"channel":"public-struc-block-trades"},{"channel":"status"},{"channel":"opt-summary","uly":"BTC-USD"},{"channel":"opt-summary","uly":"ETH-USD"},{"channel":"opt-summary","uly":"SOL-USD"}]}"#.to_string()];

    let ws_client = OkxWSClient::new(tx, None).await;
    ws_client.send(&commands).await;
    ws_client.run().await;
    ws_client.close();
}
