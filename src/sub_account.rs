use crate::utils::{
    build_user_agent, decode_selected_entities, get_configuration_rest_api, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::SUB_ACCOUNT_REST_API_PROD_URL;
use binance_sdk::sub_account::SubAccountRestApi;
use binance_sdk::sub_account::rest_api::{self as models, *};
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    unsafe {
        env::set_var(
            "BINANCE_CONNECTOR_RUST_USER_AGENT",
            build_user_agent("sub-account"),
        );
    }

    let config_rest_api = get_configuration_rest_api(profile, "sub-account").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(config_rest_api.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = match api_env.as_str() {
        "prod" => SUB_ACCOUNT_REST_API_PROD_URL,
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

    Ok(SubAccountRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct CreateAVirtualSubAccountArgs {
    #[arg(
        help = r#"Please input a string. We will create a virtual email using that string for you to register"#,
        long
    )]
    sub_account_string: Option<String>,
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
struct EnableFuturesForSubAccountArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct EnableOptionsForSubAccountArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct GetFuturesPositionRiskOfSubAccountArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct GetFuturesPositionRiskOfSubAccountV2Args {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#"1:USDT-margined Futures，2: Coin-margined Futures"#, long)]
    futures_type: Option<i64>,
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
struct GetSubAccountsStatusOnMarginOrFuturesArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct QuerySubAccountListArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    is_freeze: Option<String>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
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
struct QuerySubAccountTransactionStatisticsArgs {
    #[arg(help = r#"Managed sub-account email"#, long)]
    email: Option<String>,
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
struct AddIpRestrictionForSubAccountApiKeyArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    sub_account_api_key: Option<String>,
    #[arg(
        help = r#"IP Restriction status. 1 = IP Unrestricted. 2 = Restrict access to trusted IPs only."#,
        long
    )]
    status: Option<i64>,
    #[arg(help = r#"Insert static IP in batch, separated by commas."#, long)]
    ip_address: Option<String>,
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
struct CreateSubAccountApiKeyArgs {
    #[arg(help = r#"Sub-account email"#, long)]
    email: Option<String>,
    #[arg(help = r#"API Key name"#, long)]
    api_name: Option<String>,
    #[arg(
        help = r#"IP restriction status. 1 = unrestricted, 2 = restricted to trusted IPs, 3 = third-party IP restriction"#,
        long
    )]
    status: Option<i64>,
    #[arg(help = r#"Spot & Margin trading permission, default false"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_trade: Option<bool>,
    #[arg(help = r#"Margin borrow/repay permission, default false"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_margin_loan_repay: Option<bool>,
    #[arg(help = r#"Futures trading permission, default false"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_futures_trade: Option<bool>,
    #[arg(help = r#"Universal transfer permission, default false"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_universal_transfer: Option<bool>,
    #[arg(help = r#"Vanilla options permission, default false"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_vanilla_options: Option<bool>,
    #[arg(
        help = r#"Required when status=2. IP address list, max 500 chars"#,
        long
    )]
    ip_address: Option<String>,
    #[arg(help = r#"Required when status=3. Third-party name"#, long)]
    third_party_name: Option<String>,
    #[arg(
        help = r#"Ed25519 public key (optional, for Ed25519 type API Key)"#,
        long
    )]
    public_key: Option<String>,
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
struct DeleteIpListForASubAccountApiKeyArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    sub_account_api_key: Option<String>,
    #[arg(
        help = r#"IPs to be deleted. Can be added in batches, separated by commas"#,
        long
    )]
    ip_address: Option<String>,
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
struct DeleteSubAccountApiKeyArgs {
    #[arg(help = r#"Sub-account email"#, long)]
    email: Option<String>,
    #[arg(help = r#"The sub-account API Key to be deleted"#, long)]
    sub_account_api_key: Option<String>,
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
struct GetIpRestrictionForASubAccountApiKeyArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    sub_account_api_key: Option<String>,
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
struct ModifySubAccountApiKeyPermissionArgs {
    #[arg(help = r#"Sub-account email"#, long)]
    email: Option<String>,
    #[arg(help = r#"Sub-account API Key"#, long)]
    sub_account_api_key: Option<String>,
    #[arg(help = r#"Spot & Margin trading permission"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_trade: Option<bool>,
    #[arg(help = r#"Margin borrow/repay permission"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_margin_loan_repay: Option<bool>,
    #[arg(help = r#"Futures trading permission"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_futures_trade: Option<bool>,
    #[arg(help = r#"Universal transfer permission"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_universal_transfer: Option<bool>,
    #[arg(help = r#"Vanilla options permission"#, long, num_args = 0..=1, default_missing_value = "true")]
    can_vanilla_options: Option<bool>,
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
struct QuerySubAccountApiKeyArgs {
    #[arg(help = r#"Sub-account email"#, long)]
    email: Option<String>,
    #[arg(help = r#"Specify an API Key for exact match"#, long)]
    sub_account_api_key: Option<String>,
    #[arg(help = r#"Page number, default 1, minimum 1"#, long)]
    page: Option<i64>,
    #[arg(help = r#"Page size, default 30, maximum 100"#, long)]
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
struct FuturesTransferForSubAccountArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#"The asset being transferred"#, long)]
    asset: Option<String>,
    #[arg(help = r#"The amount to be transferred"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"1: transfer from subaccount's spot account to its USDT-margined futures account 2: transfer from
subaccount's USDT-margined futures account to its spot account 3: transfer from subaccount's spot
account to its COIN-margined futures account 4:transfer from subaccount's COIN-margined futures
account to its spot account"#,
        long
    )]
    r#type: Option<i64>,
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
struct GetDetailOnSubAccountsFuturesAccountArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct GetDetailOnSubAccountsFuturesAccountV2Args {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#"1:USDT-margined Futures，2: Coin-margined Futures"#, long)]
    futures_type: Option<i64>,
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
struct GetDetailOnSubAccountsMarginAccountArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct GetMovePositionHistoryForSubAccountArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    rows: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
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
struct GetSubAccountDepositAddressArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(
        help = r#"networks can be found in `GET /sapi/v1/capital/deposit/address`"#,
        long
    )]
    network: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
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
struct GetSubAccountDepositHistoryArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#"Default `false`, return `sourceAddress` field when set to `true`"#, long, num_args = 0..=1, default_missing_value = "true")]
    include_source: Option<bool>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(
        help = r#"Deposit status: 0=pending, 6=credited but cannot withdraw, 7=wrong deposit, 8=waiting user confirmation, 1=success."#,
        long
    )]
    status: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#""#, long)]
    offset: Option<i64>,
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(help = r#""#, long)]
    tx_id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetSummaryOfSubAccountsFuturesAccountArgs {
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
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
struct GetSummaryOfSubAccountsFuturesAccountV2Args {
    #[arg(help = r#"1:USDT-margined Futures，2: Coin-margined Futures"#, long)]
    futures_type: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
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
struct GetSummaryOfSubAccountsMarginAccountArgs {
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
struct MarginTransferForSubAccountArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#"The asset being transferred"#, long)]
    asset: Option<String>,
    #[arg(help = r#"The amount to be transferred"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"1: transfer from subaccount's spot account to margin account 2: transfer from subaccount's margin
account to its spot account"#,
        long
    )]
    r#type: Option<i64>,
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
struct MovePositionForSubAccountArgs {
    #[arg(help = r#""#, long)]
    from_user_email: Option<String>,
    #[arg(help = r#""#, long)]
    to_user_email: Option<String>,
    #[arg(help = r#""#, long)]
    product_type: Option<MovePositionForSubAccountProductTypeEnum>,
    #[arg(
        help = r#"Max 10 positions supported. When input request parameter,orderArgs.symbol should be STRING,
orderArgs.quantity should be BIGDECIMAL, and orderArgs.positionSide should be STRING, positionSide
support BOTH,LONG and SHORT. Each entry should be like
orderArgs[0].symbol=BTCUSDT,orderArgs[0].quantity=0.001,orderArgs[0].positionSide=BOTH. Example of
the request parameter array: orderArgs[0].symbol=BTCUSDT orderArgs[0].quantity=0.001
orderArgs[0].positionSide=BOTH orderArgs[1].symbol=ETHUSDT orderArgs[1].quantity=0.01
orderArgs[1].positionSide=BOTH"#,
        long
    )]
    order_args: Option<String>,
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
struct QuerySubAccountAssetsArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct QuerySubAccountAssetsAssetManagementArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct QuerySubAccountFuturesAssetTransferHistoryArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#"1:USDT-margined Futures，2: Coin-margined Futures"#, long)]
    futures_type: Option<i64>,
    #[arg(help = r#"Cannot be earlier than 1 month ago"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
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
struct QuerySubAccountSpotAssetTransferHistoryArgs {
    #[arg(help = r#""#, long)]
    from_email: Option<String>,
    #[arg(help = r#""#, long)]
    to_email: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
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
struct QuerySubAccountSpotAssetsSummaryArgs {
    #[arg(help = r#"Managed sub-account email"#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
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
struct QueryUniversalTransferHistoryArgs {
    #[arg(help = r#""#, long)]
    from_email: Option<String>,
    #[arg(help = r#""#, long)]
    to_email: Option<String>,
    #[arg(help = r#""#, long)]
    client_tran_id: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
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
struct SubAccountFuturesAssetTransferArgs {
    #[arg(help = r#"Sender email"#, long)]
    from_email: Option<String>,
    #[arg(help = r#"Recipient email"#, long)]
    to_email: Option<String>,
    #[arg(help = r#"1:USDT-margined Futures，2: Coin-margined Futures"#, long)]
    futures_type: Option<i64>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
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
struct SubAccountTransferHistoryArgs {
    #[arg(help = r#"If not sent, result of all assets will be returned"#, long)]
    asset: Option<String>,
    #[arg(help = r#"1: transfer in, 2: transfer out"#, long)]
    r#type: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Default `False`, return PROCESS and SUCCESS status history; If `True`,return PROCESS and SUCCESS and FAILURE
status history"#, long, num_args = 0..=1, default_missing_value = "true")]
    return_fail_history: Option<bool>,
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
struct TransferToMasterArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
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
struct TransferToSubAccountOfSameMasterArgs {
    #[arg(help = r#""#, long)]
    to_email: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
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
struct UniversalTransferArgs {
    #[arg(help = r#""#, long)]
    from_account_type: Option<UniversalTransferFromAccountTypeEnum>,
    #[arg(help = r#""#, long)]
    to_account_type: Option<UniversalTransferToAccountTypeEnum>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    from_email: Option<String>,
    #[arg(help = r#""#, long)]
    to_email: Option<String>,
    #[arg(help = r#"Must be unique"#, long)]
    client_tran_id: Option<String>,
    #[arg(help = r#"Only supported under ISOLATED_MARGIN type"#, long)]
    symbol: Option<String>,
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
struct DepositAssetsIntoTheManagedSubAccountArgs {
    #[arg(help = r#""#, long)]
    to_email: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
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
struct GetManagedSubAccountDepositAddressArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(
        help = r#"networks can be found in `GET /sapi/v1/capital/deposit/address`"#,
        long
    )]
    network: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
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
struct QueryManagedSubAccountAssetDetailsArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
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
struct QueryManagedSubAccountFuturesAssetDetailsArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(
        help = r#"No input or input "USDT_FUTURE" to get UM Futures account details. Input "COIN_FUTURE" to get CM Futures account details."#,
        long
    )]
    account_type: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryManagedSubAccountListArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
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
struct QueryManagedSubAccountMarginAssetDetailsArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(
        help = r#"No input or input "MARGIN" to get Cross Margin account details. Input "ISOLATED_MARGIN" to get Isolated
Margin account details."#,
        long
    )]
    account_type: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryManagedSubAccountSnapshotArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    r#type: Option<QueryManagedSubAccountSnapshotTypeEnum>,
    #[arg(
        help = r#"Query time range must be within 30 days and only supports data within the last month."#,
        long
    )]
    start_time: Option<i64>,
    #[arg(
        help = r#"If both startTime and endTime are omitted, records from the last 7 days are returned by default."#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
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
struct QueryManagedSubAccountTransferLogMasterAccountInvestorArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#"Start Time"#, long)]
    start_time: Option<i64>,
    #[arg(
        help = r#"End Time (The start time and end time interval cannot exceed half a year)"#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#"Page"#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Transfer Direction (FROM/TO)"#, long)]
    transfers: Option<String>,
    #[arg(help = r#""#, long)]
    transfer_function_account_type: Option<
        QueryManagedSubAccountTransferLogMasterAccountInvestorTransferFunctionAccountTypeEnum,
    >,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryManagedSubAccountTransferLogMasterAccountTradingArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#"Start Time"#, long)]
    start_time: Option<i64>,
    #[arg(
        help = r#"End Time (The start time and end time interval cannot exceed half a year)"#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Transfer Direction (FROM/TO)"#, long)]
    transfers: Option<String>,
    #[arg(help = r#""#, long)]
    transfer_function_account_type: Option<
        QueryManagedSubAccountTransferLogMasterAccountTradingTransferFunctionAccountTypeEnum,
    >,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryManagedSubAccountTransferLogSubAccountTradingArgs {
    #[arg(help = r#"Start Time"#, long)]
    start_time: Option<i64>,
    #[arg(
        help = r#"End Time (The start time and end time interval cannot exceed half a year)"#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Transfer Direction (from/to)"#, long)]
    transfers: Option<String>,
    #[arg(help = r#""#, long)]
    transfer_function_account_type:
        Option<QueryManagedSubAccountTransferLogSubAccountTradingTransferFunctionAccountTypeEnum>,
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
struct WithdrawlAssetsFromTheManagedSubAccountArgs {
    #[arg(help = r#""#, long)]
    from_email: Option<String>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Withdrawal will happen automatically on the selected date (UTC 0). If no date is selected,
withdrawal takes effect immediately."#,
        long
    )]
    transfer_date: Option<i64>,
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
pub enum SubAccountCommands {
    #[command(
        about = decode_selected_entities(r#"Create a Virtual Sub-account

Weight(IP): 1

Security Type: USER_DATA

Notes:
- This request generates a virtual sub-account under your master account.
- The API key used to call this endpoint must have the `trade` option enabled."#, false),
    )]
    CreateAVirtualSubAccount(CreateAVirtualSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Enable Futures for Sub-account for Master Account

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    EnableFuturesForSubAccount(EnableFuturesForSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Enable Options for Sub-account (For Master Account).

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    EnableOptionsForSubAccount(EnableOptionsForSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get Futures Position-Risk of Sub-account

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    GetFuturesPositionRiskOfSubAccount(GetFuturesPositionRiskOfSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get Futures Position-Risk of Sub-account V2

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetFuturesPositionRiskOfSubAccountV2(GetFuturesPositionRiskOfSubAccountV2Args),
    #[command(
        about = decode_selected_entities(r#"Get Sub-account's Status on Margin Or Futures

Weight(IP): 10

Security Type: USER_DATA

Notes:
- If no email sent, all sub-accounts' information will be returned."#, false),
    )]
    GetSubAccountsStatusOnMarginOrFutures(GetSubAccountsStatusOnMarginOrFuturesArgs),
    #[command(
        about = decode_selected_entities(r#"Query Sub-account List

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QuerySubAccountList(QuerySubAccountListArgs),
    #[command(
        about = decode_selected_entities(r#"Query Sub-account Transaction statistics (For Master Account).

Weight(IP): 60

Security Type: USER_DATA"#, false),
    )]
    QuerySubAccountTransactionStatistics(QuerySubAccountTransactionStatisticsArgs),
    #[command(
        about = decode_selected_entities(r#"Add IP Restriction for Sub-Account API key

Weight(UID): 3000

Security Type: USER_DATA

Notes:
- You need to enable Enable Spot & Margin Trading option for the api key which requests this endpoint"#, false),
    )]
    AddIpRestrictionForSubAccountApiKey(AddIpRestrictionForSubAccountApiKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Create a new API Key for a sub-account.

Weight(UID): 3000

Security Type: USER_DATA

Notes:
- `status=2` requires `ipAddress`
- `status=3` requires `thirdPartyName`
- Asset Sub Account is not supported
- The caller must pass the KYC IP restriction check"#, false),
    )]
    CreateSubAccountApiKey(CreateSubAccountApiKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Delete IP List For a Sub-account API Key

Weight(UID): 3000

Security Type: USER_DATA

Notes:
- You need to enable Enable Spot & Margin Trading option for the api key which requests this endpoint"#, false),
    )]
    DeleteIpListForASubAccountApiKey(DeleteIpListForASubAccountApiKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Delete an API Key of a sub-account.

Weight(UID): 3000

Security Type: USER_DATA

Notes:
- Asset Sub Account is not supported
- The caller must pass the KYC IP restriction check"#, false),
    )]
    DeleteSubAccountApiKey(DeleteSubAccountApiKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Get IP Restriction for a Sub-account API Key

Weight(UID): 3000

Security Type: USER_DATA"#, false),
    )]
    GetIpRestrictionForASubAccountApiKey(GetIpRestrictionForASubAccountApiKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Modify the trading permissions of a sub-account API Key.

Weight(UID): 3000

Security Type: USER_DATA

Notes:
- Portfolio Margin Retail User is not supported
- Asset Sub Account is not supported
- The caller must pass the KYC IP restriction check"#, false),
    )]
    ModifySubAccountApiKeyPermission(ModifySubAccountApiKeyPermissionArgs),
    #[command(
        about = decode_selected_entities(r#"Query the API Key list of a sub-account.

Weight(UID): 3000

Security Type: USER_DATA"#, false),
    )]
    QuerySubAccountApiKey(QuerySubAccountApiKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Futures Transfer for Sub-account

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    FuturesTransferForSubAccount(FuturesTransferForSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get Detail on Sub-account's Futures Account

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    GetDetailOnSubAccountsFuturesAccount(GetDetailOnSubAccountsFuturesAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get Detail on Sub-account's Futures Account

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetDetailOnSubAccountsFuturesAccountV2(GetDetailOnSubAccountsFuturesAccountV2Args),
    #[command(
        about = decode_selected_entities(r#"Get Detail on Sub-account's Margin Account

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    GetDetailOnSubAccountsMarginAccount(GetDetailOnSubAccountsMarginAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Query move position history

Weight(IP): 1

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are both omitted, records from the last 90 days are returned by default (up to 1000 records).
- If `startTime` is sent and `endTime` is omitted, records in `[max(startTime, now-90d), now]` are returned.
- If `startTime` is omitted and `endTime` is sent, records in `[max(now, endTime-90d), endTime]` are returned."#, false),
    )]
    GetMovePositionHistoryForSubAccount(GetMovePositionHistoryForSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch sub-account deposit address

Weight(IP): 1

Security Type: USER_DATA

Notes:
- `amount` needs to be sent if using LIGHTNING network"#, false),
    )]
    GetSubAccountDepositAddress(GetSubAccountDepositAddressArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch sub-account deposit history

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetSubAccountDepositHistory(GetSubAccountDepositHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Summary of Sub-account's Futures Account

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetSummaryOfSubAccountsFuturesAccount(GetSummaryOfSubAccountsFuturesAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get Summary of Sub-account's Futures Account

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    GetSummaryOfSubAccountsFuturesAccountV2(GetSummaryOfSubAccountsFuturesAccountV2Args),
    #[command(
        about = decode_selected_entities(r#"Get Summary of Sub-account's Margin Account

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    GetSummaryOfSubAccountsMarginAccount(GetSummaryOfSubAccountsMarginAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Margin Transfer for Sub-account

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    MarginTransferForSubAccount(MarginTransferForSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Move position between sub-master, master-sub, or sub-sub accounts when necessary

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to enable the `Trading` permission for the API key used to call this endpoint.
- This function is only available for VIP levels 7-9.
- Only master accounts can call this endpoint.
- `quantity` must be a positive number.
- Supported account types: normal account, PM PRO, PM PRO SPAN, and PM Retail.
- The source account must have positions.
- For orders in the same `orderArgs` request, if any symbol's total close position quantity exceeds current position quantity, all orders in that batch fail.
- Only cross margin mode is supported.
- The move position price supports `MARK_PRICE` only.
- MSA is not supported.
- Symbols configured with `Reduce-Only` are not supported."#, false),
    )]
    MovePositionForSubAccount(MovePositionForSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch sub-account assets

Weight(UID): 60

Security Type: USER_DATA"#, false),
    )]
    QuerySubAccountAssets(QuerySubAccountAssetsArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch sub-account assets

Weight(UID): 60

Security Type: USER_DATA"#, false),
    )]
    QuerySubAccountAssetsAssetManagement(QuerySubAccountAssetsAssetManagementArgs),
    #[command(
        about = decode_selected_entities(r#"Query Sub-account Futures Asset Transfer History

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QuerySubAccountFuturesAssetTransferHistory(QuerySubAccountFuturesAssetTransferHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query Sub-account Spot Asset Transfer History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- `fromEmail` and `toEmail` cannot be sent at the same time.
- If both `fromEmail` and `toEmail` are omitted, records with `fromEmail` equal to the master account are returned by default."#, false),
    )]
    QuerySubAccountSpotAssetTransferHistory(QuerySubAccountSpotAssetTransferHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get BTC valued asset summary of subaccounts.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QuerySubAccountSpotAssetsSummary(QuerySubAccountSpotAssetsSummaryArgs),
    #[command(
        about = decode_selected_entities(r#"Query Universal Transfer History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- `fromEmail` and `toEmail` cannot be sent at the same time.
- If both `fromEmail` and `toEmail` are omitted, records with `fromEmail` equal to the master account are returned by default.
- The query time range must be less than 7 days.
- If `startTime` and `endTime` are omitted, records from the last 7 days are returned by default."#, false),
    )]
    QueryUniversalTransferHistory(QueryUniversalTransferHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Sub-account Futures Asset Transfer

Weight(IP): 1

Security Type: USER_DATA

Notes:
- A master account can transfer at most 2000 times per minute.
- The futures wallet must have sufficient margin balance to execute the transfer."#, false),
    )]
    SubAccountFuturesAssetTransfer(SubAccountFuturesAssetTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Sub-account Transfer History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- If `type` is not sent, records of type `2` (transfer out) are returned by default.
- If `startTime` and `endTime` are not sent, data from the most recent 30 days is returned."#, false),
    )]
    SubAccountTransferHistory(SubAccountTransferHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Transfer to Master

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    TransferToMaster(TransferToMasterArgs),
    #[command(
        about = decode_selected_entities(r#"Transfer to Sub-account of Same Master

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint."#, false),
    )]
    TransferToSubAccountOfSameMaster(TransferToSubAccountOfSameMasterArgs),
    #[command(
        about = decode_selected_entities(r#"Universal Transfer

Weight(IP): 1

Weight(UID): 360

Security Type: USER_DATA

Notes:
- You need to enable the `internal transfer` option for the API key used to call this endpoint.
- If `fromEmail` is not sent, transfer out from the master account by default.
- If `toEmail` is not sent, transfer into the master account by default.
- When `fromAccountType` and `toAccountType` are the same, at least one of `fromEmail` or `toEmail` must be sent.
- Supported transfer scenarios:
  - `SPOT` -> `SPOT` / `USDT_FUTURE` / `COIN_FUTURE` (master or sub-account).
  - `SPOT` / `USDT_FUTURE` / `COIN_FUTURE` -> `SPOT` (master or sub-account).
  - Master account `SPOT` -> sub-account `MARGIN(Cross)` / `ISOLATED_MARGIN`.
  - Sub-account `MARGIN(Cross)` / `ISOLATED_MARGIN` -> master account `SPOT`.
  - Sub-account `MARGIN(Cross)` -> sub-account `MARGIN(Cross)`.
  - `ALPHA` -> `ALPHA` (master or sub-account)."#, false),
    )]
    UniversalTransfer(UniversalTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Deposit Assets Into The Managed Sub-account

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to enable `Enable Spot & Margin Trading` option for the api key which requests this endpoint"#, false),
    )]
    DepositAssetsIntoTheManagedSubAccount(DepositAssetsIntoTheManagedSubAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get investor's managed sub-account deposit address.

Weight(UID): 1

Security Type: USER_DATA

Notes:
- If `network` is not sent, the default `network` for the `coin` is returned.
- When using `LIGHTNING`, `amount` must be provided."#, false),
    )]
    GetManagedSubAccountDepositAddress(GetManagedSubAccountDepositAddressArgs),
    #[command(
        about = decode_selected_entities(r#"Query Managed Sub-account Asset Details

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryManagedSubAccountAssetDetails(QueryManagedSubAccountAssetDetailsArgs),
    #[command(
        about = decode_selected_entities(r#"Investor can use this api to query managed sub account futures asset details

Weight(UID): 60

Security Type: USER_DATA"#, false),
    )]
    QueryManagedSubAccountFuturesAssetDetails(QueryManagedSubAccountFuturesAssetDetailsArgs),
    #[command(
        about = decode_selected_entities(r#"Get investor's managed sub-account list.

Weight(UID): 60

Security Type: USER_DATA"#, false),
    )]
    QueryManagedSubAccountList(QueryManagedSubAccountListArgs),
    #[command(
        about = decode_selected_entities(r#"Investor can use this api to query managed sub account margin asset details

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryManagedSubAccountMarginAssetDetails(QueryManagedSubAccountMarginAssetDetailsArgs),
    #[command(
        about = decode_selected_entities(r#"Query Managed Sub-account Snapshot

Weight(IP): 2400

Security Type: USER_DATA

Notes:
- The query time range must be less than 30 days.
- Only data from the most recent month is supported.
- If `startTime` and `endTime` are omitted, records from the last 7 days are returned by default."#, false),
    )]
    QueryManagedSubAccountSnapshot(QueryManagedSubAccountSnapshotArgs),
    #[command(
        about = decode_selected_entities(r#"Query Managed Sub Account Transfer Log For Investor Master Account

Investor can use this api to query managed sub account transfer log. This endpoint is available for investor of
Managed Sub-Account. A Managed Sub-Account is an account type for investors who value flexibility in asset
allocation and account application, while delegating trades to a professional trading team.

Please refer to
[link](https://www.binance.com/en/support/faq/how-to-get-started-with-managed-sub-account-functions-and-frequently-asked-questions-0594748722704383a7c369046e489459)

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryManagedSubAccountTransferLogMasterAccountInvestor(
        QueryManagedSubAccountTransferLogMasterAccountInvestorArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Query Managed Sub Account Transfer Log For Trading Team Master Account

Trading team can use this api to query managed sub account transfer log. This endpoint is available for trading
team of Managed Sub-Account. A Managed Sub-Account is an account type for investors who value flexibility in
asset allocation and account application, while delegating trades to a professional trading team.

Please refer to
[link](https://www.binance.com/en/support/faq/how-to-get-started-with-managed-sub-account-functions-and-frequently-asked-questions-0594748722704383a7c369046e489459)

Weight(UID): 60

Security Type: USER_DATA"#, false),
    )]
    QueryManagedSubAccountTransferLogMasterAccountTrading(
        QueryManagedSubAccountTransferLogMasterAccountTradingArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Query Managed Sub Account Transfer Log (For Trading Team Sub Account)

Weight(UID): 60

Security Type: USER_DATA"#, false),
    )]
    QueryManagedSubAccountTransferLogSubAccountTrading(
        QueryManagedSubAccountTransferLogSubAccountTradingArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Withdrawl Assets From The Managed Sub-account

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Your API key must have the permission `Enable Spot & Margin Trading`."#, false),
    )]
    WithdrawlAssetsFromTheManagedSubAccount(WithdrawlAssetsFromTheManagedSubAccountArgs),
}

pub async fn handle_sub_account_command(command: SubAccountCommands) -> anyhow::Result<()> {
    match command {
        SubAccountCommands::CreateAVirtualSubAccount(args) => {
            create_a_virtual_sub_account(args).await
        }

        SubAccountCommands::EnableFuturesForSubAccount(args) => {
            enable_futures_for_sub_account(args).await
        }

        SubAccountCommands::EnableOptionsForSubAccount(args) => {
            enable_options_for_sub_account(args).await
        }

        SubAccountCommands::GetFuturesPositionRiskOfSubAccount(args) => {
            get_futures_position_risk_of_sub_account(args).await
        }

        SubAccountCommands::GetFuturesPositionRiskOfSubAccountV2(args) => {
            get_futures_position_risk_of_sub_account_v2(args).await
        }

        SubAccountCommands::GetSubAccountsStatusOnMarginOrFutures(args) => {
            get_sub_accounts_status_on_margin_or_futures(args).await
        }

        SubAccountCommands::QuerySubAccountList(args) => query_sub_account_list(args).await,

        SubAccountCommands::QuerySubAccountTransactionStatistics(args) => {
            query_sub_account_transaction_statistics(args).await
        }

        SubAccountCommands::AddIpRestrictionForSubAccountApiKey(args) => {
            add_ip_restriction_for_sub_account_api_key(args).await
        }

        SubAccountCommands::CreateSubAccountApiKey(args) => create_sub_account_api_key(args).await,

        SubAccountCommands::DeleteIpListForASubAccountApiKey(args) => {
            delete_ip_list_for_a_sub_account_api_key(args).await
        }

        SubAccountCommands::DeleteSubAccountApiKey(args) => delete_sub_account_api_key(args).await,

        SubAccountCommands::GetIpRestrictionForASubAccountApiKey(args) => {
            get_ip_restriction_for_a_sub_account_api_key(args).await
        }

        SubAccountCommands::ModifySubAccountApiKeyPermission(args) => {
            modify_sub_account_api_key_permission(args).await
        }

        SubAccountCommands::QuerySubAccountApiKey(args) => query_sub_account_api_key(args).await,

        SubAccountCommands::FuturesTransferForSubAccount(args) => {
            futures_transfer_for_sub_account(args).await
        }

        SubAccountCommands::GetDetailOnSubAccountsFuturesAccount(args) => {
            get_detail_on_sub_accounts_futures_account(args).await
        }

        SubAccountCommands::GetDetailOnSubAccountsFuturesAccountV2(args) => {
            get_detail_on_sub_accounts_futures_account_v2(args).await
        }

        SubAccountCommands::GetDetailOnSubAccountsMarginAccount(args) => {
            get_detail_on_sub_accounts_margin_account(args).await
        }

        SubAccountCommands::GetMovePositionHistoryForSubAccount(args) => {
            get_move_position_history_for_sub_account(args).await
        }

        SubAccountCommands::GetSubAccountDepositAddress(args) => {
            get_sub_account_deposit_address(args).await
        }

        SubAccountCommands::GetSubAccountDepositHistory(args) => {
            get_sub_account_deposit_history(args).await
        }

        SubAccountCommands::GetSummaryOfSubAccountsFuturesAccount(args) => {
            get_summary_of_sub_accounts_futures_account(args).await
        }

        SubAccountCommands::GetSummaryOfSubAccountsFuturesAccountV2(args) => {
            get_summary_of_sub_accounts_futures_account_v2(args).await
        }

        SubAccountCommands::GetSummaryOfSubAccountsMarginAccount(args) => {
            get_summary_of_sub_accounts_margin_account(args).await
        }

        SubAccountCommands::MarginTransferForSubAccount(args) => {
            margin_transfer_for_sub_account(args).await
        }

        SubAccountCommands::MovePositionForSubAccount(args) => {
            move_position_for_sub_account(args).await
        }

        SubAccountCommands::QuerySubAccountAssets(args) => query_sub_account_assets(args).await,

        SubAccountCommands::QuerySubAccountAssetsAssetManagement(args) => {
            query_sub_account_assets_asset_management(args).await
        }

        SubAccountCommands::QuerySubAccountFuturesAssetTransferHistory(args) => {
            query_sub_account_futures_asset_transfer_history(args).await
        }

        SubAccountCommands::QuerySubAccountSpotAssetTransferHistory(args) => {
            query_sub_account_spot_asset_transfer_history(args).await
        }

        SubAccountCommands::QuerySubAccountSpotAssetsSummary(args) => {
            query_sub_account_spot_assets_summary(args).await
        }

        SubAccountCommands::QueryUniversalTransferHistory(args) => {
            query_universal_transfer_history(args).await
        }

        SubAccountCommands::SubAccountFuturesAssetTransfer(args) => {
            sub_account_futures_asset_transfer(args).await
        }

        SubAccountCommands::SubAccountTransferHistory(args) => {
            sub_account_transfer_history(args).await
        }

        SubAccountCommands::TransferToMaster(args) => transfer_to_master(args).await,

        SubAccountCommands::TransferToSubAccountOfSameMaster(args) => {
            transfer_to_sub_account_of_same_master(args).await
        }

        SubAccountCommands::UniversalTransfer(args) => universal_transfer(args).await,

        SubAccountCommands::DepositAssetsIntoTheManagedSubAccount(args) => {
            deposit_assets_into_the_managed_sub_account(args).await
        }

        SubAccountCommands::GetManagedSubAccountDepositAddress(args) => {
            get_managed_sub_account_deposit_address(args).await
        }

        SubAccountCommands::QueryManagedSubAccountAssetDetails(args) => {
            query_managed_sub_account_asset_details(args).await
        }

        SubAccountCommands::QueryManagedSubAccountFuturesAssetDetails(args) => {
            query_managed_sub_account_futures_asset_details(args).await
        }

        SubAccountCommands::QueryManagedSubAccountList(args) => {
            query_managed_sub_account_list(args).await
        }

        SubAccountCommands::QueryManagedSubAccountMarginAssetDetails(args) => {
            query_managed_sub_account_margin_asset_details(args).await
        }

        SubAccountCommands::QueryManagedSubAccountSnapshot(args) => {
            query_managed_sub_account_snapshot(args).await
        }

        SubAccountCommands::QueryManagedSubAccountTransferLogMasterAccountInvestor(args) => {
            query_managed_sub_account_transfer_log_master_account_investor(args).await
        }

        SubAccountCommands::QueryManagedSubAccountTransferLogMasterAccountTrading(args) => {
            query_managed_sub_account_transfer_log_master_account_trading(args).await
        }

        SubAccountCommands::QueryManagedSubAccountTransferLogSubAccountTrading(args) => {
            query_managed_sub_account_transfer_log_sub_account_trading(args).await
        }

        SubAccountCommands::WithdrawlAssetsFromTheManagedSubAccount(args) => {
            withdrawl_assets_from_the_managed_sub_account(args).await
        }
    }
}

async fn create_a_virtual_sub_account(
    mut args: CreateAVirtualSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CreateAVirtualSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CreateAVirtualSubAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.sub_account_string.is_none() {
                        let sub_account_string: String = Input::new()
                            .with_prompt("Please enter the sub_account_string name")
                            .interact_text()?;

                        args.sub_account_string = Some(sub_account_string);
                    }
                }
                CreateAVirtualSubAccountParams::builder(
                    args.sub_account_string
                        .ok_or_else(|| anyhow::anyhow!("sub_account_string is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.create_a_virtual_sub_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn enable_futures_for_sub_account(
    mut args: EnableFuturesForSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EnableFuturesForSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<EnableFuturesForSubAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                EnableFuturesForSubAccountParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.enable_futures_for_sub_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn enable_options_for_sub_account(
    mut args: EnableOptionsForSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EnableOptionsForSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<EnableOptionsForSubAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                EnableOptionsForSubAccountParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.enable_options_for_sub_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_futures_position_risk_of_sub_account(
    mut args: GetFuturesPositionRiskOfSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFuturesPositionRiskOfSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFuturesPositionRiskOfSubAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                GetFuturesPositionRiskOfSubAccountParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_futures_position_risk_of_sub_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_futures_position_risk_of_sub_account_v2(
    mut args: GetFuturesPositionRiskOfSubAccountV2Args,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFuturesPositionRiskOfSubAccountV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFuturesPositionRiskOfSubAccountV2Params>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.futures_type.is_none() {
                        let futures_type: i64 = Input::new()
                            .with_prompt("Please enter the futures_type name")
                            .interact_text()?;

                        args.futures_type = Some(futures_type);
                    }
                }
                GetFuturesPositionRiskOfSubAccountV2Params::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.futures_type
                        .ok_or_else(|| anyhow::anyhow!("futures_type is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_futures_position_risk_of_sub_account_v2(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_sub_accounts_status_on_margin_or_futures(
    args: GetSubAccountsStatusOnMarginOrFuturesArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSubAccountsStatusOnMarginOrFuturesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetSubAccountsStatusOnMarginOrFuturesParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetSubAccountsStatusOnMarginOrFuturesParams::builder()
                .email(args.email)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_sub_accounts_status_on_margin_or_futures(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_account_list(args: QuerySubAccountListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubAccountListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySubAccountListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QuerySubAccountListParams::builder()
                .email(args.email)
                .is_freeze(args.is_freeze)
                .page(args.page)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_sub_account_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_account_transaction_statistics(
    args: QuerySubAccountTransactionStatisticsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubAccountTransactionStatisticsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySubAccountTransactionStatisticsParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QuerySubAccountTransactionStatisticsParams::builder()
                .email(args.email)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_sub_account_transaction_statistics(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn add_ip_restriction_for_sub_account_api_key(
    mut args: AddIpRestrictionForSubAccountApiKeyArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AddIpRestrictionForSubAccountApiKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AddIpRestrictionForSubAccountApiKeyParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.sub_account_api_key.is_none() {
                        let sub_account_api_key: String = Input::new()
                            .with_prompt("Please enter the sub_account_api_key name")
                            .interact_text()?;

                        args.sub_account_api_key = Some(sub_account_api_key);
                    }
                    if args.status.is_none() {
                        let status: i64 = Input::new()
                            .with_prompt("Please enter the status name")
                            .interact_text()?;

                        args.status = Some(status);
                    }
                }
                AddIpRestrictionForSubAccountApiKeyParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.sub_account_api_key
                        .ok_or_else(|| anyhow::anyhow!("sub_account_api_key is required"))?,
                    args.status
                        .ok_or_else(|| anyhow::anyhow!("status is required"))?,
                )
                .ip_address(args.ip_address)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .add_ip_restriction_for_sub_account_api_key(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn create_sub_account_api_key(mut args: CreateSubAccountApiKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CreateSubAccountApiKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CreateSubAccountApiKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.api_name.is_none() {
                        let api_name: String = Input::new()
                            .with_prompt("Please enter the api_name name")
                            .interact_text()?;

                        args.api_name = Some(api_name);
                    }
                    if args.status.is_none() {
                        let status: i64 = Input::new()
                            .with_prompt("Please enter the status name")
                            .interact_text()?;

                        args.status = Some(status);
                    }
                }
                CreateSubAccountApiKeyParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.api_name
                        .ok_or_else(|| anyhow::anyhow!("api_name is required"))?,
                    args.status
                        .ok_or_else(|| anyhow::anyhow!("status is required"))?,
                )
                .can_trade(args.can_trade)
                .can_margin_loan_repay(args.can_margin_loan_repay)
                .can_futures_trade(args.can_futures_trade)
                .can_universal_transfer(args.can_universal_transfer)
                .can_vanilla_options(args.can_vanilla_options)
                .ip_address(args.ip_address)
                .third_party_name(args.third_party_name)
                .public_key(args.public_key)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.create_sub_account_api_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn delete_ip_list_for_a_sub_account_api_key(
    mut args: DeleteIpListForASubAccountApiKeyArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DeleteIpListForASubAccountApiKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<DeleteIpListForASubAccountApiKeyParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.sub_account_api_key.is_none() {
                        let sub_account_api_key: String = Input::new()
                            .with_prompt("Please enter the sub_account_api_key name")
                            .interact_text()?;

                        args.sub_account_api_key = Some(sub_account_api_key);
                    }
                    if args.ip_address.is_none() {
                        let ip_address: String = Input::new()
                            .with_prompt("Please enter the ip_address name")
                            .interact_text()?;

                        args.ip_address = Some(ip_address);
                    }
                }
                DeleteIpListForASubAccountApiKeyParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.sub_account_api_key
                        .ok_or_else(|| anyhow::anyhow!("sub_account_api_key is required"))?,
                    args.ip_address
                        .ok_or_else(|| anyhow::anyhow!("ip_address is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .delete_ip_list_for_a_sub_account_api_key(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn delete_sub_account_api_key(mut args: DeleteSubAccountApiKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DeleteSubAccountApiKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DeleteSubAccountApiKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.sub_account_api_key.is_none() {
                        let sub_account_api_key: String = Input::new()
                            .with_prompt("Please enter the sub_account_api_key name")
                            .interact_text()?;

                        args.sub_account_api_key = Some(sub_account_api_key);
                    }
                }
                DeleteSubAccountApiKeyParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.sub_account_api_key
                        .ok_or_else(|| anyhow::anyhow!("sub_account_api_key is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.delete_sub_account_api_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_ip_restriction_for_a_sub_account_api_key(
    mut args: GetIpRestrictionForASubAccountApiKeyArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetIpRestrictionForASubAccountApiKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetIpRestrictionForASubAccountApiKeyParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.sub_account_api_key.is_none() {
                        let sub_account_api_key: String = Input::new()
                            .with_prompt("Please enter the sub_account_api_key name")
                            .interact_text()?;

                        args.sub_account_api_key = Some(sub_account_api_key);
                    }
                }
                GetIpRestrictionForASubAccountApiKeyParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.sub_account_api_key
                        .ok_or_else(|| anyhow::anyhow!("sub_account_api_key is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_ip_restriction_for_a_sub_account_api_key(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn modify_sub_account_api_key_permission(
    mut args: ModifySubAccountApiKeyPermissionArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ModifySubAccountApiKeyPermissionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ModifySubAccountApiKeyPermissionParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.sub_account_api_key.is_none() {
                        let sub_account_api_key: String = Input::new()
                            .with_prompt("Please enter the sub_account_api_key name")
                            .interact_text()?;

                        args.sub_account_api_key = Some(sub_account_api_key);
                    }
                }
                ModifySubAccountApiKeyPermissionParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.sub_account_api_key
                        .ok_or_else(|| anyhow::anyhow!("sub_account_api_key is required"))?,
                )
                .can_trade(args.can_trade)
                .can_margin_loan_repay(args.can_margin_loan_repay)
                .can_futures_trade(args.can_futures_trade)
                .can_universal_transfer(args.can_universal_transfer)
                .can_vanilla_options(args.can_vanilla_options)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .modify_sub_account_api_key_permission(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_account_api_key(mut args: QuerySubAccountApiKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubAccountApiKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySubAccountApiKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                QuerySubAccountApiKeyParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .sub_account_api_key(args.sub_account_api_key)
                .page(args.page)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_sub_account_api_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn futures_transfer_for_sub_account(
    mut args: FuturesTransferForSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FuturesTransferForSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<FuturesTransferForSubAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.r#type.is_none() {
                        let r#type: i64 = Input::new()
                            .with_prompt("Please enter the r#type name")
                            .interact_text()?;

                        args.r#type = Some(r#type);
                    }
                }
                FuturesTransferForSubAccountParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
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
    let response = rest_client.futures_transfer_for_sub_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_detail_on_sub_accounts_futures_account(
    mut args: GetDetailOnSubAccountsFuturesAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDetailOnSubAccountsFuturesAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDetailOnSubAccountsFuturesAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                GetDetailOnSubAccountsFuturesAccountParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_detail_on_sub_accounts_futures_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_detail_on_sub_accounts_futures_account_v2(
    mut args: GetDetailOnSubAccountsFuturesAccountV2Args,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDetailOnSubAccountsFuturesAccountV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDetailOnSubAccountsFuturesAccountV2Params>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.futures_type.is_none() {
                        let futures_type: i64 = Input::new()
                            .with_prompt("Please enter the futures_type name")
                            .interact_text()?;

                        args.futures_type = Some(futures_type);
                    }
                }
                GetDetailOnSubAccountsFuturesAccountV2Params::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.futures_type
                        .ok_or_else(|| anyhow::anyhow!("futures_type is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_detail_on_sub_accounts_futures_account_v2(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_detail_on_sub_accounts_margin_account(
    mut args: GetDetailOnSubAccountsMarginAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDetailOnSubAccountsMarginAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDetailOnSubAccountsMarginAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                GetDetailOnSubAccountsMarginAccountParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_detail_on_sub_accounts_margin_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_move_position_history_for_sub_account(
    mut args: GetMovePositionHistoryForSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetMovePositionHistoryForSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetMovePositionHistoryForSubAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.page.is_none() {
                        let page: i64 = Input::new()
                            .with_prompt("Please enter the page name")
                            .interact_text()?;

                        args.page = Some(page);
                    }
                    if args.rows.is_none() {
                        let rows: i64 = Input::new()
                            .with_prompt("Please enter the rows name")
                            .interact_text()?;

                        args.rows = Some(rows);
                    }
                }
                GetMovePositionHistoryForSubAccountParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.page
                        .ok_or_else(|| anyhow::anyhow!("page is required"))?,
                    args.rows
                        .ok_or_else(|| anyhow::anyhow!("rows is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_move_position_history_for_sub_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_sub_account_deposit_address(
    mut args: GetSubAccountDepositAddressArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSubAccountDepositAddressParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSubAccountDepositAddressParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.coin.is_none() {
                        let coin: String = Input::new()
                            .with_prompt("Please enter the coin name")
                            .interact_text()?;

                        args.coin = Some(coin);
                    }
                }
                GetSubAccountDepositAddressParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                )
                .network(args.network)
                .amount(args.amount)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_sub_account_deposit_address(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_sub_account_deposit_history(
    mut args: GetSubAccountDepositHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSubAccountDepositHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSubAccountDepositHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                GetSubAccountDepositHistoryParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .include_source(args.include_source)
                .coin(args.coin)
                .status(args.status)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .offset(args.offset)
                .recv_window(args.recv_window)
                .tx_id(args.tx_id)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_sub_account_deposit_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_summary_of_sub_accounts_futures_account(
    mut args: GetSummaryOfSubAccountsFuturesAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSummaryOfSubAccountsFuturesAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetSummaryOfSubAccountsFuturesAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.page.is_none() {
                        let page: i64 = Input::new()
                            .with_prompt("Please enter the page name")
                            .interact_text()?;

                        args.page = Some(page);
                    }
                    if args.limit.is_none() {
                        let limit: i64 = Input::new()
                            .with_prompt("Please enter the limit name")
                            .interact_text()?;

                        args.limit = Some(limit);
                    }
                }
                GetSummaryOfSubAccountsFuturesAccountParams::builder(
                    args.page
                        .ok_or_else(|| anyhow::anyhow!("page is required"))?,
                    args.limit
                        .ok_or_else(|| anyhow::anyhow!("limit is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_summary_of_sub_accounts_futures_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_summary_of_sub_accounts_futures_account_v2(
    mut args: GetSummaryOfSubAccountsFuturesAccountV2Args,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSummaryOfSubAccountsFuturesAccountV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetSummaryOfSubAccountsFuturesAccountV2Params>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.futures_type.is_none() {
                        let futures_type: i64 = Input::new()
                            .with_prompt("Please enter the futures_type name")
                            .interact_text()?;

                        args.futures_type = Some(futures_type);
                    }
                }
                GetSummaryOfSubAccountsFuturesAccountV2Params::builder(
                    args.futures_type
                        .ok_or_else(|| anyhow::anyhow!("futures_type is required"))?,
                )
                .page(args.page)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_summary_of_sub_accounts_futures_account_v2(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_summary_of_sub_accounts_margin_account(
    args: GetSummaryOfSubAccountsMarginAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSummaryOfSubAccountsMarginAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetSummaryOfSubAccountsMarginAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetSummaryOfSubAccountsMarginAccountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_summary_of_sub_accounts_margin_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_transfer_for_sub_account(
    mut args: MarginTransferForSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginTransferForSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<MarginTransferForSubAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.r#type.is_none() {
                        let r#type: i64 = Input::new()
                            .with_prompt("Please enter the r#type name")
                            .interact_text()?;

                        args.r#type = Some(r#type);
                    }
                }
                MarginTransferForSubAccountParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
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
    let response = rest_client.margin_transfer_for_sub_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn move_position_for_sub_account(
    mut args: MovePositionForSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MovePositionForSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<MovePositionForSubAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.from_user_email.is_none() {
                        let from_user_email: String = Input::new()
                            .with_prompt("Please enter the from_user_email name")
                            .interact_text()?;

                        args.from_user_email = Some(from_user_email);
                    }
                    if args.to_user_email.is_none() {
                        let to_user_email: String = Input::new()
                            .with_prompt("Please enter the to_user_email name")
                            .interact_text()?;

                        args.to_user_email = Some(to_user_email);
                    }
                    if args.product_type.is_none() {
                        let options = vec![("UM", MovePositionForSubAccountProductTypeEnum::Um)];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the product_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.product_type = Some(selected);
                    }
                    if args.order_args.is_none() {
                        let order_args: String = Input::new()
                            .with_prompt("Please enter the order_args name")
                            .interact_text()?;

                        args.order_args = Some(order_args);
                    }
                }
                MovePositionForSubAccountParams::builder(
                    args.from_user_email
                        .ok_or_else(|| anyhow::anyhow!("from_user_email is required"))?,
                    args.to_user_email
                        .ok_or_else(|| anyhow::anyhow!("to_user_email is required"))?,
                    args.product_type
                        .ok_or_else(|| anyhow::anyhow!("product_type is required"))?,
                    serde_json::from_str::<
                        Vec<models::MovePositionForSubAccountOrderArgsParameterInner>,
                    >(
                        &args
                            .order_args
                            .ok_or_else(|| anyhow::anyhow!("order_args is required"))?,
                    )?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.move_position_for_sub_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_account_assets(mut args: QuerySubAccountAssetsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubAccountAssetsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySubAccountAssetsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                QuerySubAccountAssetsParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_sub_account_assets(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_account_assets_asset_management(
    mut args: QuerySubAccountAssetsAssetManagementArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubAccountAssetsAssetManagementParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySubAccountAssetsAssetManagementParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                QuerySubAccountAssetsAssetManagementParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_sub_account_assets_asset_management(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_account_futures_asset_transfer_history(
    mut args: QuerySubAccountFuturesAssetTransferHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubAccountFuturesAssetTransferHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySubAccountFuturesAssetTransferHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.futures_type.is_none() {
                        let futures_type: i64 = Input::new()
                            .with_prompt("Please enter the futures_type name")
                            .interact_text()?;

                        args.futures_type = Some(futures_type);
                    }
                }
                QuerySubAccountFuturesAssetTransferHistoryParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.futures_type
                        .ok_or_else(|| anyhow::anyhow!("futures_type is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .page(args.page)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_sub_account_futures_asset_transfer_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_account_spot_asset_transfer_history(
    args: QuerySubAccountSpotAssetTransferHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubAccountSpotAssetTransferHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySubAccountSpotAssetTransferHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QuerySubAccountSpotAssetTransferHistoryParams::builder()
                .from_email(args.from_email)
                .to_email(args.to_email)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .page(args.page)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_sub_account_spot_asset_transfer_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_account_spot_assets_summary(
    args: QuerySubAccountSpotAssetsSummaryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubAccountSpotAssetsSummaryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QuerySubAccountSpotAssetsSummaryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QuerySubAccountSpotAssetsSummaryParams::builder()
                .email(args.email)
                .page(args.page)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_sub_account_spot_assets_summary(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_universal_transfer_history(
    args: QueryUniversalTransferHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUniversalTransferHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryUniversalTransferHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryUniversalTransferHistoryParams::builder()
                .from_email(args.from_email)
                .to_email(args.to_email)
                .client_tran_id(args.client_tran_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .page(args.page)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_universal_transfer_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn sub_account_futures_asset_transfer(
    mut args: SubAccountFuturesAssetTransferArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubAccountFuturesAssetTransferParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SubAccountFuturesAssetTransferParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.from_email.is_none() {
                        let from_email: String = Input::new()
                            .with_prompt("Please enter the from_email name")
                            .interact_text()?;

                        args.from_email = Some(from_email);
                    }
                    if args.to_email.is_none() {
                        let to_email: String = Input::new()
                            .with_prompt("Please enter the to_email name")
                            .interact_text()?;

                        args.to_email = Some(to_email);
                    }
                    if args.futures_type.is_none() {
                        let futures_type: i64 = Input::new()
                            .with_prompt("Please enter the futures_type name")
                            .interact_text()?;

                        args.futures_type = Some(futures_type);
                    }
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                SubAccountFuturesAssetTransferParams::builder(
                    args.from_email
                        .ok_or_else(|| anyhow::anyhow!("from_email is required"))?,
                    args.to_email
                        .ok_or_else(|| anyhow::anyhow!("to_email is required"))?,
                    args.futures_type
                        .ok_or_else(|| anyhow::anyhow!("futures_type is required"))?,
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
    let response = rest_client
        .sub_account_futures_asset_transfer(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn sub_account_transfer_history(
    args: SubAccountTransferHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubAccountTransferHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SubAccountTransferHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => SubAccountTransferHistoryParams::builder()
                .asset(args.asset)
                .r#type(args.r#type)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .return_fail_history(args.return_fail_history)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.sub_account_transfer_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn transfer_to_master(mut args: TransferToMasterArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TransferToMasterParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TransferToMasterParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                TransferToMasterParams::builder(
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
    let response = rest_client.transfer_to_master(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn transfer_to_sub_account_of_same_master(
    mut args: TransferToSubAccountOfSameMasterArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TransferToSubAccountOfSameMasterParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<TransferToSubAccountOfSameMasterParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.to_email.is_none() {
                        let to_email: String = Input::new()
                            .with_prompt("Please enter the to_email name")
                            .interact_text()?;

                        args.to_email = Some(to_email);
                    }
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                TransferToSubAccountOfSameMasterParams::builder(
                    args.to_email
                        .ok_or_else(|| anyhow::anyhow!("to_email is required"))?,
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
    let response = rest_client
        .transfer_to_sub_account_of_same_master(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn universal_transfer(mut args: UniversalTransferArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UniversalTransferParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UniversalTransferParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.from_account_type.is_none() {
                        let options = vec![
                            ("SPOT", UniversalTransferFromAccountTypeEnum::Spot),
                            (
                                "USDT_FUTURE",
                                UniversalTransferFromAccountTypeEnum::UsdtFuture,
                            ),
                            (
                                "COIN_FUTURE",
                                UniversalTransferFromAccountTypeEnum::CoinFuture,
                            ),
                            ("MARGIN", UniversalTransferFromAccountTypeEnum::Margin),
                            (
                                "ISOLATED_MARGIN",
                                UniversalTransferFromAccountTypeEnum::IsolatedMargin,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the from_account_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.from_account_type = Some(selected);
                    }
                    if args.to_account_type.is_none() {
                        let options = vec![
                            ("SPOT", UniversalTransferToAccountTypeEnum::Spot),
                            (
                                "USDT_FUTURE",
                                UniversalTransferToAccountTypeEnum::UsdtFuture,
                            ),
                            (
                                "COIN_FUTURE",
                                UniversalTransferToAccountTypeEnum::CoinFuture,
                            ),
                            ("MARGIN", UniversalTransferToAccountTypeEnum::Margin),
                            (
                                "ISOLATED_MARGIN",
                                UniversalTransferToAccountTypeEnum::IsolatedMargin,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the to_account_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.to_account_type = Some(selected);
                    }
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                UniversalTransferParams::builder(
                    args.from_account_type
                        .ok_or_else(|| anyhow::anyhow!("from_account_type is required"))?,
                    args.to_account_type
                        .ok_or_else(|| anyhow::anyhow!("to_account_type is required"))?,
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .from_email(args.from_email)
                .to_email(args.to_email)
                .client_tran_id(args.client_tran_id)
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.universal_transfer(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn deposit_assets_into_the_managed_sub_account(
    mut args: DepositAssetsIntoTheManagedSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DepositAssetsIntoTheManagedSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DepositAssetsIntoTheManagedSubAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.to_email.is_none() {
                        let to_email: String = Input::new()
                            .with_prompt("Please enter the to_email name")
                            .interact_text()?;

                        args.to_email = Some(to_email);
                    }
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                DepositAssetsIntoTheManagedSubAccountParams::builder(
                    args.to_email
                        .ok_or_else(|| anyhow::anyhow!("to_email is required"))?,
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
    let response = rest_client
        .deposit_assets_into_the_managed_sub_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_managed_sub_account_deposit_address(
    mut args: GetManagedSubAccountDepositAddressArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetManagedSubAccountDepositAddressParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetManagedSubAccountDepositAddressParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.coin.is_none() {
                        let coin: String = Input::new()
                            .with_prompt("Please enter the coin name")
                            .interact_text()?;

                        args.coin = Some(coin);
                    }
                }
                GetManagedSubAccountDepositAddressParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                )
                .network(args.network)
                .amount(args.amount)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_managed_sub_account_deposit_address(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_managed_sub_account_asset_details(
    mut args: QueryManagedSubAccountAssetDetailsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryManagedSubAccountAssetDetailsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryManagedSubAccountAssetDetailsParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                QueryManagedSubAccountAssetDetailsParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_managed_sub_account_asset_details(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_managed_sub_account_futures_asset_details(
    mut args: QueryManagedSubAccountFuturesAssetDetailsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryManagedSubAccountFuturesAssetDetailsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryManagedSubAccountFuturesAssetDetailsParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                QueryManagedSubAccountFuturesAssetDetailsParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .account_type(args.account_type)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_managed_sub_account_futures_asset_details(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_managed_sub_account_list(
    args: QueryManagedSubAccountListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryManagedSubAccountListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryManagedSubAccountListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryManagedSubAccountListParams::builder()
                .email(args.email)
                .page(args.page)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_managed_sub_account_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_managed_sub_account_margin_asset_details(
    mut args: QueryManagedSubAccountMarginAssetDetailsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryManagedSubAccountMarginAssetDetailsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryManagedSubAccountMarginAssetDetailsParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                }
                QueryManagedSubAccountMarginAssetDetailsParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                )
                .account_type(args.account_type)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_managed_sub_account_margin_asset_details(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_managed_sub_account_snapshot(
    mut args: QueryManagedSubAccountSnapshotArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryManagedSubAccountSnapshotParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryManagedSubAccountSnapshotParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.r#type.is_none() {
                        let options = vec![
                            ("SPOT", QueryManagedSubAccountSnapshotTypeEnum::Spot),
                            ("MARGIN", QueryManagedSubAccountSnapshotTypeEnum::Margin),
                            ("FUTURES", QueryManagedSubAccountSnapshotTypeEnum::Futures),
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
                QueryManagedSubAccountSnapshotParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_managed_sub_account_snapshot(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_managed_sub_account_transfer_log_master_account_investor(
    mut args: QueryManagedSubAccountTransferLogMasterAccountInvestorArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params =
        match read_stdin_as::<QueryManagedSubAccountTransferLogMasterAccountInvestorParams>() {
            Some(params) => params,
            None => match args.json {
                Some(json) => read_json_as::<
                    QueryManagedSubAccountTransferLogMasterAccountInvestorParams,
                >(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
                None => {
                    if args.interactive {
                        if args.email.is_none() {
                            let email: String = Input::new()
                                .with_prompt("Please enter the email name")
                                .interact_text()?;

                            args.email = Some(email);
                        }
                        if args.start_time.is_none() {
                            let start_time: i64 = Input::new()
                                .with_prompt("Please enter the start_time name")
                                .interact_text()?;

                            args.start_time = Some(start_time);
                        }
                        if args.end_time.is_none() {
                            let end_time: i64 = Input::new()
                                .with_prompt("Please enter the end_time name")
                                .interact_text()?;

                            args.end_time = Some(end_time);
                        }
                        if args.page.is_none() {
                            let page: i64 = Input::new()
                                .with_prompt("Please enter the page name")
                                .interact_text()?;

                            args.page = Some(page);
                        }
                        if args.limit.is_none() {
                            let limit: i64 = Input::new()
                                .with_prompt("Please enter the limit name")
                                .interact_text()?;

                            args.limit = Some(limit);
                        }
                    }
                    QueryManagedSubAccountTransferLogMasterAccountInvestorParams::builder(
                        args.email
                            .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                        args.start_time
                            .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                        args.end_time
                            .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                        args.page
                            .ok_or_else(|| anyhow::anyhow!("page is required"))?,
                        args.limit
                            .ok_or_else(|| anyhow::anyhow!("limit is required"))?,
                    )
                    .transfers(args.transfers)
                    .transfer_function_account_type(args.transfer_function_account_type)
                    .build()?
                }
            },
        };

    // Make the API call
    let response = rest_client
        .query_managed_sub_account_transfer_log_master_account_investor(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_managed_sub_account_transfer_log_master_account_trading(
    mut args: QueryManagedSubAccountTransferLogMasterAccountTradingArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params =
        match read_stdin_as::<QueryManagedSubAccountTransferLogMasterAccountTradingParams>() {
            Some(params) => params,
            None => match args.json {
                Some(json) => read_json_as::<
                    QueryManagedSubAccountTransferLogMasterAccountTradingParams,
                >(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
                None => {
                    if args.interactive {
                        if args.email.is_none() {
                            let email: String = Input::new()
                                .with_prompt("Please enter the email name")
                                .interact_text()?;

                            args.email = Some(email);
                        }
                        if args.start_time.is_none() {
                            let start_time: i64 = Input::new()
                                .with_prompt("Please enter the start_time name")
                                .interact_text()?;

                            args.start_time = Some(start_time);
                        }
                        if args.end_time.is_none() {
                            let end_time: i64 = Input::new()
                                .with_prompt("Please enter the end_time name")
                                .interact_text()?;

                            args.end_time = Some(end_time);
                        }
                        if args.page.is_none() {
                            let page: i64 = Input::new()
                                .with_prompt("Please enter the page name")
                                .interact_text()?;

                            args.page = Some(page);
                        }
                        if args.limit.is_none() {
                            let limit: i64 = Input::new()
                                .with_prompt("Please enter the limit name")
                                .interact_text()?;

                            args.limit = Some(limit);
                        }
                    }
                    QueryManagedSubAccountTransferLogMasterAccountTradingParams::builder(
                        args.email
                            .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                        args.start_time
                            .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                        args.end_time
                            .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                        args.page
                            .ok_or_else(|| anyhow::anyhow!("page is required"))?,
                        args.limit
                            .ok_or_else(|| anyhow::anyhow!("limit is required"))?,
                    )
                    .transfers(args.transfers)
                    .transfer_function_account_type(args.transfer_function_account_type)
                    .build()?
                }
            },
        };

    // Make the API call
    let response = rest_client
        .query_managed_sub_account_transfer_log_master_account_trading(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_managed_sub_account_transfer_log_sub_account_trading(
    mut args: QueryManagedSubAccountTransferLogSubAccountTradingArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryManagedSubAccountTransferLogSubAccountTradingParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryManagedSubAccountTransferLogSubAccountTradingParams>(json)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                    })?
            }
            None => {
                if args.interactive {
                    if args.start_time.is_none() {
                        let start_time: i64 = Input::new()
                            .with_prompt("Please enter the start_time name")
                            .interact_text()?;

                        args.start_time = Some(start_time);
                    }
                    if args.end_time.is_none() {
                        let end_time: i64 = Input::new()
                            .with_prompt("Please enter the end_time name")
                            .interact_text()?;

                        args.end_time = Some(end_time);
                    }
                    if args.page.is_none() {
                        let page: i64 = Input::new()
                            .with_prompt("Please enter the page name")
                            .interact_text()?;

                        args.page = Some(page);
                    }
                    if args.limit.is_none() {
                        let limit: i64 = Input::new()
                            .with_prompt("Please enter the limit name")
                            .interact_text()?;

                        args.limit = Some(limit);
                    }
                }
                QueryManagedSubAccountTransferLogSubAccountTradingParams::builder(
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                    args.page
                        .ok_or_else(|| anyhow::anyhow!("page is required"))?,
                    args.limit
                        .ok_or_else(|| anyhow::anyhow!("limit is required"))?,
                )
                .transfers(args.transfers)
                .transfer_function_account_type(args.transfer_function_account_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_managed_sub_account_transfer_log_sub_account_trading(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn withdrawl_assets_from_the_managed_sub_account(
    mut args: WithdrawlAssetsFromTheManagedSubAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<WithdrawlAssetsFromTheManagedSubAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<WithdrawlAssetsFromTheManagedSubAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.from_email.is_none() {
                        let from_email: String = Input::new()
                            .with_prompt("Please enter the from_email name")
                            .interact_text()?;

                        args.from_email = Some(from_email);
                    }
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                WithdrawlAssetsFromTheManagedSubAccountParams::builder(
                    args.from_email
                        .ok_or_else(|| anyhow::anyhow!("from_email is required"))?,
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .transfer_date(args.transfer_date)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .withdrawl_assets_from_the_managed_sub_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
