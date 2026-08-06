use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::REBATE_REST_API_PROD_URL;
use binance_sdk::rebate::RebateRestApi;
use binance_sdk::rebate::rest_api::*;
use clap::{Args, Subcommand};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("rebate");

    let client_config = get_client_configuration(profile, "rebate").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => REBATE_REST_API_PROD_URL.to_string(),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid BINANCE_API_ENV",
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

    Ok(RebateRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct GetSpotRebateHistoryRecordsArgs {
    #[arg(help = r#"Start time in milliseconds."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time in milliseconds."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Page number."#, long)]
    page: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum RebateCommands {
    #[command(
        about = decode_selected_entities(r#"Get Spot Rebate History Records

Weight(UID): 12000

Security Type: USER_DATA

Notes:
- The max interval between `startTime` and `endTime` is 30 days.
- If `startTime` and `endTime` are not sent, the recent 7 days' data will be returned.
- The earliest supported `startTime` is June 10, 2020.
- Return up to 200 records per request."#, false),
    )]
    GetSpotRebateHistoryRecords(GetSpotRebateHistoryRecordsArgs),
}

pub async fn handle_rebate_command(command: RebateCommands) -> anyhow::Result<()> {
    match command {
        RebateCommands::GetSpotRebateHistoryRecords(args) => {
            get_spot_rebate_history_records(args).await
        }
    }
}

async fn get_spot_rebate_history_records(
    args: GetSpotRebateHistoryRecordsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSpotRebateHistoryRecordsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSpotRebateHistoryRecordsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetSpotRebateHistoryRecordsParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .page(args.page)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_spot_rebate_history_records(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
