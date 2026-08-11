use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::MARGIN_TRADING_REST_API_PROD_URL;
use binance_sdk::margin_trading::MarginTradingRestApi;
use binance_sdk::margin_trading::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("margin-trading");

    let client_config = get_client_configuration(profile, "margin-trading").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => MARGIN_TRADING_REST_API_PROD_URL.to_string(),
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

    Ok(MarginTradingRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct AdjustCrossMarginMaxLeverageArgs {
    #[arg(
        help = r#"Can only adjust 3 , 5 or 10，Example: maxLeverage = 5 or 3 for Cross Margin Classic; maxLeverage=10 for Cross Margin Pro 10x leverage or 20x if compliance allows."#,
        long
    )]
    max_leverage: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct DisableIsolatedMarginAccountArgs {
    #[arg(help = r#""#, long)]
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
struct EnableIsolatedMarginAccountArgs {
    #[arg(help = r#""#, long)]
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
struct GetBnbBurnStatusArgs {
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
struct GetSummaryOfMarginAccountArgs {
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
struct QueryCrossIsolatedMarginCapitalFlowArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Mandatory for Isolated data"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    r#type: Option<QueryCrossIsolatedMarginCapitalFlowTypeEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    from_id: Option<i64>,
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
struct QueryCrossMarginAccountDetailsArgs {
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
struct QueryCrossMarginFeeDataArgs {
    #[arg(
        help = r#"User's current specific margin data will be returned if vipLevel is omitted"#,
        long
    )]
    vip_level: Option<i64>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
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
struct QueryEnabledIsolatedMarginAccountLimitArgs {
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
struct QueryIsolatedMarginAccountInfoArgs {
    #[arg(help = r#""#, long)]
    symbols: Option<String>,
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
struct QueryIsolatedMarginFeeDataArgs {
    #[arg(help = r#""#, long)]
    vip_level: Option<i64>,
    #[arg(help = r#""#, long)]
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
struct GetFutureHourlyInterestRateArgs {
    #[arg(help = r#""#, long)]
    assets: Option<String>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<GetFutureHourlyInterestRateIsIsolatedEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetInterestHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    isolated_symbol: Option<String>,
    #[arg(help = r#"Only supports querying data from the past 90 days."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    current: Option<i64>,
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
struct MarginAccountBorrowRepayArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"`TRUE` for Isolated Margin, `FALSE` for Cross Margin"#, long)]
    is_isolated: Option<MarginAccountBorrowRepayIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    amount: Option<String>,
    #[arg(help = r#""#, long)]
    r#type: Option<MarginAccountBorrowRepayTypeEnum>,
    #[arg(help = r#"Only for Isolated margin"#, long)]
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
struct QueryBorrowRepayRecordsInMarginAccountArgs {
    #[arg(help = r#""#, long)]
    r#type: Option<QueryBorrowRepayRecordsInMarginAccountTypeEnum>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    isolated_symbol: Option<String>,
    #[arg(help = r#""#, long)]
    tx_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    current: Option<i64>,
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
struct QueryMarginInterestRateHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    vip_level: Option<i64>,
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
struct QueryMaxBorrowArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    isolated_symbol: Option<String>,
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
struct CrossMarginCollateralRatioArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetAllCrossMarginPairsArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetAllIsolatedMarginSymbolArgs {
    #[arg(help = r#""#, long)]
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
struct GetAllMarginAssetsArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetDelistScheduleArgs {
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
struct GetLimitPricePairsArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetListScheduleArgs {
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
struct GetMarginAssetRiskBasedLiquidationRatioArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetMarginRestrictedAssetsArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryIsolatedMarginTierDataArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    tier: Option<i64>,
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
struct QueryLiabilityCoinLeverageBracketInCrossMarginProModeArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryMarginAvailableInventoryArgs {
    #[arg(help = r#""#, long)]
    r#type: Option<QueryMarginAvailableInventoryTypeEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryMarginPriceindexArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CreateSpecialKeyArgs {
    #[arg(help = r#""#, long)]
    api_name: Option<String>,
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Can be added in batches, separated by commas. Max 30 for an API key"#,
        long
    )]
    ip: Option<String>,
    #[arg(
        help = r#"1. If publicKey is inputted it will create an RSA or Ed25519
key.

2. Need to be encoded to URL-encoded format"#,
        long
    )]
    public_key: Option<String>,
    #[arg(
        help = r#"This parameter is only for the Ed25519 API key, and does not effact for other encryption methods. The value can be TRADE (TRADE for all permissions) or READ (READ for USER_DATA, FIX_API_READ_ONLY). The default value is TRADE."#,
        long
    )]
    permission_mode: Option<CreateSpecialKeyPermissionModeEnum>,
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
struct DeleteSpecialKeyArgs {
    #[arg(help = r#""#, long)]
    api_name: Option<String>,
    #[arg(help = r#""#, long)]
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
struct EditIpForSpecialKeyArgs {
    #[arg(
        help = r#"Can be added in batches, separated by commas. Max 30 for an API key"#,
        long
    )]
    ip: Option<String>,
    #[arg(help = r#"isolated margin pair"#, long)]
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
struct ExitSpecialKeyModeArgs {
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
struct GetForceLiquidationRecordArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    isolated_symbol: Option<String>,
    #[arg(help = r#""#, long)]
    current: Option<i64>,
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
struct GetSmallLiabilityExchangeCoinListArgs {
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
struct GetSmallLiabilityExchangeHistoryArgs {
    #[arg(help = r#""#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
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
struct LiquidationLoanRepayArgs {
    #[arg(help = r#"The asset to repay (e.g. USDT, USDC)"#, long)]
    asset: Option<String>,
    #[arg(help = r#"Repayment amount, must be greater than 0"#, long)]
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
struct MarginAccountCancelAllOpenOrdersOnASymbolArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<MarginAccountCancelAllOpenOrdersOnASymbolIsIsolatedEnum>,
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
struct MarginAccountCancelOcoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<MarginAccountCancelOcoIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    order_list_id: Option<i64>,
    #[arg(help = r#""#, long)]
    list_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    new_client_order_id: Option<String>,
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
struct MarginAccountCancelOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<MarginAccountCancelOrderIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    new_client_order_id: Option<String>,
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
struct MarginAccountNewOcoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<MarginAccountNewOcoSideEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<MarginAccountNewOcoIsIsolatedEnum>,
    #[arg(help = r#"A unique Id for the entire orderList"#, long)]
    list_client_order_id: Option<String>,
    #[arg(help = r#"A unique Id for the limit order"#, long)]
    limit_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    limit_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#"A unique Id for the stop loss/stop loss limit leg"#, long)]
    stop_client_order_id: Option<String>,
    #[arg(help = r#"If provided, `stopLimitTimeInForce` is required."#, long)]
    stop_limit_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    stop_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    stop_limit_time_in_force: Option<MarginAccountNewOcoStopLimitTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    new_order_resp_type: Option<MarginAccountNewOcoNewOrderRespTypeEnum>,
    #[arg(help = r#""#, long)]
    side_effect_type: Option<MarginAccountNewOcoSideEffectTypeEnum>,
    #[arg(help = r#""#, long)]
    self_trade_prevention_mode: Option<MarginAccountNewOcoSelfTradePreventionModeEnum>,
    #[arg(help = r#"Only when MARGIN_BUY or AUTO_BORROW_REPAY order takes effect, true means that the debt generated by the order needs to be repay after the order is cancelled."#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_repay_at_cancel: Option<bool>,
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
struct MarginAccountNewOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<MarginAccountNewOrderSideEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<MarginAccountNewOrderTypeEnum>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<MarginAccountNewOrderIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    quote_order_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Used with `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders."#,
        long
    )]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent."#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(
        help = r#"Used with `LIMIT`, `STOP_LOSS_LIMIT`, and `TAKE_PROFIT_LIMIT` to create an iceberg order."#,
        long
    )]
    iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"MARKET and LIMIT order types default to FULL, all other orders default to ACK."#,
        long
    )]
    new_order_resp_type: Option<MarginAccountNewOrderNewOrderRespTypeEnum>,
    #[arg(help = r#""#, long)]
    side_effect_type: Option<MarginAccountNewOrderSideEffectTypeEnum>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<MarginAccountNewOrderTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    self_trade_prevention_mode: Option<MarginAccountNewOrderSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"Used with `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders."#,
        long
    )]
    trailing_delta: Option<i64>,
    #[arg(help = r#"Only when MARGIN_BUY or AUTO_BORROW_REPAY order takes effect, true means that the debt generated by the order needs to be repaid after the order is cancelled."#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_repay_at_cancel: Option<bool>,
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
struct MarginAccountNewOtoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    working_type: Option<MarginAccountNewOtoWorkingTypeEnum>,
    #[arg(help = r#""#, long)]
    working_side: Option<MarginAccountNewOtoWorkingSideEnum>,
    #[arg(help = r#""#, long)]
    working_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Sets the quantity for the working order."#, long)]
    working_quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `workingTimeInForce` is `GTC`."#,
        long
    )]
    working_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_type: Option<MarginAccountNewOtoPendingTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_side: Option<MarginAccountNewOtoPendingSideEnum>,
    #[arg(help = r#"Sets the quantity for the pending order."#, long)]
    pending_quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<MarginAccountNewOtoIsIsolatedEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open order lists. Automatically generated if not sent.<br/>A new order list with the same listClientOrderId is accepted only when the previous one is filled or completely expired.<br/>`listClientOrderId` is distinct from the `workingClientOrderId` and the `pendingClientOrderId`."#,
        long
    )]
    list_client_order_id: Option<String>,
    #[arg(
        help = r#"MARKET and LIMIT order types default to FULL, all other orders default to ACK."#,
        long
    )]
    new_order_resp_type: Option<MarginAccountNewOtoNewOrderRespTypeEnum>,
    #[arg(help = r#""#, long)]
    side_effect_type: Option<MarginAccountNewOtoSideEffectTypeEnum>,
    #[arg(help = r#""#, long)]
    self_trade_prevention_mode: Option<MarginAccountNewOtoSelfTradePreventionModeEnum>,
    #[arg(help = r#"Only when MARGIN_BUY order takes effect, true means that the debt generated by the order needs to be repaid after the order is cancelled."#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_repay_at_cancel: Option<bool>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the working order. Automatically generated if not sent."#,
        long
    )]
    working_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    working_time_in_force: Option<MarginAccountNewOtoWorkingTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the pending order. Automatically generated if not sent."#,
        long
    )]
    pending_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    pending_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_trailing_delta: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `pendingTimeInForce` is `GTC`."#,
        long
    )]
    pending_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_time_in_force: Option<MarginAccountNewOtoPendingTimeInForceEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MarginAccountNewOtocoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    working_type: Option<MarginAccountNewOtocoWorkingTypeEnum>,
    #[arg(help = r#""#, long)]
    working_side: Option<MarginAccountNewOtocoWorkingSideEnum>,
    #[arg(help = r#""#, long)]
    working_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    working_quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_side: Option<MarginAccountNewOtocoPendingSideEnum>,
    #[arg(help = r#""#, long)]
    pending_quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_above_type: Option<MarginAccountNewOtocoPendingAboveTypeEnum>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<MarginAccountNewOtocoIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    side_effect_type: Option<MarginAccountNewOtocoSideEffectTypeEnum>,
    #[arg(help = r#"Only when MARGIN_BUY order takes effect, true means that the debt generated by the order needs to be repaid after the order is cancelled."#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_repay_at_cancel: Option<bool>,
    #[arg(
        help = r#"Arbitrary unique ID among open order lists. Automatically generated if not sent. A new order list with the same listClientOrderId is accepted only when the previous one is filled or completely expired. `listClientOrderId` is distinct from the `workingClientOrderId`, `pendingAboveClientOrderId`, and the `pendingBelowClientOrderId`."#,
        long
    )]
    list_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    new_order_resp_type: Option<MarginAccountNewOtocoNewOrderRespTypeEnum>,
    #[arg(help = r#""#, long)]
    self_trade_prevention_mode: Option<MarginAccountNewOtocoSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the working order. Automatically generated if not sent."#,
        long
    )]
    working_client_order_id: Option<String>,
    #[arg(
        help = r#"This can only be used if `workingTimeInForce` is `GTC`."#,
        long
    )]
    working_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    working_time_in_force: Option<MarginAccountNewOtocoWorkingTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the pending above order. Automatically generated if not sent."#,
        long
    )]
    pending_above_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    pending_above_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_above_stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_above_trailing_delta: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `pendingAboveTimeInForce` is `GTC`."#,
        long
    )]
    pending_above_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_above_time_in_force: Option<MarginAccountNewOtocoPendingAboveTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    pending_below_type: Option<MarginAccountNewOtocoPendingBelowTypeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the pending below order. Automatically generated if not sent."#,
        long
    )]
    pending_below_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    pending_below_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_below_stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_below_trailing_delta: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `pendingBelowTimeInForce` is `GTC`."#,
        long
    )]
    pending_below_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    pending_below_time_in_force: Option<MarginAccountNewOtocoPendingBelowTimeInForceEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MarginManualLiquidationArgs {
    #[arg(help = r#""#, long)]
    r#type: Option<MarginManualLiquidationTypeEnum>,
    #[arg(
        help = r#"When type selects `ISOLATED`, `symbol` must be filled in"#,
        long
    )]
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
struct QueryCurrentMarginOrderCountUsageArgs {
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryCurrentMarginOrderCountUsageIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
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
struct QueryLiquidationLoanArgs {
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
struct QueryLiquidationLoanRepayHistoryArgs {
    #[arg(
        help = r#"Start time in Unix timestamp (milliseconds). Defaults to 7 days ago if not specified"#,
        long
    )]
    start_time: Option<i64>,
    #[arg(
        help = r#"End time in Unix timestamp (milliseconds). Defaults to now if not specified"#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#"Current page number, default `1`"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Page size, default `50`"#, long)]
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
struct QueryMarginAccountsAllOcoArgs {
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryMarginAccountsAllOcoIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    from_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
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
struct QueryMarginAccountsAllOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryMarginAccountsAllOrdersIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
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
struct QueryMarginAccountsOcoArgs {
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryMarginAccountsOcoIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_list_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
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
struct QueryMarginAccountsOpenOcoArgs {
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryMarginAccountsOpenOcoIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
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
struct QueryMarginAccountsOpenOrdersArgs {
    #[arg(help = r#"isolated margin pair"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryMarginAccountsOpenOrdersIsIsolatedEnum>,
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
struct QueryMarginAccountsOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryMarginAccountsOrderIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
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
struct QueryMarginAccountsTradeListArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryMarginAccountsTradeListIsIsolatedEnum>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    from_id: Option<i64>,
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
struct QueryPreventedMatchesArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    prevented_match_id: Option<i64>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    from_prevented_match_id: Option<i64>,
    #[arg(help = r#""#, long)]
    is_isolated: Option<QueryPreventedMatchesIsIsolatedEnum>,
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
struct QuerySpecialKeyArgs {
    #[arg(help = r#""#, long)]
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
struct QuerySpecialKeyListArgs {
    #[arg(help = r#""#, long)]
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
struct SmallLiabilityExchangeArgs {
    #[arg(help = r#"The assets list of small liability exchange"#, long)]
    asset_names: Option<String>,
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
struct GetCrossMarginTransferHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    r#type: Option<GetCrossMarginTransferHistoryTypeEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
    #[arg(help = r#""#, long)]
    isolated_symbol: Option<String>,
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
struct QueryMaxTransferOutAmountArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    isolated_symbol: Option<String>,
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
struct CloseUserDataStreamArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct KeepaliveUserDataStreamArgs {
    #[arg(help = r#""#, long)]
    listen_key: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct StartUserDataStreamArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum MarginTradingCommands {
    #[command(
        about = decode_selected_entities(r#"Adjust cross margin max leverage

Weight(UID): 3000, 1 times/min per IP

Security Type: USER_DATA

Notes:
- The margin level need higher than the initial risk ratio of adjusted leverage, the initial risk ratio of 3x is 1.5 , the initial risk ratio of 5x is 1.25; The detail conditions on how to switch between Cross Margin Classic and Cross Margin Pro can refer to [the FAQ](https://www.binance.com/en/support/faq/how-to-activate-the-cross-margin-pro-mode-on-binance-e27786da05e743a694b8c625b3bc475d)."#, false),
    )]
    AdjustCrossMarginMaxLeverage(AdjustCrossMarginMaxLeverageArgs),
    #[command(
        about = decode_selected_entities(r#"Disable isolated margin account for a specific symbol. Each trading pair can only be deactivated once every 24 hours.

Weight(UID): 300

Security Type: TRADE"#, false),
    )]
    DisableIsolatedMarginAccount(DisableIsolatedMarginAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Enable isolated margin account for a specific symbol(Only supports activation of previously disabled accounts).

Weight(UID): 300

Security Type: TRADE"#, false),
    )]
    EnableIsolatedMarginAccount(EnableIsolatedMarginAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get BNB Burn Status

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetBnbBurnStatus(GetBnbBurnStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get personal margin level information

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    GetSummaryOfMarginAccount(GetSummaryOfMarginAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Query Cross Isolated Margin Capital Flow

Weight(IP): 100

Security Type: USER_DATA

Notes:
- Only supports querying the data of the last 90 days

- The time between startTime and endTime cannot be longer than 7 days.

- If fromId is set, the data with id > fromId will be returned.
Otherwise the latest data will be returned

- To query isolated data, Symbol needs to be entered."#, false),
    )]
    QueryCrossIsolatedMarginCapitalFlow(QueryCrossIsolatedMarginCapitalFlowArgs),
    #[command(
        about = decode_selected_entities(r#"Query Cross Margin Account Details

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    QueryCrossMarginAccountDetails(QueryCrossMarginAccountDetailsArgs),
    #[command(
        about = decode_selected_entities(r#"Get cross margin fee data collection with any vip level or user's current specific data as https://www.binance.com/en/margin-fee

Weight: 1 when coin is specified;(IP) 5 when the coin parameter is omitted(IP)

Security Type: USER_DATA"#, false),
    )]
    QueryCrossMarginFeeData(QueryCrossMarginFeeDataArgs),
    #[command(
        about = decode_selected_entities(r#"Query enabled isolated margin account limit.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryEnabledIsolatedMarginAccountLimit(QueryEnabledIsolatedMarginAccountLimitArgs),
    #[command(
        about = decode_selected_entities(r#"Query Isolated Margin Account Info

Weight(IP): 10

Security Type: USER_DATA

Notes:
- If "symbols" is not sent, all isolated assets will be returned.

- If "symbols" is sent, only the isolated assets of the sent symbols
will be returned."#, false),
    )]
    QueryIsolatedMarginAccountInfo(QueryIsolatedMarginAccountInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Get isolated margin fee data collection with any vip level or user's current specific data as https://www.binance.com/en/margin-fee

Weight: 1 when a single is specified;(IP) 10 when the symbol parameter is omitted(IP)

Security Type: USER_DATA"#, false),
    )]
    QueryIsolatedMarginFeeData(QueryIsolatedMarginFeeDataArgs),
    #[command(
        about = decode_selected_entities(r#"Get future hourly interest rate

Weight(IP): 100

Security Type: USER_DATA"#, false),
    )]
    GetFutureHourlyInterestRate(GetFutureHourlyInterestRateArgs),
    #[command(
        about = decode_selected_entities(r#"Get Interest History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Response in descending order

- If isolatedSymbol is not sent, crossed margin data will be returned

- The max interval between `startTime` and `endTime` is 30 days. It is a
MUST to ensure data correctness.

- If `startTime`and `endTime` not sent, return records of the last 7
days by default.

- If `startTime` is sent and `endTime` is not sent, return records of
[max(`startTime`, now-30d), now].

- If `startTime` is not sent and `endTime` is sent, return records of
[`endTime`-7, `endTime`]

- `type` in response has 4 enums:

- `PERIODIC` interest charged per hour

- `ON_BORROW` first interest charged on borrow

- `PERIODIC_CONVERTED` interest charged per hour converted into BNB

- `ON_BORROW_CONVERTED` first interest charged on borrow converted into
BNB

- `PORTFOLIO` interest charged daily on the portfolio margin negative
balance"#, false),
    )]
    GetInterestHistory(GetInterestHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Margin account borrow/repay

Weight(UID): 1500

Security Type: USER_DATA"#, false),
    )]
    MarginAccountBorrowRepay(MarginAccountBorrowRepayArgs),
    #[command(
        about = decode_selected_entities(r#"Query borrow/repay records in Margin account

Weight(IP): 10

Security Type: USER_DATA

Notes:
- `txId` or `startTime` must be sent. `txId` takes precedence.

- Response in descending order

- If an asset is sent, data within 30 days before `endTime`; If an asset is not sent, data within 7 days before `endTime`

- If neither `startTime` nor `endTime` is sent, the recent 7-day data will be returned.

- `startTime` set as `endTime` - 7 days by default, `endTime` set as current time by default"#, false),
    )]
    QueryBorrowRepayRecordsInMarginAccount(QueryBorrowRepayRecordsInMarginAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Interest Rate History

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryMarginInterestRateHistory(QueryMarginInterestRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query Max Borrow

Weight(IP): 50

Security Type: USER_DATA

Notes:
- If isolatedSymbol is not sent, crossed margin data will be sent.
- `borrowLimit` is also available from [https://www.binance.com/en/margin-fee](https://www.binance.com/en/margin-fee)"#, false),
    )]
    QueryMaxBorrow(QueryMaxBorrowArgs),
    #[command(
        about = decode_selected_entities(r#"Cross margin collateral ratio

Weight(IP): 100

Security Type: MARKET_DATA"#, false),
    )]
    CrossMarginCollateralRatio(CrossMarginCollateralRatioArgs),
    #[command(
        about = decode_selected_entities(r#"Get All Cross Margin Pairs

Weight(IP): 1

Security Type: MARKET_DATA"#, false),
    )]
    GetAllCrossMarginPairs(GetAllCrossMarginPairsArgs),
    #[command(
        about = decode_selected_entities(r#"Get All Isolated Margin Symbol

Weight(IP): 10

Security Type: MARKET_DATA"#, false),
    )]
    GetAllIsolatedMarginSymbol(GetAllIsolatedMarginSymbolArgs),
    #[command(
        about = decode_selected_entities(r#"Get All Margin Assets.

Weight(IP): 1

Security Type: MARKET_DATA"#, false),
    )]
    GetAllMarginAssets(GetAllMarginAssetsArgs),
    #[command(
        about = decode_selected_entities(r#"Get tokens or symbols delist schedule for cross margin and isolated margin

Weight(IP): 100

Security Type: MARKET_DATA"#, false),
    )]
    GetDelistSchedule(GetDelistScheduleArgs),
    #[command(
        about = decode_selected_entities(r#"Query trading pairs with restriction on limit price range.

In margin trading, you can place orders with limit price. Limit price
should be within (-15%, 15%) of current index price for a list of margin
trading pairs. This rule only impacts limit sell orders with limit price
that is lower than current index price and limit buy orders with limit
price that is higher than current index price.

- Buy order: Your order will be rejected with an error message
notification if the limit price is 15% above the index price.

- Sell order: Your order will be rejected with an error message
notification if the limit price is 15% below the index price.

Please review the limit price order placing strategy, backtest and
calibrate the planned order size with the trading volume and order book
depth to prevent trading loss.

Weight(IP): 1

Security Type: MARKET_DATA"#, false),
    )]
    GetLimitPricePairs(GetLimitPricePairsArgs),
    #[command(
        about = decode_selected_entities(r#"Get the upcoming tokens or symbols listing schedule for Cross Margin and Isolated Margin.

Weight(IP): 100

Security Type: MARKET_DATA"#, false),
    )]
    GetListSchedule(GetListScheduleArgs),
    #[command(
        about = decode_selected_entities(r#"Get Margin Asset Risk-Based Liquidation Ratio

Weight(IP): 1

Security Type: MARKET_DATA"#, false),
    )]
    GetMarginAssetRiskBasedLiquidationRatio(GetMarginAssetRiskBasedLiquidationRatioArgs),
    #[command(
        about = decode_selected_entities(r#"Get the list of margin-restricted assets.

Weight(IP): 1

Security Type: MARKET_DATA"#, false),
    )]
    GetMarginRestrictedAssets(GetMarginRestrictedAssetsArgs),
    #[command(
        about = decode_selected_entities(r#"Get isolated margin tier data collection with any tier as https://www.binance.com/en/margin-data

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryIsolatedMarginTierData(QueryIsolatedMarginTierDataArgs),
    #[command(
        about = decode_selected_entities(r#"Liability Coin Leverage Bracket in Cross Margin Pro Mode

Weight(IP): 1

Security Type: MARKET_DATA"#, false),
    )]
    QueryLiabilityCoinLeverageBracketInCrossMarginProMode(
        QueryLiabilityCoinLeverageBracketInCrossMarginProModeArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Margin available Inventory query

Weight(UID): 50

Security Type: USER_DATA"#, false),
    )]
    QueryMarginAvailableInventory(QueryMarginAvailableInventoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin PriceIndex

Weight(IP): 10

Security Type: MARKET_DATA"#, false),
    )]
    QueryMarginPriceindex(QueryMarginPriceindexArgs),
    #[command(
        about = decode_selected_entities(r#"**Eligibility**

- Binance Margin offers low-latency trading through a [special key](https://www.binance.com/en/support/faq/frequently-asked-questions-on-margin-special-api-key-3208663e900d4d2e9fec4140e1832f4e), available exclusively to users with VIP level 7 or higher.
- If you are VIP level 6 or below, please contact your VIP manager for eligibility criterias.
- All new Margin Special Key users are required to read, understand, and agree to the Margin Special Key Supplemental Product Terms at the master account level before creating a Margin Special Key.
- Once signed at the master account level, the agreement applies to all sub-accounts. The master account and all sub-accounts (Cross Margin Classic and Portfolio Margin Pro) are authorized to create a Margin Special Key and are subject to the LiquidationLoan policy.

For more information, please refer to [FAQ](https://www.binance.com/en/support/faq/detail/3208663e900d4d2e9fec4140e1832f4e).

**Supported Products:**

- Cross Margin
- Isolated Margin
- Portfolio Margin Pro

**Unsupported Products:**

- Portfolio Margin

We support several types of API keys:

* Ed25519 (recommended)
* HMAC
* RSA

We recommend to **use Ed25519 API keys** as it should provide the best performance and security out of all supported key types. We accept PKCS#8 (BEGIN PUBLIC KEY). For how to generate an RSA key pair to send API requests on Binance. Please refer to the document below [FAQ](https://www.binance.com/en/support/faq/how-to-generate-an-rsa-key-pair-to-send-api-requests-on-binance-2b79728f331e43079b27440d9d15c5db) .

**How to use the Margin Special Key**
- Use the below `sapi` endpoint to create your margin special API Key.
- For accessing the Cross Margin account, do not send the `symbol` parameter.
- For accessing the Isolated Margin account(s), pass the relevant `symbol` parameter in the API Key creation request.
- Use the generated API Key (and Secret key, if applicable) to perform margin trading and listenKey generation via **Spot** REST API (`https://api.binance.com/api/v3/*`) endpoints.

Read [REST API](/products/spot/rest-api#signed-trade-and-user_data-endpoint-security) or [WebSocket API](/products/spot/web-socket-api#request-security) documentation to learn how to use different API keys

You need to enable Permits “Enable Spot & Margin Trading” option for the API Key which requests this endpoint.

Weight(UID): 1

Security Type: TRADE

Response Notes:
- Error Code Description

- **UNSUPPORTED_OPERATION** : Portfolio Margin is an unsupported
product, please change the account type to a supported margin product.

- **Forbidden**:  Cross Margin Pro accounts require additional
agreements, please contact your relationship manager."#, false),
    )]
    CreateSpecialKey(CreateSpecialKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Deleting your Margin Special Key alone does not exit you from the Margin Special Key framework or discharge your obligations under the Margin Special Key Supplemental Product Terms. To fully exit, you must:

1. Delete your Margin Special Key.
2. Ensure there are no outstanding liabilities on the account.
3. Call the Exit Margin Special Key API endpoint.
4. Confirm the exit status via the API response.

Only after step 4 is completed and the exit status is confirmed by Binance will your account revert to standard liquidation logic and no longer be subject to the Margin Special Key Supplemental Product Terms.

If apiKey is given, apiName will be ignored. If apiName is given with no
apiKey, all apikeys with given apiName will be deleted.

You need to enable Permits “Enable Spot & Margin” option for the API Key
which requests this endpoint.

Weight(UID): 1

Security Type: TRADE"#, false),
    )]
    DeleteSpecialKey(DeleteSpecialKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Edit ip restriction. This only applies to Special Key for Low Latency
Trading.

You need to enable Permits “Enable Spot & Margin” option for the API Key
which requests this endpoint.

Weight(UID): 1

Security Type: TRADE"#, false),
    )]
    EditIpForSpecialKey(EditIpForSpecialKeyArgs),
    #[command(
        about = decode_selected_entities(r#"Exit the Margin Special Key mode for Cross Margin Classic accounts.

**All outstanding liabilities under the Cross Margin Classic account must be fully repaid before calling this endpoint.** Deleting the Margin Special Key alone does not constitute a valid exit.

When a user creates a Margin Special API Key, the account enters "Special Key Mode". Upon a successful request, the following actions will be performed atomically:

1. All existing Margin Special API Keys under the Cross Margin Classic mode account will be deleted.
2. All pre-execution margin checks (including Open-order-loss calculation) will revert to standard mode.
3. A cooldown period (default: 24 hours) will be enforced, during which the account will not be permitted to create new Margin Special API Keys.

For more information, please refer to [FAQ](https://www.binance.com/en/support/faq/detail/3208663e900d4d2e9fec4140e1832f4e).

**Preconditions:**

The following conditions must be met; otherwise the request will be rejected:

- Account type must be **Cross Margin Classic**.
- Account must currently be in **Special Key Mode**. If not, the request silently succeeds.
- Account must **not be in liquidation**.
- Account must **have no liability**.

You need to enable "Permits Enable Spot & Margin Trading" option for the API Key which requests this endpoint.

Weight(UID): 10

Security Type: TRADE"#, false),
    )]
    ExitSpecialKeyMode(ExitSpecialKeyModeArgs),
    #[command(
        about = decode_selected_entities(r#"Get Force Liquidation Record

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Response in descending order"#, false),
    )]
    GetForceLiquidationRecord(GetForceLiquidationRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Query the coins which can be small liability exchange

Weight(IP): 100

Security Type: USER_DATA"#, false),
    )]
    GetSmallLiabilityExchangeCoinList(GetSmallLiabilityExchangeCoinListArgs),
    #[command(
        about = decode_selected_entities(r#"Get Small liability Exchange History

Weight(UID): 100

Security Type: USER_DATA"#, false),
    )]
    GetSmallLiabilityExchangeHistory(GetSmallLiabilityExchangeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Repays the outstanding cross-margin liquidation loan from the user's spot wallet. A liquidation loan represents the account deficit incurred when account equity turns negative during liquidation (bankruptcy). The repayment amount must be greater than 0 and cannot exceed the remaining loan balance. If the Spot Account has insufficient USDC balance, the repayment will fail.

Weight(UID): 100

Security Type: MARGIN"#, false),
    )]
    LiquidationLoanRepay(LiquidationLoanRepayArgs),
    #[command(
        about = decode_selected_entities(r#"Cancels all active orders on a symbol for margin account.<br></br>
This includes OCO orders.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    MarginAccountCancelAllOpenOrdersOnASymbol(MarginAccountCancelAllOpenOrdersOnASymbolArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an entire Order List for a margin account.

Weight(UID): 1

Security Type: TRADE

Notes:
- Canceling an individual leg will cancel the entire OCO"#, false),
    )]
    MarginAccountCancelOco(MarginAccountCancelOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an active order for margin account.

Weight(IP): 10

Security Type: TRADE

Notes:
- Either orderId or origClientOrderId must be sent."#, false),
    )]
    MarginAccountCancelOrder(MarginAccountCancelOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a new OCO for a margin account

Weight: 6(UID) or 1500(UID) when sideEffectType is MARGIN_BUY or AUTO_BORROW_REPAY

Security Type: TRADE

Notes:
- autoRepayAtCancel is suggested to set as “FALSE” to keep liability unrepaid under high frequent new order/cancel order execution"#, false),
    )]
    MarginAccountNewOco(MarginAccountNewOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Post a new order for margin account.

Weight: 6(UID) or 1500(UID) when sideEffectType is MARGIN_BUY or AUTO_BORROW_REPAY

Security Type: TRADE

Notes:
- autoRepayAtCancel is suggested to set as “FALSE” to keep liability unrepaid under high frequent new order/cancel order execution"#, false),
    )]
    MarginAccountNewOrder(MarginAccountNewOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Post a new OTO order for margin account:

- An OTO (One-Triggers-the-Other) is an order list comprised of 2
orders.

- The first order is called the **working order** and must be `LIMIT` or
`LIMIT_MAKER`. Initially, only the working order goes on the order book.

- The second order is called the **pending order**. It can be any order
type except for `MARKET` orders using parameter `quoteOrderQty`. The
pending order is only placed on the order book when the working order
gets **fully filled**.

- If either the working order or the pending order is cancelled
individually, the other order in the order list will also be canceled or
expired.

- When the order list is placed, if the working order gets **immediately
fully filled**, the placement response will show the working order as
`FILLED` but the pending order will still appear as `PENDING_NEW`. You
need to query the status of the pending order again to see its updated
status.

- OTOs add **2 orders** to the unfilled order count,
`EXCHANGE_MAX_NUM_ORDERS` filter and `MAX_NUM_ORDERS` filter.

Weight: 6(UID) or 1500(UID) when sideEffectType is MARGIN_BUY or AUTO_BORROW_REPAY

Security Type: TRADE

Notes:
- autoRepayAtCancel is suggested to set as “FALSE” to keep liability unrepaid under high frequent new order/cancel order execution
- Depending on the `pendingType` or `workingType`, some optional
- parameters will become mandatory: | Type                                                     | Additional mandatory parameters                              | Additional information | | -------------------------------------------------------- | ------------------------------------------------------------ | ---------------------- | | `workingType` = `LIMIT`                                  | `workingTimeInForce`                                         |                        | | `pendingType` = `LIMIT`                                  | `pendingPrice`, `pendingTimeInForce`                         |                        | | `pendingType` = `STOP_LOSS` or `TAKE_PROFIT`             | `pendingStopPrice` and/or `pendingTrailingDelta`             |                        | | `pendingType` = `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT` | `pendingPrice`, `pendingStopPrice` and/or `pendingTrailingDelta`, `pendingTimeInForce` |                        | | `pendingTrailingDelta` is provided | `pendingPrice` |                        |"#, false),
    )]
    MarginAccountNewOto(MarginAccountNewOtoArgs),
    #[command(
        about = decode_selected_entities(r#"Post a new OTOCO order for margin account：


- An OTOCO (One-Triggers-One-Cancels-the-Other) is an order list
comprised of 3 orders.

- The first order is called the **working order** and must be `LIMIT` or
`LIMIT_MAKER`. Initially, only the working order goes on the order book.
  - The behavior of the working order is the same as the OTO.
- OTOCO has 2 pending orders (pending above and pending below), forming
an OCO pair. The pending orders are only placed on the order book when
the working order gets **fully filled**.
  - The rules of the pending above and pending below follow the same rules as the [Order List OCO](https://developers.binance.com/en/docs/catalog/core-trading-margin-trading/api/rest-api/trade#margin-account-new-oco).
- OTOCOs add **3 orders** against the unfilled order count,
`EXCHANGE_MAX_NUM_ORDERS` filter, and `MAX_NUM_ORDERS` filter.

Weight: 6(UID) or 1500(UID) when sideEffectType is MARGIN_BUY or AUTO_BORROW_REPAY

Security Type: TRADE

Notes:
- autoRepayAtCancel is suggested to set as “FALSE” to keep liability unrepaid under high frequent new order/cancel order execution
- Depending on the `pendingAboveType`/`pendingBelowType` or `workingType`, some optional parameters will become mandatory: | Type                                 | Additional mandatory parameters                              | Additional information | | ------------------------------------ | ------------------------------------------------------------ | ---------------------- | | `workingType` = `LIMIT`              | `workingTimeInForce`                                         |                        | | `pendingAboveType`= `LIMIT_MAKER`    | `pendingAbovePrice`                                          |                        | | `pendingAboveType`= `STOP_LOSS`      | `pendingAboveStopPrice` and/or `pendingAboveTrailingDelta`   |                        | | `pendingAboveType`=`STOP_LOSS_LIMIT` | `pendingAbovePrice`, `pendingAboveStopPrice` and/or `pendingAboveTrailingDelta`, `pendingAboveTimeInForce` |                        | | `pendingBelowType`= `LIMIT_MAKER`    | `pendingBelowPrice`                                          |                        | | `pendingBelowType`= `STOP_LOSS`      | `pendingBelowStopPrice` and/or `pendingBelowTrailingDelta`   |                        | | `pendingBelowType`=`STOP_LOSS_LIMIT` | `pendingBelowPrice`, `pendingBelowStopPrice` and/or `pendingBelowTrailingDelta`, `pendingBelowTimeInForce` |                        | | `pendingAboveTrailingDelta` is provided | `pendingAbovePrice` |                        | | `pendingBelowTrailingDelta` is provided | `pendingBelowPrice` |                        |"#, false),
    )]
    MarginAccountNewOtoco(MarginAccountNewOtocoArgs),
    #[command(
        about = decode_selected_entities(r#"Margin Manual Liquidation

Weight(UID): 3000

Security Type: TRADE

Notes:
- This endpoint supports Cross Margin Classic Mode and Pro Mode.
- Isolated Margin is only supported in restricted regions."#, false),
    )]
    MarginManualLiquidation(MarginManualLiquidationArgs),
    #[command(
        about = decode_selected_entities(r#"Displays the user's current margin order count usage for all intervals.

Weight(IP): 20

Security Type: TRADE"#, false),
    )]
    QueryCurrentMarginOrderCountUsage(QueryCurrentMarginOrderCountUsageArgs),
    #[command(
        about = decode_selected_entities(r#"Query the current user's cross-margin liquidation loan information, including the original loan amount, repaid amount, and remaining amount. When a cross-margin account is liquidated and the account equity turns negative (bankruptcy), the system generates a liquidation loan record representing the deficit. This represents the shortfall amount denominated in USDC.

Weight(UID): 100

Security Type: USER_DATA"#, false),
    )]
    QueryLiquidationLoan(QueryLiquidationLoanArgs),
    #[command(
        about = decode_selected_entities(r#"Query the repayment history of cross-margin liquidation loans (deficit caused by bankruptcy during liquidation). Supports time-range filtering and pagination.

Weight(UID): 100

Security Type: USER_DATA

Notes:
- The maximum query range is 90 days. If `startTime` is earlier than 90 days ago, it will be clamped to 90 days ago.
- Only records with status `SUCCESS` or `PENDING` are returned. Failed repayment records are excluded."#, false),
    )]
    QueryLiquidationLoanRepayHistory(QueryLiquidationLoanRepayHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Retrieves all OCO for a specific margin account based on provided optional parameters

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryMarginAccountsAllOco(QueryMarginAccountsAllOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Account's All Orders

Weight(IP): 200

Security Type: USER_DATA

Notes:
- If orderId is set, it will get orders >= that orderId. Otherwise the
orders within 24 hours are returned.

- For some historical orders cummulativeQuoteQty will be < 0, meaning
the data is not available at this time.

- Less than 24 hours between startTime and endTime."#, false),
    )]
    QueryMarginAccountsAllOrders(QueryMarginAccountsAllOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Retrieves a specific OCO based on provided optional parameters

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    QueryMarginAccountsOco(QueryMarginAccountsOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Account's Open OCO

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    QueryMarginAccountsOpenOco(QueryMarginAccountsOpenOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Account's Open Orders

Weight(IP): 10

Security Type: USER_DATA

Notes:
- If the symbol is not sent, orders for all symbols will be returned in
an array.

- When all symbols are returned, the number of requests counted against
the rate limiter is equal to the number of symbols currently trading on
the exchange.

- If isIsolated ="TRUE", symbol must be sent."#, false),
    )]
    QueryMarginAccountsOpenOrders(QueryMarginAccountsOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Account's Order

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Either orderId or origClientOrderId must be sent.

- For some historical orders cummulativeQuoteQty will be < 0, meaning
the data is not available at this time."#, false),
    )]
    QueryMarginAccountsOrder(QueryMarginAccountsOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Account's Trade List

Weight(IP): 10

Security Type: USER_DATA

Notes:
- If fromId is set, it will get trades >= that fromId. Otherwise the
trades within 24 hours are returned.

- Less than 24 hours between startTime and endTime."#, false),
    )]
    QueryMarginAccountsTradeList(QueryMarginAccountsTradeListArgs),
    #[command(
        about = decode_selected_entities(r#"Displays the list of orders that were expired due to STP. (Self-Trade Prevention).

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Supported parameter combinations:

- `symbol` + `preventedMatchId`

- `symbol` + `orderId`

- `symbol` + `orderId` + `fromPreventedMatchId`

- If `orderId` is provided, all prevented matches for that order will be
returned.

- If `preventedMatchId` is provided, the specific prevented match will
be returned.

- A single request returns a maximum of 500 records. If there are more
than 500 records, use `symbol` + `orderId` + `fromPreventedMatchId`
combination for pagination."#, false),
    )]
    QueryPreventedMatches(QueryPreventedMatchesArgs),
    #[command(
        about = decode_selected_entities(r#"Query Special Key Information.

This only applies to Special Key for Low Latency Trading.

Weight(UID): 1

Security Type: TRADE"#, false),
    )]
    QuerySpecialKey(QuerySpecialKeyArgs),
    #[command(
        about = decode_selected_entities(r#"This only applies to Special Key for Low Latency Trading.

Weight(UID): 1

Security Type: TRADE"#, false),
    )]
    QuerySpecialKeyList(QuerySpecialKeyListArgs),
    #[command(
        about = decode_selected_entities(r#"Small Liability Exchange

Weight(UID): 3000

Security Type: MARGIN

Notes:
- Only convert once within 6 hours
- Only liability valuation less than 10 USDT are supported
- The maximum number of coin is 10"#, false),
    )]
    SmallLiabilityExchange(SmallLiabilityExchangeArgs),
    #[command(
        about = decode_selected_entities(r#"Get Cross Margin Transfer History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Response in descending order
- The max interval between `startTime` and `endTime` is 30 days.
- Returns data for last 7 days by default"#, false),
    )]
    GetCrossMarginTransferHistory(GetCrossMarginTransferHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query Max Transfer-Out Amount

Weight(IP): 50

Security Type: USER_DATA

Notes:
- If isolatedSymbol is not sent, crossed margin data will be sent."#, false),
    )]
    QueryMaxTransferOutAmount(QueryMaxTransferOutAmountArgs),
    #[command(
        about = decode_selected_entities(r#"Close out a user data stream.

Weight(UID): 3000

Security Type: USER_STREAM"#, false),
    )]
    CloseUserDataStream(CloseUserDataStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Keepalive a user data stream to prevent a time out.

Weight(UID): 1

Security Type: USER_STREAM"#, false),
    )]
    KeepaliveUserDataStream(KeepaliveUserDataStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Start a new user data stream.

Weight(UID): 1

Security Type: USER_STREAM"#, false),
    )]
    StartUserDataStream(StartUserDataStreamArgs),
}

pub async fn handle_margin_trading_command(command: MarginTradingCommands) -> anyhow::Result<()> {
    match command {
        MarginTradingCommands::AdjustCrossMarginMaxLeverage(args) => {
            adjust_cross_margin_max_leverage(args).await
        }

        MarginTradingCommands::DisableIsolatedMarginAccount(args) => {
            disable_isolated_margin_account(args).await
        }

        MarginTradingCommands::EnableIsolatedMarginAccount(args) => {
            enable_isolated_margin_account(args).await
        }

        MarginTradingCommands::GetBnbBurnStatus(args) => get_bnb_burn_status(args).await,

        MarginTradingCommands::GetSummaryOfMarginAccount(args) => {
            get_summary_of_margin_account(args).await
        }

        MarginTradingCommands::QueryCrossIsolatedMarginCapitalFlow(args) => {
            query_cross_isolated_margin_capital_flow(args).await
        }

        MarginTradingCommands::QueryCrossMarginAccountDetails(args) => {
            query_cross_margin_account_details(args).await
        }

        MarginTradingCommands::QueryCrossMarginFeeData(args) => {
            query_cross_margin_fee_data(args).await
        }

        MarginTradingCommands::QueryEnabledIsolatedMarginAccountLimit(args) => {
            query_enabled_isolated_margin_account_limit(args).await
        }

        MarginTradingCommands::QueryIsolatedMarginAccountInfo(args) => {
            query_isolated_margin_account_info(args).await
        }

        MarginTradingCommands::QueryIsolatedMarginFeeData(args) => {
            query_isolated_margin_fee_data(args).await
        }

        MarginTradingCommands::GetFutureHourlyInterestRate(args) => {
            get_future_hourly_interest_rate(args).await
        }

        MarginTradingCommands::GetInterestHistory(args) => get_interest_history(args).await,

        MarginTradingCommands::MarginAccountBorrowRepay(args) => {
            margin_account_borrow_repay(args).await
        }

        MarginTradingCommands::QueryBorrowRepayRecordsInMarginAccount(args) => {
            query_borrow_repay_records_in_margin_account(args).await
        }

        MarginTradingCommands::QueryMarginInterestRateHistory(args) => {
            query_margin_interest_rate_history(args).await
        }

        MarginTradingCommands::QueryMaxBorrow(args) => query_max_borrow(args).await,

        MarginTradingCommands::CrossMarginCollateralRatio(args) => {
            cross_margin_collateral_ratio(args).await
        }

        MarginTradingCommands::GetAllCrossMarginPairs(args) => {
            get_all_cross_margin_pairs(args).await
        }

        MarginTradingCommands::GetAllIsolatedMarginSymbol(args) => {
            get_all_isolated_margin_symbol(args).await
        }

        MarginTradingCommands::GetAllMarginAssets(args) => get_all_margin_assets(args).await,

        MarginTradingCommands::GetDelistSchedule(args) => get_delist_schedule(args).await,

        MarginTradingCommands::GetLimitPricePairs(args) => get_limit_price_pairs(args).await,

        MarginTradingCommands::GetListSchedule(args) => get_list_schedule(args).await,

        MarginTradingCommands::GetMarginAssetRiskBasedLiquidationRatio(args) => {
            get_margin_asset_risk_based_liquidation_ratio(args).await
        }

        MarginTradingCommands::GetMarginRestrictedAssets(args) => {
            get_margin_restricted_assets(args).await
        }

        MarginTradingCommands::QueryIsolatedMarginTierData(args) => {
            query_isolated_margin_tier_data(args).await
        }

        MarginTradingCommands::QueryLiabilityCoinLeverageBracketInCrossMarginProMode(args) => {
            query_liability_coin_leverage_bracket_in_cross_margin_pro_mode(args).await
        }

        MarginTradingCommands::QueryMarginAvailableInventory(args) => {
            query_margin_available_inventory(args).await
        }

        MarginTradingCommands::QueryMarginPriceindex(args) => query_margin_priceindex(args).await,

        MarginTradingCommands::CreateSpecialKey(args) => create_special_key(args).await,

        MarginTradingCommands::DeleteSpecialKey(args) => delete_special_key(args).await,

        MarginTradingCommands::EditIpForSpecialKey(args) => edit_ip_for_special_key(args).await,

        MarginTradingCommands::ExitSpecialKeyMode(args) => exit_special_key_mode(args).await,

        MarginTradingCommands::GetForceLiquidationRecord(args) => {
            get_force_liquidation_record(args).await
        }

        MarginTradingCommands::GetSmallLiabilityExchangeCoinList(args) => {
            get_small_liability_exchange_coin_list(args).await
        }

        MarginTradingCommands::GetSmallLiabilityExchangeHistory(args) => {
            get_small_liability_exchange_history(args).await
        }

        MarginTradingCommands::LiquidationLoanRepay(args) => liquidation_loan_repay(args).await,

        MarginTradingCommands::MarginAccountCancelAllOpenOrdersOnASymbol(args) => {
            margin_account_cancel_all_open_orders_on_a_symbol(args).await
        }

        MarginTradingCommands::MarginAccountCancelOco(args) => {
            margin_account_cancel_oco(args).await
        }

        MarginTradingCommands::MarginAccountCancelOrder(args) => {
            margin_account_cancel_order(args).await
        }

        MarginTradingCommands::MarginAccountNewOco(args) => margin_account_new_oco(args).await,

        MarginTradingCommands::MarginAccountNewOrder(args) => margin_account_new_order(args).await,

        MarginTradingCommands::MarginAccountNewOto(args) => margin_account_new_oto(args).await,

        MarginTradingCommands::MarginAccountNewOtoco(args) => margin_account_new_otoco(args).await,

        MarginTradingCommands::MarginManualLiquidation(args) => {
            margin_manual_liquidation(args).await
        }

        MarginTradingCommands::QueryCurrentMarginOrderCountUsage(args) => {
            query_current_margin_order_count_usage(args).await
        }

        MarginTradingCommands::QueryLiquidationLoan(args) => query_liquidation_loan(args).await,

        MarginTradingCommands::QueryLiquidationLoanRepayHistory(args) => {
            query_liquidation_loan_repay_history(args).await
        }

        MarginTradingCommands::QueryMarginAccountsAllOco(args) => {
            query_margin_accounts_all_oco(args).await
        }

        MarginTradingCommands::QueryMarginAccountsAllOrders(args) => {
            query_margin_accounts_all_orders(args).await
        }

        MarginTradingCommands::QueryMarginAccountsOco(args) => {
            query_margin_accounts_oco(args).await
        }

        MarginTradingCommands::QueryMarginAccountsOpenOco(args) => {
            query_margin_accounts_open_oco(args).await
        }

        MarginTradingCommands::QueryMarginAccountsOpenOrders(args) => {
            query_margin_accounts_open_orders(args).await
        }

        MarginTradingCommands::QueryMarginAccountsOrder(args) => {
            query_margin_accounts_order(args).await
        }

        MarginTradingCommands::QueryMarginAccountsTradeList(args) => {
            query_margin_accounts_trade_list(args).await
        }

        MarginTradingCommands::QueryPreventedMatches(args) => query_prevented_matches(args).await,

        MarginTradingCommands::QuerySpecialKey(args) => query_special_key(args).await,

        MarginTradingCommands::QuerySpecialKeyList(args) => query_special_key_list(args).await,

        MarginTradingCommands::SmallLiabilityExchange(args) => small_liability_exchange(args).await,

        MarginTradingCommands::GetCrossMarginTransferHistory(args) => {
            get_cross_margin_transfer_history(args).await
        }

        MarginTradingCommands::QueryMaxTransferOutAmount(args) => {
            query_max_transfer_out_amount(args).await
        }

        MarginTradingCommands::CloseUserDataStream(args) => close_user_data_stream(args).await,

        MarginTradingCommands::KeepaliveUserDataStream(args) => {
            keepalive_user_data_stream(args).await
        }

        MarginTradingCommands::StartUserDataStream(args) => start_user_data_stream(args).await,
    }
}

async fn adjust_cross_margin_max_leverage(
    mut args: AdjustCrossMarginMaxLeverageArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AdjustCrossMarginMaxLeverageParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<AdjustCrossMarginMaxLeverageParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.max_leverage.is_none() {
                        let max_leverage: i64 = Input::new()
                            .with_prompt("Input max_leverage:")
                            .interact_text()?;

                        args.max_leverage = Some(max_leverage);
                    }
                }
                AdjustCrossMarginMaxLeverageParams::builder(
                    args.max_leverage
                        .ok_or_else(|| anyhow::anyhow!("max_leverage is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.adjust_cross_margin_max_leverage(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn disable_isolated_margin_account(
    mut args: DisableIsolatedMarginAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DisableIsolatedMarginAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<DisableIsolatedMarginAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                DisableIsolatedMarginAccountParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.disable_isolated_margin_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn enable_isolated_margin_account(
    mut args: EnableIsolatedMarginAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EnableIsolatedMarginAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<EnableIsolatedMarginAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                EnableIsolatedMarginAccountParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.enable_isolated_margin_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_bnb_burn_status(args: GetBnbBurnStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBnbBurnStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBnbBurnStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetBnbBurnStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_bnb_burn_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_summary_of_margin_account(args: GetSummaryOfMarginAccountArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSummaryOfMarginAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSummaryOfMarginAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetSummaryOfMarginAccountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_summary_of_margin_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_cross_isolated_margin_capital_flow(
    args: QueryCrossIsolatedMarginCapitalFlowArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCrossIsolatedMarginCapitalFlowParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCrossIsolatedMarginCapitalFlowParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryCrossIsolatedMarginCapitalFlowParams::builder()
                .asset(args.asset)
                .symbol(args.symbol)
                .r#type(args.r#type)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .from_id(args.from_id)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_cross_isolated_margin_capital_flow(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_cross_margin_account_details(
    args: QueryCrossMarginAccountDetailsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCrossMarginAccountDetailsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryCrossMarginAccountDetailsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryCrossMarginAccountDetailsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_cross_margin_account_details(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_cross_margin_fee_data(args: QueryCrossMarginFeeDataArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCrossMarginFeeDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCrossMarginFeeDataParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryCrossMarginFeeDataParams::builder()
                .vip_level(args.vip_level)
                .coin(args.coin)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_cross_margin_fee_data(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_enabled_isolated_margin_account_limit(
    args: QueryEnabledIsolatedMarginAccountLimitArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryEnabledIsolatedMarginAccountLimitParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryEnabledIsolatedMarginAccountLimitParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryEnabledIsolatedMarginAccountLimitParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_enabled_isolated_margin_account_limit(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_isolated_margin_account_info(
    args: QueryIsolatedMarginAccountInfoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryIsolatedMarginAccountInfoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryIsolatedMarginAccountInfoParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryIsolatedMarginAccountInfoParams::builder()
                .symbols(args.symbols)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_isolated_margin_account_info(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_isolated_margin_fee_data(
    args: QueryIsolatedMarginFeeDataArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryIsolatedMarginFeeDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryIsolatedMarginFeeDataParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryIsolatedMarginFeeDataParams::builder()
                .vip_level(args.vip_level)
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_isolated_margin_fee_data(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_future_hourly_interest_rate(
    mut args: GetFutureHourlyInterestRateArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFutureHourlyInterestRateParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFutureHourlyInterestRateParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.assets.is_none() {
                        let assets: String =
                            Input::new().with_prompt("Input assets:").interact_text()?;

                        args.assets = Some(assets);
                    }
                    if args.is_isolated.is_none() {
                        let options = vec![
                            ("TRUE", GetFutureHourlyInterestRateIsIsolatedEnum::True),
                            ("FALSE", GetFutureHourlyInterestRateIsIsolatedEnum::False),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the is_isolated")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.is_isolated = Some(selected);
                    }
                }
                GetFutureHourlyInterestRateParams::builder(
                    args.assets
                        .ok_or_else(|| anyhow::anyhow!("assets is required"))?,
                    args.is_isolated
                        .ok_or_else(|| anyhow::anyhow!("is_isolated is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_future_hourly_interest_rate(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_interest_history(args: GetInterestHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetInterestHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetInterestHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetInterestHistoryParams::builder()
                .asset(args.asset)
                .isolated_symbol(args.isolated_symbol)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_interest_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_borrow_repay(mut args: MarginAccountBorrowRepayArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountBorrowRepayParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<MarginAccountBorrowRepayParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String =
                            Input::new().with_prompt("Input asset:").interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.is_isolated.is_none() {
                        let options = vec![
                            ("TRUE", MarginAccountBorrowRepayIsIsolatedEnum::True),
                            ("FALSE", MarginAccountBorrowRepayIsIsolatedEnum::False),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the is_isolated")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.is_isolated = Some(selected);
                    }
                    if args.amount.is_none() {
                        let amount: String =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.r#type.is_none() {
                        let options = vec![
                            ("BORROW", MarginAccountBorrowRepayTypeEnum::Borrow),
                            ("REPAY", MarginAccountBorrowRepayTypeEnum::Repay),
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
                MarginAccountBorrowRepayParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                    args.is_isolated
                        .ok_or_else(|| anyhow::anyhow!("is_isolated is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_account_borrow_repay(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_borrow_repay_records_in_margin_account(
    mut args: QueryBorrowRepayRecordsInMarginAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryBorrowRepayRecordsInMarginAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryBorrowRepayRecordsInMarginAccountParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.r#type.is_none() {
                        let options = vec![
                            (
                                "BORROW",
                                QueryBorrowRepayRecordsInMarginAccountTypeEnum::Borrow,
                            ),
                            (
                                "REPAY",
                                QueryBorrowRepayRecordsInMarginAccountTypeEnum::Repay,
                            ),
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
                QueryBorrowRepayRecordsInMarginAccountParams::builder(
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .asset(args.asset)
                .isolated_symbol(args.isolated_symbol)
                .tx_id(args.tx_id)
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
    let response = rest_client
        .query_borrow_repay_records_in_margin_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_interest_rate_history(
    mut args: QueryMarginInterestRateHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginInterestRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMarginInterestRateHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String =
                            Input::new().with_prompt("Input asset:").interact_text()?;

                        args.asset = Some(asset);
                    }
                }
                QueryMarginInterestRateHistoryParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .vip_level(args.vip_level)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_margin_interest_rate_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_max_borrow(mut args: QueryMaxBorrowArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMaxBorrowParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryMaxBorrowParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String =
                            Input::new().with_prompt("Input asset:").interact_text()?;

                        args.asset = Some(asset);
                    }
                }
                QueryMaxBorrowParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .isolated_symbol(args.isolated_symbol)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_max_borrow(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cross_margin_collateral_ratio(args: CrossMarginCollateralRatioArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.cross_margin_collateral_ratio().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_all_cross_margin_pairs(args: GetAllCrossMarginPairsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetAllCrossMarginPairsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetAllCrossMarginPairsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetAllCrossMarginPairsParams::builder()
                .symbol(args.symbol)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_all_cross_margin_pairs(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_all_isolated_margin_symbol(
    args: GetAllIsolatedMarginSymbolArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetAllIsolatedMarginSymbolParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetAllIsolatedMarginSymbolParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetAllIsolatedMarginSymbolParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_all_isolated_margin_symbol(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_all_margin_assets(args: GetAllMarginAssetsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetAllMarginAssetsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetAllMarginAssetsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetAllMarginAssetsParams::builder()
                .asset(args.asset)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_all_margin_assets(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_delist_schedule(args: GetDelistScheduleArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetDelistScheduleParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDelistScheduleParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetDelistScheduleParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_delist_schedule(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_limit_price_pairs(args: GetLimitPricePairsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.get_limit_price_pairs().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_list_schedule(args: GetListScheduleArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetListScheduleParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetListScheduleParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetListScheduleParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_list_schedule(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_margin_asset_risk_based_liquidation_ratio(
    args: GetMarginAssetRiskBasedLiquidationRatioArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client
        .get_margin_asset_risk_based_liquidation_ratio()
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_margin_restricted_assets(args: GetMarginRestrictedAssetsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.get_margin_restricted_assets().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_isolated_margin_tier_data(
    mut args: QueryIsolatedMarginTierDataArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryIsolatedMarginTierDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryIsolatedMarginTierDataParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                QueryIsolatedMarginTierDataParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .tier(args.tier)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_isolated_margin_tier_data(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_liability_coin_leverage_bracket_in_cross_margin_pro_mode(
    args: QueryLiabilityCoinLeverageBracketInCrossMarginProModeArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client
        .query_liability_coin_leverage_bracket_in_cross_margin_pro_mode()
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_available_inventory(
    mut args: QueryMarginAvailableInventoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAvailableInventoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMarginAvailableInventoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.r#type.is_none() {
                        let options = vec![
                            ("MARGIN", QueryMarginAvailableInventoryTypeEnum::Margin),
                            ("ISOLATED", QueryMarginAvailableInventoryTypeEnum::Isolated),
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
                QueryMarginAvailableInventoryParams::builder(
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_margin_available_inventory(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_priceindex(mut args: QueryMarginPriceindexArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<QueryMarginPriceindexParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryMarginPriceindexParams>(json).ok_or_else(|| {
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
                QueryMarginPriceindexParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_margin_priceindex(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn create_special_key(mut args: CreateSpecialKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CreateSpecialKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CreateSpecialKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.api_name.is_none() {
                        let api_name: String = Input::new()
                            .with_prompt("Input api_name:")
                            .interact_text()?;

                        args.api_name = Some(api_name);
                    }
                }
                CreateSpecialKeyParams::builder(
                    args.api_name
                        .ok_or_else(|| anyhow::anyhow!("api_name is required"))?,
                )
                .symbol(args.symbol)
                .ip(args.ip)
                .public_key(args.public_key)
                .permission_mode(args.permission_mode)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.create_special_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn delete_special_key(args: DeleteSpecialKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DeleteSpecialKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DeleteSpecialKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => DeleteSpecialKeyParams::builder()
                .api_name(args.api_name)
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.delete_special_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn edit_ip_for_special_key(mut args: EditIpForSpecialKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EditIpForSpecialKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<EditIpForSpecialKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.ip.is_none() {
                        let ip: String = Input::new().with_prompt("Input ip:").interact_text()?;

                        args.ip = Some(ip);
                    }
                }
                EditIpForSpecialKeyParams::builder(
                    args.ip.ok_or_else(|| anyhow::anyhow!("ip is required"))?,
                )
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.edit_ip_for_special_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn exit_special_key_mode(args: ExitSpecialKeyModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ExitSpecialKeyModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ExitSpecialKeyModeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ExitSpecialKeyModeParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.exit_special_key_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_force_liquidation_record(args: GetForceLiquidationRecordArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetForceLiquidationRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetForceLiquidationRecordParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetForceLiquidationRecordParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .isolated_symbol(args.isolated_symbol)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_force_liquidation_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_small_liability_exchange_coin_list(
    args: GetSmallLiabilityExchangeCoinListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSmallLiabilityExchangeCoinListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetSmallLiabilityExchangeCoinListParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetSmallLiabilityExchangeCoinListParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_small_liability_exchange_coin_list(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_small_liability_exchange_history(
    mut args: GetSmallLiabilityExchangeHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetSmallLiabilityExchangeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSmallLiabilityExchangeHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.current.is_none() {
                        let current: i64 =
                            Input::new().with_prompt("Input current:").interact_text()?;

                        args.current = Some(current);
                    }
                    if args.size.is_none() {
                        let size: i64 = Input::new().with_prompt("Input size:").interact_text()?;

                        args.size = Some(size);
                    }
                }
                GetSmallLiabilityExchangeHistoryParams::builder(
                    args.current
                        .ok_or_else(|| anyhow::anyhow!("current is required"))?,
                    args.size
                        .ok_or_else(|| anyhow::anyhow!("size is required"))?,
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
        .get_small_liability_exchange_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn liquidation_loan_repay(mut args: LiquidationLoanRepayArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<LiquidationLoanRepayParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<LiquidationLoanRepayParams>(json).ok_or_else(|| {
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
                LiquidationLoanRepayParams::builder(
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
    let response = rest_client.liquidation_loan_repay(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_cancel_all_open_orders_on_a_symbol(
    mut args: MarginAccountCancelAllOpenOrdersOnASymbolArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountCancelAllOpenOrdersOnASymbolParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountCancelAllOpenOrdersOnASymbolParams>(json)
                .ok_or_else(|| {
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
                MarginAccountCancelAllOpenOrdersOnASymbolParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .is_isolated(args.is_isolated)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .margin_account_cancel_all_open_orders_on_a_symbol(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_cancel_oco(mut args: MarginAccountCancelOcoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountCancelOcoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountCancelOcoParams>(json).ok_or_else(|| {
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
                MarginAccountCancelOcoParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .is_isolated(args.is_isolated)
                .order_list_id(args.order_list_id)
                .list_client_order_id(args.list_client_order_id)
                .new_client_order_id(args.new_client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_account_cancel_oco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_cancel_order(mut args: MarginAccountCancelOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountCancelOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<MarginAccountCancelOrderParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                MarginAccountCancelOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .is_isolated(args.is_isolated)
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .new_client_order_id(args.new_client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_account_cancel_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_new_oco(mut args: MarginAccountNewOcoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountNewOcoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountNewOcoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", MarginAccountNewOcoSideEnum::Buy),
                            ("SELL", MarginAccountNewOcoSideEnum::Sell),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the side")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.side = Some(selected);
                    }
                    if args.quantity.is_none() {
                        let quantity: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input quantity:")
                            .interact_text()?;

                        args.quantity = Some(quantity);
                    }
                    if args.price.is_none() {
                        let price: rust_decimal::Decimal =
                            Input::new().with_prompt("Input price:").interact_text()?;

                        args.price = Some(price);
                    }
                    if args.stop_price.is_none() {
                        let stop_price: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input stop_price:")
                            .interact_text()?;

                        args.stop_price = Some(stop_price);
                    }
                }
                MarginAccountNewOcoParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                    args.price
                        .ok_or_else(|| anyhow::anyhow!("price is required"))?,
                    args.stop_price
                        .ok_or_else(|| anyhow::anyhow!("stop_price is required"))?,
                )
                .is_isolated(args.is_isolated)
                .list_client_order_id(args.list_client_order_id)
                .limit_client_order_id(args.limit_client_order_id)
                .limit_iceberg_qty(args.limit_iceberg_qty)
                .stop_client_order_id(args.stop_client_order_id)
                .stop_limit_price(args.stop_limit_price)
                .stop_iceberg_qty(args.stop_iceberg_qty)
                .stop_limit_time_in_force(args.stop_limit_time_in_force)
                .new_order_resp_type(args.new_order_resp_type)
                .side_effect_type(args.side_effect_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .auto_repay_at_cancel(args.auto_repay_at_cancel)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_account_new_oco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_new_order(mut args: MarginAccountNewOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountNewOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountNewOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", MarginAccountNewOrderSideEnum::Buy),
                            ("SELL", MarginAccountNewOrderSideEnum::Sell),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the side")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.side = Some(selected);
                    }
                    if args.r#type.is_none() {
                        let options = vec![
                            ("LIMIT", MarginAccountNewOrderTypeEnum::Limit),
                            ("MARKET", MarginAccountNewOrderTypeEnum::Market),
                            ("STOP_LOSS", MarginAccountNewOrderTypeEnum::StopLoss),
                            (
                                "STOP_LOSS_LIMIT",
                                MarginAccountNewOrderTypeEnum::StopLossLimit,
                            ),
                            ("TAKE_PROFIT", MarginAccountNewOrderTypeEnum::TakeProfit),
                            (
                                "TAKE_PROFIT_LIMIT",
                                MarginAccountNewOrderTypeEnum::TakeProfitLimit,
                            ),
                            ("LIMIT_MAKER", MarginAccountNewOrderTypeEnum::LimitMaker),
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
                MarginAccountNewOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .is_isolated(args.is_isolated)
                .quantity(args.quantity)
                .quote_order_qty(args.quote_order_qty)
                .price(args.price)
                .stop_price(args.stop_price)
                .new_client_order_id(args.new_client_order_id)
                .iceberg_qty(args.iceberg_qty)
                .new_order_resp_type(args.new_order_resp_type)
                .side_effect_type(args.side_effect_type)
                .time_in_force(args.time_in_force)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .trailing_delta(args.trailing_delta)
                .auto_repay_at_cancel(args.auto_repay_at_cancel)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_account_new_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_new_oto(mut args: MarginAccountNewOtoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountNewOtoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountNewOtoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.working_type.is_none() {
                        let options = vec![
                            ("LIMIT", MarginAccountNewOtoWorkingTypeEnum::Limit),
                            (
                                "LIMIT_MAKER",
                                MarginAccountNewOtoWorkingTypeEnum::LimitMaker,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the working_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.working_type = Some(selected);
                    }
                    if args.working_side.is_none() {
                        let options = vec![
                            ("BUY", MarginAccountNewOtoWorkingSideEnum::Buy),
                            ("SELL", MarginAccountNewOtoWorkingSideEnum::Sell),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the working_side")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.working_side = Some(selected);
                    }
                    if args.working_price.is_none() {
                        let working_price: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input working_price:")
                            .interact_text()?;

                        args.working_price = Some(working_price);
                    }
                    if args.working_quantity.is_none() {
                        let working_quantity: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input working_quantity:")
                            .interact_text()?;

                        args.working_quantity = Some(working_quantity);
                    }
                    if args.working_iceberg_qty.is_none() {
                        let working_iceberg_qty: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input working_iceberg_qty:")
                            .interact_text()?;

                        args.working_iceberg_qty = Some(working_iceberg_qty);
                    }
                    if args.pending_type.is_none() {
                        let options = vec![
                            ("LIMIT", MarginAccountNewOtoPendingTypeEnum::Limit),
                            ("MARKET", MarginAccountNewOtoPendingTypeEnum::Market),
                            ("STOP_LOSS", MarginAccountNewOtoPendingTypeEnum::StopLoss),
                            (
                                "STOP_LOSS_LIMIT",
                                MarginAccountNewOtoPendingTypeEnum::StopLossLimit,
                            ),
                            (
                                "TAKE_PROFIT",
                                MarginAccountNewOtoPendingTypeEnum::TakeProfit,
                            ),
                            (
                                "TAKE_PROFIT_LIMIT",
                                MarginAccountNewOtoPendingTypeEnum::TakeProfitLimit,
                            ),
                            (
                                "LIMIT_MAKER",
                                MarginAccountNewOtoPendingTypeEnum::LimitMaker,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the pending_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.pending_type = Some(selected);
                    }
                    if args.pending_side.is_none() {
                        let options = vec![
                            ("BUY", MarginAccountNewOtoPendingSideEnum::Buy),
                            ("SELL", MarginAccountNewOtoPendingSideEnum::Sell),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the pending_side")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.pending_side = Some(selected);
                    }
                    if args.pending_quantity.is_none() {
                        let pending_quantity: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input pending_quantity:")
                            .interact_text()?;

                        args.pending_quantity = Some(pending_quantity);
                    }
                }
                MarginAccountNewOtoParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.working_type
                        .ok_or_else(|| anyhow::anyhow!("working_type is required"))?,
                    args.working_side
                        .ok_or_else(|| anyhow::anyhow!("working_side is required"))?,
                    args.working_price
                        .ok_or_else(|| anyhow::anyhow!("working_price is required"))?,
                    args.working_quantity
                        .ok_or_else(|| anyhow::anyhow!("working_quantity is required"))?,
                    args.working_iceberg_qty
                        .ok_or_else(|| anyhow::anyhow!("working_iceberg_qty is required"))?,
                    args.pending_type
                        .ok_or_else(|| anyhow::anyhow!("pending_type is required"))?,
                    args.pending_side
                        .ok_or_else(|| anyhow::anyhow!("pending_side is required"))?,
                    args.pending_quantity
                        .ok_or_else(|| anyhow::anyhow!("pending_quantity is required"))?,
                )
                .is_isolated(args.is_isolated)
                .list_client_order_id(args.list_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .side_effect_type(args.side_effect_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .auto_repay_at_cancel(args.auto_repay_at_cancel)
                .working_client_order_id(args.working_client_order_id)
                .working_time_in_force(args.working_time_in_force)
                .pending_client_order_id(args.pending_client_order_id)
                .pending_price(args.pending_price)
                .pending_stop_price(args.pending_stop_price)
                .pending_trailing_delta(args.pending_trailing_delta)
                .pending_iceberg_qty(args.pending_iceberg_qty)
                .pending_time_in_force(args.pending_time_in_force)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_account_new_oto(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_new_otoco(mut args: MarginAccountNewOtocoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountNewOtocoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountNewOtocoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.working_type.is_none() {
                        let options = vec![
                            ("LIMIT", MarginAccountNewOtocoWorkingTypeEnum::Limit),
                            (
                                "LIMIT_MAKER",
                                MarginAccountNewOtocoWorkingTypeEnum::LimitMaker,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the working_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.working_type = Some(selected);
                    }
                    if args.working_side.is_none() {
                        let options = vec![
                            ("BUY", MarginAccountNewOtocoWorkingSideEnum::Buy),
                            ("SELL", MarginAccountNewOtocoWorkingSideEnum::Sell),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the working_side")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.working_side = Some(selected);
                    }
                    if args.working_price.is_none() {
                        let working_price: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input working_price:")
                            .interact_text()?;

                        args.working_price = Some(working_price);
                    }
                    if args.working_quantity.is_none() {
                        let working_quantity: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input working_quantity:")
                            .interact_text()?;

                        args.working_quantity = Some(working_quantity);
                    }
                    if args.pending_side.is_none() {
                        let options = vec![
                            ("BUY", MarginAccountNewOtocoPendingSideEnum::Buy),
                            ("SELL", MarginAccountNewOtocoPendingSideEnum::Sell),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the pending_side")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.pending_side = Some(selected);
                    }
                    if args.pending_quantity.is_none() {
                        let pending_quantity: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input pending_quantity:")
                            .interact_text()?;

                        args.pending_quantity = Some(pending_quantity);
                    }
                    if args.pending_above_type.is_none() {
                        let options = vec![
                            (
                                "LIMIT_MAKER",
                                MarginAccountNewOtocoPendingAboveTypeEnum::LimitMaker,
                            ),
                            (
                                "STOP_LOSS",
                                MarginAccountNewOtocoPendingAboveTypeEnum::StopLoss,
                            ),
                            (
                                "STOP_LOSS_LIMIT",
                                MarginAccountNewOtocoPendingAboveTypeEnum::StopLossLimit,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the pending_above_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.pending_above_type = Some(selected);
                    }
                }
                MarginAccountNewOtocoParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.working_type
                        .ok_or_else(|| anyhow::anyhow!("working_type is required"))?,
                    args.working_side
                        .ok_or_else(|| anyhow::anyhow!("working_side is required"))?,
                    args.working_price
                        .ok_or_else(|| anyhow::anyhow!("working_price is required"))?,
                    args.working_quantity
                        .ok_or_else(|| anyhow::anyhow!("working_quantity is required"))?,
                    args.pending_side
                        .ok_or_else(|| anyhow::anyhow!("pending_side is required"))?,
                    args.pending_quantity
                        .ok_or_else(|| anyhow::anyhow!("pending_quantity is required"))?,
                    args.pending_above_type
                        .ok_or_else(|| anyhow::anyhow!("pending_above_type is required"))?,
                )
                .is_isolated(args.is_isolated)
                .side_effect_type(args.side_effect_type)
                .auto_repay_at_cancel(args.auto_repay_at_cancel)
                .list_client_order_id(args.list_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .working_client_order_id(args.working_client_order_id)
                .working_iceberg_qty(args.working_iceberg_qty)
                .working_time_in_force(args.working_time_in_force)
                .pending_above_client_order_id(args.pending_above_client_order_id)
                .pending_above_price(args.pending_above_price)
                .pending_above_stop_price(args.pending_above_stop_price)
                .pending_above_trailing_delta(args.pending_above_trailing_delta)
                .pending_above_iceberg_qty(args.pending_above_iceberg_qty)
                .pending_above_time_in_force(args.pending_above_time_in_force)
                .pending_below_type(args.pending_below_type)
                .pending_below_client_order_id(args.pending_below_client_order_id)
                .pending_below_price(args.pending_below_price)
                .pending_below_stop_price(args.pending_below_stop_price)
                .pending_below_trailing_delta(args.pending_below_trailing_delta)
                .pending_below_iceberg_qty(args.pending_below_iceberg_qty)
                .pending_below_time_in_force(args.pending_below_time_in_force)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_account_new_otoco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_manual_liquidation(mut args: MarginManualLiquidationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginManualLiquidationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginManualLiquidationParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.r#type.is_none() {
                        let options = vec![
                            ("MARGIN", MarginManualLiquidationTypeEnum::Margin),
                            ("ISOLATED", MarginManualLiquidationTypeEnum::Isolated),
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
                MarginManualLiquidationParams::builder(
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_manual_liquidation(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_margin_order_count_usage(
    args: QueryCurrentMarginOrderCountUsageArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentMarginOrderCountUsageParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCurrentMarginOrderCountUsageParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryCurrentMarginOrderCountUsageParams::builder()
                .is_isolated(args.is_isolated)
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_current_margin_order_count_usage(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_liquidation_loan(args: QueryLiquidationLoanArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryLiquidationLoanParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryLiquidationLoanParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryLiquidationLoanParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_liquidation_loan(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_liquidation_loan_repay_history(
    args: QueryLiquidationLoanRepayHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryLiquidationLoanRepayHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryLiquidationLoanRepayHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryLiquidationLoanRepayHistoryParams::builder()
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
        .query_liquidation_loan_repay_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_accounts_all_oco(args: QueryMarginAccountsAllOcoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountsAllOcoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMarginAccountsAllOcoParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryMarginAccountsAllOcoParams::builder()
                .is_isolated(args.is_isolated)
                .symbol(args.symbol)
                .from_id(args.from_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_margin_accounts_all_oco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_accounts_all_orders(
    mut args: QueryMarginAccountsAllOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountsAllOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMarginAccountsAllOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                QueryMarginAccountsAllOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .is_isolated(args.is_isolated)
                .order_id(args.order_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_margin_accounts_all_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_accounts_oco(args: QueryMarginAccountsOcoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountsOcoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryMarginAccountsOcoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryMarginAccountsOcoParams::builder()
                .is_isolated(args.is_isolated)
                .symbol(args.symbol)
                .order_list_id(args.order_list_id)
                .orig_client_order_id(args.orig_client_order_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_margin_accounts_oco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_accounts_open_oco(
    args: QueryMarginAccountsOpenOcoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountsOpenOcoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMarginAccountsOpenOcoParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryMarginAccountsOpenOcoParams::builder()
                .is_isolated(args.is_isolated)
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_margin_accounts_open_oco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_accounts_open_orders(
    args: QueryMarginAccountsOpenOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountsOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMarginAccountsOpenOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryMarginAccountsOpenOrdersParams::builder()
                .symbol(args.symbol)
                .is_isolated(args.is_isolated)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_margin_accounts_open_orders(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_accounts_order(mut args: QueryMarginAccountsOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountsOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMarginAccountsOrderParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                QueryMarginAccountsOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .is_isolated(args.is_isolated)
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_margin_accounts_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_accounts_trade_list(
    mut args: QueryMarginAccountsTradeListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountsTradeListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMarginAccountsTradeListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                QueryMarginAccountsTradeListParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .is_isolated(args.is_isolated)
                .order_id(args.order_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .from_id(args.from_id)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_margin_accounts_trade_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_prevented_matches(mut args: QueryPreventedMatchesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryPreventedMatchesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryPreventedMatchesParams>(json).ok_or_else(|| {
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
                QueryPreventedMatchesParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .prevented_match_id(args.prevented_match_id)
                .order_id(args.order_id)
                .from_prevented_match_id(args.from_prevented_match_id)
                .is_isolated(args.is_isolated)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_prevented_matches(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_special_key(args: QuerySpecialKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySpecialKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySpecialKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QuerySpecialKeyParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_special_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_special_key_list(args: QuerySpecialKeyListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySpecialKeyListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySpecialKeyListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QuerySpecialKeyListParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_special_key_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn small_liability_exchange(mut args: SmallLiabilityExchangeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SmallLiabilityExchangeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SmallLiabilityExchangeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.asset_names.is_none() {
                        let asset_names: String = Input::new()
                            .with_prompt("Input asset_names:")
                            .interact_text()?;

                        args.asset_names = Some(asset_names);
                    }
                }
                SmallLiabilityExchangeParams::builder(
                    args.asset_names
                        .ok_or_else(|| anyhow::anyhow!("asset_names is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.small_liability_exchange(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_cross_margin_transfer_history(
    args: GetCrossMarginTransferHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCrossMarginTransferHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetCrossMarginTransferHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetCrossMarginTransferHistoryParams::builder()
                .asset(args.asset)
                .r#type(args.r#type)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .isolated_symbol(args.isolated_symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_cross_margin_transfer_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_max_transfer_out_amount(
    mut args: QueryMaxTransferOutAmountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMaxTransferOutAmountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryMaxTransferOutAmountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String =
                            Input::new().with_prompt("Input asset:").interact_text()?;

                        args.asset = Some(asset);
                    }
                }
                QueryMaxTransferOutAmountParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .isolated_symbol(args.isolated_symbol)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_max_transfer_out_amount(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn close_user_data_stream(args: CloseUserDataStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.close_user_data_stream().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn keepalive_user_data_stream(mut args: KeepaliveUserDataStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<KeepaliveUserDataStreamParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<KeepaliveUserDataStreamParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.listen_key.is_none() {
                        let listen_key: String = Input::new()
                            .with_prompt("Input listen_key:")
                            .interact_text()?;

                        args.listen_key = Some(listen_key);
                    }
                }
                KeepaliveUserDataStreamParams::builder(
                    args.listen_key
                        .ok_or_else(|| anyhow::anyhow!("listen_key is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.keepalive_user_data_stream(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn start_user_data_stream(args: StartUserDataStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.start_user_data_stream().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
