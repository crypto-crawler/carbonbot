#!/bin/bash
LOCAL_TMP_DIR="/your/local/path"        # required

AWS_ACCESS_KEY_ID="you access key"      # required
AWS_SECRET_ACCESS_KEY="your secret key" # required
AWS_S3_DIR="s3://bucket/path"           # required

MINIO_ACCESS_KEY_ID="your access key"
MINIO_SECRET_ACCESS_KEY="your secret key"
MINIO_DIR="minio://bucket/path"
MINIO_ENDPOINT_URL="http://ip-address:9000"

docker pull ghcr.io/crypto-crawler/carbonbot:latest

mkdir -p $LOCAL_TMP_DIR

# l2_snapshot and open_interest are not included, better deploy them in a different network
msg_types=("trade" "l2_event" "l2_topk" "l3_event" "bbo" "ticker" "candlestick" "funding_rate" "other")

for msg_type in ${msg_types[@]}; do
  docker stop carbonbot-$msg_type && docker rm carbonbot-$msg_type
  docker run -d --name carbonbot-$msg_type --restart unless-stopped -v $LOCAL_TMP_DIR:/carbonbot_data -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY -e AWS_S3_DIR=$AWS_S3_DIR -e MINIO_ACCESS_KEY_ID=$MINIO_ACCESS_KEY_ID -e MINIO_SECRET_ACCESS_KEY=$MINIO_SECRET_ACCESS_KEY -e MINIO_ENDPOINT_URL=$MINIO_ENDPOINT_URL -e MINIO_DIR=$MINIO_DIR -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.$msg_type.config.jsdone
done

docker system prune -af
