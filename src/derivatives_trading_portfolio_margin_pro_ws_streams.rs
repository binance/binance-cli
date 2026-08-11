use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, wait_for_shutdown,
};
use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::constants::DERIVATIVES_TRADING_PORTFOLIO_MARGIN_PRO_WS_STREAMS_PROD_URL;
use binance_sdk::derivatives_trading_portfolio_margin_pro::DerivativesTradingPortfolioMarginProWsStreams;
use binance_sdk::derivatives_trading_portfolio_margin_pro::websocket_streams::WebsocketStreamsHandle;
use clap::Subcommand;
use dialoguer::Input;
use std::env;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>) -> Result<WebsocketStreamsHandle, Error> {
    init_user_agent("derivatives-trading-portfolio-margin-pro");

    let client_config =
        get_client_configuration(profile, "derivatives-trading-portfolio-margin-pro").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config
        .base_path_ws_streams
        .unwrap_or(match api_env.as_str() {
            "prod" => DERIVATIVES_TRADING_PORTFOLIO_MARGIN_PRO_WS_STREAMS_PROD_URL.to_string(),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid api env, valid values: prod",
                ));
            }
        });

    let builder = ConfigurationWebsocketStreams::builder().ws_url(base_path);

    let ws_config = builder
        .build()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    Ok(DerivativesTradingPortfolioMarginProWsStreams::from_config(
        ws_config,
    ))
}

#[derive(Subcommand)]
pub enum DerivativesTradingPortfolioMarginProWebsocketStreamsCommands {
    #[command(
        about = decode_selected_entities(r#"Subscribes to the user data WebSocket stream using the provided listen key."#, false),
    )]
    UserData(UserDataArgs),
}

pub async fn handle_derivatives_trading_portfolio_margin_pro_ws_streams_command(
    command: DerivativesTradingPortfolioMarginProWebsocketStreamsCommands,
) -> anyhow::Result<()> {
    match command {
        DerivativesTradingPortfolioMarginProWebsocketStreamsCommands::UserData(args) => {
            user_data(args).await
        }
    }
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
