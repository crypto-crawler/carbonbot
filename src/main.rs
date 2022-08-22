use carbonbot::{crawl_other, create_writer_threads};
use crypto_crawler::*;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use log::*;
use std::{env, str::FromStr};

fn get_message_gap(exchange: &'static str, market_type: MarketType, msg_type: MessageType) -> u64 {
    match msg_type {
        MessageType::Trade | MessageType::Ticker => match market_type {
            MarketType::Spot
            | MarketType::InverseFuture
            | MarketType::InverseSwap
            | MarketType::LinearFuture
            | MarketType::LinearSwap => 30,
            _ => 300, // 5 minutes
        },
        MessageType::L2Event | MessageType::L3Event | MessageType::L2TopK | MessageType::BBO => {
            match market_type {
                MarketType::Spot
                | MarketType::InverseFuture
                | MarketType::InverseSwap
                | MarketType::LinearFuture
                | MarketType::LinearSwap => 5,
                _ => 300, // 5 minutes
            }
        }
        MessageType::FundingRate => {
            match exchange {
                "bitmex" => 3600 * 8, // Sent every funding interval (usually 8hrs), see https://www.bitmex.com/app/wsAPI
                "huobi" => 60, // Funding rate will be pushed every 60 seconds by default, see https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#unsubscribe-funding-rate-data-no-authentication-unsub
                "okx" => 90, // Data will be pushed in 30s to 90s, see https://www.okx.com/docs-v5/en/#websocket-api-public-channel-funding-rate-channel
                _ => 3600,
            }
        }
        MessageType::Candlestick => 60,
        _ => 300, // 5 minutes
    }
}

pub async fn crawl(
    exchange: &'static str,
    market_type: MarketType,
    msg_type: MessageType,
    data_dir: Option<String>,
    redis_url: Option<String>,
    symbols: Option<&[String]>,
) {
    if data_dir.is_none() && redis_url.is_none() {
        error!("Both DATA_DIR and REDIS_URL are not set");
        return;
    }
    let (tx, rx) = std::sync::mpsc::channel::<Message>();
    let timeout_secs = get_message_gap(exchange, market_type, msg_type);
    let writer_threads = create_writer_threads(rx, data_dir, redis_url, timeout_secs);

    if msg_type == MessageType::Candlestick {
        crawl_candlestick(exchange, market_type, None, tx).await;
    } else if msg_type == MessageType::OpenInterest {
        tokio::task::spawn_blocking(move || crawl_open_interest(exchange, market_type, tx));
    } else if msg_type == MessageType::Other {
        crawl_other(exchange, market_type, tx).await;
    } else {
        match msg_type {
            MessageType::BBO => {
                crawl_bbo(exchange, market_type, symbols, tx).await;
            }
            MessageType::Trade => {
                crawl_trade(exchange, market_type, symbols, tx).await;
            }
            MessageType::L2Event => {
                crawl_l2_event(exchange, market_type, symbols, tx).await;
            }
            MessageType::L3Event => {
                crawl_l3_event(exchange, market_type, symbols, tx).await;
            }
            MessageType::L2Snapshot => {
                let symbols = if let Some(symbols) = symbols {
                    symbols.to_vec()
                } else {
                    vec![]
                };
                tokio::task::spawn_blocking(move || {
                    let symbols_local = symbols;
                    crawl_l2_snapshot(exchange, market_type, Some(&symbols_local), tx)
                });
            }
            MessageType::L2TopK => {
                crawl_l2_topk(exchange, market_type, symbols, tx).await;
            }
            MessageType::L3Snapshot => {
                let symbols = if let Some(symbols) = symbols {
                    symbols.to_vec()
                } else {
                    vec![]
                };
                tokio::task::spawn_blocking(move || {
                    let symbols_local = symbols;
                    crawl_l3_snapshot(exchange, market_type, Some(&symbols_local), tx)
                });
            }
            MessageType::Ticker => {
                crawl_ticker(exchange, market_type, symbols, tx).await;
            }
            MessageType::FundingRate => {
                crawl_funding_rate(exchange, market_type, symbols, tx).await
            }
            _ => panic!("Not implemented"),
        };
    }
    for thread in writer_threads {
        thread.join().unwrap();
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 && args.len() != 5 {
        println!("Usage: carbonbot <exchange> <market_type> <msg_type> [comma_seperated_symbols]");
        return;
    }

    let exchange: &'static str = Box::leak(args[1].clone().into_boxed_str());

    let market_type_str = args[2].as_str();
    let market_type = MarketType::from_str(market_type_str);
    if market_type.is_err() {
        println!("Unknown market type: {}", market_type_str);
        return;
    }
    let market_type = market_type.unwrap();

    let msg_type_str = args[3].as_str();
    let msg_type = MessageType::from_str(msg_type_str);
    if msg_type.is_err() {
        println!("Unknown msg type: {}", msg_type_str);
        return;
    }
    let msg_type = msg_type.unwrap();

    let data_dir = if std::env::var("DATA_DIR").is_err() {
        info!("The DATA_DIR environment variable does not exist");
        None
    } else {
        let url = std::env::var("DATA_DIR").unwrap();
        Some(url)
    };

    let redis_url = if std::env::var("REDIS_URL").is_err() {
        info!("The REDIS_URL environment variable does not exist");
        None
    } else {
        let url = std::env::var("REDIS_URL").unwrap();
        Some(url)
    };

    let specified_symbols = if args.len() == 4 {
        Vec::new()
    } else {
        let mut symbols = fetch_symbols_retry(exchange, market_type);
        symbols.retain(|symbol| args[4].split(',').any(|part| symbol.contains(part)));
        info!("target symbols: {:?}", symbols);
        symbols
    };

    if data_dir.is_none() && redis_url.is_none() {
        panic!("The environment variable DATA_DIR and REDIS_URL are not set, at least one of them should be set");
    }

    let pid = std::process::id();
    // write pid to file
    {
        let mut dir = std::env::temp_dir()
            .join("carbonbot-pids")
            .join(msg_type_str);
        let _ = std::fs::create_dir_all(&dir);
        dir.push(format!("{}.{}.{}", exchange, market_type_str, msg_type_str));
        std::fs::write(dir.as_path(), pid.to_string()).expect("Unable to write pid to file");
    }
    crawl(
        exchange,
        market_type,
        msg_type,
        data_dir,
        redis_url,
        if specified_symbols.is_empty() {
            None
        } else {
            Some(&specified_symbols)
        },
    )
    .await;
}
