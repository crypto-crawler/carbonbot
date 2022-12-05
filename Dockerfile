FROM rust:latest AS builder

RUN mkdir /project
WORKDIR /project

COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src/

RUN apt -qy update && apt -qy install pkg-config libssl-dev \
 && RUSTFLAGS="-C target-cpu=x86-64-v3" cargo build --release


FROM node:bullseye-slim

COPY --from=builder /project/target/release/carbonbot /usr/local/bin/carbonbot

# procps provides the ps command, which is needed by pm2
RUN apt-get -qy update && apt-get -qy --no-install-recommends install \
    ca-certificates curl htop logrotate procps pigz sudo tree xz-utils \
 && chown -R node:node /var/lib/logrotate/ \
 && npm install pm2 -g --production \
 && apt-get -qy install gzip unzip && curl https://rclone.org/install.sh | bash \
 && echo "node ALL=(ALL:ALL) NOPASSWD:ALL" >> /etc/sudoers \
 && apt-get -qy autoremove && apt-get clean && rm -rf /var/lib/apt/lists/* && rm -rf /tmp/*

# Install fixuid
RUN ARCH="$(dpkg --print-architecture)" && \
    curl -SsL https://github.com/boxboat/fixuid/releases/download/v0.5.1/fixuid-0.5.1-linux-amd64.tar.gz | tar -C /usr/local/bin -xzf - && \
    chown root:root /usr/local/bin/fixuid && \
    chmod 4755 /usr/local/bin/fixuid && \
    mkdir -p /etc/fixuid && \
    printf "user: node\ngroup: node\npaths:\n  - /home/node\n  - /var/lib/logrotate/\n" > /etc/fixuid/config.yml

COPY --chown=node:node ./conf/pm2/pm2.*.config.js /home/node/
COPY ./conf/logrotate/logrotate.*.conf /usr/local/etc/
COPY --chown=node:node ./conf/rclone.conf /home/node/.config/rclone/rclone.conf
COPY ./conf/logrotate.sh /usr/local/bin/logrotate.sh
COPY ./conf/compress.sh /usr/local/bin/compress.sh
COPY ./conf/upload.sh /usr/local/bin/upload.sh

ENV RUST_LOG "warn"
ENV RUST_BACKTRACE 1

VOLUME [ "/carbonbot_data" ]
ENV DATA_DIR /carbonbot_data

USER node:node
ENV USER node
WORKDIR /home/node

ENTRYPOINT ["fixuid", "-q"]
