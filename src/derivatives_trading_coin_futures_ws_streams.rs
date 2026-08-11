use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as, wait_for_shutdown,
};
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::constants::{
    DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_PROD_URL,
    DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_TESTNET_URL,
};
use binance_sdk::derivatives_trading_coin_futures::DerivativesTradingCoinFuturesWsStreams;
use binance_sdk::derivatives_trading_coin_futures::websocket_streams::WebsocketStreamsHandle;
use binance_sdk::derivatives_trading_coin_futures::websocket_streams::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>) -> Result<WebsocketStreamsHandle, Error> {
    init_user_agent("derivatives-trading-coin-futures");

    let client_config =
        get_client_configuration(profile, "derivatives-trading-coin-futures").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config
        .base_path_ws_streams
        .unwrap_or(match api_env.as_str() {
            "testnet" | "demo" => {
                DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_TESTNET_URL.to_string()
            }
            "prod" => DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_PROD_URL.to_string(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid api env, valid values: testnet, demo, prod",
                ));
            }
        });

    let builder = ConfigurationWebsocketStreams::builder().ws_url(base_path);

    let ws_config = builder
        .build()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    Ok(DerivativesTradingCoinFuturesWsStreams::from_config(
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
struct ContinuousContractKlineCandlestickStreamsArgs {
    #[arg(help = r#"The pair parameter"#, long)]
    pair: Option<String>,
    #[arg(help = r#"The contractType parameter"#, long)]
    contract_type: Option<ContinuousContractKlineCandlestickStreamsContractTypeEnum>,
    #[arg(help = r#"The interval parameter"#, long)]
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
struct DiffBookDepthStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
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
struct IndexKlineCandlestickStreamsArgs {
    #[arg(help = r#"The pair parameter"#, long)]
    pair: Option<String>,
    #[arg(help = r#"The interval parameter"#, long)]
    interval: Option<IndexKlineCandlestickStreamsIntervalEnum>,
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
struct IndexPriceStreamArgs {
    #[arg(help = r#"The pair parameter"#, long)]
    pair: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(help = r#"WebSocket stream update speed"#, long)]
    update_speed: Option<IndexPriceStreamUpdateSpeedEnum>,
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
struct MarkPriceKlineCandlestickStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"The interval parameter"#, long)]
    interval: Option<MarkPriceKlineCandlestickStreamsIntervalEnum>,
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
struct MarkPriceOfAllSymbolsOfAPairArgs {
    #[arg(help = r#"The pair parameter"#, long)]
    pair: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(help = r#"WebSocket stream update speed"#, long)]
    update_speed: Option<MarkPriceOfAllSymbolsOfAPairUpdateSpeedEnum>,
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
struct MarketLiquidationOrderStreamsArgs {
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

#[derive(Subcommand)]
pub enum DerivativesTradingCoinFuturesWebsocketStreamsCommands {
    #[command(
        about = decode_selected_entities(r#"The Aggregate Trade Streams push market trade information that is aggregated for fills with same price and taking side every 100 milliseconds.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 100ms"#, false),
    )]
    AggregateTradeStreams(AggregateTradeStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes any update to the best bid or ask's price or quantity in real-time for all symbols.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: Real-time"#, false),
    )]
    AllBookTickersStream(AllBookTickersStreamArgs),
    #[command(
        about = decode_selected_entities(r#"The All Liquidation Order Snapshot Streams push force liquidation order information for all symbols in the market. For each symbol，only the latest one liquidation order within 1000ms will be pushed as the snapshot. If no liquidation happens in the interval of 1000ms, no stream will be pushed.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 1000ms"#, false),
    )]
    AllMarketLiquidationOrderStreams(AllMarketLiquidationOrderStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window mini-ticker statistics for all symbols. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before. Note that only tickers that have changed will be present in the array.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 1000ms"#, false),
    )]
    AllMarketMiniTickersStream(AllMarketMiniTickersStreamArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window ticker statistics for all symbols. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before. Note that only tickers that have changed will be present in the array.

> **After CM migration**, this stream pushes the merged UM + CM universe (subscribable on both `fstream` and `dstream`); each payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 1000ms"#, false),
    )]
    AllMarketTickersStreams(AllMarketTickersStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Kline update every second

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
        about = decode_selected_entities(r#"Bids and asks, pushed every 250 milliseconds, 500 milliseconds, or 100 milliseconds

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 250ms or 500ms or 100ms"#, false),
    )]
    DiffBookDepthStreams(DiffBookDepthStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Index Kline/Candlestick Streams

> **After CM migration**, both `fstream` and `dstream` may subscribe to CM symbols on this stream.

Update Speed: 250ms"#, false),
    )]
    IndexKlineCandlestickStreams(IndexKlineCandlestickStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Index Price Stream

Update Speed: 3000ms OR 1000ms"#, false),
    )]
    IndexPriceStream(IndexPriceStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes any update to the best bid or ask's price or quantity in real-time for a specified symbol.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: Real-time"#, false),
    )]
    IndividualSymbolBookTickerStreams(IndividualSymbolBookTickerStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window mini-ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 500ms"#, false),
    )]
    IndividualSymbolMiniTickerStream(IndividualSymbolMiniTickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 500ms"#, false),
    )]
    IndividualSymbolTickerStreams(IndividualSymbolTickerStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"The Kline/Candlestick Stream push updates to the current klines/candlestick every 250 milliseconds (if existing).

> **After CM migration**, both `fstream` and `dstream` may subscribe to either UM or CM symbols on this stream.

Update Speed: 250ms"#, false),
    )]
    KlineCandlestickStreams(KlineCandlestickStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Mark Price Kline/Candlestick Streams

> **After CM migration**, both `fstream` and `dstream` may subscribe to CM symbols on this stream.

Update Speed: 250ms"#, false),
    )]
    MarkPriceKlineCandlestickStreams(MarkPriceKlineCandlestickStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Mark Price of All Symbols of a Pair

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM); both `fstream` and `dstream` may subscribe to either UM or CM symbols on this stream.

Update Speed: 3000ms OR 1000ms"#, false),
    )]
    MarkPriceOfAllSymbolsOfAPair(MarkPriceOfAllSymbolsOfAPairArgs),
    #[command(
        about = decode_selected_entities(r#"Mark price update stream

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM); both `fstream` and `dstream` may subscribe to either UM or CM symbols on this stream.

Update Speed: 3000ms OR 1000ms"#, false),
    )]
    MarkPriceStream(MarkPriceStreamArgs),
    #[command(
        about = decode_selected_entities(r#"The Liquidation Order Snapshot Streams push force liquidation order information for specific symbol. For each symbol，only the latest one liquidation order within 1000ms will be pushed as the snapshot. If no liquidation happens in the interval of 1000ms, no stream will be pushed.

Update Speed: 1000ms"#, false),
    )]
    MarketLiquidationOrderStreams(MarketLiquidationOrderStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Top levels bids and asks.

> **After CM migration**, the payload is appended with a new `st` field (`1` = UM, `2` = CM).

Update Speed: 250ms, 500ms or 100ms"#, false),
    )]
    PartialBookDepthStreams(PartialBookDepthStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribes to the user data WebSocket stream using the provided listen key."#, false),
    )]
    UserData(UserDataArgs),
}

pub async fn handle_derivatives_trading_coin_futures_ws_streams_command(
    command: DerivativesTradingCoinFuturesWebsocketStreamsCommands,
) -> anyhow::Result<()> {
    match command {

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::AggregateTradeStreams (args) => aggregate_trade_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::AllBookTickersStream (args) => all_book_tickers_stream(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::AllMarketLiquidationOrderStreams (args) => all_market_liquidation_order_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::AllMarketMiniTickersStream (args) => all_market_mini_tickers_stream(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::AllMarketTickersStreams (args) => all_market_tickers_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::ContinuousContractKlineCandlestickStreams (args) => continuous_contract_kline_candlestick_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::ContractInfoStream (args) => contract_info_stream(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::DiffBookDepthStreams (args) => diff_book_depth_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::IndexKlineCandlestickStreams (args) => index_kline_candlestick_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::IndexPriceStream (args) => index_price_stream(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::IndividualSymbolBookTickerStreams (args) => individual_symbol_book_ticker_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::IndividualSymbolMiniTickerStream (args) => individual_symbol_mini_ticker_stream(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::IndividualSymbolTickerStreams (args) => individual_symbol_ticker_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::KlineCandlestickStreams (args) => kline_candlestick_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::MarkPriceKlineCandlestickStreams (args) => mark_price_kline_candlestick_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::MarkPriceOfAllSymbolsOfAPair (args) => mark_price_of_all_symbols_of_a_pair(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::MarkPriceStream (args) => mark_price_stream(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::MarketLiquidationOrderStreams (args) => market_liquidation_order_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::PartialBookDepthStreams (args) => partial_book_depth_streams(args).await,

          DerivativesTradingCoinFuturesWebsocketStreamsCommands::UserData(args) => user_data(args).await,
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

async fn index_kline_candlestick_streams(
    mut args: IndexKlineCandlestickStreamsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<IndexKlineCandlestickStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<IndexKlineCandlestickStreamsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.pair.is_none() {
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

                        args.pair = Some(pair);
                    }
                    if args.interval.is_none() {
                        let options = vec![
                            ("1m", IndexKlineCandlestickStreamsIntervalEnum::Interval1m),
                            ("3m", IndexKlineCandlestickStreamsIntervalEnum::Interval3m),
                            ("5m", IndexKlineCandlestickStreamsIntervalEnum::Interval5m),
                            ("15m", IndexKlineCandlestickStreamsIntervalEnum::Interval15m),
                            ("30m", IndexKlineCandlestickStreamsIntervalEnum::Interval30m),
                            ("1h", IndexKlineCandlestickStreamsIntervalEnum::Interval1h),
                            ("2h", IndexKlineCandlestickStreamsIntervalEnum::Interval2h),
                            ("4h", IndexKlineCandlestickStreamsIntervalEnum::Interval4h),
                            ("6h", IndexKlineCandlestickStreamsIntervalEnum::Interval6h),
                            ("8h", IndexKlineCandlestickStreamsIntervalEnum::Interval8h),
                            ("12h", IndexKlineCandlestickStreamsIntervalEnum::Interval12h),
                            ("1d", IndexKlineCandlestickStreamsIntervalEnum::Interval1d),
                            ("3d", IndexKlineCandlestickStreamsIntervalEnum::Interval3d),
                            ("1w", IndexKlineCandlestickStreamsIntervalEnum::Interval1w),
                            ("1M", IndexKlineCandlestickStreamsIntervalEnum::Interval1M),
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
                IndexKlineCandlestickStreamsParams::builder(
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
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
    let stream = connection.index_kline_candlestick_streams(params).await?;

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

async fn index_price_stream(mut args: IndexPriceStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<IndexPriceStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<IndexPriceStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.pair.is_none() {
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

                        args.pair = Some(pair);
                    }
                }
                IndexPriceStreamParams::builder(
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
                )
                .id(args.id)
                .update_speed(args.update_speed)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.index_price_stream(params).await?;

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

async fn mark_price_kline_candlestick_streams(
    mut args: MarkPriceKlineCandlestickStreamsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params =
        match read_stdin_as::<MarkPriceKlineCandlestickStreamsParams>() {
            Some(params) => params,
            None => {
                match args.json {
                    Some(json) => read_json_as::<MarkPriceKlineCandlestickStreamsParams>(json)
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "failed to parse json params",
                            )
                        })?,
                    None => {
                        if args.interactive {
                            if args.symbol.is_none() {
                                let symbol: String =
                                    Input::new().with_prompt("Input symbol:").interact_text()?;

                                args.symbol = Some(symbol);
                            }
                            if args.interval.is_none() {
                                let options =
                                    vec![
                        ("1m", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval1m),
                        ("3m", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval3m),
                        ("5m", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval5m),
                        ("15m", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval15m),
                        ("30m", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval30m),
                        ("1h", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval1h),
                        ("2h", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval2h),
                        ("4h", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval4h),
                        ("6h", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval6h),
                        ("8h", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval8h),
                        ("12h", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval12h),
                        ("1d", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval1d),
                        ("3d", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval3d),
                        ("1w", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval1w),
                        ("1M", MarkPriceKlineCandlestickStreamsIntervalEnum::Interval1M),
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
                        MarkPriceKlineCandlestickStreamsParams::builder(
                            args.symbol
                                .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                            args.interval
                                .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                        )
                        .id(args.id)
                        .build()?
                    }
                }
            }
        };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection
        .mark_price_kline_candlestick_streams(params)
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

async fn mark_price_of_all_symbols_of_a_pair(
    mut args: MarkPriceOfAllSymbolsOfAPairArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<MarkPriceOfAllSymbolsOfAPairParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<MarkPriceOfAllSymbolsOfAPairParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.pair.is_none() {
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

                        args.pair = Some(pair);
                    }
                }
                MarkPriceOfAllSymbolsOfAPairParams::builder(
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
                )
                .id(args.id)
                .update_speed(args.update_speed)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection
        .mark_price_of_all_symbols_of_a_pair(params)
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

async fn market_liquidation_order_streams(
    mut args: MarketLiquidationOrderStreamsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<MarketLiquidationOrderStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<MarketLiquidationOrderStreamsParams>(json).ok_or_else(|| {
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
                MarketLiquidationOrderStreamsParams::builder(
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
    let stream = connection.market_liquidation_order_streams(params).await?;

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
