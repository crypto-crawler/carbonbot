#!/bin/bash

containers=$(docker ps --format '{{.Names}}' -f ancestor=ghcr.io/crypto-crawler/carbonbot:latest)

echo "Cleaning up pm2 logs..."

for container in ${containers[@]}; do
  echo "$container"
  docker exec -it "$container" bash -c "truncate -s 0 /home/node/.pm2/logs/*.log; pm2 reset all -s"
done
