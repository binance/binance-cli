use crate::utils::{
    build_user_agent, decode_selected_entities, get_configuration_rest_api, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::STAKING_REST_API_PROD_URL;
use binance_sdk::staking::StakingRestApi;
use binance_sdk::staking::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    unsafe {
        env::set_var(
            "BINANCE_CONNECTOR_RUST_USER_AGENT",
            build_user_agent("staking"),
        );
    }

    let config_rest_api = get_configuration_rest_api(profile, "staking").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(config_rest_api.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = match api_env.as_str() {
        "prod" => STAKING_REST_API_PROD_URL,
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

    Ok(StakingRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct EthStakingAccountArgs {
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
struct GetCurrentEthStakingQuotaArgs {
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
struct GetEthRedemptionHistoryArgs {
    #[arg(help = r#""#, long)]
    redeem_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetEthStakingHistoryArgs {
    #[arg(help = r#""#, long)]
    purchase_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetWbethRateHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetWbethRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetWbethUnwrapHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetWbethWrapHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct RedeemEthArgs {
    #[arg(help = r#"Amount in BETH, limit 8 decimals"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    asset: Option<RedeemEthAssetEnum>,
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
struct SubscribeEthStakingArgs {
    #[arg(help = r#"Amount in ETH, limit 4 decimals"#, long)]
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
struct WrapBethArgs {
    #[arg(help = r#"Amount in BETH, limit 4 decimals"#, long)]
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
struct GetOnChainYieldsLockedPersonalLeftQuotaArgs {
    #[arg(help = r#""#, long)]
    project_id: Option<String>,
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
struct GetOnChainYieldsLockedProductListArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
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
struct GetOnChainYieldsLockedProductPositionArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long)]
    project_id: Option<String>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results per page."#, long)]
    size: Option<i64>,
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
struct GetOnChainYieldsLockedRedemptionRecordArgs {
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
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetOnChainYieldsLockedRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetOnChainYieldsLockedSubscriptionPreviewArgs {
    #[arg(help = r#""#, long)]
    project_id: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
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
struct GetOnChainYieldsLockedSubscriptionRecordArgs {
    #[arg(help = r#""#, long)]
    purchase_id: Option<String>,
    #[arg(help = r#""#, long)]
    client_id: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct OnChainYieldsAccountArgs {
    #[arg(help = r#"The value cannot be greater than `60000`"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RedeemOnChainYieldsLockedProductArgs {
    #[arg(help = r#"Locked product position ID"#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long)]
    channel_id: Option<String>,
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
struct SetOnChainYieldsLockedAutoSubscribeArgs {
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
struct SetOnChainYieldsLockedProductRedeemOptionArgs {
    #[arg(help = r#""#, long)]
    position_id: Option<String>,
    #[arg(help = r#""#, long)]
    redeem_to: Option<SetOnChainYieldsLockedProductRedeemOptionRedeemToEnum>,
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
struct SubscribeOnChainYieldsLockedProductArgs {
    #[arg(help = r#""#, long)]
    project_id: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_subscribe: Option<bool>,
    #[arg(help = r#""#, long)]
    source_account: Option<SubscribeOnChainYieldsLockedProductSourceAccountEnum>,
    #[arg(help = r#"Takes effect when Auto Subscribe is false"#, long)]
    redeem_to: Option<SubscribeOnChainYieldsLockedProductRedeemToEnum>,
    #[arg(help = r#""#, long)]
    channel_id: Option<String>,
    #[arg(help = r#""#, long)]
    client_id: Option<String>,
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
struct GetSoftStakingProductListArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetSoftStakingRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct SetSoftStakingArgs {
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    soft_staking: Option<bool>,
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
struct ClaimBoostRewardsArgs {
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
struct GetBnsolRateHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
    #[arg(help = r#"The value cannot be greater than 60000"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetBnsolRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetBoostRewardsHistoryArgs {
    #[arg(help = r#""#, long)]
    r#type: Option<GetBoostRewardsHistoryTypeEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetSolRedemptionHistoryArgs {
    #[arg(help = r#""#, long)]
    redeem_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetSolStakingHistoryArgs {
    #[arg(help = r#""#, long)]
    purchase_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Currently querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct GetSolStakingQuotaDetailsArgs {
    #[arg(help = r#"The value cannot be greater than 60000"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetUnclaimedRewardsArgs {
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
struct RedeemSolArgs {
    #[arg(help = r#"Amount in BNSOL, limit 8 decimals"#, long)]
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
struct SolStakingAccountArgs {
    #[arg(help = r#"The value cannot be greater than 60000"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SubscribeSolStakingArgs {
    #[arg(help = r#"Amount in SOL."#, long)]
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

#[derive(Subcommand)]
pub enum StakingCommands {
    #[command(
        about = decode_selected_entities(r#"ETH Staking account

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    EthStakingAccount(EthStakingAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get current ETH staking quota

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetCurrentEthStakingQuota(GetCurrentEthStakingQuotaArgs),
    #[command(
        about = decode_selected_entities(r#"Get ETH redemption history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetEthRedemptionHistory(GetEthRedemptionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get ETH staking history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetEthStakingHistory(GetEthStakingHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get WBETH Rate History

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetWbethRateHistory(GetWbethRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get WBETH rewards history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetWbethRewardsHistory(GetWbethRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get WBETH unwrap history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetWbethUnwrapHistory(GetWbethUnwrapHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get WBETH wrap history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetWbethWrapHistory(GetWbethWrapHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem WBETH or BETH and get ETH

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    RedeemEth(RedeemEthArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribe ETH Staking

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    SubscribeEthStaking(SubscribeEthStakingArgs),
    #[command(
        about = decode_selected_entities(r#"Wrap BETH

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    WrapBeth(WrapBethArgs),
    #[command(
        about = decode_selected_entities(r#"Get On-chain Yields Locked Personal Left Quota

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    GetOnChainYieldsLockedPersonalLeftQuota(GetOnChainYieldsLockedPersonalLeftQuotaArgs),
    #[command(
        about = decode_selected_entities(r#"Get available On-chain Yields Locked product list

Weight(IP): 50

Security Type: USER_DATA

Notes:
- Get available On-chain Yields Locked product list"#, false),
    )]
    GetOnChainYieldsLockedProductList(GetOnChainYieldsLockedProductListArgs),
    #[command(
        about = decode_selected_entities(r#"Get On-chain Yields Locked Product Position

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    GetOnChainYieldsLockedProductPosition(GetOnChainYieldsLockedProductPositionArgs),
    #[command(
        about = decode_selected_entities(r#"Get On-chain Yields Locked Redemption Record

Weight(IP): 50

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetOnChainYieldsLockedRedemptionRecord(GetOnChainYieldsLockedRedemptionRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Get On-chain Yields Locked Rewards History

Weight(IP): 50

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetOnChainYieldsLockedRewardsHistory(GetOnChainYieldsLockedRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get On-chain Yields Locked Subscription Preview

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    GetOnChainYieldsLockedSubscriptionPreview(GetOnChainYieldsLockedSubscriptionPreviewArgs),
    #[command(
        about = decode_selected_entities(r#"Get On-chain Yields Locked Subscription Record

Weight(IP): 50

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetOnChainYieldsLockedSubscriptionRecord(GetOnChainYieldsLockedSubscriptionRecordArgs),
    #[command(
        about = decode_selected_entities(r#"On-chain Yields Account query

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    OnChainYieldsAccount(OnChainYieldsAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem On-chain Yields Locked Product

Weight(IP): 200

Security Type: TRADE

Notes:
- You need to open `Enable Spot & Margin Trading` permission for the API Key which requests this endpoint."#, false),
    )]
    RedeemOnChainYieldsLockedProduct(RedeemOnChainYieldsLockedProductArgs),
    #[command(
        about = decode_selected_entities(r#"Set On-chain Yield locked auto subscribe

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    SetOnChainYieldsLockedAutoSubscribe(SetOnChainYieldsLockedAutoSubscribeArgs),
    #[command(
        about = decode_selected_entities(r#"Set On-chain Yields redeem option for Locked product

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    SetOnChainYieldsLockedProductRedeemOption(SetOnChainYieldsLockedProductRedeemOptionArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribe On-chain Yields Locked Product

Weight(IP): 200

Security Type: TRADE

Notes:
- You need to open `Enable Spot & Margin Trading` permission for the API Key which requests this endpoint."#, false),
    )]
    SubscribeOnChainYieldsLockedProduct(SubscribeOnChainYieldsLockedProductArgs),
    #[command(
        about = decode_selected_entities(r#"Get the available Soft Staking product list.

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    GetSoftStakingProductList(GetSoftStakingProductListArgs),
    #[command(
        about = decode_selected_entities(r#"Get Soft Staking Rewards History

Weight(IP): 50

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetSoftStakingRewardsHistory(GetSoftStakingRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Enable or disable Soft Staking.

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    SetSoftStaking(SetSoftStakingArgs),
    #[command(
        about = decode_selected_entities(r#"Claim Boost APR Airdrop Rewards

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    ClaimBoostRewards(ClaimBoostRewardsArgs),
    #[command(
        about = decode_selected_entities(r#"Get BNSOL Rate History

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetBnsolRateHistory(GetBnsolRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get BNSOL rewards history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetBnsolRewardsHistory(GetBnsolRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Boost rewards history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetBoostRewardsHistory(GetBoostRewardsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get SOL redemption history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetSolRedemptionHistory(GetSolRedemptionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get SOL staking history

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetSolStakingHistory(GetSolStakingHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get SOL staking quota

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    GetSolStakingQuotaDetails(GetSolStakingQuotaDetailsArgs),
    #[command(
        about = decode_selected_entities(r#"Get Unclaimed rewards

Weight(IP): 150

Security Type: USER_DATA

Notes:
- The time between `startTime` and `endTime` cannot be longer than 3 months.
- If `startTime` and `endTime`
  are both not sent, then the last 30 days' data will be returned.
- If `startTime` is sent but `endTime` is not
  sent, the next 30 days' data beginning from `startTime` will be returned.
- If `endTime` is sent but
  `startTime` is not sent, the 30 days' data before `endTime` will be returned."#, false),
    )]
    GetUnclaimedRewards(GetUnclaimedRewardsArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem BNSOL get SOL

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    RedeemSol(RedeemSolArgs),
    #[command(
        about = decode_selected_entities(r#"SOL Staking account

Weight(IP): 150

Security Type: USER_DATA"#, false),
    )]
    SolStakingAccount(SolStakingAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribe SOL Staking

Weight(IP): 150

Security Type: TRADE

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    SubscribeSolStaking(SubscribeSolStakingArgs),
}

pub async fn handle_staking_command(command: StakingCommands) -> anyhow::Result<()> {
    match command {
        StakingCommands::EthStakingAccount(args) => eth_staking_account(args).await,

        StakingCommands::GetCurrentEthStakingQuota(args) => {
            get_current_eth_staking_quota(args).await
        }

        StakingCommands::GetEthRedemptionHistory(args) => get_eth_redemption_history(args).await,

        StakingCommands::GetEthStakingHistory(args) => get_eth_staking_history(args).await,

        StakingCommands::GetWbethRateHistory(args) => get_wbeth_rate_history(args).await,

        StakingCommands::GetWbethRewardsHistory(args) => get_wbeth_rewards_history(args).await,

        StakingCommands::GetWbethUnwrapHistory(args) => get_wbeth_unwrap_history(args).await,

        StakingCommands::GetWbethWrapHistory(args) => get_wbeth_wrap_history(args).await,

        StakingCommands::RedeemEth(args) => redeem_eth(args).await,

        StakingCommands::SubscribeEthStaking(args) => subscribe_eth_staking(args).await,

        StakingCommands::WrapBeth(args) => wrap_beth(args).await,

        StakingCommands::GetOnChainYieldsLockedPersonalLeftQuota(args) => {
            get_on_chain_yields_locked_personal_left_quota(args).await
        }

        StakingCommands::GetOnChainYieldsLockedProductList(args) => {
            get_on_chain_yields_locked_product_list(args).await
        }

        StakingCommands::GetOnChainYieldsLockedProductPosition(args) => {
            get_on_chain_yields_locked_product_position(args).await
        }

        StakingCommands::GetOnChainYieldsLockedRedemptionRecord(args) => {
            get_on_chain_yields_locked_redemption_record(args).await
        }

        StakingCommands::GetOnChainYieldsLockedRewardsHistory(args) => {
            get_on_chain_yields_locked_rewards_history(args).await
        }

        StakingCommands::GetOnChainYieldsLockedSubscriptionPreview(args) => {
            get_on_chain_yields_locked_subscription_preview(args).await
        }

        StakingCommands::GetOnChainYieldsLockedSubscriptionRecord(args) => {
            get_on_chain_yields_locked_subscription_record(args).await
        }

        StakingCommands::OnChainYieldsAccount(args) => on_chain_yields_account(args).await,

        StakingCommands::RedeemOnChainYieldsLockedProduct(args) => {
            redeem_on_chain_yields_locked_product(args).await
        }

        StakingCommands::SetOnChainYieldsLockedAutoSubscribe(args) => {
            set_on_chain_yields_locked_auto_subscribe(args).await
        }

        StakingCommands::SetOnChainYieldsLockedProductRedeemOption(args) => {
            set_on_chain_yields_locked_product_redeem_option(args).await
        }

        StakingCommands::SubscribeOnChainYieldsLockedProduct(args) => {
            subscribe_on_chain_yields_locked_product(args).await
        }

        StakingCommands::GetSoftStakingProductList(args) => {
            get_soft_staking_product_list(args).await
        }

        StakingCommands::GetSoftStakingRewardsHistory(args) => {
            get_soft_staking_rewards_history(args).await
        }

        StakingCommands::SetSoftStaking(args) => set_soft_staking(args).await,

        StakingCommands::ClaimBoostRewards(args) => claim_boost_rewards(args).await,

        StakingCommands::GetBnsolRateHistory(args) => get_bnsol_rate_history(args).await,

        StakingCommands::GetBnsolRewardsHistory(args) => get_bnsol_rewards_history(args).await,

        StakingCommands::GetBoostRewardsHistory(args) => get_boost_rewards_history(args).await,

        StakingCommands::GetSolRedemptionHistory(args) => get_sol_redemption_history(args).await,

        StakingCommands::GetSolStakingHistory(args) => get_sol_staking_history(args).await,

        StakingCommands::GetSolStakingQuotaDetails(args) => {
            get_sol_staking_quota_details(args).await
        }

        StakingCommands::GetUnclaimedRewards(args) => get_unclaimed_rewards(args).await,

        StakingCommands::RedeemSol(args) => redeem_sol(args).await,

        StakingCommands::SolStakingAccount(args) => sol_staking_account(args).await,

        StakingCommands::SubscribeSolStaking(args) => subscribe_sol_staking(args).await,
    }
}

async fn eth_staking_account(args: EthStakingAccountArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EthStakingAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<EthStakingAccountParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => EthStakingAccountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.eth_staking_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_current_eth_staking_quota(
    args: GetCurrentEthStakingQuotaArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCurrentEthStakingQuotaParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetCurrentEthStakingQuotaParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetCurrentEthStakingQuotaParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_current_eth_staking_quota(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_eth_redemption_history(args: GetEthRedemptionHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetEthRedemptionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetEthRedemptionHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetEthRedemptionHistoryParams::builder()
                .redeem_id(args.redeem_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_eth_redemption_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_eth_staking_history(args: GetEthStakingHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetEthStakingHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetEthStakingHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetEthStakingHistoryParams::builder()
                .purchase_id(args.purchase_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_eth_staking_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_wbeth_rate_history(args: GetWbethRateHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetWbethRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetWbethRateHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetWbethRateHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_wbeth_rate_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_wbeth_rewards_history(args: GetWbethRewardsHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetWbethRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetWbethRewardsHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetWbethRewardsHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_wbeth_rewards_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_wbeth_unwrap_history(args: GetWbethUnwrapHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetWbethUnwrapHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetWbethUnwrapHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetWbethUnwrapHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_wbeth_unwrap_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_wbeth_wrap_history(args: GetWbethWrapHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetWbethWrapHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetWbethWrapHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetWbethWrapHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_wbeth_wrap_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn redeem_eth(mut args: RedeemEthArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RedeemEthParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RedeemEthParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                RedeemEthParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .asset(args.asset)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.redeem_eth(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn subscribe_eth_staking(mut args: SubscribeEthStakingArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubscribeEthStakingParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SubscribeEthStakingParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                SubscribeEthStakingParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.subscribe_eth_staking(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn wrap_beth(mut args: WrapBethArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<WrapBethParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<WrapBethParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                WrapBethParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.wrap_beth(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_on_chain_yields_locked_personal_left_quota(
    mut args: GetOnChainYieldsLockedPersonalLeftQuotaArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOnChainYieldsLockedPersonalLeftQuotaParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOnChainYieldsLockedPersonalLeftQuotaParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.project_id.is_none() {
                        let project_id: String = Input::new()
                            .with_prompt("Please enter the project_id name")
                            .interact_text()?;

                        args.project_id = Some(project_id);
                    }
                }
                GetOnChainYieldsLockedPersonalLeftQuotaParams::builder(
                    args.project_id
                        .ok_or_else(|| anyhow::anyhow!("project_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_on_chain_yields_locked_personal_left_quota(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_on_chain_yields_locked_product_list(
    args: GetOnChainYieldsLockedProductListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOnChainYieldsLockedProductListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOnChainYieldsLockedProductListParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetOnChainYieldsLockedProductListParams::builder()
                .asset(args.asset)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_on_chain_yields_locked_product_list(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_on_chain_yields_locked_product_position(
    args: GetOnChainYieldsLockedProductPositionArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOnChainYieldsLockedProductPositionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOnChainYieldsLockedProductPositionParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetOnChainYieldsLockedProductPositionParams::builder()
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
    let response = rest_client
        .get_on_chain_yields_locked_product_position(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_on_chain_yields_locked_redemption_record(
    args: GetOnChainYieldsLockedRedemptionRecordArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOnChainYieldsLockedRedemptionRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOnChainYieldsLockedRedemptionRecordParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetOnChainYieldsLockedRedemptionRecordParams::builder()
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
    let response = rest_client
        .get_on_chain_yields_locked_redemption_record(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_on_chain_yields_locked_rewards_history(
    args: GetOnChainYieldsLockedRewardsHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOnChainYieldsLockedRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOnChainYieldsLockedRewardsHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetOnChainYieldsLockedRewardsHistoryParams::builder()
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
    let response = rest_client
        .get_on_chain_yields_locked_rewards_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_on_chain_yields_locked_subscription_preview(
    mut args: GetOnChainYieldsLockedSubscriptionPreviewArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOnChainYieldsLockedSubscriptionPreviewParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOnChainYieldsLockedSubscriptionPreviewParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.project_id.is_none() {
                        let project_id: String = Input::new()
                            .with_prompt("Please enter the project_id name")
                            .interact_text()?;

                        args.project_id = Some(project_id);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                GetOnChainYieldsLockedSubscriptionPreviewParams::builder(
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
    let response = rest_client
        .get_on_chain_yields_locked_subscription_preview(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_on_chain_yields_locked_subscription_record(
    args: GetOnChainYieldsLockedSubscriptionRecordArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOnChainYieldsLockedSubscriptionRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOnChainYieldsLockedSubscriptionRecordParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetOnChainYieldsLockedSubscriptionRecordParams::builder()
                .purchase_id(args.purchase_id)
                .client_id(args.client_id)
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
    let response = rest_client
        .get_on_chain_yields_locked_subscription_record(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn on_chain_yields_account(args: OnChainYieldsAccountArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OnChainYieldsAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OnChainYieldsAccountParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => OnChainYieldsAccountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.on_chain_yields_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn redeem_on_chain_yields_locked_product(
    mut args: RedeemOnChainYieldsLockedProductArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RedeemOnChainYieldsLockedProductParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<RedeemOnChainYieldsLockedProductParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.position_id.is_none() {
                        let position_id: String = Input::new()
                            .with_prompt("Please enter the position_id name")
                            .interact_text()?;

                        args.position_id = Some(position_id);
                    }
                }
                RedeemOnChainYieldsLockedProductParams::builder(
                    args.position_id
                        .ok_or_else(|| anyhow::anyhow!("position_id is required"))?,
                )
                .channel_id(args.channel_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .redeem_on_chain_yields_locked_product(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_on_chain_yields_locked_auto_subscribe(
    mut args: SetOnChainYieldsLockedAutoSubscribeArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetOnChainYieldsLockedAutoSubscribeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SetOnChainYieldsLockedAutoSubscribeParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.position_id.is_none() {
                        let position_id: String = Input::new()
                            .with_prompt("Please enter the position_id name")
                            .interact_text()?;

                        args.position_id = Some(position_id);
                    }
                    if args.auto_subscribe.is_none() {
                        let auto_subscribe: bool = Input::new()
                            .with_prompt("Please enter the auto_subscribe name")
                            .interact_text()?;

                        args.auto_subscribe = Some(auto_subscribe);
                    }
                }
                SetOnChainYieldsLockedAutoSubscribeParams::builder(
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
    let response = rest_client
        .set_on_chain_yields_locked_auto_subscribe(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_on_chain_yields_locked_product_redeem_option(
    mut args: SetOnChainYieldsLockedProductRedeemOptionArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetOnChainYieldsLockedProductRedeemOptionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SetOnChainYieldsLockedProductRedeemOptionParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.position_id.is_none() {
                        let position_id: String = Input::new()
                            .with_prompt("Please enter the position_id name")
                            .interact_text()?;

                        args.position_id = Some(position_id);
                    }
                    if args.redeem_to.is_none() {
                        let options = vec![
                            (
                                "SPOT",
                                SetOnChainYieldsLockedProductRedeemOptionRedeemToEnum::Spot,
                            ),
                            (
                                "FLEXIBLE",
                                SetOnChainYieldsLockedProductRedeemOptionRedeemToEnum::Flexible,
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
                SetOnChainYieldsLockedProductRedeemOptionParams::builder(
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
    let response = rest_client
        .set_on_chain_yields_locked_product_redeem_option(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn subscribe_on_chain_yields_locked_product(
    mut args: SubscribeOnChainYieldsLockedProductArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubscribeOnChainYieldsLockedProductParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SubscribeOnChainYieldsLockedProductParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.project_id.is_none() {
                        let project_id: String = Input::new()
                            .with_prompt("Please enter the project_id name")
                            .interact_text()?;

                        args.project_id = Some(project_id);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                SubscribeOnChainYieldsLockedProductParams::builder(
                    args.project_id
                        .ok_or_else(|| anyhow::anyhow!("project_id is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .auto_subscribe(args.auto_subscribe)
                .source_account(args.source_account)
                .redeem_to(args.redeem_to)
                .channel_id(args.channel_id)
                .client_id(args.client_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .subscribe_on_chain_yields_locked_product(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_soft_staking_product_list(
    args: GetSoftStakingProductListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSoftStakingProductListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSoftStakingProductListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetSoftStakingProductListParams::builder()
                .asset(args.asset)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_soft_staking_product_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_soft_staking_rewards_history(
    args: GetSoftStakingRewardsHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSoftStakingRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSoftStakingRewardsHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetSoftStakingRewardsHistoryParams::builder()
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
    let response = rest_client.get_soft_staking_rewards_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_soft_staking(mut args: SetSoftStakingArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetSoftStakingParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SetSoftStakingParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.soft_staking.is_none() {
                        let soft_staking: bool = Input::new()
                            .with_prompt("Please enter the soft_staking name")
                            .interact_text()?;

                        args.soft_staking = Some(soft_staking);
                    }
                }
                SetSoftStakingParams::builder(
                    args.soft_staking
                        .ok_or_else(|| anyhow::anyhow!("soft_staking is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.set_soft_staking(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn claim_boost_rewards(args: ClaimBoostRewardsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ClaimBoostRewardsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ClaimBoostRewardsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ClaimBoostRewardsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.claim_boost_rewards(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_bnsol_rate_history(args: GetBnsolRateHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBnsolRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBnsolRateHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetBnsolRateHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bnsol_rate_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_bnsol_rewards_history(args: GetBnsolRewardsHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBnsolRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBnsolRewardsHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetBnsolRewardsHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bnsol_rewards_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_boost_rewards_history(mut args: GetBoostRewardsHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBoostRewardsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBoostRewardsHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.r#type.is_none() {
                        let options = vec![
                            ("CLAIM", GetBoostRewardsHistoryTypeEnum::Claim),
                            ("DISTRIBUTE", GetBoostRewardsHistoryTypeEnum::Distribute),
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
                GetBoostRewardsHistoryParams::builder(
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
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
    let response = rest_client.get_boost_rewards_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_sol_redemption_history(args: GetSolRedemptionHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSolRedemptionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetSolRedemptionHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetSolRedemptionHistoryParams::builder()
                .redeem_id(args.redeem_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_sol_redemption_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_sol_staking_history(args: GetSolStakingHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSolStakingHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetSolStakingHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetSolStakingHistoryParams::builder()
                .purchase_id(args.purchase_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_sol_staking_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_sol_staking_quota_details(
    args: GetSolStakingQuotaDetailsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSolStakingQuotaDetailsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSolStakingQuotaDetailsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetSolStakingQuotaDetailsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_sol_staking_quota_details(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_unclaimed_rewards(args: GetUnclaimedRewardsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUnclaimedRewardsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetUnclaimedRewardsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetUnclaimedRewardsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_unclaimed_rewards(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn redeem_sol(mut args: RedeemSolArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RedeemSolParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RedeemSolParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                RedeemSolParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.redeem_sol(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn sol_staking_account(args: SolStakingAccountArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SolStakingAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SolStakingAccountParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => SolStakingAccountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.sol_staking_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn subscribe_sol_staking(mut args: SubscribeSolStakingArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubscribeSolStakingParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SubscribeSolStakingParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                SubscribeSolStakingParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.subscribe_sol_staking(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
