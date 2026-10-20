use crate::utils::{wait_for_shutdown, init_user_agent, get_client_configuration, decode_selected_entities, read_json_as, read_stdin_as};
use binance_sdk::cms::websocket_streams::{self as models, *};
use binance_sdk::config::{ConfigurationWebsocketStreams, PrivateKey};
use binance_sdk::constants::{ CMS_WS_STREAMS_PROD_URL, };
use binance_sdk::cms::websocket_streams::WebsocketStreamsHandle;
use binance_sdk::cms::CmsWsStreams;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>) -> Result<WebsocketStreamsHandle, Error> {
    init_user_agent("cms");

    let client_config = get_client_configuration(profile, "cms").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path_ws_streams.unwrap_or(match api_env.as_str() {
        "prod" => CMS_WS_STREAMS_PROD_URL.to_string(),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid api env, valid values: prod",
            ));
        }
    });

    let mut builder = ConfigurationWebsocketStreams::builder().ws_url(base_path);

    builder = builder
    .api_key(client_config.api_key)
    .api_secret(client_config.api_secret);

    if client_config.private_key.is_some() {
        builder = builder.private_key(PrivateKey::File(client_config.private_key.unwrap()));
    }

    let ws_config = builder
        .build()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

        
    Ok(CmsWsStreams::from_config(ws_config))
}


#[derive(Args, Debug)]
struct ComAnnouncementEnArgs {
#[arg(help = r#"Select a profile"#, long)]
profile: Option<String>,
#[arg(help = r#"Pretty-print the JSON output"#, long)]
pretty: bool,
#[arg(help = r#"Stop listening after the given number of messages"#, long = "stream-limit")]
stream_limit: Option<u64>,
#[arg(help = r#"Stop listening after the given duration in milliseconds"#, long = "stream-duration")]
stream_duration: Option<u64>,
}


#[derive(Subcommand)]
pub enum CmsWebsocketStreamsCommands {
  
    #[command(
        about = decode_selected_entities(r#"Receive push notifications when Binance publishes announcements in English (language code: en)."#, false),
    )]
  ComAnnouncementEn(ComAnnouncementEnArgs),
}

pub async fn handle_cms_ws_streams_command(command: CmsWebsocketStreamsCommands) -> anyhow::Result<()> {
    match command {
        
          CmsWebsocketStreamsCommands::ComAnnouncementEn (args) => com_announcement_en(args).await,
        
    }
}


async fn com_announcement_en(mut args: ComAnnouncementEnArgs) -> anyhow::Result<()> {
    let ws_client = get_client(args.profile.as_deref())?;


    let connection = ws_client
        .connect()
        .await?;

    // Make the API call
    let stream = connection
        .com_announcement_en()
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
            let count = message_count_for_handler
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                + 1;

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

