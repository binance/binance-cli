use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as, wait_for_shutdown,
};
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::constants::STOCKS_WS_STREAMS_PROD_URL;
use binance_sdk::stocks::StocksWsStreams;
use binance_sdk::stocks::websocket_streams::WebsocketStreamsHandle;
use binance_sdk::stocks::websocket_streams::*;
use clap::{Args, Subcommand};
use dialoguer::Input;
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>) -> Result<WebsocketStreamsHandle, Error> {
    init_user_agent("stocks");

    let client_config = get_client_configuration(profile, "stocks").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config
        .base_path_ws_streams
        .unwrap_or(match api_env.as_str() {
            "prod" => STOCKS_WS_STREAMS_PROD_URL.to_string(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid api env, valid values: prod",
                ));
            }
        });

    let builder = ConfigurationWebsocketStreams::builder().ws_url(base_path);

    let ws_config = builder.build().map_err(Error::other)?;

    Ok(StocksWsStreams::from_config(ws_config))
}

#[derive(Args, Debug)]
struct CalendarStreamArgs {
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
struct KlineStreamArgs {
    #[arg(help = r#"US-equity ticker (UPPERCASE), e.g. `AAPL`."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Kline interval — `5m` / `1h` / `1d` / `1w` / `1M`."#, long)]
    interval: Option<String>,
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
struct PriceStreamArgs {
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
struct QuoteStreamArgs {
    #[arg(help = r#"US-equity ticker (UPPERCASE), e.g. `AAPL`."#, long)]
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
struct TradabilityStreamArgs {
    #[arg(help = r#"US-equity ticker (UPPERCASE), e.g. `AAPL`."#, long)]
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
struct TradingStatusStreamArgs {
    #[arg(help = r#"US-equity ticker (UPPERCASE), e.g. `AAPL`."#, long)]
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
struct OrderReportStreamArgs {
    #[arg(
        help = r#"User data listen key obtained from the Listen Key endpoint."#,
        long
    )]
    listen_key: Option<String>,
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
pub enum StocksWebsocketStreamsCommands {
    #[command(
        about = decode_selected_entities(r#"Single-stream broadcast of market-phase transitions. One message per transition; no periodic heartbeat payload. Server polls every 5 seconds. Also reachable via the SUBSCRIBE/UNSUBSCRIBE RPC — see Subscribing via RPC."#, false),
    )]
    CalendarStream(CalendarStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Per-symbol candlestick updates. One stream per (symbol, interval) combination. Supported intervals: 5m, 1h, 1d, 1w, 1M. Also reachable via the SUBSCRIBE/UNSUBSCRIBE RPC — see Subscribing via RPC."#, false),
    )]
    KlineStream(KlineStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Push-all price snapshot, polled every 3 seconds. One message carries the latest price for every active US-equity symbol. Also reachable via the SUBSCRIBE/UNSUBSCRIBE RPC — see Subscribing via RPC."#, false),
    )]
    PriceStream(PriceStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Per-symbol real-time best-bid / best-ask. Each symbol has its own stream. Per-symbol throttle: at most one push per symbol every 200 ms. Also reachable via the SUBSCRIBE/UNSUBSCRIBE RPC — see Subscribing via RPC."#, false),
    )]
    QuoteStream(QuoteStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Per-symbol push whenever the tradable direction of a symbol changes. Pushed only when the value actually changes (new value ≠ old value). Also reachable via the SUBSCRIBE/UNSUBSCRIBE RPC — see Subscribing via RPC."#, false),
    )]
    TradabilityStream(TradabilityStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Per-symbol trading-status transitions (halts, resumes, SSR, LULD pauses, etc.). Events that do not match a known status/reason rule are not pushed. Also reachable via the SUBSCRIBE/UNSUBSCRIBE RPC — see Subscribing via RPC."#, false),
    )]
    TradingStatusStream(TradingStatusStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Real-time push of the authenticated user's order state transitions — both open-state updates (`ORDER_UPDATE`) and terminal-state notifications (`ORDER_TERMINAL`). Prerequisite: obtain a `listenKey` via the Listen Key endpoint first."#, false),
    )]
    OrderReportStream(OrderReportStreamArgs),
}

pub async fn handle_stocks_ws_streams_command(
    command: StocksWebsocketStreamsCommands,
) -> anyhow::Result<()> {
    match command {
        StocksWebsocketStreamsCommands::CalendarStream(args) => calendar_stream(args).await,

        StocksWebsocketStreamsCommands::KlineStream(args) => kline_stream(args).await,

        StocksWebsocketStreamsCommands::PriceStream(args) => price_stream(args).await,

        StocksWebsocketStreamsCommands::QuoteStream(args) => quote_stream(args).await,

        StocksWebsocketStreamsCommands::TradabilityStream(args) => tradability_stream(args).await,

        StocksWebsocketStreamsCommands::TradingStatusStream(args) => {
            trading_status_stream(args).await
        }

        StocksWebsocketStreamsCommands::OrderReportStream(args) => order_report_stream(args).await,
    }
}

async fn calendar_stream(args: CalendarStreamArgs) -> anyhow::Result<()> {
    let ws_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<CalendarStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CalendarStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => CalendarStreamParams::builder().id(args.id).build()?,
        },
    };

    let connection = ws_client.connect().await?;

    // Make the API call
    let stream = connection.calendar_stream(params).await?;

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

async fn kline_stream(mut args: KlineStreamArgs) -> anyhow::Result<()> {
    let ws_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<KlineStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<KlineStreamParams>(json).ok_or_else(|| {
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
                        let interval: String = Input::new()
                            .with_prompt("Input interval:")
                            .interact_text()?;

                        args.interval = Some(interval);
                    }
                }
                KlineStreamParams::builder(
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

    let connection = ws_client.connect().await?;

    // Make the API call
    let stream = connection.kline_stream(params).await?;

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

async fn price_stream(args: PriceStreamArgs) -> anyhow::Result<()> {
    let ws_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<PriceStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PriceStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => PriceStreamParams::builder().id(args.id).build()?,
        },
    };

    let connection = ws_client.connect().await?;

    // Make the API call
    let stream = connection.price_stream(params).await?;

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

async fn quote_stream(mut args: QuoteStreamArgs) -> anyhow::Result<()> {
    let ws_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<QuoteStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuoteStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.symbol.is_none() {
                    let symbol: String =
                        Input::new().with_prompt("Input symbol:").interact_text()?;

                    args.symbol = Some(symbol);
                }
                QuoteStreamParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = ws_client.connect().await?;

    // Make the API call
    let stream = connection.quote_stream(params).await?;

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

async fn tradability_stream(mut args: TradabilityStreamArgs) -> anyhow::Result<()> {
    let ws_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<TradabilityStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TradabilityStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.symbol.is_none() {
                    let symbol: String =
                        Input::new().with_prompt("Input symbol:").interact_text()?;

                    args.symbol = Some(symbol);
                }
                TradabilityStreamParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = ws_client.connect().await?;

    // Make the API call
    let stream = connection.tradability_stream(params).await?;

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

async fn trading_status_stream(mut args: TradingStatusStreamArgs) -> anyhow::Result<()> {
    let ws_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<TradingStatusStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TradingStatusStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.symbol.is_none() {
                    let symbol: String =
                        Input::new().with_prompt("Input symbol:").interact_text()?;

                    args.symbol = Some(symbol);
                }
                TradingStatusStreamParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = ws_client.connect().await?;

    // Make the API call
    let stream = connection.trading_status_stream(params).await?;

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

async fn order_report_stream(mut args: OrderReportStreamArgs) -> anyhow::Result<()> {
    let ws_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<OrderReportStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderReportStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.listen_key.is_none() {
                    let listen_key: String = Input::new()
                        .with_prompt("Input listen_key:")
                        .interact_text()?;

                    args.listen_key = Some(listen_key);
                }
                OrderReportStreamParams::builder(
                    args.listen_key
                        .ok_or_else(|| anyhow::anyhow!("listen-key is required"))?,
                )
                .id(args.id)
                .build()?
            }
        },
    };

    let connection = ws_client.connect().await?;

    // Make the API call
    let stream = connection.order_report_stream(params).await?;

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
