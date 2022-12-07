// see src/market_type.rs in crypto-markets
const market_types = {
    bitfinex: ["spot", "linear_swap"],
    bitstamp: ["spot"],
    coinbase_pro: ["spot"],
};

const apps = [];

Object.keys(market_types).forEach((exchange) => {
    market_types[exchange].forEach((market_ype) => {
        const app = {
            name: `crawler-l3_event-${exchange}-${market_ype}`,
            script: "carbonbot",
            args: `${exchange} ${market_ype} l3_event`,
            exec_interpreter: "none",
            exec_mode: "fork_mode",
            instances: 1,
            exp_backoff_restart_delay: 5000,
        };

        apps.push(app);
    });
});

apps.push({
    name: "logrotate",
    script: "/usr/local/bin/logrotate.sh",
    args: "/usr/local/etc/logrotate.l3_event.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

apps.push({
    name: "compress",
    script: "/usr/local/bin/compress.sh",
    args: "l3_event",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

apps.push({
    name: "upload",
    script: "/usr/local/bin/upload.sh",
    args: "l3_event",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

module.exports = {
    apps,
};
