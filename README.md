# carbonbot

A CLI tool based on the crypto-crawler-rs library to crawl trade, level2, level3, ticker, funding rate, etc.

## 1. Sample Data Download

### AWS S3

```bash
aws s3 ls --request-payer requester s3://carbonbot/
aws s3 sync --request-payer requester s3://carbonbot/monthly/parsed .
```

The S3 bucket `s3://carbonbot` has **Requester Pays** enabled.


### BitTorrent

TODO


## 2. Run Crawlers

Copy `conf/run_crawlers.sh` to somewhere, change `LOCAL_TMP_DIR` to a local SSD directory and `DEST_DIR` to a directory on a large disk, and run this shell script. Run `docker ps` and you'll see all crawlers are running! 

Use `tail -f file` to check files under `LOCAL_TMP_DIR`, you'll see data in realtime; watch the `DEST_DIR` dirctory, you'll see new files are moved from  `LOCAL_TMP_DIR` to `DEST_DIR` every 15 minutes.

## 3. Output Destinations

Crawlers running in the `ghcr.io/crypto-crawler/carbonbot:latest` container write data to the local temporary path `/carbonbot_data` first, then move data to multiple destinations every 15 minutes.

Four kinds of destinations are supported: directory, AWS S3, MinIO and Redis.

### Directory

To save data to a local directory or a NFS directory, users need to mount this directory into the docker container, and specify a `DEST_DIR` environment variable pointing to this directory. For example:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -v $DEST_DIR:/dest_dir -e DEST_DIR=/dest_dir -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.trade.config.js
```

### AWS S3

To upload data to AWS S3 automatically, uses need to specify three environment variables, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` and `AWS_S3_DIR`. For example:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e AWS_S3_DIR="s3://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.trade.config.js
```

Optionally, users can specify the `AWS_REGION` environment variable, see [Configuring the AWS SDK for Go
](https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html).

### MinIO

To upload data to AWS S3 automatically, users need to specify three environment variables, `MINIO_ACCESS_KEY_ID`, `MINIO_SECRET_ACCESS_KEY`, `MINIO_ENDPOINT_URL` and `MINIO_DIR`. For example:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e MINIO_ACCESS_KEY_ID="YOUR_ACCESS_KEY" -e MINIO_SECRET_ACCESS_KEY="YOUR_SECRET_KEY" -e MINIO_ENDPOINT_URL="http://ip:9000" -e MINIO_DIR="minio://YOUR_BUCKET/path" -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.trade.config.js
```

### Redis

To output data to Redis, users needs to specify a `REDIS_URL` environment variable. For example:

```bash
docker run -d --name carbonbot-trade --restart always -v $YOUR_LOCAL_PATH:/carbonbot_data -e REDIS_URL=redis://172.17.0.1:6379 -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.trade.config.js
```


## 4. Build

```bash
docker pull rust:latest && docker pull node:bullseye-slim
docker build -t ghcr.io/crypto-crawler/carbonbot:latest .
docker push ghcr.io/crypto-crawler/carbonbot:latest
```
