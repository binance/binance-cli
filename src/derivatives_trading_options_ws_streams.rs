use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as, wait_for_shutdown,
};
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::constants::{
    DERIVATIVES_TRADING_OPTIONS_WS_STREAMS_PROD_URL,
    DERIVATIVES_TRADING_OPTIONS_WS_STREAMS_TESTNET_URL,
};
use binance_sdk::derivatives_trading_options::DerivativesTradingOptionsWsStreams;
use binance_sdk::derivatives_trading_options::websocket_streams::WebsocketStreamsHandle;
use binance_sdk::derivatives_trading_options::websocket_streams::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>) -> Result<WebsocketStreamsHandle, Error> {
    init_user_agent("derivatives-trading-options");

    let client_config = get_client_configuration(profile, "derivatives-trading-options").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config
        .base_path_ws_streams
        .unwrap_or(match api_env.as_str() {
            "testnet" | "demo" => DERIVATIVES_TRADING_OPTIONS_WS_STREAMS_TESTNET_URL.to_string(),
            "prod" => DERIVATIVES_TRADING_OPTIONS_WS_STREAMS_PROD_URL.to_string(),
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

    Ok(DerivativesTradingOptionsWsStreams::from_config(ws_config))
}

#[derive(Args, Debug)]
struct IndexPriceStreamsArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<u32>,
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
    id: Option<u32>,
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
struct NewSymbolInfoArgs {
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<u32>,
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
struct OpenInterestArgs {
    #[arg(help = r#"The underlying parameter"#, long)]
    underlying: Option<String>,
    #[arg(help = r#"The expirationDate parameter"#, long)]
    expiration_date: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<u32>,
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
struct OptionMarkPriceArgs {
    #[arg(help = r#"The underlying parameter"#, long)]
    underlying: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<u32>,
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
    #[arg(help = r#"WebSocket stream update speed"#, long)]
    update_speed: Option<DiffBookDepthStreamsUpdateSpeedEnum>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<u32>,
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
struct Hour24TickerArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<u32>,
    #[arg(help = r#"The expiration date parameter"#, long)]
    expiration_date: Option<String>,
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
    id: Option<u32>,
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
    #[arg(help = r#"The level parameter"#, long)]
    level: Option<PartialBookDepthStreamsLevelEnum>,
    #[arg(help = r#"WebSocket stream update speed"#, long)]
    update_speed: Option<PartialBookDepthStreamsUpdateSpeedEnum>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<u32>,
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
struct TradeStreamsArgs {
    #[arg(help = r#"The symbol parameter"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Unique WebSocket request ID."#, long)]
    id: Option<u32>,
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
pub enum DerivativesTradingOptionsWebsocketStreamsCommands {
    #[command(
        about = decode_selected_entities(r#"Underlying(e.g ETHUSDT) index stream.

Update Speed: 1000ms"#, false),
    )]
    IndexPriceStreams(IndexPriceStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"The Kline/Candlestick Stream push updates to the current klines/candlestick every 1000 milliseconds (if existing).

Update Speed: 1000ms"#, false),
    )]
    KlineCandlestickStreams(KlineCandlestickStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"New symbol listing stream.

Update Speed: 50ms"#, false),
    )]
    NewSymbolInfo(NewSymbolInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Option open interest for specific underlying asset on specific expiration date. E.g.[ethusdt@openInterest@221125](wss://fstream.binance.com/market/stream?streams=ethusdt@openInterest@221125)

Update Speed: 60s"#, false),
    )]
    OpenInterest(OpenInterestArgs),
    #[command(
        about = decode_selected_entities(r#"The mark price for all option symbols on specific underlying asset. E.g.[btcusdt@optionMarkPrice](wss://fstream.binance.com/market/stream?streams=btcusdt@optionMarkPrice)

Update Speed: 1000ms"#, false),
    )]
    OptionMarkPrice(OptionMarkPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Bids and asks, pushed every 500 milliseconds, 100 milliseconds (if existing)

Update Speed: 100ms or 500ms"#, false),
    )]
    DiffBookDepthStreams(DiffBookDepthStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"24hr ticker info for all symbols. Only symbols whose ticker info changed will be sent.

Update Speed: 1000ms"#, false),
    )]
    Hour24Ticker(Hour24TickerArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes any update to the best bid or ask's price or quantity in real-time for a specified symbol.

Update Speed: Real-Time"#, false),
    )]
    IndividualSymbolBookTickerStreams(IndividualSymbolBookTickerStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Top <levels> bids and asks. Valid <levels> are 5, 10, 20.

Update Speed: 100ms or 500ms"#, false),
    )]
    PartialBookDepthStreams(PartialBookDepthStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"The Trade Streams push raw trade information for specific symbol or underlying asset. E.g.[btcusdt@optionTrade](wss://fstream.binance.com/public/stream?streams=btcusdt@optionTrade)

Update Speed: 50ms"#, false),
    )]
    TradeStreams(TradeStreamsArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribes to the user data WebSocket stream using the provided listen key."#, false),
    )]
    UserData(UserDataArgs),
}

pub async fn handle_derivatives_trading_options_ws_streams_command(
    command: DerivativesTradingOptionsWebsocketStreamsCommands,
) -> anyhow::Result<()> {
    match command {
        DerivativesTradingOptionsWebsocketStreamsCommands::IndexPriceStreams(args) => {
            index_price_streams(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::KlineCandlestickStreams(args) => {
            kline_candlestick_streams(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::NewSymbolInfo(args) => {
            new_symbol_info(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::OpenInterest(args) => {
            open_interest(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::OptionMarkPrice(args) => {
            option_mark_price(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::DiffBookDepthStreams(args) => {
            diff_book_depth_streams(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::Hour24Ticker(args) => {
            hour24_ticker(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::IndividualSymbolBookTickerStreams(
            args,
        ) => individual_symbol_book_ticker_streams(args).await,

        DerivativesTradingOptionsWebsocketStreamsCommands::PartialBookDepthStreams(args) => {
            partial_book_depth_streams(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::TradeStreams(args) => {
            trade_streams(args).await
        }

        DerivativesTradingOptionsWebsocketStreamsCommands::UserData(args) => user_data(args).await,
    }
}

async fn index_price_streams(args: IndexPriceStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<IndexPriceStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<IndexPriceStreamsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => IndexPriceStreamsParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.index_price_streams(params).await?;

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
                            ("12h", KlineCandlestickStreamsIntervalEnum::Interval12h),
                            ("1d", KlineCandlestickStreamsIntervalEnum::Interval1d),
                            ("3d", KlineCandlestickStreamsIntervalEnum::Interval3d),
                            ("1w", KlineCandlestickStreamsIntervalEnum::Interval1w),
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

async fn new_symbol_info(args: NewSymbolInfoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<NewSymbolInfoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewSymbolInfoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => NewSymbolInfoParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.new_symbol_info(params).await?;

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

async fn open_interest(mut args: OpenInterestArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<OpenInterestParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OpenInterestParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.underlying.is_none() {
                        let underlying: String = Input::new()
                            .with_prompt("Input underlying:")
                            .interact_text()?;

                        args.underlying = Some(underlying);
                    }
                    if args.expiration_date.is_none() {
                        let expiration_date: String = Input::new()
                            .with_prompt("Input expiration_date:")
                            .interact_text()?;

                        args.expiration_date = Some(expiration_date);
                    }
                }
                OpenInterestParams::builder(
                    args.underlying
                        .ok_or_else(|| anyhow::anyhow!("underlying is required"))?,
                    args.expiration_date
                        .ok_or_else(|| anyhow::anyhow!("expiration_date is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.open_interest(params).await?;

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

async fn option_mark_price(mut args: OptionMarkPriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<OptionMarkPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OptionMarkPriceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.underlying.is_none() {
                        let underlying: String = Input::new()
                            .with_prompt("Input underlying:")
                            .interact_text()?;

                        args.underlying = Some(underlying);
                    }
                }
                OptionMarkPriceParams::builder(
                    args.underlying
                        .ok_or_else(|| anyhow::anyhow!("underlying is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.option_mark_price(params).await?;

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
                    if args.update_speed.is_none() {
                        let options = vec![
                            (
                                "100ms",
                                DiffBookDepthStreamsUpdateSpeedEnum::UpdateSpeed100ms,
                            ),
                            (
                                "500ms",
                                DiffBookDepthStreamsUpdateSpeedEnum::UpdateSpeed500ms,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the update_speed")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.update_speed = Some(selected);
                    }
                }
                DiffBookDepthStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.update_speed
                        .ok_or_else(|| anyhow::anyhow!("update_speed is required"))?,
                )
                .id(args.id)
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

async fn hour24_ticker(mut args: Hour24TickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<Hour24TickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<Hour24TickerParams>(json).ok_or_else(|| {
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
                Hour24TickerParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .expiration_date(args.expiration_date)
                .build()?
            }
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.hour24_ticker(params).await?;

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
                    if args.level.is_none() {
                        let options = vec![
                            ("5", PartialBookDepthStreamsLevelEnum::Level5),
                            ("10", PartialBookDepthStreamsLevelEnum::Level10),
                            ("20", PartialBookDepthStreamsLevelEnum::Level20),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the level")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.level = Some(selected);
                    }
                    if args.update_speed.is_none() {
                        let options = vec![
                            (
                                "100ms",
                                PartialBookDepthStreamsUpdateSpeedEnum::UpdateSpeed100ms,
                            ),
                            (
                                "500ms",
                                PartialBookDepthStreamsUpdateSpeedEnum::UpdateSpeed500ms,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the update_speed")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.update_speed = Some(selected);
                    }
                }
                PartialBookDepthStreamsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.level
                        .ok_or_else(|| anyhow::anyhow!("level is required"))?,
                    args.update_speed
                        .ok_or_else(|| anyhow::anyhow!("update_speed is required"))?,
                )
                .id(args.id)
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

async fn trade_streams(mut args: TradeStreamsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<TradeStreamsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TradeStreamsParams>(json).ok_or_else(|| {
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
                TradeStreamsParams::builder(
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
    let stream = connection.trade_streams(params).await?;

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
