use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::SIMPLE_EARN_REST_API_PROD_URL;
use binance_sdk::simple_earn::SimpleEarnRestApi;
use binance_sdk::simple_earn::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("simple-earn");

    let client_config = get_client_configuration(profile, "simple-earn").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => SIMPLE_EARN_REST_API_PROD_URL.to_string(),
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

    Ok(SimpleEarnRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct GetBfusdAccountArgs {
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetBfusdQuotaDetailsArgs {
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetBfusdRateHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetBfusdRedemptionHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetBfusdRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetBfusdSubscriptionHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<GetBfusdSubscriptionHistoryAssetEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RedeemBfusdArgs {
    #[arg(help = r#"Amount in BFUSD"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"defaults to STANDARD"#, long)]
    r#type: Option<RedeemBfusdTypeEnum>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SubscribeBfusdArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Amount"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetCollateralRecordArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFlexiblePersonalLeftQuotaArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFlexibleProductPositionArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFlexibleRedemptionRecordArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long)]
    redeem_id: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFlexibleRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(
        help = r#"`BONUS` - Bonus tiered APR, `REALTIME` - Real-time APR, `REWARDS` - Historical rewards, `ALL` - All types. Default: `ALL`"#,
        long
    )]
    r#type: Option<GetFlexibleRewardsHistoryTypeEnum>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFlexibleSubscriptionPreviewArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFlexibleSubscriptionRecordArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long)]
    purchase_id: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetLockedPersonalLeftQuotaArgs {
    #[arg(help = r#""#, long)]
    project_id: Option<String>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetLockedProductPositionArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long)]
    project_id: Option<String>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetLockedRedemptionRecordArgs {
    #[arg(help = r#""#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long)]
    redeem_id: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetLockedRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetLockedSubscriptionPreviewArgs {
    #[arg(help = r#""#, long)]
    project_id: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"default true."#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_subscribe: Option<bool>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetLockedSubscriptionRecordArgs {
    #[arg(help = r#""#, long)]
    purchase_id: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetRateHistoryArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long)]
    apr_period: Option<GetRateHistoryAprPeriodEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page"#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetSimpleEarnFlexibleProductListArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetSimpleEarnLockedProductListArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Currently querying page. Starts from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RedeemFlexibleProductArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    redeem_all: Option<bool>,
    #[arg(help = r#"if redeemAll is false, amount is mandatory"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    dest_account: Option<RedeemFlexibleProductDestAccountEnum>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RedeemLockedProductArgs {
    #[arg(help = r#"Locked product position ID"#, long)]
    position_id: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SetFlexibleAutoSubscribeArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_subscribe: Option<bool>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SetLockedAutoSubscribeArgs {
    #[arg(help = r#""#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_subscribe: Option<bool>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SetLockedProductRedeemOptionArgs {
    #[arg(help = r#""#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long)]
    redeem_to: Option<SetLockedProductRedeemOptionRedeemToEnum>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SimpleAccountArgs {
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SubscribeFlexibleProductArgs {
    #[arg(help = r#""#, long)]
    product_id: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_subscribe: Option<bool>,
    #[arg(help = r#""#, long)]
    source_account: Option<SubscribeFlexibleProductSourceAccountEnum>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SubscribeLockedProductArgs {
    #[arg(help = r#""#, long)]
    project_id: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_subscribe: Option<bool>,
    #[arg(help = r#""#, long)]
    source_account: Option<SubscribeLockedProductSourceAccountEnum>,
    #[arg(help = r#""#, long)]
    redeem_to: Option<SubscribeLockedProductRedeemToEnum>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetRwusdAccountArgs {
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetRwusdQuotaDetailsArgs {
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetRwusdRateHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page"#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetRwusdRedemptionHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page"#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetRwusdRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page"#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetRwusdSubscriptionHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<GetRwusdSubscriptionHistoryAssetEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page"#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000 (ms)"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RedeemRwusdArgs {
    #[arg(help = r#"Amount in RWUSD"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    r#type: Option<RedeemRwusdTypeEnum>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SubscribeRwusdArgs {
    #[arg(help = r#""#, long)]
    asset: Option<SubscribeRwusdAssetEnum>,
    #[arg(help = r#"Amount"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetYieldArenaActivitiesArgs {
    #[arg(
        help = r#"Locale tag for `title` and `description` (e.g. `en`, `zh-CN`, `pt-BR`). Default: `en`.
If the value is missing, malformed, or has no translation configured, content is returned in `en`."#,
        long
    )]
    lang: Option<String>,
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum SimpleEarnCommands {
    #[command(
        about = decode_selected_entities(r#"Get BFUSD account information.

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetBfusdAccount(GetBfusdAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get BFUSD quota details including subscription quota, fast redemption quota, and standard redemption quota.

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetBfusdQuotaDetails(GetBfusdQuotaDetailsArgs),
    #[command(
        about = decode_selected_entities(r#"Get BFUSD rate history sorted by descending order.

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 6 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, `endTime` will default to current time, and results from `startTime` onward will be returned.
- If
  `endTime` is sent but `startTime` is not sent, `startTime` defaults to the current time minus one month, and
  data between `startTime` and `endTime` will be returned."#, false),
    )]
    GetBfusdRateHistory(GetBfusdRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get BFUSD redemption history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 6 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, `endTime` will default to current time, and results from `startTime` onward will be returned.
- If
  `endTime` is sent but `startTime` is not sent, `startTime` defaults to the current time minus one month, and
  data between `startTime` and `endTime` will be returned."#, false),
    )]
    GetBfusdRedemptionHistory(GetBfusdRedemptionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get BFUSD rewards history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 6 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, `endTime` will default to current time, and results from `startTime` onward will be returned.
- If
  `endTime` is sent but `startTime` is not sent, `startTime` defaults to the current time minus one month, and
  data between `startTime` and `endTime` will be returned."#, false),
    )]
    GetBfusdRewardsHistory(GetBfusdRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get BFUSD subscription history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 6 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, `endTime` will default to current time, and results from `startTime` onward will be returned.
- If
  `endTime` is sent but `startTime` is not sent, `startTime` defaults to the current time advanced by one month,
  and data between `startTime` and `endTime` will be returned."#, false),
    )]
    GetBfusdSubscriptionHistory(GetBfusdSubscriptionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem BFUSD to USDT

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint.
- This API only supports BFUSD redemption to the Spot Account. Redemptions to the Funding Account or any other account type are not supported."#, false),
    )]
    RedeemBfusd(RedeemBfusdArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribe BFUSD

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint.
- This API only supports BFUSD subscription using assets held in the Spot Account. Subscriptions initiated from the Funding Account or any other account type are not supported."#, false),
    )]
    SubscribeBfusd(SubscribeBfusdArgs),
    #[command(
        about = decode_selected_entities(r#"Get Collateral Record

Weight(IP): 1

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 30 days.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetCollateralRecord(GetCollateralRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Personal Left Quota

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetFlexiblePersonalLeftQuota(GetFlexiblePersonalLeftQuotaArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Product Position

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetFlexibleProductPosition(GetFlexibleProductPositionArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Redemption Record

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 30 days.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetFlexibleRedemptionRecord(GetFlexibleRedemptionRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Rewards History

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 30 days.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetFlexibleRewardsHistory(GetFlexibleRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Subscription Preview

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetFlexibleSubscriptionPreview(GetFlexibleSubscriptionPreviewArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Subscription Record

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 30 days.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetFlexibleSubscriptionRecord(GetFlexibleSubscriptionRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Get Locked Personal Left Quota

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetLockedPersonalLeftQuota(GetLockedPersonalLeftQuotaArgs),
    #[command(
        about = decode_selected_entities(r#"Get Locked Product Position

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetLockedProductPosition(GetLockedProductPositionArgs),
    #[command(
        about = decode_selected_entities(r#"Get Locked Redemption Record

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 30 days.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetLockedRedemptionRecord(GetLockedRedemptionRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Get Locked Rewards History

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 30 days.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetLockedRewardsHistory(GetLockedRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Locked Subscription Preview

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetLockedSubscriptionPreview(GetLockedSubscriptionPreviewArgs),
    #[command(
        about = decode_selected_entities(r#"Get Locked Subscription Record

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 30 days.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetLockedSubscriptionRecord(GetLockedSubscriptionRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Get Rate History

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between startTime and endTime cannot be longer than 1 year.
- If `startTime` and `endTime` are
  both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetRateHistory(GetRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get available Simple Earn flexible product list

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetSimpleEarnFlexibleProductList(GetSimpleEarnFlexibleProductListArgs),
    #[command(
        about = decode_selected_entities(r#"Get Simple Earn Locked Product List

Weight(IP): 150

Security Type: USER_DATA

Notes:
- Get available Simple Earn locked product list"#, false),
    )]
    GetSimpleEarnLockedProductList(GetSimpleEarnLockedProductListArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem Flexible Product

Weight(IP): 1

Security Type: TRADE

Notes:
- You need to open `Enable Spot & Margin Trading` permission for the API Key which requests this endpoint."#, false),
    )]
    RedeemFlexibleProduct(RedeemFlexibleProductArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem Locked Product

Weight(IP): 1

Security Type: TRADE

Notes:
- You need to open `Enable Spot & Margin Trading` permission for the API Key which requests this endpoint."#, false),
    )]
    RedeemLockedProduct(RedeemLockedProductArgs),
    #[command(
        about = decode_selected_entities(r#"Set Flexible Auto Subscribe

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    SetFlexibleAutoSubscribe(SetFlexibleAutoSubscribeArgs),
    #[command(
        about = decode_selected_entities(r#"Set locked auto subscribe

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    SetLockedAutoSubscribe(SetLockedAutoSubscribeArgs),
    #[command(
        about = decode_selected_entities(r#"Set redeem option for Locked product

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    SetLockedProductRedeemOption(SetLockedProductRedeemOptionArgs),
    #[command(
        about = decode_selected_entities(r#"Simple Account query

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    SimpleAccount(SimpleAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribe Flexible Product

Weight(IP): 1

Security Type: TRADE

Notes:
- You need to open `Enable Spot & Margin Trading` permission for the API Key which requests this endpoint."#, false),
    )]
    SubscribeFlexibleProduct(SubscribeFlexibleProductArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribe Locked Product

Weight(IP): 1

Security Type: TRADE

Notes:
- You need to open `Enable Spot & Margin Trading` permission for the API Key which requests this endpoint."#, false),
    )]
    SubscribeLockedProduct(SubscribeLockedProductArgs),
    #[command(
        about = decode_selected_entities(r#"Get RWUSD account information.

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetRwusdAccount(GetRwusdAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get RWUSD quota details including subscription quota, fast redemption quota, and standard redemption quota.

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetRwusdQuotaDetails(GetRwusdQuotaDetailsArgs),
    #[command(
        about = decode_selected_entities(r#"Get RWUSD rate history sorted by descending order.

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 6 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, `endTime` will default to current time, and results from `startTime` onward will be returned.
- If
  `endTime` is sent but `startTime` is not sent, `startTime` defaults to the current time minus one month, and
  data between `startTime` and `endTime` will be returned."#, false),
    )]
    GetRwusdRateHistory(GetRwusdRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get RWUSD redemption history.

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 6 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, `endTime` will default to current time, and results from `startTime` onward will be returned.
- If
  `endTime` is sent but `startTime` is not sent, `startTime` defaults to the current time minus one month, and
  data between `startTime` and `endTime` will be returned."#, false),
    )]
    GetRwusdRedemptionHistory(GetRwusdRedemptionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get RWUSD rewards history.

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 6 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, `endTime` will default to current time, and results from `startTime` onward will be returned.
- If
  `endTime` is sent but `startTime` is not sent, `startTime` defaults to the current time minus one month, and
  data between `startTime` and `endTime` will be returned."#, false),
    )]
    GetRwusdRewardsHistory(GetRwusdRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get RWUSD subscription history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 6 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, `endTime` will default to current time, and results from `startTime` onward will be returned.
- If
  `endTime` is sent but `startTime` is not sent, `startTime` defaults to the current time advanced by one month,
  and data between `startTime` and `endTime` will be returned."#, false),
    )]
    GetRwusdSubscriptionHistory(GetRwusdSubscriptionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem RWUSD to USDC

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint.
- This API only supports RWUSD redemption to the Spot Account. Redemptions to the Funding Account or any other account type are not supported."#, false),
    )]
    RedeemRwusd(RedeemRwusdArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribe RWUSD

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint.
- This API only supports RWUSD subscription using assets held in the Spot Account. Subscriptions initiated from the Funding Account or any other account type are not supported."#, false),
    )]
    SubscribeRwusd(SubscribeRwusdArgs),
    #[command(
        about = decode_selected_entities(r#"Get the list of Earn Yield Arena giveaway activities currently available to the user.

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetYieldArenaActivities(GetYieldArenaActivitiesArgs),
}

pub async fn handle_simple_earn_command(command: SimpleEarnCommands) -> anyhow::Result<()> {
    match command {
        SimpleEarnCommands::GetBfusdAccount(args) => get_bfusd_account(args).await,

        SimpleEarnCommands::GetBfusdQuotaDetails(args) => get_bfusd_quota_details(args).await,

        SimpleEarnCommands::GetBfusdRateHistory(args) => get_bfusd_rate_history(args).await,

        SimpleEarnCommands::GetBfusdRedemptionHistory(args) => {
            get_bfusd_redemption_history(args).await
        }

        SimpleEarnCommands::GetBfusdRewardsHistory(args) => get_bfusd_rewards_history(args).await,

        SimpleEarnCommands::GetBfusdSubscriptionHistory(args) => {
            get_bfusd_subscription_history(args).await
        }

        SimpleEarnCommands::RedeemBfusd(args) => redeem_bfusd(args).await,

        SimpleEarnCommands::SubscribeBfusd(args) => subscribe_bfusd(args).await,

        SimpleEarnCommands::GetCollateralRecord(args) => get_collateral_record(args).await,

        SimpleEarnCommands::GetFlexiblePersonalLeftQuota(args) => {
            get_flexible_personal_left_quota(args).await
        }

        SimpleEarnCommands::GetFlexibleProductPosition(args) => {
            get_flexible_product_position(args).await
        }

        SimpleEarnCommands::GetFlexibleRedemptionRecord(args) => {
            get_flexible_redemption_record(args).await
        }

        SimpleEarnCommands::GetFlexibleRewardsHistory(args) => {
            get_flexible_rewards_history(args).await
        }

        SimpleEarnCommands::GetFlexibleSubscriptionPreview(args) => {
            get_flexible_subscription_preview(args).await
        }

        SimpleEarnCommands::GetFlexibleSubscriptionRecord(args) => {
            get_flexible_subscription_record(args).await
        }

        SimpleEarnCommands::GetLockedPersonalLeftQuota(args) => {
            get_locked_personal_left_quota(args).await
        }

        SimpleEarnCommands::GetLockedProductPosition(args) => {
            get_locked_product_position(args).await
        }

        SimpleEarnCommands::GetLockedRedemptionRecord(args) => {
            get_locked_redemption_record(args).await
        }

        SimpleEarnCommands::GetLockedRewardsHistory(args) => get_locked_rewards_history(args).await,

        SimpleEarnCommands::GetLockedSubscriptionPreview(args) => {
            get_locked_subscription_preview(args).await
        }

        SimpleEarnCommands::GetLockedSubscriptionRecord(args) => {
            get_locked_subscription_record(args).await
        }

        SimpleEarnCommands::GetRateHistory(args) => get_rate_history(args).await,

        SimpleEarnCommands::GetSimpleEarnFlexibleProductList(args) => {
            get_simple_earn_flexible_product_list(args).await
        }

        SimpleEarnCommands::GetSimpleEarnLockedProductList(args) => {
            get_simple_earn_locked_product_list(args).await
        }

        SimpleEarnCommands::RedeemFlexibleProduct(args) => redeem_flexible_product(args).await,

        SimpleEarnCommands::RedeemLockedProduct(args) => redeem_locked_product(args).await,

        SimpleEarnCommands::SetFlexibleAutoSubscribe(args) => {
            set_flexible_auto_subscribe(args).await
        }

        SimpleEarnCommands::SetLockedAutoSubscribe(args) => set_locked_auto_subscribe(args).await,

        SimpleEarnCommands::SetLockedProductRedeemOption(args) => {
            set_locked_product_redeem_option(args).await
        }

        SimpleEarnCommands::SimpleAccount(args) => simple_account(args).await,

        SimpleEarnCommands::SubscribeFlexibleProduct(args) => {
            subscribe_flexible_product(args).await
        }

        SimpleEarnCommands::SubscribeLockedProduct(args) => subscribe_locked_product(args).await,

        SimpleEarnCommands::GetRwusdAccount(args) => get_rwusd_account(args).await,

        SimpleEarnCommands::GetRwusdQuotaDetails(args) => get_rwusd_quota_details(args).await,

        SimpleEarnCommands::GetRwusdRateHistory(args) => get_rwusd_rate_history(args).await,

        SimpleEarnCommands::GetRwusdRedemptionHistory(args) => {
            get_rwusd_redemption_history(args).await
        }

        SimpleEarnCommands::GetRwusdRewardsHistory(args) => get_rwusd_rewards_history(args).await,

        SimpleEarnCommands::GetRwusdSubscriptionHistory(args) => {
            get_rwusd_subscription_history(args).await
        }

        SimpleEarnCommands::RedeemRwusd(args) => redeem_rwusd(args).await,

        SimpleEarnCommands::SubscribeRwusd(args) => subscribe_rwusd(args).await,

        SimpleEarnCommands::GetYieldArenaActivities(args) => get_yield_arena_activities(args).await,
    }
}

async fn get_bfusd_account(args: GetBfusdAccountArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBfusdAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBfusdAccountParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetBfusdAccountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bfusd_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_bfusd_quota_details(args: GetBfusdQuotaDetailsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBfusdQuotaDetailsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBfusdQuotaDetailsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetBfusdQuotaDetailsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bfusd_quota_details(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_bfusd_rate_history(args: GetBfusdRateHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBfusdRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBfusdRateHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetBfusdRateHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bfusd_rate_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_bfusd_redemption_history(args: GetBfusdRedemptionHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBfusdRedemptionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetBfusdRedemptionHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetBfusdRedemptionHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bfusd_redemption_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_bfusd_rewards_history(args: GetBfusdRewardsHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBfusdRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBfusdRewardsHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetBfusdRewardsHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bfusd_rewards_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_bfusd_subscription_history(
    args: GetBfusdSubscriptionHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBfusdSubscriptionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetBfusdSubscriptionHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetBfusdSubscriptionHistoryParams::builder()
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bfusd_subscription_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn redeem_bfusd(mut args: RedeemBfusdArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RedeemBfusdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RedeemBfusdParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.r#type.is_none() {
                        let options = vec![
                            ("FAST", RedeemBfusdTypeEnum::Fast),
                            ("STANDARD", RedeemBfusdTypeEnum::Standard),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the r#type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.r#type = Some(selected);
                    }
                }
                RedeemBfusdParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.redeem_bfusd(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn subscribe_bfusd(mut args: SubscribeBfusdArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubscribeBfusdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SubscribeBfusdParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String =
                            Input::new().with_prompt("Input asset:").interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                SubscribeBfusdParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.subscribe_bfusd(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_collateral_record(args: GetCollateralRecordArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCollateralRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetCollateralRecordParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetCollateralRecordParams::builder()
                .product_id(args.product_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_collateral_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_personal_left_quota(
    mut args: GetFlexiblePersonalLeftQuotaArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexiblePersonalLeftQuotaParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexiblePersonalLeftQuotaParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.product_id.is_none() {
                        let product_id: String = Input::new()
                            .with_prompt("Input product_id:")
                            .interact_text()?;

                        args.product_id = Some(product_id);
                    }
                }
                GetFlexiblePersonalLeftQuotaParams::builder(
                    args.product_id
                        .ok_or_else(|| anyhow::anyhow!("product_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_flexible_personal_left_quota(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_product_position(args: GetFlexibleProductPositionArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleProductPositionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleProductPositionParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFlexibleProductPositionParams::builder()
                .asset(args.asset)
                .product_id(args.product_id)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_flexible_product_position(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_redemption_record(
    args: GetFlexibleRedemptionRecordArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleRedemptionRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleRedemptionRecordParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFlexibleRedemptionRecordParams::builder()
                .product_id(args.product_id)
                .redeem_id(args.redeem_id)
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_flexible_redemption_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_rewards_history(args: GetFlexibleRewardsHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleRewardsHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFlexibleRewardsHistoryParams::builder()
                .product_id(args.product_id)
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .r#type(args.r#type)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_flexible_rewards_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_subscription_preview(
    mut args: GetFlexibleSubscriptionPreviewArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleSubscriptionPreviewParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleSubscriptionPreviewParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.product_id.is_none() {
                        let product_id: String = Input::new()
                            .with_prompt("Input product_id:")
                            .interact_text()?;

                        args.product_id = Some(product_id);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                GetFlexibleSubscriptionPreviewParams::builder(
                    args.product_id
                        .ok_or_else(|| anyhow::anyhow!("product_id is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_flexible_subscription_preview(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_subscription_record(
    args: GetFlexibleSubscriptionRecordArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleSubscriptionRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleSubscriptionRecordParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFlexibleSubscriptionRecordParams::builder()
                .product_id(args.product_id)
                .purchase_id(args.purchase_id)
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_flexible_subscription_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_locked_personal_left_quota(
    mut args: GetLockedPersonalLeftQuotaArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLockedPersonalLeftQuotaParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetLockedPersonalLeftQuotaParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.project_id.is_none() {
                        let project_id: String = Input::new()
                            .with_prompt("Input project_id:")
                            .interact_text()?;

                        args.project_id = Some(project_id);
                    }
                }
                GetLockedPersonalLeftQuotaParams::builder(
                    args.project_id
                        .ok_or_else(|| anyhow::anyhow!("project_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_locked_personal_left_quota(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_locked_product_position(args: GetLockedProductPositionArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLockedProductPositionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetLockedProductPositionParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetLockedProductPositionParams::builder()
                .asset(args.asset)
                .position_id(args.position_id)
                .project_id(args.project_id)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_locked_product_position(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_locked_redemption_record(args: GetLockedRedemptionRecordArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLockedRedemptionRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetLockedRedemptionRecordParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetLockedRedemptionRecordParams::builder()
                .position_id(args.position_id)
                .redeem_id(args.redeem_id)
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_locked_redemption_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_locked_rewards_history(args: GetLockedRewardsHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLockedRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetLockedRewardsHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetLockedRewardsHistoryParams::builder()
                .position_id(args.position_id)
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_locked_rewards_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_locked_subscription_preview(
    mut args: GetLockedSubscriptionPreviewArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLockedSubscriptionPreviewParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetLockedSubscriptionPreviewParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.project_id.is_none() {
                        let project_id: String = Input::new()
                            .with_prompt("Input project_id:")
                            .interact_text()?;

                        args.project_id = Some(project_id);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                GetLockedSubscriptionPreviewParams::builder(
                    args.project_id
                        .ok_or_else(|| anyhow::anyhow!("project_id is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .auto_subscribe(args.auto_subscribe)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_locked_subscription_preview(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_locked_subscription_record(
    args: GetLockedSubscriptionRecordArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLockedSubscriptionRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetLockedSubscriptionRecordParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetLockedSubscriptionRecordParams::builder()
                .purchase_id(args.purchase_id)
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_locked_subscription_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_rate_history(mut args: GetRateHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetRateHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.product_id.is_none() {
                        let product_id: String = Input::new()
                            .with_prompt("Input product_id:")
                            .interact_text()?;

                        args.product_id = Some(product_id);
                    }
                }
                GetRateHistoryParams::builder(
                    args.product_id
                        .ok_or_else(|| anyhow::anyhow!("product_id is required"))?,
                )
                .apr_period(args.apr_period)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_rate_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_simple_earn_flexible_product_list(
    args: GetSimpleEarnFlexibleProductListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSimpleEarnFlexibleProductListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSimpleEarnFlexibleProductListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetSimpleEarnFlexibleProductListParams::builder()
                .asset(args.asset)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_simple_earn_flexible_product_list(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_simple_earn_locked_product_list(
    args: GetSimpleEarnLockedProductListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSimpleEarnLockedProductListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSimpleEarnLockedProductListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetSimpleEarnLockedProductListParams::builder()
                .asset(args.asset)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_simple_earn_locked_product_list(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn redeem_flexible_product(mut args: RedeemFlexibleProductArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RedeemFlexibleProductParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RedeemFlexibleProductParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.product_id.is_none() {
                        let product_id: String = Input::new()
                            .with_prompt("Input product_id:")
                            .interact_text()?;

                        args.product_id = Some(product_id);
                    }
                }
                RedeemFlexibleProductParams::builder(
                    args.product_id
                        .ok_or_else(|| anyhow::anyhow!("product_id is required"))?,
                )
                .redeem_all(args.redeem_all)
                .amount(args.amount)
                .dest_account(args.dest_account)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.redeem_flexible_product(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn redeem_locked_product(mut args: RedeemLockedProductArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RedeemLockedProductParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RedeemLockedProductParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.position_id.is_none() {
                        let position_id: String = Input::new()
                            .with_prompt("Input position_id:")
                            .interact_text()?;

                        args.position_id = Some(position_id);
                    }
                }
                RedeemLockedProductParams::builder(
                    args.position_id
                        .ok_or_else(|| anyhow::anyhow!("position_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.redeem_locked_product(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_flexible_auto_subscribe(mut args: SetFlexibleAutoSubscribeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetFlexibleAutoSubscribeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SetFlexibleAutoSubscribeParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.product_id.is_none() {
                        let product_id: String = Input::new()
                            .with_prompt("Input product_id:")
                            .interact_text()?;

                        args.product_id = Some(product_id);
                    }
                    if args.auto_subscribe.is_none() {
                        let auto_subscribe: bool = Input::new()
                            .with_prompt("Input auto_subscribe:")
                            .interact_text()?;

                        args.auto_subscribe = Some(auto_subscribe);
                    }
                }
                SetFlexibleAutoSubscribeParams::builder(
                    args.product_id
                        .ok_or_else(|| anyhow::anyhow!("product_id is required"))?,
                    args.auto_subscribe
                        .ok_or_else(|| anyhow::anyhow!("auto_subscribe is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.set_flexible_auto_subscribe(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_locked_auto_subscribe(mut args: SetLockedAutoSubscribeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetLockedAutoSubscribeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SetLockedAutoSubscribeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.position_id.is_none() {
                        let position_id: String = Input::new()
                            .with_prompt("Input position_id:")
                            .interact_text()?;

                        args.position_id = Some(position_id);
                    }
                    if args.auto_subscribe.is_none() {
                        let auto_subscribe: bool = Input::new()
                            .with_prompt("Input auto_subscribe:")
                            .interact_text()?;

                        args.auto_subscribe = Some(auto_subscribe);
                    }
                }
                SetLockedAutoSubscribeParams::builder(
                    args.position_id
                        .ok_or_else(|| anyhow::anyhow!("position_id is required"))?,
                    args.auto_subscribe
                        .ok_or_else(|| anyhow::anyhow!("auto_subscribe is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.set_locked_auto_subscribe(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_locked_product_redeem_option(
    mut args: SetLockedProductRedeemOptionArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetLockedProductRedeemOptionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SetLockedProductRedeemOptionParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.position_id.is_none() {
                        let position_id: String = Input::new()
                            .with_prompt("Input position_id:")
                            .interact_text()?;

                        args.position_id = Some(position_id);
                    }
                    if args.redeem_to.is_none() {
                        let options = vec![
                            ("SPOT", SetLockedProductRedeemOptionRedeemToEnum::Spot),
                            (
                                "FLEXIBLE",
                                SetLockedProductRedeemOptionRedeemToEnum::Flexible,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the redeem_to")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.redeem_to = Some(selected);
                    }
                }
                SetLockedProductRedeemOptionParams::builder(
                    args.position_id
                        .ok_or_else(|| anyhow::anyhow!("position_id is required"))?,
                    args.redeem_to
                        .ok_or_else(|| anyhow::anyhow!("redeem_to is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.set_locked_product_redeem_option(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn simple_account(args: SimpleAccountArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SimpleAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SimpleAccountParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => SimpleAccountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.simple_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn subscribe_flexible_product(mut args: SubscribeFlexibleProductArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubscribeFlexibleProductParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SubscribeFlexibleProductParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.product_id.is_none() {
                        let product_id: String = Input::new()
                            .with_prompt("Input product_id:")
                            .interact_text()?;

                        args.product_id = Some(product_id);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                SubscribeFlexibleProductParams::builder(
                    args.product_id
                        .ok_or_else(|| anyhow::anyhow!("product_id is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .auto_subscribe(args.auto_subscribe)
                .source_account(args.source_account)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.subscribe_flexible_product(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn subscribe_locked_product(mut args: SubscribeLockedProductArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubscribeLockedProductParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SubscribeLockedProductParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.project_id.is_none() {
                        let project_id: String = Input::new()
                            .with_prompt("Input project_id:")
                            .interact_text()?;

                        args.project_id = Some(project_id);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                SubscribeLockedProductParams::builder(
                    args.project_id
                        .ok_or_else(|| anyhow::anyhow!("project_id is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .auto_subscribe(args.auto_subscribe)
                .source_account(args.source_account)
                .redeem_to(args.redeem_to)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.subscribe_locked_product(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_rwusd_account(args: GetRwusdAccountArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRwusdAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetRwusdAccountParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetRwusdAccountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_rwusd_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_rwusd_quota_details(args: GetRwusdQuotaDetailsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRwusdQuotaDetailsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetRwusdQuotaDetailsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetRwusdQuotaDetailsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_rwusd_quota_details(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_rwusd_rate_history(args: GetRwusdRateHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRwusdRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetRwusdRateHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetRwusdRateHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_rwusd_rate_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_rwusd_redemption_history(args: GetRwusdRedemptionHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRwusdRedemptionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetRwusdRedemptionHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetRwusdRedemptionHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_rwusd_redemption_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_rwusd_rewards_history(args: GetRwusdRewardsHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRwusdRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetRwusdRewardsHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetRwusdRewardsHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_rwusd_rewards_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_rwusd_subscription_history(
    args: GetRwusdSubscriptionHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRwusdSubscriptionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetRwusdSubscriptionHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetRwusdSubscriptionHistoryParams::builder()
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_rwusd_subscription_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn redeem_rwusd(mut args: RedeemRwusdArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RedeemRwusdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RedeemRwusdParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.r#type.is_none() {
                        let options = vec![
                            ("FAST", RedeemRwusdTypeEnum::Fast),
                            ("STANDARD", RedeemRwusdTypeEnum::Standard),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the r#type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.r#type = Some(selected);
                    }
                }
                RedeemRwusdParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.redeem_rwusd(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn subscribe_rwusd(mut args: SubscribeRwusdArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubscribeRwusdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SubscribeRwusdParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let options = vec![
                            ("USDT", SubscribeRwusdAssetEnum::Usdt),
                            ("USDC", SubscribeRwusdAssetEnum::Usdc),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the asset")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.asset = Some(selected);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                SubscribeRwusdParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.subscribe_rwusd(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_yield_arena_activities(args: GetYieldArenaActivitiesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetYieldArenaActivitiesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetYieldArenaActivitiesParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetYieldArenaActivitiesParams::builder()
                .lang(args.lang)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_yield_arena_activities(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
