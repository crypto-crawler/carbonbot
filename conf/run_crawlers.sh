#!/bin/bash
LOCAL_TMP_DIR="/your/local/path"        # required
AWS_ACCESS_KEY_ID="you access key"      # required
AWS_SECRET_ACCESS_KEY="your secret key" # required
AWS_S3_DIR="s3://bucket/your/s3/path"   # required

docker pull ghcr.io/crypto-crawler/carbonbot:latest

mkdir -p $LOCAL_DATA_DIR

# l2_snapshot and open_interest are not included, better deploy them in a different network
msg_types=("trade" "l2_event" "l2_topk" "l3_event" "bbo" "ticker" "candlestick" "funding_rate" "other")

for msg_type in ${msg_types[@]}; do
  docker stop carbonbot-$msg_type && docker rm carbonbot-$msg_type
  docker run -d --name carbonbot-$msg_type --restart always -v $LOCAL_TMP_DIR:/carbonbot_data -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY -e AWS_S3_DIR=$AWS_S3_DIR -u "$(id -u):$(id -g)" ghcr.io/crypto-crawler/carbonbot:latest pm2-runtime start pm2.$msg_type.config.js
done

docker system prune -af

echo "Cleaning up pm2 logs"
sleep 5
for msg_type in ${msg_types[@]}; do
  docker exec -it carbonbot-$msg_type bash -c "truncate -s 0 /home/node/.pm2/logs/*.log"
done
