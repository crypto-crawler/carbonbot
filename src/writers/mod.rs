pub(super) mod file_writer;

use crypto_crawler::*;
use log::*;
use redis::{self, Commands};
use std::{
    collections::HashMap,
    path::Path,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
    time::{Duration, Instant},
};

pub trait Writer {
    fn write(&mut self, s: &str);
    fn close(&mut self);
}

pub use file_writer::FileWriter;

// If there are no messages coming in for `timeout` seconds, exit the process.
fn create_file_writer_thread(
    rx: Receiver<Message>,
    data_dir: String,
    tx_redis: Option<Sender<Message>>,
    timeout_secs: u64,
) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let timeout = Duration::from_secs(timeout_secs);
        let mut writers: HashMap<String, FileWriter> = HashMap::new();

        let start_time = Instant::now();
        while let Ok(msg) = rx.recv_timeout(timeout) {
            let file_name = format!("{}.{}.{}", msg.exchange, msg.market_type, msg.msg_type);
            if !writers.contains_key(&file_name) {
                let data_dir = Path::new(&data_dir)
                    .join(msg.msg_type.to_string())
                    .join(&msg.exchange)
                    .join(msg.market_type.to_string())
                    .into_os_string();
                std::fs::create_dir_all(data_dir.as_os_str()).unwrap();
                let file_path = Path::new(data_dir.as_os_str())
                    .join(file_name.clone())
                    .into_os_string();
                writers.insert(
                    file_name.clone(),
                    FileWriter::new(file_path.as_os_str().to_str().unwrap()),
                );
            }

            if let Some(writer) = writers.get_mut(&file_name) {
                // JSON, serde_json::to_string(&msg); CSV, msg.to_tsv_string()
                let s = serde_json::to_string(&msg).unwrap();
                writer.write(&s);
            }
            // copy to redis
            if let Some(ref tx_redis) = tx_redis {
                tx_redis.send(msg).unwrap();
            }
        }

        for mut writer in writers {
            writer.1.close();
        }

        let elapsed = start_time.elapsed().as_secs();
        error!(
            "This crawler has been running stably for {} seconds, until there are no messages for {} seconds, exiting now",
            elapsed, timeout_secs
        );
        std::process::exit(1); // pm2 will restart this process
    })
}

fn connect_redis(redis_url: &str) -> Result<redis::Connection, redis::RedisError> {
    assert!(!redis_url.is_empty(), "redis_url is empty");

    let mut redis_error: Option<redis::RedisError> = None;
    let mut conn: Option<redis::Connection> = None;
    for _ in 0..3 {
        match redis::Client::open(redis_url) {
            Ok(client) => match client.get_connection() {
                Ok(connection) => {
                    conn = Some(connection);
                    break;
                }
                Err(err) => redis_error = Some(err),
            },
            Err(err) => redis_error = Some(err),
        }
    }

    if let Some(connection) = conn {
        Ok(connection)
    } else {
        Err(redis_error.unwrap())
    }
}

fn create_redis_writer_thread(rx: Receiver<Message>, redis_url: String) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let mut redis_conn = connect_redis(&redis_url).unwrap();
        for msg in rx {
            let msg_type = msg.msg_type;
            let s = serde_json::to_string(&msg).unwrap();
            let topic = format!("carbonbot:{}", msg_type);
            if let Err(err) = redis_conn.publish::<&str, String, i64>(&topic, s) {
                error!("{}", err);
                return;
            }
        }
    })
}

#[allow(clippy::unnecessary_unwrap)]
pub fn create_writer_threads(
    rx: Receiver<Message>,
    data_dir: Option<String>,
    redis_url: Option<String>,
    timeout_secs: u64,
) -> Vec<JoinHandle<()>> {
    let mut threads = Vec::new();
    if data_dir.is_none() && redis_url.is_none() {
        error!("Both DATA_DIR and REDIS_URL are not set");
        return threads;
    }

    if data_dir.is_some() && redis_url.is_some() {
        // channel for Redis
        let (tx_redis, rx_redis) = std::sync::mpsc::channel::<Message>();
        threads.push(create_file_writer_thread(
            rx,
            data_dir.unwrap(),
            Some(tx_redis),
            timeout_secs,
        ));
        threads.push(create_redis_writer_thread(rx_redis, redis_url.unwrap()));
    } else if data_dir.is_some() {
        threads.push(create_file_writer_thread(
            rx,
            data_dir.unwrap(),
            None,
            timeout_secs,
        ))
    } else {
        threads.push(create_redis_writer_thread(rx, redis_url.unwrap()));
    }
    threads
}
