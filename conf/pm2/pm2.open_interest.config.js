// see src/market_type.rs in crypto-markets
const market_types = {
    binance: ["linear_future", "inverse_future", "linear_swap", "inverse_swap"],
    bitget: ["inverse_swap", "linear_swap"],
    bybit: ["inverse_future", "inverse_swap", "linear_swap"],
    deribit: ["unknown"], // https://www.deribit.com/api/v2/public/get_book_summary_by_currency?currency={BTC,ETH,SOL,USDC} contains all markets
    dydx: ["linear_swap"],
    gate: ["linear_swap", "inverse_swap"],
    huobi: ["inverse_future", "linear_swap", "inverse_swap"],
    kucoin: ["unknown"], // https://api-futures.kucoin.com/api/v1/contracts/active contains all open interests
    okx: [
        "linear_future",
        "inverse_future",
        "linear_swap",
        "inverse_swap",
        "european_option",
    ],
    zbg: ["inverse_swap", "linear_swap"],
};

const apps = [];

Object.keys(market_types).forEach((exchange) => {
    market_types[exchange].forEach((market_ype) => {
        const app = {
            name: `crawler-open_interest-${exchange}-${market_ype}`,
            script: "carbonbot",
            args: `${exchange} ${market_ype} open_interest`,
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
    args: "/usr/local/etc/logrotate.open_interest.conf",
    exec_interpreter: "none",
    exec_mode: "fork_mode",
    cron_restart: "*/15 * * * *",
    autorestart: false,
});

apps.push({
    name: "compress",
    script: "/usr/local/bin/compress.sh",
    args: "open_interest",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

apps.push({
    name: "upload",
    script: "/usr/local/bin/upload.sh",
    args: "open_interest",
    exec_interpreter: "bash",
    exec_mode: "fork_mode",
    instances: 1,
    restart_delay: 5000, // 5 seconds
});

module.exports = {
    apps,
};
