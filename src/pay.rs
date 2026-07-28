use crate::utils::{
    build_user_agent, decode_selected_entities, get_configuration_rest_api, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::PAY_REST_API_PROD_URL;
use binance_sdk::pay::PayRestApi;
use binance_sdk::pay::rest_api::*;
use clap::{Args, Subcommand};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    unsafe {
        env::set_var("BINANCE_CONNECTOR_RUST_USER_AGENT", build_user_agent("pay"));
    }

    let config_rest_api = get_configuration_rest_api(profile, "pay").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(config_rest_api.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = match api_env.as_str() {
        "prod" => PAY_REST_API_PROD_URL,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid BINANCE_API_ENV",
            ));
        }
    };

    let mut builder = ConfigurationRestApi::builder().base_path(base_path);

    if is_signed {
        builder = builder
            .api_key(config_rest_api.api_key)
            .api_secret(config_rest_api.api_secret);

        if config_rest_api.private_key.is_some()  {
            builder = builder.private_key(PrivateKey::File(config_rest_api.private_key.unwrap()));
        }
    }

    let rest_conf = builder
        .build()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    Ok(PayRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct GetPayTradeHistoryArgs {
    #[arg(help = r#"Start time in milliseconds."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time in milliseconds."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of records to return."#, long)]
    limit: Option<i64>,
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
pub enum PayCommands {
    #[command(
        about = decode_selected_entities(r#"Get Pay Trade History

Weight(UID): 3000

Notes:
- If `startTime` and `endTime` are not sent, the recent 90 days' data will be returned.
- The max interval between `startTime` and `endTime` is 90 days.
- Support for querying orders within the last 18 months.
- `payerInfo` and `receiverInfo` return different fields in different `orderType` values:
  - C2C sender: `payerInfo=binanceId`; `receiverInfo=name, binanceId/accountId/email/countryCode/phoneNumber/mobileCode` (based on user input).
  - C2C receiver: `payerInfo=name`; `receiverInfo=binanceId`.
  - CRYPTO_BOX sender: `payerInfo=binanceId`; `receiverInfo=name` (always `"Crypto Box"`).
  - CRYPTO_BOX receiver: `payerInfo=name`; `receiverInfo=binanceId`.
  - PAY sender: `payerInfo=binanceId`; `receiverInfo=name`.
  - PAY receiver: `payerInfo=name`; `receiverInfo=binanceId, name`.
  - PAY_REFUND sender: `payerInfo=binanceId, name`; `receiverInfo=name, accountId`.
  - PAY_REFUND receiver: `payerInfo=name`; `receiverInfo=binanceId`.
  - PAYOUT sender: `payerInfo=binanceId, name`; `receiverInfo=name, accountId`.
  - PAYOUT receiver: `payerInfo=name`; `receiverInfo=binanceId`.
  - CRYPTO_BOX_RF receiver: `payerInfo=name` (always `"Crypto Box"`); `receiverInfo=binanceId`.
  - REMITTANCE sender: `payerInfo=binanceId`; `receiverInfo=name, institutionName, cardNumber, digitalWalletId`."#, false),
    )]
    GetPayTradeHistory(GetPayTradeHistoryArgs),
}

pub async fn handle_pay_command(command: PayCommands) -> anyhow::Result<()> {
    match command {
        PayCommands::GetPayTradeHistory(args) => get_pay_trade_history(args).await,
    }
}

async fn get_pay_trade_history(args: GetPayTradeHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetPayTradeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetPayTradeHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetPayTradeHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_pay_trade_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
