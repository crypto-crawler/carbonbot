# carbonbot

A CLI tool based on the crypto-crawler-rs library to crawl trade, level2, level3, ticker, funding rate, etc.

## Run

To quickly get started, copy `conf/run_crawlers.sh` to somewhere, fill in neccesary parameters and run it.

### Trade

Crawl tick-by-tick trades:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.trade.config.js
```

### Level2 orderbook updates

Crawl realtime level2 orderbook incremental updates:

```bash
docker run -d --name carbonbot-l2_event --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.l2_event.config.js
```

### Level2 orderbook full snapshots from RESTful API

Crawl level2 orderbook full snapshots from RESTful API:

```bash
docker run -d --name carbonbot-l2_snapshot --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.l2_snapshot.config.js
```

### Level2 orderbook top-k snapshots

Crawl realtime level2 orderbook top-K snapshots:

```bash
docker run -d --name carbonbot-l2_topk --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.l2_topk.config.js
```

### Level3 orderbook updates

Crawl realtime level3 orderbook incremental updates:

```bash
docker run -d --name carbonbot-l3_event --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.l3_event.config.js
```

### BBO

Crawl realtime BBO:

```bash
docker run -d --name carbonbot-bbo --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.bbo.config.js
```

### Ticker

Crawl 24hr rolling window tickers:

```bash
docker run -d --name carbonbot-ticker --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.ticker.config.js
```

### Candlestick

Crawl candlesticks(i.e., OHLCV)

```bash
docker run -d --name carbonbot-candlestick --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.candlestick.config.js
```

### Funding rate

Crawl funding rates

```bash
docker run -d --name carbonbot-funding_rate --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.funding_rate.config.js
```

### Open interest

```bash
docker run -d --name carbonbot-open_interest --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.open_interest.config.js
```

### Other

```bash
docker run -d --name carbonbot-other --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.other.config.js
```

## Output Destinations

Crawlers running in the `ghcr.io/crypto-crawler/carbonbot:latest` container write data to the local temporary path `/carbonbot_data` first, then copy data to multiple destinations every 15 minutes, and delete source files in `/carbonbot_data`.

### AWS S3

To upload data to AWS S3 automatically, uses need to specify three environment variables, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` and `AWS_S3_DIR`. For example:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.trade.config.js
```

### Redis

To output data to Redis, users needs to specify a `REDIS_URL` environment variable. For example:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e REDIS_URL=redis://172.17.0.1:6379 -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.trade.config.js
```

### Local Directory

To output data to a local directory on host machine, users need to mount this local directory into the docker container, and specify a `DEST_DIR` environment variable pointing to this directory. For example:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -v $ANOTHER_LOCAL_PATH:/dest_dir -e DEST_DIR=/dest_dir -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.trade.config.js
```

## Build

```bash
docker pull rust:latest && docker pull node:bullseye-slim
docker build -t ghcr.io/crypto-crawler/carbonbot:latest .
docker push ghcr.io/crypto-crawler/carbonbot:latest
```

## Download

| File Name                | MD5                              | Size        | Magnet Link                                                                                                                                                                                                                                                    |
| ------------------------ | -------------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| parsed-trade-2021-07.zip | a4a4088c3c9ebccc70e4b10f77c044c3 | 84213736076 | magnet:?xt=urn:btih:557afe1132dd5a67dada971009733ae6019fd84b&dn=parsed-trade-2021-07.zip&tr=http%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&tr=http%3A%2F%2Fp4p.arenabg.com%3A1337%2Fannounce |
