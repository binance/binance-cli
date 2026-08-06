use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as, wait_for_shutdown,
};
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::constants::{
    DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_PROD_URL,
    DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_TESTNET_URL,
};
use binance_sdk::derivatives_trading_usds_futures::DerivativesTradingUsdsFuturesWsStreams;
use binance_sdk::derivatives_trading_usds_futures::websocket_streams::WebsocketStreamsHandle;
use binance_sdk::derivatives_trading_usds_futures::websocket_streams::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>) -> Result<WebsocketStreamsHandle, Error> {
    init_user_agent("derivatives-trading-usds-futures");

    let client_config =
        get_client_configuration(profile, "derivatives-trading-usds-futures").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config
        .base_path_ws_streams
        .unwrap_or(match api_env.as_str() {
            "testnet" => DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_TESTNET_URL.to_string(),
            "prod" => DERIVATIVES_TRADING_USDS_FUTURES_WS_STREAMS_PROD_URL.to_string(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "invalid BINANCE_API_ENV",
                ));
            }
        });

    let builder = ConfigurationWebsocketStreams::builder().ws_url(base_path);

    let ws_config = builder
        .build()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    Ok(DerivativesTradingUsdsFuturesWsStreams::from_config(
        ws_config,
    ))
}

#[derive(Args, Debug)]
struct AggregateTradeStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct AllMarketLiquidationOrderStreamsArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct AllMarketMiniTickersStreamArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct AllMarketTickersStreamsArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct AssetIndexArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct CompositeIndexSymbolInformationStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct ContinuousContractKlineCandlestickStreamsArgs {
    #[arg(help = r#""#, long)]
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    contract_type: Option<ContinuousContractKlineCandlestickStreamsContractTypeEnum>,
    #[arg(help = r#""#, long)]
    interval: Option<ContinuousContractKlineCandlestickStreamsIntervalEnum>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct ContractInfoStreamArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct IndividualSymbolMiniTickerStreamArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct IndividualSymbolTickerStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct KlineCandlestickStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"The interval parameter"#, long)]
    interval: Option<KlineCandlestickStreamsIntervalEnum>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct LiquidationOrderStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct MarkPriceStreamArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(help = r#"WebSocket stream update speed"#, long)]
    update_speed: Option<MarkPriceStreamUpdateSpeedEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct MarkPriceStreamForAllMarketArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(help = r#"WebSocket stream update speed"#, long)]
    update_speed: Option<MarkPriceStreamForAllMarketUpdateSpeedEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct TradingSessionStreamArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct AllBookTickersStreamArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct DiffBookDepthStreamsArgs {
    #[arg(help = r#"Trading pair symbol."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(help = r#"WebSocket stream update speed"#, long)]
    update_speed: Option<DiffBookDepthStreamsUpdateSpeedEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct IndividualSymbolBookTickerStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct PartialBookDepthStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"The levels parameter"#, long)]
    levels: Option<PartialBookDepthStreamsLevelsEnum>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(help = r#"WebSocket stream update speed"#, long)]
    update_speed: Option<PartialBookDepthStreamsUpdateSpeedEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}
#[derive(Args, Debug)]
struct RpiDiffBookDepthStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
    #[arg(help = r#"Pretty-print the JSON output"#, long)]
    pretty: bool,
    #[arg(
        help = r#"Stop listening after the given number of messages"#,
        long = "stream-limit"
    )]
    stream_limit: Option<u64>,
    #[arg(
        help = r#"Stop listening after the given duration in milliseconds"#,
        long = "stream-duration"
    )]
    stream_duration: Option<u64>,
}

#[derive(Subcommand)]
pub enum DerivativesTradingUsdsFuturesWebsocketStreamsCommands {
    #[command(
        about = decode_selected_entities(r#"The Aggregate Trade Streams push market trade information that is aggregated for fills with same price and taking side every 100 milliseconds. Only market trades will be aggregated, which means the insurance fund trades and ADL trades won't be aggregated.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 100ms

Response Notes:
- Retail Price Improvement(RPI) orders are aggregated into field q and without special tags to be distinguished."#, false),
    )]
    AggregateTradeStreams(AggregateTradeStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"The All Liquidation Order Snapshot Streams push force liquidation order information for all symbols in the market. For each symbol，only the latest one liquidation order within 1000ms will be pushed as the snapshot. If no liquidation happens in the interval of 1000ms, no stream will be pushed.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 1000ms"#, false),
    )]
    AllMarketLiquidationOrderStreams(AllMarketLiquidationOrderStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window mini-ticker statistics for all symbols. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before. Note that only tickers that have changed will be present in the array.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 1000ms"#, false),
    )]
    AllMarketMiniTickersStream(AllMarketMiniTickersStreamArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window ticker statistics for all symbols. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before. Note that only tickers that have changed will be present in the array.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 1000ms"#, false),
    )]
    AllMarketTickersStreams(AllMarketTickersStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Asset index price. Subscribe with `!assetIndex@arr` for all assets, or `<assetSymbol>@assetIndex` for a specific asset.

> **CM-UM Integration (Effective 2026-06-30):** Renamed from *Multi-Assets Mode Asset Index*. The stream `!assetIndex@arr` now additionally pushes COIN-M settlement-asset price index entries (e.g., `BTCUSD`, `ETHUSD`, `BNBUSD`). The on-the-wire stream key is unchanged; existing subscriptions continue to work.

Update Speed: 1s"#, false),
    )]
    AssetIndex(AssetIndexArgs),
    #[command(
        about = decode_selected_entities(r#"Composite index information for index symbols pushed every second.

Update Speed: 1000ms"#, false),
    )]
    CompositeIndexSymbolInformationStreams(CompositeIndexSymbolInformationStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Continuous Contract Kline/Candlestick Streams

> **After CM migration**, both `fstream` and `dstream` may subscribe to either UM or CM symbols on this stream.

Update Speed: 250ms"#, false),
    )]
    ContinuousContractKlineCandlestickStreams(ContinuousContractKlineCandlestickStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"ContractInfo stream pushes when contract info updates(listing/settlement/contract bracket update). bks field only shows up when bracket gets updated.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: Real-time"#, false),
    )]
    ContractInfoStream(ContractInfoStreamArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window mini-ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 2s"#, false),
    )]
    IndividualSymbolMiniTickerStream(IndividualSymbolMiniTickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 2000ms"#, false),
    )]
    IndividualSymbolTickerStreams(IndividualSymbolTickerStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"The Kline/Candlestick Stream push updates to the current klines/candlestick every 250 milliseconds (if existing).

> **After CM migration**, both `fstream` and `dstream` may subscribe to either UM or CM symbols on this stream.

Update Speed: 250ms"#, false),
    )]
    KlineCandlestickStreams(KlineCandlestickStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"The Liquidation Order Snapshot Streams push force liquidation order information for specific symbol. For each symbol，only the latest one liquidation order within 1000ms will be pushed as the snapshot. If no liquidation happens in the interval of 1000ms, no stream will be pushed.

Update Speed: 1000ms"#, false),
    )]
    LiquidationOrderStreams(LiquidationOrderStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Mark price and funding rate for a single symbol pushed every 3 seconds or every second.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM); both `fstream` and `dstream` may subscribe to either UM or CM symbols on this stream."#, false),
    )]
    MarkPriceStream(MarkPriceStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Mark price and funding rate for all symbols pushed every 3 seconds or every second.

**Note:**
- TradFi symbols will be pushed through a seperate message.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM); both `fstream` and `dstream` may subscribe to either UM or CM symbols on this stream.

Update Speed: 3s or 1s"#, false),
    )]
    MarkPriceStreamForAllMarket(MarkPriceStreamForAllMarketArgs),
    #[command(
        about = decode_selected_entities(r#"Trading session information for the underlying assets of TradFi Perpetual contracts, covering the U.S. equity market, Korean equity market, Hong Kong equity market, and the commodity market, is updated every second. Trading session information for different underlying markets is pushed in separate messages.

**Event type:**

- `EquityUpdate`: Session types for the U.S. equity market include "PRE_MARKET", "REGULAR", "AFTER_MARKET", "OVERNIGHT", and "NO_TRADING".
- `CommodityUpdate`: Session types for the commodity market include "REGULAR" and "NO_TRADING".
- `KR_EquityUpdate`: Session types for the Korean equity market include "REGULAR" and "NO_TRADING".
- `HK_EquityUpdate`: Session types for the Hong Kong equity market include "REGULAR" and "NO_TRADING".

Update Speed: 1s"#, false),
    )]
    TradingSessionStream(TradingSessionStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes any update to the best bid or ask's price or quantity in real-time for all symbols.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 5s

Response Notes:
- Retail Price Improvement(RPI) orders are not visible and excluded in the response message."#, false),
    )]
    AllBookTickersStream(AllBookTickersStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Bids and asks, pushed every 250 milliseconds, 500 milliseconds, 100 milliseconds (if existing).

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 250ms, 500ms, 100ms

Response Notes:
- Retail Price Improvement(RPI) orders are not visible and excluded in the response message."#, false),
    )]
    DiffBookDepthStreams(DiffBookDepthStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes any update to the best bid or ask's price or quantity in real-time for a specified symbol.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: Real-time

Response Notes:
Retail Price Improvement (RPI) orders are not visible and excluded in the response message."#, false),
    )]
    IndividualSymbolBookTickerStreams(IndividualSymbolBookTickerStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Top <levels> bids and asks

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 250ms or 500ms or 100ms

Response Notes:
Retail Price Improvement (RPI) orders are not visible and excluded in the response message."#, false),
    )]
    PartialBookDepthStreams(PartialBookDepthStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Bids and asks including RPI orders, pushed every 500 milliseconds

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM) and a new `ps` field (pair symbol).

Update Speed: 500ms

Response Notes:
- RPI(Retail Price Improvement) orders are included and aggreated in the response message. When the quantity of a price level to be updated is equal to 0, it means either all quotations for this price have been filled/canceled, or the quantity of crossed RPI orders for this price are hidden"#, false),
    )]
    RpiDiffBookDepthStreams(RpiDiffBookDepthStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribes to the user data WebSocket stream using the provided listen key."#, false),
    )]
    UserData(UserDataArgs),
}

pub async fn handle_derivatives_trading_usds_futures_ws_streams_command(
    command: DerivativesTradingUsdsFuturesWebsocketStreamsCommands,
) -> anyhow::Result<()> {
    match command {

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::AggregateTradeStreams (args) => aggregate_trade_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::AllMarketLiquidationOrderStreams (args) => all_market_liquidation_order_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::AllMarketMiniTickersStream (args) => all_market_mini_tickers_stream(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::AllMarketTickersStreams (args) => all_market_tickers_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::AssetIndex (args) => asset_index(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::CompositeIndexSymbolInformationStreams (args) => composite_index_symbol_information_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::ContinuousContractKlineCandlestickStreams (args) => continuous_contract_kline_candlestick_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::ContractInfoStream (args) => contract_info_stream(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::IndividualSymbolMiniTickerStream (args) => individual_symbol_mini_ticker_stream(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::IndividualSymbolTickerStreams (args) => individual_symbol_ticker_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::KlineCandlestickStreams (args) => kline_candlestick_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::LiquidationOrderStreams (args) => liquidation_order_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::MarkPriceStream (args) => mark_price_stream(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::MarkPriceStreamForAllMarket (args) => mark_price_stream_for_all_market(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::TradingSessionStream (args) => trading_session_stream(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::AllBookTickersStream (args) => all_book_tickers_stream(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::DiffBookDepthStreams (args) => diff_book_depth_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::IndividualSymbolBookTickerStreams (args) => individual_symbol_book_ticker_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::PartialBookDepthStreams (args) => partial_book_depth_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::RpiDiffBookDepthStreams (args) => rpi_diff_book_depth_streams(args).await,

          DerivativesTradingUsdsFuturesWebsocketStreamsCommands::UserData(args) => user_data(args).await,
    }
}

async fn aggregate_trade_streams(mut args: AggregateTradeStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AggregateTradeStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AggregateTradeStreamsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                AggregateTradeStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.aggregate_trade_streams(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn all_market_liquidation_order_streams(
    args: AllMarketLiquidationOrderStreamsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllMarketLiquidationOrderStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<AllMarketLiquidationOrderStreamsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => AllMarketLiquidationOrderStreamsParams::builder()
                .id(args.id)
                .build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection
        .all_market_liquidation_order_streams(params)
        .await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn all_market_mini_tickers_stream(
    args: AllMarketMiniTickersStreamArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllMarketMiniTickersStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<AllMarketMiniTickersStreamParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => AllMarketMiniTickersStreamParams::builder()
                .id(args.id)
                .build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_market_mini_tickers_stream(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn all_market_tickers_streams(args: AllMarketTickersStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllMarketTickersStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllMarketTickersStreamsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllMarketTickersStreamsParams::builder()
                .id(args.id)
                .build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_market_tickers_streams(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn asset_index(args: AssetIndexArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AssetIndexParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AssetIndexParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AssetIndexParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.asset_index(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn composite_index_symbol_information_streams(
    mut args: CompositeIndexSymbolInformationStreamsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<CompositeIndexSymbolInformationStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CompositeIndexSymbolInformationStreamsParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                CompositeIndexSymbolInformationStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection
        .composite_index_symbol_information_streams(params)
        .await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn continuous_contract_kline_candlestick_streams(
    mut args: ContinuousContractKlineCandlestickStreamsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<ContinuousContractKlineCandlestickStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ContinuousContractKlineCandlestickStreamsParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.pair.is_none() {
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

                        args.pair = Some(pair);
                    }
                    if args.contract_type.is_none() {
                        let options = vec![
                        ("perpetual", ContinuousContractKlineCandlestickStreamsContractTypeEnum::Perpetual),
                        ("current_quarter", ContinuousContractKlineCandlestickStreamsContractTypeEnum::CurrentQuarter),
                        ("next_quarter", ContinuousContractKlineCandlestickStreamsContractTypeEnum::NextQuarter),
                        ("tradifi_perpetual", ContinuousContractKlineCandlestickStreamsContractTypeEnum::TradifiPerpetual),
                    ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the contract_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.contract_type = Some(selected);
                    }
                    if args.interval.is_none() {
                        let options =
                            vec![
                        ("1s", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval1s),
                        ("1m", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval1m),
                        ("3m", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval3m),
                        ("5m", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval5m),
                        ("15m", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval15m),
                        ("30m", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval30m),
                        ("1h", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval1h),
                        ("2h", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval2h),
                        ("4h", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval4h),
                        ("6h", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval6h),
                        ("8h", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval8h),
                        ("12h", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval12h),
                        ("1d", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval1d),
                        ("3d", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval3d),
                        ("1w", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval1w),
                        ("1M", ContinuousContractKlineCandlestickStreamsIntervalEnum::Interval1M),
                    ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the interval")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.interval = Some(selected);
                    }
                }
                ContinuousContractKlineCandlestickStreamsParams::builder(
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
                    args.contract_type
                        .ok_or_else(|| anyhow::anyhow!("contract_type is required"))?,
                    args.interval
                        .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection
        .continuous_contract_kline_candlestick_streams(params)
        .await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn contract_info_stream(args: ContractInfoStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<ContractInfoStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ContractInfoStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ContractInfoStreamParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.contract_info_stream(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn individual_symbol_mini_ticker_stream(
    mut args: IndividualSymbolMiniTickerStreamArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<IndividualSymbolMiniTickerStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<IndividualSymbolMiniTickerStreamParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                IndividualSymbolMiniTickerStreamParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection
        .individual_symbol_mini_ticker_stream(params)
        .await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn individual_symbol_ticker_streams(
    mut args: IndividualSymbolTickerStreamsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<IndividualSymbolTickerStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<IndividualSymbolTickerStreamsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                IndividualSymbolTickerStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.individual_symbol_ticker_streams(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn kline_candlestick_streams(mut args: KlineCandlestickStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<KlineCandlestickStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<KlineCandlestickStreamsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.interval.is_none() {
                        let options = vec![
                            ("1m", KlineCandlestickStreamsIntervalEnum::Interval1m),
                            ("3m", KlineCandlestickStreamsIntervalEnum::Interval3m),
                            ("5m", KlineCandlestickStreamsIntervalEnum::Interval5m),
                            ("15m", KlineCandlestickStreamsIntervalEnum::Interval15m),
                            ("30m", KlineCandlestickStreamsIntervalEnum::Interval30m),
                            ("1h", KlineCandlestickStreamsIntervalEnum::Interval1h),
                            ("2h", KlineCandlestickStreamsIntervalEnum::Interval2h),
                            ("4h", KlineCandlestickStreamsIntervalEnum::Interval4h),
                            ("6h", KlineCandlestickStreamsIntervalEnum::Interval6h),
                            ("8h", KlineCandlestickStreamsIntervalEnum::Interval8h),
                            ("12h", KlineCandlestickStreamsIntervalEnum::Interval12h),
                            ("1d", KlineCandlestickStreamsIntervalEnum::Interval1d),
                            ("3d", KlineCandlestickStreamsIntervalEnum::Interval3d),
                            ("1w", KlineCandlestickStreamsIntervalEnum::Interval1w),
                            ("1M", KlineCandlestickStreamsIntervalEnum::Interval1M),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the interval")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.interval = Some(selected);
                    }
                }
                KlineCandlestickStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.interval
                        .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.kline_candlestick_streams(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn liquidation_order_streams(mut args: LiquidationOrderStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<LiquidationOrderStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<LiquidationOrderStreamsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                LiquidationOrderStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.liquidation_order_streams(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn mark_price_stream(mut args: MarkPriceStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<MarkPriceStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarkPriceStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                MarkPriceStreamParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .update_speed(args.update_speed)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.mark_price_stream(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn mark_price_stream_for_all_market(
    args: MarkPriceStreamForAllMarketArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<MarkPriceStreamForAllMarketParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<MarkPriceStreamForAllMarketParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => MarkPriceStreamForAllMarketParams::builder()
                .id(args.id)
                .update_speed(args.update_speed)
                .build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.mark_price_stream_for_all_market(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn trading_session_stream(args: TradingSessionStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<TradingSessionStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TradingSessionStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => TradingSessionStreamParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.trading_session_stream(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn all_book_tickers_stream(args: AllBookTickersStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllBookTickersStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllBookTickersStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllBookTickersStreamParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_book_tickers_stream(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn diff_book_depth_streams(mut args: DiffBookDepthStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<DiffBookDepthStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DiffBookDepthStreamsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                DiffBookDepthStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .update_speed(args.update_speed)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.diff_book_depth_streams(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn individual_symbol_book_ticker_streams(
    mut args: IndividualSymbolBookTickerStreamsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<IndividualSymbolBookTickerStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<IndividualSymbolBookTickerStreamsParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                IndividualSymbolBookTickerStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection
        .individual_symbol_book_ticker_streams(params)
        .await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn partial_book_depth_streams(mut args: PartialBookDepthStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<PartialBookDepthStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PartialBookDepthStreamsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.levels.is_none() {
                        let options = vec![
                            ("5", PartialBookDepthStreamsLevelsEnum::Levels5),
                            ("10", PartialBookDepthStreamsLevelsEnum::Levels10),
                            ("20", PartialBookDepthStreamsLevelsEnum::Levels20),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the levels")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.levels = Some(selected);
                    }
                }
                PartialBookDepthStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.levels
                        .ok_or_else(|| anyhow::anyhow!("levels is required"))?,
                )
                .id(args.id)
                .update_speed(args.update_speed)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.partial_book_depth_streams(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

async fn rpi_diff_book_depth_streams(mut args: RpiDiffBookDepthStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<RpiDiffBookDepthStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RpiDiffBookDepthStreamsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                RpiDiffBookDepthStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.rpi_diff_book_depth_streams(params).await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}

#[derive(clap::Parser, Debug)]
pub struct UserDataArgs {
    #[arg(long)]
    pub profile: Option<String>,

    #[arg(long = "listen-key")]
    pub listen_key: Option<String>,

    #[arg(long)]
    pub id: Option<String>,

    /// Send all fields as JSON
    #[arg(long)]
    pub json: Option<String>,

    #[arg(long)]
    pub interactive: bool,

    /// Pretty-print the JSON output
    #[arg(long)]
    pub pretty: bool,

    /// Stop listening after the given number of messages
    #[arg(long = "stream-limit")]
    pub stream_limit: Option<u64>,

    /// Stop listening after the given duration in ms
    #[arg(long = "stream-duration")]
    pub stream_duration: Option<u64>,
}

async fn user_data(mut args: UserDataArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    if args.interactive {
        if args.listen_key.is_none() {
            let listen_key: String = Input::new()
                .with_prompt("Input listenKey")
                .interact_text()?;

            args.listen_key = Some(listen_key);
        }
    }

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection
        .user_data(
            args.listen_key
                .ok_or_else(|| anyhow::anyhow!("listen_key is required"))?,
            args.id,
        )
        .await?;

    let message_count = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let stop = std::sync::Arc::new(tokio::sync::Notify::new());

    let pretty = args.pretty;
    let stream_limit = args.stream_limit;
    let stream_duration = args.stream_duration;

    let message_count_for_handler = std::sync::Arc::clone(&message_count);
    let stop_for_handler = std::sync::Arc::clone(&stop);

    stream.on_message(move |data| {
        let output = if pretty {
            serde_json::to_string_pretty(&data)
        } else {
            serde_json::to_string(&data)
        };

        println!("{}", output.unwrap());

        if let Some(limit) = stream_limit {
            let count =
                message_count_for_handler.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            if count >= limit {
                stop_for_handler.notify_one();
            }
        }
    });

    if stream_limit == Some(0) {
        stop.notify_one();
    }

    if let Some(ms) = stream_duration {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(ms)) => {
                eprintln!("stream duration reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    } else {
        tokio::select! {
            _ = stop.notified() => {
                eprintln!("stream limit reached");
            }
            _ = wait_for_shutdown() => {
                eprintln!("shutdown requested");
            }
        }
    }

    Ok(())
}
