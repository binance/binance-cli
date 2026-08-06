use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as, wait_for_shutdown,
};
use binance_sdk::alpha::AlphaWsStreams;
use binance_sdk::alpha::websocket_streams::WebsocketStreamsHandle;
use binance_sdk::alpha::websocket_streams::*;
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::constants::ALPHA_WS_STREAMS_PROD_URL;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>) -> Result<WebsocketStreamsHandle, Error> {
    init_user_agent("alpha");

    let client_config = get_client_configuration(profile, "alpha").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config
        .base_path_ws_streams
        .unwrap_or(match api_env.as_str() {
            "prod" => ALPHA_WS_STREAMS_PROD_URL.to_string(),
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

    Ok(AlphaWsStreams::from_config(ws_config))
}

#[derive(Args, Debug)]
struct AggregateTradeStreamArgs {
    #[arg(help = r#"Symbol to subscribe, in lowercase stream format."#, long)]
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
struct AllBookTickerStreamArgs {
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
struct AllMiniTickerStreamArgs {
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
struct AllTickerStreamArgs {
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
struct AllTokens24hTickerStreamArgs {
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
struct BookTickerStreamArgs {
    #[arg(help = r#"Symbol to subscribe, in lowercase stream format."#, long)]
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
struct ContractKlineStreamArgs {
    #[arg(help = r#"Contract address."#, long)]
    contract_address: Option<String>,
    #[arg(help = r#"Chain ID."#, long)]
    chain_id: Option<String>,
    #[arg(help = r#"Kline interval."#, long)]
    interval: Option<ContractKlineStreamIntervalEnum>,
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
struct FullDepthStreamArgs {
    #[arg(help = r#"Symbol to subscribe, in lowercase stream format."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Update interval."#, long)]
    interval: Option<FullDepthStreamIntervalEnum>,
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
struct KlineStreamArgs {
    #[arg(help = r#"Symbol to subscribe, in lowercase stream format."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Kline interval."#, long)]
    interval: Option<KlineStreamIntervalEnum>,
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
struct MiniTickerStreamArgs {
    #[arg(help = r#"Symbol to subscribe, in lowercase stream format."#, long)]
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
struct PartialDepthStreamArgs {
    #[arg(help = r#"Symbol to subscribe, in lowercase stream format."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Depth levels."#, long)]
    levels: Option<PartialDepthStreamLevelsEnum>,
    #[arg(help = r#"Update interval."#, long)]
    interval: Option<PartialDepthStreamIntervalEnum>,
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
struct TickerStreamArgs {
    #[arg(help = r#"Symbol to subscribe, in lowercase stream format."#, long)]
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
struct TradeStreamArgs {
    #[arg(help = r#"Symbol to subscribe, in lowercase stream format."#, long)]
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
pub enum AlphaWebsocketStreamsCommands {
    #[command(
        about = decode_selected_entities(r#"Pushes aggregate trade updates for a symbol."#, false),
    )]
    AggregateTradeStream(AggregateTradeStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes best bid/ask updates for all symbols."#, false),
    )]
    AllBookTickerStream(AllBookTickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes mini ticker statistics for all symbols."#, false),
    )]
    AllMiniTickerStream(AllMiniTickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes full ticker statistics for all symbols."#, false),
    )]
    AllTickerStream(AllTickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes 24h ticker-like metrics for all tokens."#, false),
    )]
    AllTokens24hTickerStream(AllTokens24hTickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes best bid/ask updates for a symbol."#, false),
    )]
    BookTickerStream(BookTickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes kline updates by contractAddress@chainId."#, false),
    )]
    ContractKlineStream(ContractKlineStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Returns all available depth, including UI and API orders."#, false),
    )]
    FullDepthStream(FullDepthStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes kline updates for a symbol."#, false),
    )]
    KlineStream(KlineStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes 24h rolling mini ticker statistics."#, false),
    )]
    MiniTickerStream(MiniTickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes partial depth updates (UI orders only)."#, false),
    )]
    PartialDepthStream(PartialDepthStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes full 24h rolling ticker statistics."#, false),
    )]
    TickerStream(TickerStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Pushes raw trade updates for a symbol."#, false),
    )]
    TradeStream(TradeStreamArgs),
}

pub async fn handle_alpha_ws_streams_command(
    command: AlphaWebsocketStreamsCommands,
) -> anyhow::Result<()> {
    match command {
        AlphaWebsocketStreamsCommands::AggregateTradeStream(args) => {
            aggregate_trade_stream(args).await
        }

        AlphaWebsocketStreamsCommands::AllBookTickerStream(args) => {
            all_book_ticker_stream(args).await
        }

        AlphaWebsocketStreamsCommands::AllMiniTickerStream(args) => {
            all_mini_ticker_stream(args).await
        }

        AlphaWebsocketStreamsCommands::AllTickerStream(args) => all_ticker_stream(args).await,

        AlphaWebsocketStreamsCommands::AllTokens24hTickerStream(args) => {
            all_tokens24h_ticker_stream(args).await
        }

        AlphaWebsocketStreamsCommands::BookTickerStream(args) => book_ticker_stream(args).await,

        AlphaWebsocketStreamsCommands::ContractKlineStream(args) => {
            contract_kline_stream(args).await
        }

        AlphaWebsocketStreamsCommands::FullDepthStream(args) => full_depth_stream(args).await,

        AlphaWebsocketStreamsCommands::KlineStream(args) => kline_stream(args).await,

        AlphaWebsocketStreamsCommands::MiniTickerStream(args) => mini_ticker_stream(args).await,

        AlphaWebsocketStreamsCommands::PartialDepthStream(args) => partial_depth_stream(args).await,

        AlphaWebsocketStreamsCommands::TickerStream(args) => ticker_stream(args).await,

        AlphaWebsocketStreamsCommands::TradeStream(args) => trade_stream(args).await,
    }
}

async fn aggregate_trade_stream(mut args: AggregateTradeStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AggregateTradeStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AggregateTradeStreamParams>(json).ok_or_else(|| {
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
                AggregateTradeStreamParams::builder(
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
    let stream = connection.aggregate_trade_stream(params).await?;

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

async fn all_book_ticker_stream(args: AllBookTickerStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllBookTickerStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllBookTickerStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllBookTickerStreamParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_book_ticker_stream(params).await?;

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

async fn all_mini_ticker_stream(args: AllMiniTickerStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllMiniTickerStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllMiniTickerStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllMiniTickerStreamParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_mini_ticker_stream(params).await?;

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

async fn all_ticker_stream(args: AllTickerStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllTickerStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllTickerStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllTickerStreamParams::builder().id(args.id).build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_ticker_stream(params).await?;

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

async fn all_tokens24h_ticker_stream(args: AllTokens24hTickerStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<AllTokens24hTickerStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<AllTokens24hTickerStreamParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => AllTokens24hTickerStreamParams::builder()
                .id(args.id)
                .build()?,
        },
    };

    let connection = rest_client.connect().await?;

    // Make the API call
    let stream = connection.all_tokens24h_ticker_stream(params).await?;

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

async fn book_ticker_stream(mut args: BookTickerStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<BookTickerStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BookTickerStreamParams>(json).ok_or_else(|| {
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
                BookTickerStreamParams::builder(
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
    let stream = connection.book_ticker_stream(params).await?;

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

async fn contract_kline_stream(mut args: ContractKlineStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<ContractKlineStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ContractKlineStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.contract_address.is_none() {
                        let contract_address: String = Input::new()
                            .with_prompt("Input contract_address:")
                            .interact_text()?;

                        args.contract_address = Some(contract_address);
                    }
                    if args.chain_id.is_none() {
                        let chain_id: String = Input::new()
                            .with_prompt("Input chain_id:")
                            .interact_text()?;

                        args.chain_id = Some(chain_id);
                    }
                    if args.interval.is_none() {
                        let options = vec![
                            ("1s", ContractKlineStreamIntervalEnum::Interval1s),
                            ("1m", ContractKlineStreamIntervalEnum::Interval1m),
                            ("5m", ContractKlineStreamIntervalEnum::Interval5m),
                            ("15m", ContractKlineStreamIntervalEnum::Interval15m),
                            ("1h", ContractKlineStreamIntervalEnum::Interval1h),
                            ("4h", ContractKlineStreamIntervalEnum::Interval4h),
                            ("1d", ContractKlineStreamIntervalEnum::Interval1d),
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
                ContractKlineStreamParams::builder(
                    args.contract_address
                        .ok_or_else(|| anyhow::anyhow!("contract_address is required"))?,
                    args.chain_id
                        .ok_or_else(|| anyhow::anyhow!("chain_id is required"))?,
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
    let stream = connection.contract_kline_stream(params).await?;

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

async fn full_depth_stream(mut args: FullDepthStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<FullDepthStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FullDepthStreamParams>(json).ok_or_else(|| {
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
                            ("0ms", FullDepthStreamIntervalEnum::Interval0ms),
                            ("100ms", FullDepthStreamIntervalEnum::Interval100ms),
                            ("500ms", FullDepthStreamIntervalEnum::Interval500ms),
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
                FullDepthStreamParams::builder(
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
    let stream = connection.full_depth_stream(params).await?;

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
    let rest_client = get_client(args.profile.as_deref())?;

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
                        let options = vec![
                            ("1m", KlineStreamIntervalEnum::Interval1m),
                            ("3m", KlineStreamIntervalEnum::Interval3m),
                            ("5m", KlineStreamIntervalEnum::Interval5m),
                            ("15m", KlineStreamIntervalEnum::Interval15m),
                            ("30m", KlineStreamIntervalEnum::Interval30m),
                            ("1h", KlineStreamIntervalEnum::Interval1h),
                            ("2h", KlineStreamIntervalEnum::Interval2h),
                            ("4h", KlineStreamIntervalEnum::Interval4h),
                            ("6h", KlineStreamIntervalEnum::Interval6h),
                            ("8h", KlineStreamIntervalEnum::Interval8h),
                            ("12h", KlineStreamIntervalEnum::Interval12h),
                            ("1d", KlineStreamIntervalEnum::Interval1d),
                            ("3d", KlineStreamIntervalEnum::Interval3d),
                            ("1w", KlineStreamIntervalEnum::Interval1w),
                            ("1M", KlineStreamIntervalEnum::Interval1M),
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

    let connection = rest_client.connect().await?;

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

async fn mini_ticker_stream(mut args: MiniTickerStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<MiniTickerStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MiniTickerStreamParams>(json).ok_or_else(|| {
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
                MiniTickerStreamParams::builder(
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
    let stream = connection.mini_ticker_stream(params).await?;

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

async fn partial_depth_stream(mut args: PartialDepthStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<PartialDepthStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PartialDepthStreamParams>(json).ok_or_else(|| {
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
                            ("5", PartialDepthStreamLevelsEnum::Levels5),
                            ("10", PartialDepthStreamLevelsEnum::Levels10),
                            ("20", PartialDepthStreamLevelsEnum::Levels20),
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
                    if args.interval.is_none() {
                        let options = vec![
                            ("0ms", PartialDepthStreamIntervalEnum::Interval0ms),
                            ("100ms", PartialDepthStreamIntervalEnum::Interval100ms),
                            ("500ms", PartialDepthStreamIntervalEnum::Interval500ms),
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
                PartialDepthStreamParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.levels
                        .ok_or_else(|| anyhow::anyhow!("levels is required"))?,
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
    let stream = connection.partial_depth_stream(params).await?;

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

async fn ticker_stream(mut args: TickerStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<TickerStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TickerStreamParams>(json).ok_or_else(|| {
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
                TickerStreamParams::builder(
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
    let stream = connection.ticker_stream(params).await?;

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

async fn trade_stream(mut args: TradeStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref())?;

    let params = match read_stdin_as::<TradeStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TradeStreamParams>(json).ok_or_else(|| {
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
                TradeStreamParams::builder(
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
    let stream = connection.trade_stream(params).await?;

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
