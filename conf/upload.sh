#!/bin/bash
# Linted by https://www.shellcheck.net/

# This script aims to harvest .json files generated by logrotate,
# compress and upload them to S3.

msg_type=$1 # trade, l2_event, etc.

if [[ -z "${DATA_DIR}" ]]; then
  echo "DATA_DIR must be set" >&2
  exit 1
fi

if [[ -z "${DEST_DIR}" && -z "${AWS_S3_DIR}" && -z "${MINIO_DIR}" && -z "${REDIS_URL}" ]]; then
  echo "At least one of DEST_DIR, AWS_S3_DIR, MINIO_DIR or REDIS_URL must be set" >&2
  exit 1
fi

if [[ -n "${AWS_S3_DIR}" ]]; then
  if [[ -z "${AWS_ACCESS_KEY_ID}"  ||  -z "${AWS_SECRET_ACCESS_KEY}" ]]; then
    echo "AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY must be set" >&2
  exit 1
  fi
fi

if [[ -n "${MINIO_DIR}" ]]; then
  if [[ -z "${AWS_ACCESS_KEY_ID}"  ||  -z "${AWS_SECRET_ACCESS_KEY}" ||  -z "${MINIO_ENDPOINT_URL}" ]]; then
    echo "AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY and MINIO_ENDPOINT_URL must be set" >&2
  exit 1
  fi
fi

if [[ -n "${DEST_DIR}" ]]; then
  mkdir -p "$DEST_DIR/$msg_type"
fi

# Infinite while loop
while :
do
  # Find .json files and compress them
  # deprecated and replaced by compress.sh since 2022-04-23
  # find "$DATA_DIR/$msg_type" -name "*.json" -type f -mmin +1 | xargs -r -n 1 pigz -f
  success=true
  if [[ -n "${AWS_S3_DIR}" ]]; then
    rclone --s3-region "${AWS_REGION:-us-east-1}" copy "$DATA_DIR/$msg_type" "$AWS_S3_DIR/$msg_type" --include '*.json.gz' --no-traverse --transfers=8
    if [ $? -ne 0 ]; then
      success=false
    fi
  fi
  if [[ -n "${MINIO_DIR}" ]]; then
    rclone --s3-region "${AWS_REGION:-us-east-1}" --s3-endpoint $MINIO_ENDPOINT_URL copy "$DATA_DIR/$msg_type" "$MINIO_DIR/$msg_type" --include '*.json.gz' --no-traverse --transfers=8
    if [ $? -ne 0 ]; then
      success=false
    fi
  fi
  if [[ -n "${DEST_DIR}" ]]; then
    rclone move "$DATA_DIR/$msg_type" "$DEST_DIR/$msg_type" --include '*.json.gz' --no-traverse --transfers=8
    if [ $? -ne 0 ]; then
      success=false
    fi
  fi

  if [ "$success" = true ]; then
    rclone delete "$DATA_DIR/$msg_type" --include '*.json.gz'
  fi

  sleep 3
done
