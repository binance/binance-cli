use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as, wait_for_shutdown,
};
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::constants::{
    SPOT_WS_STREAMS_DEMO_URL, SPOT_WS_STREAMS_PROD_URL, SPOT_WS_STREAMS_TESTNET_URL,
};
use binance_sdk::spot::SpotWsStreams;
use binance_sdk::spot::websocket_streams::WebsocketStreamsHandle;
use binance_sdk::spot::websocket_streams::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>) -> Result<WebsocketStreamsHandle, Error> {
    init_user_agent("spot");

    let client_config = get_client_configuration(profile, "spot").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config
        .base_path_ws_streams
        .unwrap_or(match api_env.as_str() {
            "testnet" => SPOT_WS_STREAMS_TESTNET_URL.to_string(),
            "demo" => SPOT_WS_STREAMS_DEMO_URL.to_string(),
            "prod" => SPOT_WS_STREAMS_PROD_URL.to_string(),
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

    Ok(SpotWsStreams::from_config(ws_config))
}

#[derive(Args, Debug)]
struct AggTradeArgs {
    #[arg(help = r#"Symbol to query"#, long)]
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
struct AllMarketRollingWindowTickerArgs {
    #[arg(help = r#""#, long)]
    window_size: Option<AllMarketRollingWindowTickerWindowSizeEnum>,
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
struct AllMiniTickerArgs {
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
struct AvgPriceArgs {
    #[arg(help = r#"Symbol to query"#, long)]
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
struct BlockTradeArgs {
    #[arg(help = r#"Symbol to query"#, long)]
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
struct BookTickerArgs {
    #[arg(help = r#"Symbol to query"#, long)]
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
struct DiffBookDepthArgs {
    #[arg(help = r#"Symbol to query"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(help = r#"Optional stream update speed suffix"#, long)]
    update_speed: Option<DiffBookDepthUpdateSpeedEnum>,
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
struct KlineArgs {
    #[arg(help = r#"Symbol to query"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    interval: Option<KlineIntervalEnum>,
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
struct KlineOffsetArgs {
    #[arg(help = r#"Symbol to query"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    interval: Option<KlineOffsetIntervalEnum>,
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
struct MiniTickerArgs {
    #[arg(help = r#"Symbol to query"#, long)]
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
struct PartialBookDepthArgs {
    #[arg(help = r#"Symbol to query"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    levels: Option<PartialBookDepthLevelsEnum>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<String>,
    #[arg(help = r#"Optional stream update speed suffix"#, long)]
    update_speed: Option<PartialBookDepthUpdateSpeedEnum>,
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
struct ReferencePriceArgs {
    #[arg(help = r#"Symbol to query"#, long)]
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
struct RollingWindowTickerArgs {
    #[arg(help = r#"Symbol to query"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    window_size: Option<RollingWindowTickerWindowSizeEnum>,
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
struct TickerArgs {
    #[arg(help = r#"Symbol to query"#, long)]
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
struct TradeArgs {
    #[arg(help = r#"Symbol to query"#, long)]
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
pub enum SpotWebsocketStreamsCommands {
    #[command(
        about = decode_selected_entities(r#"The Aggregate Trade Streams push trade information that is aggregated
for a single taker order.

Update Speed: Real-time"#, false),
    )]
    AggTrade(AggTradeArgs),
    #[command(
        about = decode_selected_entities(r#"Rolling window ticker statistics for all market symbols, computed over
multiple windows.

Note that only tickers that have changed will be present in the array.

Update Speed: 1000ms"#, false),
    )]
    AllMarketRollingWindowTicker(AllMarketRollingWindowTickerArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window mini-ticker statistics for all symbols that changed
in an array. These are NOT the statistics of the UTC day, but a 24hr
rolling window for the previous 24hrs. Note that only tickers that have
changed will be present in the array.

Update Speed: 1000ms"#, false),
    )]
    AllMiniTicker(AllMiniTickerArgs),
    #[command(
        about = decode_selected_entities(r#"Average price streams push changes in the average price over a fixed time interval.

Update Speed: 1000ms"#, false),
    )]
    AvgPrice(AvgPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Block Trade Streams push block trade information in real-time.

Update Speed: Real-time"#, false),
    )]
    BlockTrade(BlockTradeArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes any update to the best bid or ask's price or quantity in
real-time for a specified symbol.

Multiple `<symbol>@bookTicker` streams can be subscribed to over one
connection.

Update Speed: Real-time"#, false),
    )]
    BookTicker(BookTickerArgs),
    #[command(
        about = decode_selected_entities(r#"Order book price and quantity depth updates used to locally manage an order book.

Update Speed: 1000ms or 100ms"#, false),
    )]
    DiffBookDepth(DiffBookDepthArgs),
    #[command(
        about = decode_selected_entities(r#"The Kline/Candlestick Stream push updates to the current
klines/candlestick every second in `UTC+0` timezone

Update Speed: 1000ms for `1s`, 2000ms for the other intervals"#, false),
    )]
    Kline(KlineArgs),
    #[command(
        about = decode_selected_entities(r#"The Kline/Candlestick Stream push updates to the current
klines/candlestick every second in `UTC+8` timezone

**Kline/Candlestick chart intervals:**

Supported intervals: See Kline/Candlestick chart intervals

**UTC+8 timezone offset:**
  - Kline intervals open and close in the UTC+8 timezone. For example the 1d klines will open at the beginning of the UTC+8 day, and close at the end of the UTC+8 day.
  - Note that E (event time), t (start time) and T (close time) in the payload are Unix timestamps, which are always interpreted in UTC.

Update Speed: 1000ms for `1s`, 2000ms for the other intervals"#, false),
    )]
    KlineOffset(KlineOffsetArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window mini-ticker statistics. These are NOT the statistics
of the UTC day, but a 24hr rolling window for the previous 24hrs.

Update Speed: 1000ms"#, false),
    )]
    MiniTicker(MiniTickerArgs),
    #[command(
        about = decode_selected_entities(r#"Top **\<levels\>** bids and asks, pushed every second.

Update Speed: 1000ms or 100ms"#, false),
    )]
    PartialBookDepth(PartialBookDepthArgs),
    #[command(
        about = decode_selected_entities(r#"Reference price stream for a symbol.

Update Speed: 1000ms"#, false),
    )]
    ReferencePrice(ReferencePriceArgs),
    #[command(
        about = decode_selected_entities(r#"Rolling window ticker statistics for a single symbol, computed over
multiple windows.

**Note:** This stream is different from the `<symbol>@ticker` stream. The open time `"O"` always starts on a minute, while the closing time `"C"` is the current time
of the update. As such, the effective window might be up to 59999ms wider than `<window_size>`.

Update Speed: 1000ms"#, false),
    )]
    RollingWindowTicker(RollingWindowTickerArgs),
    #[command(
        about = decode_selected_entities(r#"24hr rolling window ticker statistics for a single symbol. These are NOT
the statistics of the UTC day, but a 24hr rolling window for the
previous 24hrs.

Update Speed: 1000ms"#, false),
    )]
    Ticker(TickerArgs),
    #[command(
        about = decode_selected_entities(r#"The Trade Streams push raw trade information; each trade has a unique
buyer and seller.

Update Speed: Real-time"#, false),
    )]
    Trade(TradeArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribes to the user data WebSocket stream using the provided listen key."#, false),
    )]
    UserData(UserDataArgs),
}

pub async fn handle_spot_ws_streams_command(
    command: SpotWebsocketStreamsCommands,
) -> anyhow::Result<()> {
    match command {
        SpotWebsocketStreamsCommands::AggTrade(args) => agg_trade(args).await,

        SpotWebsocketStreamsCommands::AllMarketRollingWindowTicker(args) => {
            all_market_rolling_window_ticker(args).await
        }

        SpotWebsocketStreamsCommands::AllMiniTicker(args) => all_mini_ticker(args).await,

        SpotWebsocketStreamsCommands::AvgPrice(args) => avg_price(args).await,

        SpotWebsocketStreamsCommands::BlockTrade(args) => block_trade(args).await,

        SpotWebsocketStreamsCommands::BookTicker(args) => book_ticker(args).await,

        SpotWebsocketStreamsCommands::DiffBookDepth(args) => diff_book_depth(args).await,

        SpotWebsocketStreamsCommands::Kline(args) => kline(args).await,

        SpotWebsocketStreamsCommands::KlineOffset(args) => kline_offset(args).await,

        SpotWebsocketStreamsCommands::MiniTicker(args) => mini_ticker(args).await,

        SpotWebsocketStreamsCommands::PartialBookDepth(args) => partial_book_depth(args).await,

        SpotWebsocketStreamsCommands::ReferencePrice(args) => reference_price(args).await,

        SpotWebsocketStreamsCommands::RollingWindowTicker(args) => {
            rolling_window_ticker(args).await
        }

        SpotWebsocketStreamsCommands::Ticker(args) => ticker(args).await,

        SpotWebsocketStreamsCommands::Trade(args) => trade(args).await,

        SpotWebsocketStreamsCommands::UserData(args) => user_data(args).await,
    }
}

async fn agg_trade(mut args: AggTradeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AggTradeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AggTradeParams>(json).ok_or_else(|| {
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
                AggTradeParams::builder(
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
    let stream = connection.agg_trade(params).await?;

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

async fn all_market_rolling_window_ticker(
    mut args: AllMarketRollingWindowTickerArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params =
        match read_stdin_as::<AllMarketRollingWindowTickerParams>() {
            Some(params) => params,
            None => {
                match args.json {
                    Some(json) => read_json_as::<AllMarketRollingWindowTickerParams>(json)
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "failed to parse json params",
                            )
                        })?,
                    None => {
                        if args.interactive {
                            if args.window_size.is_none() {
                                let options =
                                    vec![
                        ("1h", AllMarketRollingWindowTickerWindowSizeEnum::WindowSize1h),
                        ("4h", AllMarketRollingWindowTickerWindowSizeEnum::WindowSize4h),
                        ("1d", AllMarketRollingWindowTickerWindowSizeEnum::WindowSize1d),
                    ];

                                let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                                let selected = Select::new()
                                    .with_prompt("Please select the window_size")
                                    .items(&labels)
                                    .default(0)
                                    .interact()?;

                                let selected = options[selected].1.clone();

                                println!("Selected option: {:?}", selected);

                                args.window_size = Some(selected);
                            }
                        }
                        AllMarketRollingWindowTickerParams::builder(
                            args.window_size
                                .ok_or_else(|| anyhow::anyhow!("window_size is required"))?,
                        )
                        .id(args.id)
                        .build()?
                    }
                }
            }
        };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_market_rolling_window_ticker(params).await?;

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

async fn all_mini_ticker(args: AllMiniTickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllMiniTickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllMiniTickerParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllMiniTickerParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_mini_ticker(params).await?;

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

async fn avg_price(mut args: AvgPriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AvgPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AvgPriceParams>(json).ok_or_else(|| {
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
                AvgPriceParams::builder(
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
    let stream = connection.avg_price(params).await?;

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

async fn block_trade(mut args: BlockTradeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<BlockTradeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BlockTradeParams>(json).ok_or_else(|| {
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
                BlockTradeParams::builder(
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
    let stream = connection.block_trade(params).await?;

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

async fn book_ticker(mut args: BookTickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<BookTickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BookTickerParams>(json).ok_or_else(|| {
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
                BookTickerParams::builder(
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
    let stream = connection.book_ticker(params).await?;

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

async fn diff_book_depth(mut args: DiffBookDepthArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<DiffBookDepthParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DiffBookDepthParams>(json).ok_or_else(|| {
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
                DiffBookDepthParams::builder(
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
    let stream = connection.diff_book_depth(params).await?;

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

async fn kline(mut args: KlineArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<KlineParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<KlineParams>(json).ok_or_else(|| {
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
                            ("1s", KlineIntervalEnum::Interval1s),
                            ("1m", KlineIntervalEnum::Interval1m),
                            ("3m", KlineIntervalEnum::Interval3m),
                            ("5m", KlineIntervalEnum::Interval5m),
                            ("15m", KlineIntervalEnum::Interval15m),
                            ("30m", KlineIntervalEnum::Interval30m),
                            ("1h", KlineIntervalEnum::Interval1h),
                            ("2h", KlineIntervalEnum::Interval2h),
                            ("4h", KlineIntervalEnum::Interval4h),
                            ("6h", KlineIntervalEnum::Interval6h),
                            ("8h", KlineIntervalEnum::Interval8h),
                            ("12h", KlineIntervalEnum::Interval12h),
                            ("1d", KlineIntervalEnum::Interval1d),
                            ("3d", KlineIntervalEnum::Interval3d),
                            ("1w", KlineIntervalEnum::Interval1w),
                            ("1M", KlineIntervalEnum::Interval1M),
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
                KlineParams::builder(
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
    let stream = connection.kline(params).await?;

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

async fn kline_offset(mut args: KlineOffsetArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<KlineOffsetParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<KlineOffsetParams>(json).ok_or_else(|| {
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
                            ("1s", KlineOffsetIntervalEnum::Interval1s),
                            ("1m", KlineOffsetIntervalEnum::Interval1m),
                            ("3m", KlineOffsetIntervalEnum::Interval3m),
                            ("5m", KlineOffsetIntervalEnum::Interval5m),
                            ("15m", KlineOffsetIntervalEnum::Interval15m),
                            ("30m", KlineOffsetIntervalEnum::Interval30m),
                            ("1h", KlineOffsetIntervalEnum::Interval1h),
                            ("2h", KlineOffsetIntervalEnum::Interval2h),
                            ("4h", KlineOffsetIntervalEnum::Interval4h),
                            ("6h", KlineOffsetIntervalEnum::Interval6h),
                            ("8h", KlineOffsetIntervalEnum::Interval8h),
                            ("12h", KlineOffsetIntervalEnum::Interval12h),
                            ("1d", KlineOffsetIntervalEnum::Interval1d),
                            ("3d", KlineOffsetIntervalEnum::Interval3d),
                            ("1w", KlineOffsetIntervalEnum::Interval1w),
                            ("1M", KlineOffsetIntervalEnum::Interval1M),
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
                KlineOffsetParams::builder(
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
    let stream = connection.kline_offset(params).await?;

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

async fn mini_ticker(mut args: MiniTickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<MiniTickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MiniTickerParams>(json).ok_or_else(|| {
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
                MiniTickerParams::builder(
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
    let stream = connection.mini_ticker(params).await?;

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

async fn partial_book_depth(mut args: PartialBookDepthArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<PartialBookDepthParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PartialBookDepthParams>(json).ok_or_else(|| {
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
                            ("5", PartialBookDepthLevelsEnum::Levels5),
                            ("10", PartialBookDepthLevelsEnum::Levels10),
                            ("20", PartialBookDepthLevelsEnum::Levels20),
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
                PartialBookDepthParams::builder(
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
    let stream = connection.partial_book_depth(params).await?;

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

async fn reference_price(mut args: ReferencePriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<ReferencePriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ReferencePriceParams>(json).ok_or_else(|| {
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
                ReferencePriceParams::builder(
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
    let stream = connection.reference_price(params).await?;

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

async fn rolling_window_ticker(mut args: RollingWindowTickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<RollingWindowTickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RollingWindowTickerParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.window_size.is_none() {
                        let options = vec![
                            ("1h", RollingWindowTickerWindowSizeEnum::WindowSize1h),
                            ("4h", RollingWindowTickerWindowSizeEnum::WindowSize4h),
                            ("1d", RollingWindowTickerWindowSizeEnum::WindowSize1d),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the window_size")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.window_size = Some(selected);
                    }
                }
                RollingWindowTickerParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.window_size
                        .ok_or_else(|| anyhow::anyhow!("window_size is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.rolling_window_ticker(params).await?;

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

async fn ticker(mut args: TickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<TickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TickerParams>(json).ok_or_else(|| {
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
                TickerParams::builder(
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
    let stream = connection.ticker(params).await?;

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

async fn trade(mut args: TradeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<TradeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TradeParams>(json).ok_or_else(|| {
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
                TradeParams::builder(
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
    let stream = connection.trade(params).await?;

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
