use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::COPY_TRADING_REST_API_PROD_URL;
use binance_sdk::copy_trading::CopyTradingRestApi;
use binance_sdk::copy_trading::rest_api::*;
use clap::{Args, Subcommand};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("copy-trading");

    let client_config = get_client_configuration(profile, "copy-trading").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => COPY_TRADING_REST_API_PROD_URL.to_string(),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid api env, valid values: prod",
            ));
        }
    });

    let mut builder = ConfigurationRestApi::builder().base_path(base_path);

    if is_signed {
        builder = builder
            .api_key(client_config.api_key)
            .api_secret(client_config.api_secret);

        if client_config.private_key.is_some() {
            builder = builder.private_key(PrivateKey::File(client_config.private_key.unwrap()));
        }
    }

    let rest_conf = builder
        .build()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    Ok(CopyTradingRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct GetFuturesLeadTraderStatusArgs {
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFuturesLeadTradingSymbolWhitelistArgs {
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum CopyTradingCommands {
    #[command(
        about = decode_selected_entities(r#"Get Futures Lead Trader Status

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    GetFuturesLeadTraderStatus(GetFuturesLeadTraderStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get Futures Lead Trading Symbol Whitelist

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetFuturesLeadTradingSymbolWhitelist(GetFuturesLeadTradingSymbolWhitelistArgs),
}

pub async fn handle_copy_trading_command(command: CopyTradingCommands) -> anyhow::Result<()> {
    match command {
        CopyTradingCommands::GetFuturesLeadTraderStatus(args) => {
            get_futures_lead_trader_status(args).await
        }

        CopyTradingCommands::GetFuturesLeadTradingSymbolWhitelist(args) => {
            get_futures_lead_trading_symbol_whitelist(args).await
        }
    }
}

async fn get_futures_lead_trader_status(
    args: GetFuturesLeadTraderStatusArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFuturesLeadTraderStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFuturesLeadTraderStatusParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFuturesLeadTraderStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_futures_lead_trader_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_futures_lead_trading_symbol_whitelist(
    args: GetFuturesLeadTradingSymbolWhitelistArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFuturesLeadTradingSymbolWhitelistParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFuturesLeadTradingSymbolWhitelistParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetFuturesLeadTradingSymbolWhitelistParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_futures_lead_trading_symbol_whitelist(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
