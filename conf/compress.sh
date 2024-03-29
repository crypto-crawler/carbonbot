#!/bin/bash
# Linted by https://www.shellcheck.net/

msg_type=$1 # trade, l2_event, etc.

if [[ -z "${DATA_DIR}" ]]; then
  echo "DATA_DIR must be set" >&2
  exit 1
fi

# Infinite while loop
while :
do
  # Find .json files older than 1 minute and compress them
  find "$DATA_DIR/$msg_type" -name "*.json" -type f -mmin +1 | xargs -r -n 1 pigz --best -f
  sleep 3
done
