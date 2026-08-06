use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::c2c::C2CRestApi;
use binance_sdk::c2c::rest_api::*;
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::C2C_REST_API_PROD_URL;
use clap::{Args, Subcommand};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("c2c");

    let client_config = get_client_configuration(profile, "c2c").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => C2C_REST_API_PROD_URL.to_string(),
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

    Ok(C2CRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct GetC2CTradeHistoryArgs {
    #[arg(help = r#"Trade side filter"#, long)]
    trade_type: Option<GetC2CTradeHistoryTradeTypeEnum>,
    #[arg(help = r#""#, long)]
    start_timestamp: Option<i64>,
    #[arg(help = r#""#, long)]
    end_timestamp: Option<i64>,
    #[arg(help = r#"Page number"#, long)]
    page: Option<i64>,
    #[arg(help = r#"Number of records per page"#, long)]
    rows: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum C2CCommands {
    #[command(
        about = decode_selected_entities(r#"Get C2C Trade History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- The max interval between `startTimestamp` and `endTimestamp` is 30 days.
- If `startTimestamp` and `endTimestamp` are not sent, the recent 30 days' data is returned.
- You can only view data from the past 6 months. For all C2C orders, check `https://c2c.binance.com/en/fiatOrder`."#, false),
    )]
    GetC2CTradeHistory(GetC2CTradeHistoryArgs),
}

pub async fn handle_c2c_command(command: C2CCommands) -> anyhow::Result<()> {
    match command {
        C2CCommands::GetC2CTradeHistory(args) => get_c2_c_trade_history(args).await,
    }
}

async fn get_c2_c_trade_history(args: GetC2CTradeHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetC2CTradeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetC2CTradeHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetC2CTradeHistoryParams::builder()
                .trade_type(args.trade_type)
                .start_timestamp(args.start_timestamp)
                .end_timestamp(args.end_timestamp)
                .page(args.page)
                .rows(args.rows)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_c2_c_trade_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
