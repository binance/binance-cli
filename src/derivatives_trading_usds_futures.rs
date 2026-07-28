use crate::utils::{
    build_user_agent, decode_selected_entities, get_configuration_rest_api, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::{
    DERIVATIVES_TRADING_USDS_FUTURES_REST_API_DEMO_URL,
    DERIVATIVES_TRADING_USDS_FUTURES_REST_API_PROD_URL,
    DERIVATIVES_TRADING_USDS_FUTURES_REST_API_TESTNET_URL,
};
use binance_sdk::derivatives_trading_usds_futures::DerivativesTradingUsdsFuturesRestApi;
use binance_sdk::derivatives_trading_usds_futures::rest_api::{self as models, *};
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    unsafe {
        env::set_var(
            "BINANCE_CONNECTOR_RUST_USER_AGENT",
            build_user_agent("derivatives-trading-usds-futures"),
        );
    }

    let config_rest_api =
        get_configuration_rest_api(profile, "derivatives-trading-usds-futures").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(config_rest_api.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = match api_env.as_str() {
        "testnet" => DERIVATIVES_TRADING_USDS_FUTURES_REST_API_TESTNET_URL,
        "demo" => DERIVATIVES_TRADING_USDS_FUTURES_REST_API_DEMO_URL,
        "prod" => DERIVATIVES_TRADING_USDS_FUTURES_REST_API_PROD_URL,
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

    Ok(DerivativesTradingUsdsFuturesRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct AccountInformationV2Args {
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
struct AccountInformationV3Args {
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
struct FuturesAccountBalanceV2Args {
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
struct FuturesAccountBalanceV3Args {
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
struct FuturesAccountConfigurationArgs {
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
struct FuturesTradingQuantitativeRulesIndicatorsArgs {
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
struct GetCurrentMultiAssetsModeArgs {
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
struct GetCurrentPositionModeArgs {
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
struct GetDownloadIdForFuturesOrderHistoryArgs {
    #[arg(help = r#"Timestamp in ms"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms"#, long)]
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
struct GetDownloadIdForFuturesTradeHistoryArgs {
    #[arg(help = r#"Timestamp in ms"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms"#, long)]
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
struct GetDownloadIdForFuturesTransactionHistoryArgs {
    #[arg(help = r#"Timestamp in ms"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms"#, long)]
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
struct GetFuturesOrderHistoryDownloadLinkByIdArgs {
    #[arg(help = r#"get by download id api"#, long)]
    download_id: Option<String>,
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
struct GetFuturesTradeDownloadLinkByIdArgs {
    #[arg(help = r#"get by download id api"#, long)]
    download_id: Option<String>,
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
struct GetFuturesTransactionHistoryDownloadLinkByIdArgs {
    #[arg(help = r#"get by download id api"#, long)]
    download_id: Option<String>,
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
struct GetIncomeHistoryArgs {
    #[arg(help = r#"Trading symbol."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Income type."#, long)]
    income_type: Option<GetIncomeHistoryIncomeTypeEnum>,
    #[arg(help = r#"Timestamp in milliseconds (inclusive start)."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in milliseconds (inclusive end)."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Pagination page number."#, long)]
    page: Option<i64>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
struct NotionalAndLeverageBracketsArgs {
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
struct QueryUserRateLimitArgs {
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
struct SymbolConfigurationArgs {
    #[arg(help = r#"Symbol"#, long)]
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
struct ToggleBnbBurnOnFuturesTradeArgs {
    #[arg(help = r#""true": Fee Discount On; "false": Fee Discount Off"#, long)]
    fee_burn: Option<String>,
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
struct UserCommissionRateArgs {
    #[arg(help = r#"Symbol"#, long)]
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
struct AcceptTheOfferedQuoteArgs {
    #[arg(help = r#""#, long)]
    quote_id: Option<String>,
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
struct ListAllConvertPairsArgs {
    #[arg(help = r#"User spends coin"#, long)]
    from_asset: Option<String>,
    #[arg(help = r#"User receives coin"#, long)]
    to_asset: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderStatusArgs {
    #[arg(help = r#"Either orderId or quoteId is required"#, long)]
    order_id: Option<String>,
    #[arg(help = r#"Either orderId or quoteId is required"#, long)]
    quote_id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SendQuoteRequestArgs {
    #[arg(help = r#""#, long)]
    from_asset: Option<String>,
    #[arg(help = r#""#, long)]
    to_asset: Option<String>,
    #[arg(
        help = r#"When specified, it is the amount you will be debited after the conversion"#,
        long
    )]
    from_amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"When specified, it is the amount you will be credited after the conversion"#,
        long
    )]
    to_amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"10s, default 10s"#, long)]
    valid_time: Option<String>,
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
struct AdlRiskArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct AssetIndexArgs {
    #[arg(help = r#"Asset pair"#, long)]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct BasisArgs {
    #[arg(help = r#""#, long)]
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    contract_type: Option<BasisContractTypeEnum>,
    #[arg(help = r#""#, long)]
    period: Option<BasisPeriodEnum>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CheckServerTimeArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CompositeIndexSymbolInformationArgs {
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
struct CompressedAggregateTradesListArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"ID to get aggregate trades from INCLUSIVE."#, long)]
    from_id: Option<i64>,
    #[arg(
        help = r#"Timestamp in ms to get aggregate trades from INCLUSIVE."#,
        long
    )]
    start_time: Option<i64>,
    #[arg(
        help = r#"Timestamp in ms to get aggregate trades until INCLUSIVE."#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ContinuousContractKlineCandlestickDataArgs {
    #[arg(
        help = r#"After CM migration, accepts both UM and CM pair values."#,
        long
    )]
    pair: Option<String>,
    #[arg(help = r#"Futurestype"#, long)]
    contract_type: Option<ContinuousContractKlineCandlestickDataContractTypeEnum>,
    #[arg(help = r#""#, long)]
    interval: Option<ContinuousContractKlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ExchangeInformationArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFundingRateHistoryArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Timestamp in ms to get funding rate from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding rate until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFundingRateInfoArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct IndexPriceKlineCandlestickDataArgs {
    #[arg(
        help = r#"After CM migration, accepts both UM and CM pair values."#,
        long
    )]
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    interval: Option<IndexPriceKlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct KlineCandlestickDataArgs {
    #[arg(help = r#"After CM migration, accepts both UM and CM symbols."#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    interval: Option<KlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct LongShortRatioArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    period: Option<LongShortRatioPeriodEnum>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MarkPriceArgs {
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
struct MarkPriceKlineCandlestickDataArgs {
    #[arg(help = r#"After CM migration, accepts both UM and CM symbols."#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    interval: Option<MarkPriceKlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OldTradesLookupArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(
        help = r#"TradeId to fetch from. Default gets most recent trades."#,
        long
    )]
    from_id: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OpenInterestArgs {
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
struct OpenInterestStatisticsArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    period: Option<OpenInterestStatisticsPeriodEnum>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderBookArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Valid limits:[5, 10, 20, 50, 100, 500, 1000]"#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct PremiumIndexKlineDataArgs {
    #[arg(help = r#"After CM migration, accepts both UM and CM symbols."#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    interval: Option<PremiumIndexKlineDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QuarterlyContractSettlementPriceArgs {
    #[arg(help = r#""#, long)]
    pair: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryIndexPriceConstituentsArgs {
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
struct QueryInsuranceFundBalanceSnapshotArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RecentTradesListArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RpiOrderBookArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Valid limits:[1000]"#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SymbolOrderBookTickerArgs {
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
struct SymbolPriceTickerArgs {
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
struct SymbolPriceTickerV2Args {
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
struct TakerBuySellVolumeArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    period: Option<TakerBuySellVolumePeriodEnum>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TestConnectivityArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct Ticker24hrPriceChangeStatisticsArgs {
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
struct TopTraderLongShortRatioAccountsArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    period: Option<TopTraderLongShortRatioAccountsPeriodEnum>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TopTraderLongShortRatioPositionsArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    period: Option<TopTraderLongShortRatioPositionsPeriodEnum>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TradingScheduleArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ClassicPortfolioMarginAccountInformationArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
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
struct AccountTradeListArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Must be used together with parameter `symbol`."#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(
        help = r#"Trade id to fetch from. Default gets most recent trades."#,
        long
    )]
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
struct AllOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
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
struct AutoCancelAllOpenOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Countdown in milliseconds. `1000` means 1 second; `0` disables countdown cancel-all."#,
        long
    )]
    countdown_time: Option<i64>,
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
struct CancelAlgoOrderArgs {
    #[arg(help = r#""#, long)]
    algo_id: Option<i64>,
    #[arg(help = r#""#, long)]
    client_algo_id: Option<String>,
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
struct CancelAllAlgoOpenOrdersArgs {
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
struct CancelAllOpenOrdersArgs {
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
struct CancelMultipleOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id_list: Option<Vec<i64>>,
    #[arg(help = r#""#, long)]
    orig_client_order_id_list: Option<Vec<String>>,
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
struct CancelOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
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
struct ChangeInitialLeverageArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"target initial leverage"#, long)]
    leverage: Option<i64>,
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
struct ChangeMarginTypeArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    margin_type: Option<ChangeMarginTypeMarginTypeEnum>,
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
struct ChangeMultiAssetsModeArgs {
    #[arg(
        help = r#""true": Multi-Assets Mode; "false": Single-Asset Mode"#,
        long
    )]
    multi_assets_margin: Option<String>,
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
struct ChangePositionModeArgs {
    #[arg(help = r#""true": Hedge Mode; "false": One-way Mode"#, long)]
    dual_side_position: Option<String>,
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
struct CurrentAllAlgoOpenOrdersArgs {
    #[arg(help = r#""#, long)]
    algo_type: Option<String>,
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    algo_id: Option<i64>,
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
struct CurrentAllOpenOrdersArgs {
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
struct FuturesTradfiPerpsContractArgs {
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
struct GetOrderModifyHistoryArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"Timestamp in ms to get modification history from INCLUSIVE"#,
        long
    )]
    start_time: Option<i64>,
    #[arg(
        help = r#"Timestamp in ms to get modification history until INCLUSIVE"#,
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
struct GetPositionMarginChangeHistoryArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"1: Add position margin，2: Reduce position margin"#, long)]
    r#type: Option<String>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"time if not pass"#, long)]
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
struct ModifyIsolatedPositionMarginArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Margin asset"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"1: Add position margin，2: Reduce position margin"#, long)]
    r#type: Option<i32>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent with Hedge Mode."#,
        long
    )]
    position_side: Option<String>,
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
struct ModifyMultipleOrdersArgs {
    #[arg(help = r#"order list. Max 5 orders"#, long)]
    batch_orders: Option<String>,
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
struct ModifyOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<ModifyOrderSideEnum>,
    #[arg(
        help = r#"Order quantity, cannot be sent with `closePosition=true`"#,
        long
    )]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<ModifyOrderPriceMatchEnum>,
    #[arg(
        help = r#"User-defined modification identifier, returned as-is in the response. Optional; not validated for uniqueness."#,
        long
    )]
    modify_id: Option<i64>,
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
struct NewAlgoOrderArgs {
    #[arg(help = r#""#, long)]
    algo_type: Option<NewAlgoOrderAlgoTypeEnum>,
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<NewAlgoOrderSideEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<NewAlgoOrderTypeEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<String>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewAlgoOrderTimeInForceEnum>,
    #[arg(
        help = r#"Cannot be sent with `closePosition`=`true`(Close-All)"#,
        long
    )]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Trigger price"#, long)]
    trigger_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    working_type: Option<NewAlgoOrderWorkingTypeEnum>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<NewAlgoOrderPriceMatchEnum>,
    #[arg(
        help = r#"Close-All，used with `STOP_MARKET` or `TAKE_PROFIT_MARKET`."#,
        long
    )]
    close_position: Option<NewAlgoOrderClosePositionEnum>,
    #[arg(
        help = r#"Used with `STOP_MARKET` or `TAKE_PROFIT_MARKET` order. when price reaches the triggerPrice ，the difference rate between "MARK_PRICE" and "CONTRACT_PRICE" cannot be larger than the Price Protection Threshold of the symbol.'"#,
        long
    )]
    price_protect: Option<NewAlgoOrderPriceProtectEnum>,
    #[arg(
        help = r#"Cannot be sent in Hedge Mode; cannot be sent with `closePosition`=`true`'"#,
        long
    )]
    reduce_only: Option<NewAlgoOrderReduceOnlyEnum>,
    #[arg(
        help = r#"Used with `TRAILING_STOP_MARKET` orders, default as the latest price(supporting different `workingType`)"#,
        long
    )]
    activate_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Used with `TRAILING_STOP_MARKET` orders"#, long)]
    callback_rate: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,36}$`"#,
        long
    )]
    client_algo_id: Option<String>,
    #[arg(help = r#""#, long)]
    new_order_resp_type: Option<NewAlgoOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"`EXPIRE_TAKER`:expire taker order when STP triggers / `EXPIRE_MAKER`:expire taker order when STP triggers/ `EXPIRE_BOTH`:expire both orders when STP triggers; default `NONE`"#,
        long
    )]
    self_trade_prevention_mode: Option<NewAlgoOrderSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"order cancel time for timeInForce `GTD`, mandatory when `timeInforce` set to `GTD`; order the timestamp only retains second-level precision, ms part will be ignored; The goodTillDate timestamp must be greater than the current time plus 600 seconds and smaller than 253402300799000"#,
        long
    )]
    good_till_date: Option<i64>,
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
struct NewOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<NewOrderSideEnum>,
    #[arg(help = r#"Order type"#, long)]
    r#type: Option<NewOrderTypeEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<String>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewOrderTimeInForceEnum>,
    #[arg(help = r#"Cannot be sent in Hedge Mode"#, long)]
    reduce_only: Option<NewOrderReduceOnlyEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,36}$`"#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    new_order_resp_type: Option<NewOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<NewOrderPriceMatchEnum>,
    #[arg(
        help = r#"`EXPIRE_TAKER`:expire taker order when STP triggers/ `EXPIRE_MAKER`:expire taker order when STP triggers/ `EXPIRE_BOTH`:expire both orders when STP triggers; default `EXPIRE_MAKER`"#,
        long
    )]
    self_trade_prevention_mode: Option<NewOrderSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"order cancel time for timeInForce `GTD`, mandatory when `timeInforce` set to `GTD`; order the timestamp only retains second-level precision, ms part will be ignored; The goodTillDate timestamp must be greater than the current time plus 600 seconds and smaller than 253402300799000"#,
        long
    )]
    good_till_date: Option<i64>,
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
struct PlaceMultipleOrdersArgs {
    #[arg(help = r#"order list. Max 5 orders"#, long)]
    batch_orders: Option<String>,
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
struct PositionAdlQuantileEstimationArgs {
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
struct PositionInformationV2Args {
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
struct PositionInformationV3Args {
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
struct QueryAlgoOrderArgs {
    #[arg(help = r#"Order ID"#, long)]
    algo_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
    client_algo_id: Option<String>,
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
struct QueryAllAlgoOrdersArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    algo_id: Option<i64>,
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
struct QueryCurrentOpenOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
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
struct QueryOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
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
struct TestOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<TestOrderSideEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<TestOrderTypeEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<TestOrderPositionSideEnum>,
    #[arg(
        help = r#"Cannot be sent in Hedge Mode; cannot be sent with `closePosition`=`true`"#,
        long
    )]
    reduce_only: Option<TestOrderReduceOnlyEnum>,
    #[arg(
        help = r#"Cannot be sent with `closePosition`=`true`(Close-All)"#,
        long
    )]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,36}$`"#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(
        help = r#"Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders."#,
        long
    )]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Close-All，used with `STOP_MARKET` or `TAKE_PROFIT_MARKET`.""#,
        long
    )]
    close_position: Option<TestOrderClosePositionEnum>,
    #[arg(
        help = r#"Used with `TRAILING_STOP_MARKET` orders, default as the latest price(supporting different `workingType`)"#,
        long
    )]
    activation_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Used with `TRAILING_STOP_MARKET` orders"#, long)]
    callback_rate: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<TestOrderTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    working_type: Option<TestOrderWorkingTypeEnum>,
    #[arg(help = r#""#, long)]
    price_protect: Option<TestOrderPriceProtectEnum>,
    #[arg(help = r#""#, long)]
    new_order_resp_type: Option<TestOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<TestOrderPriceMatchEnum>,
    #[arg(
        help = r#"`NONE`:No STP / `EXPIRE_TAKER`:expire taker order when STP triggers/ `EXPIRE_MAKER`:expire taker order when STP triggers/ `EXPIRE_BOTH`:expire both orders when STP triggers; default `NONE`"#,
        long
    )]
    self_trade_prevention_mode: Option<TestOrderSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"order cancel time for timeInForce `GTD`, mandatory when `timeInforce` set to `GTD`; order the timestamp only retains second-level precision, ms part will be ignored; The goodTillDate timestamp must be greater than the current time plus 600 seconds and smaller than 253402300799000"#,
        long
    )]
    good_till_date: Option<i64>,
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
struct UsersForceOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#""LIQUIDATION" for liquidation orders, "ADL" for ADL orders."#,
        long
    )]
    auto_close_type: Option<UsersForceOrdersAutoCloseTypeEnum>,
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
struct CloseUserDataStreamArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct KeepaliveUserDataStreamArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct StartUserDataStreamArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum DerivativesTradingUsdsFuturesCommands {
    #[command(
        about = decode_selected_entities(r#"Get current account information. User in single-asset/ multi-assets mode will see different value, see comments in response section for detail.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    AccountInformationV2(AccountInformationV2Args),
    #[command(
        about = decode_selected_entities(r#"Get current account information. User in single-asset/ multi-assets mode will see different value, see comments in response section for detail.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    AccountInformationV3(AccountInformationV3Args),
    #[command(
        about = decode_selected_entities(r#"Query account balance information.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    FuturesAccountBalanceV2(FuturesAccountBalanceV2Args),
    #[command(
        about = decode_selected_entities(r#"Query account balance information.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    FuturesAccountBalanceV3(FuturesAccountBalanceV3Args),
    #[command(
        about = decode_selected_entities(r#"Query account configuration

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    FuturesAccountConfiguration(FuturesAccountConfigurationArgs),
    #[command(
        about = decode_selected_entities(r#"Futures trading quantitative rules indicators, for more information on this, please refer to the [Futures Trading Quantitative Rules](https://www.binance.com/en/support/faq/4f462ebe6ff445d4a170be7d9e897272)

Weight: - **1** for a single symbol
- **10** when the symbol parameter is omitted

Security Type: USER_DATA"#, false),
    )]
    FuturesTradingQuantitativeRulesIndicators(FuturesTradingQuantitativeRulesIndicatorsArgs),
    #[command(
        about = decode_selected_entities(r#"Get user's BNB Fee Discount (Fee Discount On or Fee Discount Off )

Weight(IP): 30

Security Type: USER_DATA"#, false),
    )]
    GetBnbBurnStatus(GetBnbBurnStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get user's Multi-Assets mode (Multi-Assets Mode or Single-Asset Mode) on ***Every symbol***

Weight(IP): 30

Security Type: USER_DATA"#, false),
    )]
    GetCurrentMultiAssetsMode(GetCurrentMultiAssetsModeArgs),
    #[command(
        about = decode_selected_entities(r#"Get user's position mode (Hedge Mode or One-way Mode ) on ***EVERY symbol***

Weight(IP): 30

Security Type: USER_DATA"#, false),
    )]
    GetCurrentPositionMode(GetCurrentPositionModeArgs),
    #[command(
        about = decode_selected_entities(r#"Get Download Id For Futures Order History

Weight(IP): 1000

Security Type: USER_DATA

Notes:
- Request Limitation is 10 times per month, shared by front end download page and rest api
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForFuturesOrderHistory(GetDownloadIdForFuturesOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get download id for futures trade history

Weight(IP): 1000

Security Type: USER_DATA

Notes:
- Request Limitation is 5 times per month, shared by front end download page and rest api
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForFuturesTradeHistory(GetDownloadIdForFuturesTradeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get download id for futures transaction history

Weight(IP): 1000

Security Type: USER_DATA

Notes:
- Request Limitation is 5 times per month, shared by front end download page and rest api
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForFuturesTransactionHistory(GetDownloadIdForFuturesTransactionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get futures order history download link by Id

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetFuturesOrderHistoryDownloadLinkById(GetFuturesOrderHistoryDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Get futures trade download link by Id

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetFuturesTradeDownloadLinkById(GetFuturesTradeDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Get futures transaction history download link by Id

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetFuturesTransactionHistoryDownloadLinkById(GetFuturesTransactionHistoryDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Query income history

Weight(IP): 30

Security Type: USER_DATA

Notes:
- If `incomeType ` is not sent, all kinds of flow will be returned
- If `startTime` and `endTime` are not sent, the recent 7-day data will be returned.
- `trandId` is unique in the same `incomeType` for a user.
- Income history only contains data for the last three months."#, false),
    )]
    GetIncomeHistory(GetIncomeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query user notional and leverage bracket on speicfic symbol

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    NotionalAndLeverageBrackets(NotionalAndLeverageBracketsArgs),
    #[command(
        about = decode_selected_entities(r#"Query User Rate Limit

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryUserRateLimit(QueryUserRateLimitArgs),
    #[command(
        about = decode_selected_entities(r#"Get current account symbol configuration.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    SymbolConfiguration(SymbolConfigurationArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's BNB Fee Discount (Fee Discount On or Fee Discount Off ) on ***EVERY symbol***

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ToggleBnbBurnOnFuturesTrade(ToggleBnbBurnOnFuturesTradeArgs),
    #[command(
        about = decode_selected_entities(r#"Get User Commission Rate

Weight(IP): 20

Security Type: USER_DATA"#, false),
    )]
    UserCommissionRate(UserCommissionRateArgs),
    #[command(
        about = decode_selected_entities(r#"Accept the offered quote by quote ID.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    AcceptTheOfferedQuote(AcceptTheOfferedQuoteArgs),
    #[command(
        about = decode_selected_entities(r#"Query for all convertible token pairs and the tokens’ respective upper/lower limits

Weight(IP): 20

Notes:
- User needs to supply either or both of the input parameter
- If not defined for both fromAsset and toAsset, only partial token pairs will be returned
- Asset BNFCR is only available to convert for MICA region users."#, false),
    )]
    ListAllConvertPairs(ListAllConvertPairsArgs),
    #[command(
        about = decode_selected_entities(r#"Query order status by order ID.

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    OrderStatus(OrderStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Request a quote for the requested token pairs

Weight: 50(IP)
360/hour, 500/day

Security Type: USER_DATA

Notes:
- Either fromAmount or toAmount should be sent
- `quoteId` will be returned only if you have enough funds to convert"#, false),
    )]
    SendQuoteRequest(SendQuoteRequestArgs),
    #[command(
        about = decode_selected_entities(r#"Query the symbol-level ADL risk rating.

The ADL risk rating measures the likelihood of ADL during liquidation,
and the rating takes into account the insurance fund balance, position
concentration on the symbol, order book depth, price volatility, average
leverage, unrealized PnL, and margin utilization at the symbol level.

The rating can be high, medium and low, and is updated every 30 minutes.

Weight(IP): 1"#, false),
    )]
    AdlRisk(AdlRiskArgs),
    #[command(
        about = decode_selected_entities(r#"Asset index price.

> **CM-UM Integration (Effective 2026-06-30):** Renamed from *Multi-Assets Mode Asset Index*. The response now additionally pushes COIN-M settlement-asset price index entries (e.g., `BTCUSD`, `ETHUSD`, `BNBUSD`). The endpoint path `/fapi/v1/assetIndex` is unchanged.

Weight: **1** for a single symbol; **10** when the symbol parameter is omitted"#, false),
    )]
    AssetIndex(AssetIndexArgs),
    #[command(
        about = decode_selected_entities(r#"Query future basis

Weight(IP): 0

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available."#, false),
    )]
    Basis(BasisArgs),
    #[command(
        about = decode_selected_entities(r#"Test connectivity to the Rest API and get the current server time.

Weight(IP): 1"#, false),
    )]
    CheckServerTime(CheckServerTimeArgs),
    #[command(
        about = decode_selected_entities(r#"Query composite index symbol information

Weight(IP): 1

Notes:
- Only for composite index symbols"#, false),
    )]
    CompositeIndexSymbolInformation(CompositeIndexSymbolInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Get compressed, aggregate market trades. Market trades that fill in
100ms with the same price and the same taking side will have the
quantity aggregated.

Retail Price Improvement(RPI) orders are aggregated and without special
tags to be distinguished.

Weight(IP): 20

Notes:
- support querying futures trade histories that are not older than 24 hours
- If both `startTime` and `endTime` are sent, time between `startTime` and `endTime` must be less than 1 hour.
- If `fromId`, `startTime`, and `endTime` are not sent, the most recent aggregate trades will be returned.
- Only market trades will be aggregated and returned, which means the insurance fund trades and ADL trades won't be aggregated.
- Sending both `startTime`/`endTime` and `fromId` might cause response timeout, please send either `fromId` or `startTime`/`endTime`"#, false),
    )]
    CompressedAggregateTradesList(CompressedAggregateTradesListArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for a specific contract type.
Klines are uniquely identified by their open time.

Weight: based on parameter `LIMIT`

| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| > 1000      | 10     |

Notes:
- If startTime and endTime are not sent, the most recent klines are returned."#, false),
    )]
    ContinuousContractKlineCandlestickData(ContinuousContractKlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Current exchange trading rules and symbol information

Weight(IP): 1"#, false),
    )]
    ExchangeInformation(ExchangeInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Get Funding Rate History

Weight: share 500/5min/IP rate limit with GET /fapi/v1/fundingInfo

Notes:
- If `startTime` and `endTime` are not sent, the most recent 200 records are returned.
- If the number of data between `startTime` and `endTime` is larger than `limit`, return as `startTime` + `limit`.
- In ascending order."#, false),
    )]
    GetFundingRateHistory(GetFundingRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query funding rate info for symbols that had FundingRateCap/FundingRateFloor / fundingIntervalHours adjustment

Weight: **0**

share 500/5min/IP rate limit with `GET /fapi/v1/fundingRate`"#, false),
    )]
    GetFundingRateInfo(GetFundingRateInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for the index price of a pair.
Klines are uniquely identified by their open time.

Weight: based on parameter `LIMIT`

| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| > 1000      | 10     |

Notes:
- If startTime and endTime are not sent, the most recent klines are returned."#, false),
    )]
    IndexPriceKlineCandlestickData(IndexPriceKlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for a symbol.
Klines are uniquely identified by their open time.

Weight: based on parameter `LIMIT`

| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| > 1000      | 10     |

Notes:
- If startTime and endTime are not sent, the most recent klines are returned."#, false),
    )]
    KlineCandlestickData(KlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Query symbol Long/Short Ratio

Weight(IP): 0

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available.
- IP rate limit 1000 requests/5min"#, false),
    )]
    LongShortRatio(LongShortRatioArgs),
    #[command(
        about = decode_selected_entities(r#"Mark Price and Funding Rate

Weight: **1** with symbol, **10** without symbol"#, false),
    )]
    MarkPrice(MarkPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for the mark price of a symbol.
Klines are uniquely identified by their open time.

Weight: based on parameter `LIMIT`

| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| > 1000      | 10     |

Notes:
- If startTime and endTime are not sent, the most recent klines are returned."#, false),
    )]
    MarkPriceKlineCandlestickData(MarkPriceKlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Get older market historical trades.

Weight(IP): 20

Security Type: MARKET_DATA

Notes:
- Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won't be returned.
- Only supports data from within the last one month"#, false),
    )]
    OldTradesLookup(OldTradesLookupArgs),
    #[command(
        about = decode_selected_entities(r#"Get present open interest of a specific symbol.

Weight(IP): 1"#, false),
    )]
    OpenInterest(OpenInterestArgs),
    #[command(
        about = decode_selected_entities(r#"Open Interest Statistics

Weight(IP): 0

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 1 month is available.
- IP rate limit 1000 requests/5min"#, false),
    )]
    OpenInterestStatistics(OpenInterestStatisticsArgs),
    #[command(
        about = decode_selected_entities(r#"Query symbol orderbook

Retail Price Improvement(RPI) orders are not visible and excluded in the
response message.

Weight: Adjusted based on the limit:

| Limit         | Weight |
| ------------- | ------ |
| 5, 10, 20, 50 | 2      |
| 100           | 5      |
| 500           | 10     |
| 1000          | 20     |"#, false),
    )]
    OrderBook(OrderBookArgs),
    #[command(
        about = decode_selected_entities(r#"Premium index kline bars of a symbol. Klines are uniquely identified by their open time.

Weight: based on parameter `LIMIT`

| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| > 1000      | 10     |

Notes:
- If startTime and endTime are not sent, the most recent klines are returned."#, false),
    )]
    PremiumIndexKlineData(PremiumIndexKlineDataArgs),
    #[command(
        about = decode_selected_entities(r#"Latest price for a symbol or symbols.

Weight(IP): 0"#, false),
    )]
    QuarterlyContractSettlementPrice(QuarterlyContractSettlementPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Query index price constituents

**Note**:
Prices from constituents of TradFi perps will be hiden and displayed as -1.

Weight(IP): 2"#, false),
    )]
    QueryIndexPriceConstituents(QueryIndexPriceConstituentsArgs),
    #[command(
        about = decode_selected_entities(r#"Query Insurance Fund Balance Snapshot

Weight(IP): 1"#, false),
    )]
    QueryInsuranceFundBalanceSnapshot(QueryInsuranceFundBalanceSnapshotArgs),
    #[command(
        about = decode_selected_entities(r#"Get recent market trades

Weight(IP): 5

Notes:
- Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won't be returned."#, false),
    )]
    RecentTradesList(RecentTradesListArgs),
    #[command(
        about = decode_selected_entities(r#"Query symbol orderbook with RPI orders

RPI(Retail Price Improvement) orders are included and aggreated in the
response message. Crossed price levels are hidden and invisible.

Weight: Adjusted based on the limit:

| Limit         | Weight |
| ------------- | ------ |
| 1000          | 20     |"#, false),
    )]
    RpiOrderBook(RpiOrderBookArgs),
    #[command(
        about = decode_selected_entities(r#"Best price/qty on the order book for a symbol or symbols.

Retail Price Improvement(RPI) orders are not visible and excluded in the
response message.

Weight: **2** for a single symbol;
**5** when the symbol parameter is omitted

Notes:
- If the symbol is not sent, bookTickers for all symbols will be returned in an array.
- The field `X-MBX-USED-WEIGHT-1M` in response header is not accurate from this endpoint, please ignore."#, false),
    )]
    SymbolOrderBookTicker(SymbolOrderBookTickerArgs),
    #[command(
        about = decode_selected_entities(r#"Latest price for a symbol or symbols.

Weight: 1 for a single symbol;
2 when the symbol parameter is omitted

Notes:
- If the symbol is not sent, prices for all symbols will be returned in an array."#, false),
    )]
    SymbolPriceTicker(SymbolPriceTickerArgs),
    #[command(
        about = decode_selected_entities(r#"Latest price for a symbol or symbols.

Weight: 1 for a single symbol;
2 when the symbol parameter is omitted

Notes:
- If the symbol is not sent, prices for all symbols will be returned in an array.
- The field `X-MBX-USED-WEIGHT-1M` in response header is not accurate from this endpoint, please ignore."#, false),
    )]
    SymbolPriceTickerV2(SymbolPriceTickerV2Args),
    #[command(
        about = decode_selected_entities(r#"Taker Buy/Sell Volume

Weight(IP): 0

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available.
- IP rate limit 1000 requests/5min"#, false),
    )]
    TakerBuySellVolume(TakerBuySellVolumeArgs),
    #[command(
        about = decode_selected_entities(r#"Test connectivity to the Rest API.

Weight(IP): 1"#, false),
    )]
    TestConnectivity(TestConnectivityArgs),
    #[command(
        about = decode_selected_entities(r#"24 hour rolling window price change statistics.
**Careful** when accessing this with no symbol.

Weight: **1** for a single symbol;
**40** when the symbol parameter is omitted

Notes:
- If the symbol is not sent, tickers for all symbols will be returned in an array."#, false),
    )]
    Ticker24hrPriceChangeStatistics(Ticker24hrPriceChangeStatisticsArgs),
    #[command(
        about = decode_selected_entities(r#"The proportion of net long and net short accounts to total accounts of
the top 20% users with the highest margin balance. Each account is
counted once only.

Long Account % = Accounts of top traders with net long positions / Total
accounts of top traders with open positions

Short Account % = Accounts of top traders with net short positions /
Total accounts of top traders with open positions

Long/Short Ratio (Accounts) = Long Account % / Short Account %

Security Type: MARKET_DATA

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available.
- IP rate limit 1000 requests/5min"#, false),
    )]
    TopTraderLongShortRatioAccounts(TopTraderLongShortRatioAccountsArgs),
    #[command(
        about = decode_selected_entities(r#"The proportion of net long and net short positions to total open
positions of the top 20% users with the highest margin balance.

Long Position % = Long positions of top traders / Total open positions
of top traders

Short Position % = Short positions of top traders / Total open positions
of top traders

Long/Short Ratio (Positions) = Long Position % / Short Position %

Weight(IP): 0

Security Type: MARKET_DATA

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available.
- IP rate limit 1000 requests/5min"#, false),
    )]
    TopTraderLongShortRatioPositions(TopTraderLongShortRatioPositionsArgs),
    #[command(
        about = decode_selected_entities(r#"Trading session schedules for the underlying assets of TradFi Perps are provided for a one-week period forward and one-week period backward starting from the day prior to the query time, covering the U.S. equity market, Korean equity market, Hong Kong equity market, and the commodity market.

Session types per market:
- U.S. equity market: "PRE_MARKET", "REGULAR", "AFTER_MARKET", "OVERNIGHT", "NO_TRADING".
- Commodity market: "REGULAR", "NO_TRADING".
- Korean equity market: "REGULAR", "NO_TRADING".
- Hong Kong equity market: "REGULAR", "NO_TRADING".

Weight(IP): 5"#, false),
    )]
    TradingSchedule(TradingScheduleArgs),
    #[command(
        about = decode_selected_entities(r#"Get Classic Portfolio Margin current account information.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- maxWithdrawAmount is for asset transfer out to the spot wallet."#, false),
    )]
    ClassicPortfolioMarginAccountInformation(ClassicPortfolioMarginAccountInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Get trades for a specific account and symbol.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are both not sent, then the last 7 days' data will be returned.
- The time between `startTime` and `endTime` cannot be longer than 7 days.
- The parameter `fromId` cannot be sent with `startTime` or `endTime`.
- Only support querying trade in the past 6 months"#, false),
    )]
    AccountTradeList(AccountTradeListArgs),
    #[command(
        about = decode_selected_entities(r#"Get all account orders; active, canceled, or filled.

- These orders will not be found:
  - order status is `CANCELED` or `EXPIRED` **AND** order has NO filled trade **AND** created time + 3 days < current time
  - order create time + 90 days < current time

Weight(IP): 5

Security Type: USER_DATA

Notes:
- If `orderId` is set, it will get orders >= that `orderId`. Otherwise most recent orders are returned.
- The query time period must be less then 7 days( default as the recent 7 days)."#, false),
    )]
    AllOrders(AllOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel all open orders of the specified symbol at the end of the specified countdown.

The endpoint should be called repeatedly as heartbeats so that the existing countdown time can be canceled and
replaced by a new one.

Example usage:

Call this endpoint at 30s intervals with an countdownTime of 120000 (120s).
If this endpoint is not called within 120 seconds, all your orders of the specified symbol will be automatically
canceled.
If this endpoint is called with an countdownTime of 0, the countdown timer will be stopped.

The system will check all countdowns **approximately every 10 milliseconds**, so please note that sufficient
redundancy should be considered when using this function. We do not recommend setting the countdown time to be
too precise or too small.

Weight(IP): 10

Security Type: TRADE"#, false),
    )]
    AutoCancelAllOpenOrders(AutoCancelAllOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an active algo (conditional) order, including TP/SL (Take Profit / Stop Loss) and trailing stop orders on USD-M Futures.

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `algoId` or `clientAlgoId` must be sent."#, false),
    )]
    CancelAlgoOrder(CancelAlgoOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel all open algo (conditional) orders on a symbol, including TP/SL (Take Profit / Stop Loss) and trailing stop orders on USD-M Futures.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAllAlgoOpenOrders(CancelAllAlgoOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel All Open Orders

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAllOpenOrders(CancelAllOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel Multiple Orders

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderIdList` or `origClientOrderIdList ` must be sent."#, false),
    )]
    CancelMultipleOrders(CancelMultipleOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an active order.

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent."#, false),
    )]
    CancelOrder(CancelOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's initial leverage of specific symbol market.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeInitialLeverage(ChangeInitialLeverageArgs),
    #[command(
        about = decode_selected_entities(r#"Change symbol level margin type

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeMarginType(ChangeMarginTypeArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's Multi-Assets mode (Multi-Assets Mode or Single-Asset Mode) on ***Every symbol***

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeMultiAssetsMode(ChangeMultiAssetsModeArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's position mode (Hedge Mode or One-way Mode ) on ***EVERY symbol***.

**After CM migration**, UM and CM share the **same** `dualSidePosition` setting. Calling this endpoint flips both UM and CM at once. If either side has any open order or open position, the change is rejected:
- `-4067` (open orders exist)
- `-4068` (open position exists)

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangePositionMode(ChangePositionModeArgs),
    #[command(
        about = decode_selected_entities(r#"Get all open algo (conditional) orders on a symbol, including TP/SL (Take Profit / Stop Loss) and trailing stop orders on USD-M Futures.

Weight: **1** for a single symbol; **40** when the symbol parameter is omitted

**Careful** when accessing this with no symbol.

Security Type: USER_DATA

Notes:
- If the symbol is not sent, orders for all symbols will be returned in an array."#, false),
    )]
    CurrentAllAlgoOpenOrders(CurrentAllAlgoOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get all open orders on a symbol.

Weight: **1** for a single symbol; **40** when the symbol parameter is omitted

**Careful** when accessing this with no symbol.

Security Type: USER_DATA

Notes:
- If the symbol is not sent, orders for all symbols will be returned in an array."#, false),
    )]
    CurrentAllOpenOrders(CurrentAllOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Sign TradFi-Perps agreement contract

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    FuturesTradfiPerpsContract(FuturesTradfiPerpsContractArgs),
    #[command(
        about = decode_selected_entities(r#"Get order modification history

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `orderId` or `origClientOrderId` must be sent, and the
`orderId` will prevail if both are sent.

- Order modify history longer than 3 month is not avaliable"#, false),
    )]
    GetOrderModifyHistory(GetOrderModifyHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Position Margin Change History

Weight(IP): 1

Security Type: TRADE

Notes:
- Support querying future histories that are not older than 30 days
- The time between `startTime` and `endTime`can't be more than 30 days"#, false),
    )]
    GetPositionMarginChangeHistory(GetPositionMarginChangeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Modify Isolated Position Margin

Weight(IP): 1

Security Type: TRADE

Notes:
- Only for isolated symbol"#, false),
    )]
    ModifyIsolatedPositionMargin(ModifyIsolatedPositionMarginArgs),
    #[command(
        about = decode_selected_entities(r#"Modify Multiple Orders (TRADE)

Weight: 5 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
5 on IP rate limit(x-mbx-used-weight-1m);

Security Type: TRADE

Notes:
- Parameter rules are same with `Modify Order`
- Batch modify orders are processed concurrently, and the order of matching is not guaranteed.
- The order of returned contents for batch modify orders is the same as the order of the order list.
- One order can only be modfied for less than 10000 times"#, false),
    )]
    ModifyMultipleOrders(ModifyMultipleOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue

Weight: 1 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
0 on IP rate limit(x-mbx-used-weight-1m)

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent, and the `orderId` will prevail if both are sent.
- Both `quantity` and `price` must be sent, which is different from dapi modify order endpoint.
- When the new `quantity` or `price` doesn't satisfy PRICE_FILTER / PERCENT_FILTER / LOT_SIZE, amendment will be rejected and the order will stay as it is.
- However the order will be cancelled by the amendment in the following situations:
  - when the order is in partially filled status and the new `quantity` <= `executedQty`
  - When the order is `GTX` and the new price will cause it to be executed immediately
- One order can only be modfied for less than 10000 times"#, false),
    )]
    ModifyOrder(ModifyOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a new algo (conditional) order. Use this endpoint to place **TP/SL (Take Profit / Stop Loss)** and trailing stop orders on USD-M Futures. Supported order types under `algoType=CONDITIONAL` are `STOP_MARKET`, `TAKE_PROFIT_MARKET`, `STOP`, `TAKE_PROFIT`, and `TRAILING_STOP_MARKET`.

Weight: 1 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
0 on IP rate limit(x-mbx-used-weight-1m)

Security Type: TRADE

Notes:
- Algo order with type `STOP`, parameter `timeInForce` can be sent (default `GTC`).
- Algo order with type `TAKE_PROFIT`, parameter `timeInForce` can be sent ( default `GTC`).
- Condition orders will be triggered when:
  - If parameter`priceProtect`is sent as true:
    - when price reaches the `triggerPrice` ，the difference rate between "MARK_PRICE" and "CONTRACT_PRICE" cannot be larger than the "triggerProtect" of the symbol
    - "triggerProtect" of a symbol can be got from `GET /fapi/v1/exchangeInfo`
  - `STOP`, `STOP_MARKET`:
    - BUY: latest price ("MARK_PRICE" or "CONTRACT_PRICE") >= `triggerPrice`
    - SELL: latest price ("MARK_PRICE" or "CONTRACT_PRICE")
  - `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`:
    - BUY: latest price ("MARK_PRICE" or "CONTRACT_PRICE")
    - SELL: latest price ("MARK_PRICE" or "CONTRACT_PRICE") >= `triggerPrice`
  - `TRAILING_STOP_MARKET`:
    - BUY: the lowest price after order placed = the lowest price * (1 + `callbackRate`)
    - SELL: the highest price after order placed >= `activatePrice`, and the latest price
- For `TRAILING_STOP_MARKET`, if you got such error code. > `{"code": -2021, "msg": "Order would immediately trigger."}` > means that the parameters you send do not meet the following requirements:
  - BUY: `activatePrice` should be smaller than latest price.
  - SELL: `activatePrice` should be larger than latest price.
- `STOP_MARKET`, `TAKE_PROFIT_MARKET` with `closePosition`=`true`:
  - Follow the same rules for condition orders.
  - If triggered，**close all** current long position( if `SELL`) or current short position( if `BUY`).
  - Cannot be used with `quantity` paremeter
  - Cannot be used with `reduceOnly` parameter
  - In Hedge Mode,cannot be used with `BUY` orders in `LONG` position side. and cannot be used with `SELL` orders in `SHORT` position side
- `selfTradePreventionMode` is only effective when `timeInForce` set to `IOC` or `GTC` or `GTD`."#, false),
    )]
    NewAlgoOrder(NewAlgoOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a new order.

Weight: 1 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
0 on IP rate limit(x-mbx-used-weight-1m)

Security Type: TRADE

Notes:
Additional mandatory parameters based on `type`:

| Type | Additional mandatory parameters |
|------|----------------------------------|
| `LIMIT` | `timeInForce`, `quantity`, `price` |
| `MARKET` | `quantity` |

- If `newOrderRespType` is sent as `RESULT`:
  - `MARKET` order: the final FILLED result of the order will be returned directly.
  - `LIMIT` order with special `timeInForce`: the final status result of the order (FILLED or EXPIRED) will be returned directly.
- `selfTradePreventionMode` is only effective when `timeInForce` is set to `IOC`, `GTC`, or `GTD`.
- In extreme market conditions, `timeInForce` `GTD` order auto-cancel time might be delayed compared to `goodTillDate`."#, false),
    )]
    NewOrder(NewOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Place Multiple Orders

Weight: 5 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
5 on IP rate limit(x-mbx-used-weight-1m);

Security Type: TRADE

Notes:
- Paremeter rules are same with `New Order`
- Batch orders are processed concurrently, and the order of matching is not guaranteed.
- The order of returned contents for batch orders is the same as the order of the order list."#, false),
    )]
    PlaceMultipleOrders(PlaceMultipleOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Position ADL Quantile Estimation

* Values update every 30s.
* Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
* For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode, "LONG", "SHORT", and "BOTH" will be returned to show the positions' adl quantiles of different position sides.
* If the positions of the symbol are crossed margined in Hedge Mode:
  * "HEDGE" as a sign will be returned instead of "BOTH";
  * A same value caculated on unrealized pnls on long and short sides' positions will be shown for "LONG" and "SHORT" when there are positions in both of long and short sides.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    PositionAdlQuantileEstimation(PositionAdlQuantileEstimationArgs),
    #[command(
        about = decode_selected_entities(r#"Get current position information.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- Please use with user data stream `ACCOUNT_UPDATE` to meet your timeliness and accuracy needs."#, false),
    )]
    PositionInformationV2(PositionInformationV2Args),
    #[command(
        about = decode_selected_entities(r#"Get current position information(only symbol that has position or open
orders will be returned).

Weight(IP): 5

Security Type: USER_DATA

Notes:
- Please use with user data stream `ACCOUNT_UPDATE` to meet your timeliness and accuracy needs."#, false),
    )]
    PositionInformationV3(PositionInformationV3Args),
    #[command(
        about = decode_selected_entities(r#"Check the status of an algo (conditional) order, such as TP/SL (Take Profit / Stop Loss) or trailing stop orders on USD-M Futures.

* These orders will not be found:
  * order status is `CANCELED` or `EXPIRED` **AND** order has NO filled trade **AND** created time + 3 days < current time
  * order create time + 90 days < current time

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `algoId` or `clientAlgoId` must be sent.
- `algoId` is self-increment for each specific `symbol`"#, false),
    )]
    QueryAlgoOrder(QueryAlgoOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Get all algo (conditional) orders — active, CANCELED, TRIGGERED, or FINISHED — including TP/SL (Take Profit / Stop Loss) and trailing stop orders on USD-M Futures.

* These orders will not be found:
  * order status is `CANCELED` or `EXPIRED` **AND** order has NO filled trade **AND** created time + 3 days < current time
  * order create time + 90 days < current time

Weight(IP): 5

Security Type: USER_DATA

Notes:
- If `algoId` is set, it will get orders >= that `algoId`. Otherwise most recent orders are returned.
- The query time period must be less then 7 days( default as the recent 7 days)."#, false),
    )]
    QueryAllAlgoOrders(QueryAllAlgoOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query open order

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either`orderId` or `origClientOrderId` must be sent
- If the queried order has been filled or cancelled, the error message "Order does not exist" will be returned."#, false),
    )]
    QueryCurrentOpenOrder(QueryCurrentOpenOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Check an order's status.

* These orders will not be found:
  * order status is `CANCELED` or `EXPIRED` **AND** order has NO filled trade **AND** created time + 3 days < current time
  * order create time + 90 days < current time

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `orderId` or `origClientOrderId` must be sent.
- `orderId` is self-increment for each specific `symbol`"#, false),
    )]
    QueryOrder(QueryOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Testing order request, this order will not be submitted to matching engine

Security Type: TRADE

Notes:
Additional mandatory parameters based on `type`:

| Type                             | Additional mandatory parameters    |
| -------------------------------- | ---------------------------------- |
| `LIMIT`                          | `timeInForce`, `quantity`, `price` |
| `MARKET`                         | `quantity`                         |
| `STOP/TAKE_PROFIT`               | `quantity`,  `price`, `stopPrice`  |
| `STOP_MARKET/TAKE_PROFIT_MARKET` | `stopPrice`                        |
| `TRAILING_STOP_MARKET`           | `callbackRate`                     |

- Order with type `STOP`, parameter `timeInForce` can be sent ( default `GTC`).
- Order with type `TAKE_PROFIT`, parameter `timeInForce` can be sent (default `GTC`).
- Condition orders will be triggered when:
  - If parameter`priceProtect`is sent as true:
    - when price reaches the `stopPrice` ，the difference rate between "MARK_PRICE" and "CONTRACT_PRICE" cannot be larger than the "triggerProtect" of the symbol
    - "triggerProtect" of a symbol can be got from `GET /fapi/v1/exchangeInfo`
  - `STOP`, `STOP_MARKET`:
    - BUY: latest price ("MARK_PRICE" or "CONTRACT_PRICE") >= `stopPrice`
    - SELL: latest price ("MARK_PRICE" or "CONTRACT_PRICE")
  - `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`:
    - BUY: latest price ("MARK_PRICE" or "CONTRACT_PRICE")
    - SELL: latest price ("MARK_PRICE" or "CONTRACT_PRICE") >= `stopPrice`
  - `TRAILING_STOP_MARKET`:
    - BUY: the lowest price after order placed ``= the lowest price * (1 + `callbackRate`)
    - SELL: the highest price after order placed >= `activationPrice`, and the latest price
- For `TRAILING_STOP_MARKET`, if you got such error code. > `{"code": -2021, "msg": "Order would immediately trigger."}` > means that the parameters you send do not meet the following requirements:
  - BUY: `activationPrice` should be smaller than latest price.
  - SELL: `activationPrice` should be larger than latest price.
- If `newOrderRespType ` is sent as `RESULT` :
  - `MARKET` order: the final FILLED result of the order will be return directly.
  - `LIMIT` order with special `timeInForce`: the final status result of the order(FILLED or EXPIRED) will be returned directly.
- `STOP_MARKET`, `TAKE_PROFIT_MARKET` with `closePosition`=`true`:
  - Follow the same rules for condition orders.
  - If triggered，**close all** current long position( if `SELL`) or current short position( if `BUY`).
  - Cannot be used with `quantity` paremeter
  - Cannot be used with `reduceOnly` parameter
  - In Hedge Mode,cannot be used with `BUY` orders in `LONG` position side. and cannot be used with `SELL` orders in `SHORT` position side
- `selfTradePreventionMode` is only effective when `timeInForce` set to `IOC` or `GTC` or `GTD`.
- In extreme market conditions, timeInForce `GTD` order auto cancel time might be delayed comparing to `goodTillDate`"#, false),
    )]
    TestOrder(TestOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query user's Force Orders

Weight: **20** with symbol, **50** without symbol

Security Type: USER_DATA

Notes:
- If "autoCloseType" is not sent, orders with both of the types will be returned
- If "startTime" is not sent, data within 7 days before "endTime" can be queried"#, false),
    )]
    UsersForceOrders(UsersForceOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Close out a user data stream.

Weight(IP): 1

Security Type: USER_STREAM"#, false),
    )]
    CloseUserDataStream(CloseUserDataStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Keepalive a user data stream to prevent a time out. User data streams
will close after 60 minutes. It's recommended to send a ping about every
60 minutes.

Weight(IP): 1

Security Type: USER_STREAM"#, false),
    )]
    KeepaliveUserDataStream(KeepaliveUserDataStreamArgs),
    #[command(
        about = decode_selected_entities(r#"Start a new user data stream. The stream will close after 60 minutes
unless a keepalive is sent. If the account has an active `listenKey`,
that `listenKey` will be returned and its validity will be extended for
60 minutes.

Weight(IP): 1

Security Type: USER_STREAM"#, false),
    )]
    StartUserDataStream(StartUserDataStreamArgs),
}

pub async fn handle_derivatives_trading_usds_futures_command(
    command: DerivativesTradingUsdsFuturesCommands,
) -> anyhow::Result<()> {
    match command {
        DerivativesTradingUsdsFuturesCommands::AccountInformationV2(args) => {
            account_information_v2(args).await
        }

        DerivativesTradingUsdsFuturesCommands::AccountInformationV3(args) => {
            account_information_v3(args).await
        }

        DerivativesTradingUsdsFuturesCommands::FuturesAccountBalanceV2(args) => {
            futures_account_balance_v2(args).await
        }

        DerivativesTradingUsdsFuturesCommands::FuturesAccountBalanceV3(args) => {
            futures_account_balance_v3(args).await
        }

        DerivativesTradingUsdsFuturesCommands::FuturesAccountConfiguration(args) => {
            futures_account_configuration(args).await
        }

        DerivativesTradingUsdsFuturesCommands::FuturesTradingQuantitativeRulesIndicators(args) => {
            futures_trading_quantitative_rules_indicators(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetBnbBurnStatus(args) => {
            get_bnb_burn_status(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetCurrentMultiAssetsMode(args) => {
            get_current_multi_assets_mode(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetCurrentPositionMode(args) => {
            get_current_position_mode(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetDownloadIdForFuturesOrderHistory(args) => {
            get_download_id_for_futures_order_history(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetDownloadIdForFuturesTradeHistory(args) => {
            get_download_id_for_futures_trade_history(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetDownloadIdForFuturesTransactionHistory(args) => {
            get_download_id_for_futures_transaction_history(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetFuturesOrderHistoryDownloadLinkById(args) => {
            get_futures_order_history_download_link_by_id(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetFuturesTradeDownloadLinkById(args) => {
            get_futures_trade_download_link_by_id(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetFuturesTransactionHistoryDownloadLinkById(
            args,
        ) => get_futures_transaction_history_download_link_by_id(args).await,

        DerivativesTradingUsdsFuturesCommands::GetIncomeHistory(args) => {
            get_income_history(args).await
        }

        DerivativesTradingUsdsFuturesCommands::NotionalAndLeverageBrackets(args) => {
            notional_and_leverage_brackets(args).await
        }

        DerivativesTradingUsdsFuturesCommands::QueryUserRateLimit(args) => {
            query_user_rate_limit(args).await
        }

        DerivativesTradingUsdsFuturesCommands::SymbolConfiguration(args) => {
            symbol_configuration(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ToggleBnbBurnOnFuturesTrade(args) => {
            toggle_bnb_burn_on_futures_trade(args).await
        }

        DerivativesTradingUsdsFuturesCommands::UserCommissionRate(args) => {
            user_commission_rate(args).await
        }

        DerivativesTradingUsdsFuturesCommands::AcceptTheOfferedQuote(args) => {
            accept_the_offered_quote(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ListAllConvertPairs(args) => {
            list_all_convert_pairs(args).await
        }

        DerivativesTradingUsdsFuturesCommands::OrderStatus(args) => order_status(args).await,

        DerivativesTradingUsdsFuturesCommands::SendQuoteRequest(args) => {
            send_quote_request(args).await
        }

        DerivativesTradingUsdsFuturesCommands::AdlRisk(args) => adl_risk(args).await,

        DerivativesTradingUsdsFuturesCommands::AssetIndex(args) => asset_index(args).await,

        DerivativesTradingUsdsFuturesCommands::Basis(args) => basis(args).await,

        DerivativesTradingUsdsFuturesCommands::CheckServerTime(args) => {
            check_server_time(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CompositeIndexSymbolInformation(args) => {
            composite_index_symbol_information(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CompressedAggregateTradesList(args) => {
            compressed_aggregate_trades_list(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ContinuousContractKlineCandlestickData(args) => {
            continuous_contract_kline_candlestick_data(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ExchangeInformation(args) => {
            exchange_information(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetFundingRateHistory(args) => {
            get_funding_rate_history(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetFundingRateInfo(args) => {
            get_funding_rate_info(args).await
        }

        DerivativesTradingUsdsFuturesCommands::IndexPriceKlineCandlestickData(args) => {
            index_price_kline_candlestick_data(args).await
        }

        DerivativesTradingUsdsFuturesCommands::KlineCandlestickData(args) => {
            kline_candlestick_data(args).await
        }

        DerivativesTradingUsdsFuturesCommands::LongShortRatio(args) => long_short_ratio(args).await,

        DerivativesTradingUsdsFuturesCommands::MarkPrice(args) => mark_price(args).await,

        DerivativesTradingUsdsFuturesCommands::MarkPriceKlineCandlestickData(args) => {
            mark_price_kline_candlestick_data(args).await
        }

        DerivativesTradingUsdsFuturesCommands::OldTradesLookup(args) => {
            old_trades_lookup(args).await
        }

        DerivativesTradingUsdsFuturesCommands::OpenInterest(args) => open_interest(args).await,

        DerivativesTradingUsdsFuturesCommands::OpenInterestStatistics(args) => {
            open_interest_statistics(args).await
        }

        DerivativesTradingUsdsFuturesCommands::OrderBook(args) => order_book(args).await,

        DerivativesTradingUsdsFuturesCommands::PremiumIndexKlineData(args) => {
            premium_index_kline_data(args).await
        }

        DerivativesTradingUsdsFuturesCommands::QuarterlyContractSettlementPrice(args) => {
            quarterly_contract_settlement_price(args).await
        }

        DerivativesTradingUsdsFuturesCommands::QueryIndexPriceConstituents(args) => {
            query_index_price_constituents(args).await
        }

        DerivativesTradingUsdsFuturesCommands::QueryInsuranceFundBalanceSnapshot(args) => {
            query_insurance_fund_balance_snapshot(args).await
        }

        DerivativesTradingUsdsFuturesCommands::RecentTradesList(args) => {
            recent_trades_list(args).await
        }

        DerivativesTradingUsdsFuturesCommands::RpiOrderBook(args) => rpi_order_book(args).await,

        DerivativesTradingUsdsFuturesCommands::SymbolOrderBookTicker(args) => {
            symbol_order_book_ticker(args).await
        }

        DerivativesTradingUsdsFuturesCommands::SymbolPriceTicker(args) => {
            symbol_price_ticker(args).await
        }

        DerivativesTradingUsdsFuturesCommands::SymbolPriceTickerV2(args) => {
            symbol_price_ticker_v2(args).await
        }

        DerivativesTradingUsdsFuturesCommands::TakerBuySellVolume(args) => {
            taker_buy_sell_volume(args).await
        }

        DerivativesTradingUsdsFuturesCommands::TestConnectivity(args) => {
            test_connectivity(args).await
        }

        DerivativesTradingUsdsFuturesCommands::Ticker24hrPriceChangeStatistics(args) => {
            ticker24hr_price_change_statistics(args).await
        }

        DerivativesTradingUsdsFuturesCommands::TopTraderLongShortRatioAccounts(args) => {
            top_trader_long_short_ratio_accounts(args).await
        }

        DerivativesTradingUsdsFuturesCommands::TopTraderLongShortRatioPositions(args) => {
            top_trader_long_short_ratio_positions(args).await
        }

        DerivativesTradingUsdsFuturesCommands::TradingSchedule(args) => {
            trading_schedule(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ClassicPortfolioMarginAccountInformation(args) => {
            classic_portfolio_margin_account_information(args).await
        }

        DerivativesTradingUsdsFuturesCommands::AccountTradeList(args) => {
            account_trade_list(args).await
        }

        DerivativesTradingUsdsFuturesCommands::AllOrders(args) => all_orders(args).await,

        DerivativesTradingUsdsFuturesCommands::AutoCancelAllOpenOrders(args) => {
            auto_cancel_all_open_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CancelAlgoOrder(args) => {
            cancel_algo_order(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CancelAllAlgoOpenOrders(args) => {
            cancel_all_algo_open_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CancelAllOpenOrders(args) => {
            cancel_all_open_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CancelMultipleOrders(args) => {
            cancel_multiple_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CancelOrder(args) => cancel_order(args).await,

        DerivativesTradingUsdsFuturesCommands::ChangeInitialLeverage(args) => {
            change_initial_leverage(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ChangeMarginType(args) => {
            change_margin_type(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ChangeMultiAssetsMode(args) => {
            change_multi_assets_mode(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ChangePositionMode(args) => {
            change_position_mode(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CurrentAllAlgoOpenOrders(args) => {
            current_all_algo_open_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CurrentAllOpenOrders(args) => {
            current_all_open_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::FuturesTradfiPerpsContract(args) => {
            futures_tradfi_perps_contract(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetOrderModifyHistory(args) => {
            get_order_modify_history(args).await
        }

        DerivativesTradingUsdsFuturesCommands::GetPositionMarginChangeHistory(args) => {
            get_position_margin_change_history(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ModifyIsolatedPositionMargin(args) => {
            modify_isolated_position_margin(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ModifyMultipleOrders(args) => {
            modify_multiple_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::ModifyOrder(args) => modify_order(args).await,

        DerivativesTradingUsdsFuturesCommands::NewAlgoOrder(args) => new_algo_order(args).await,

        DerivativesTradingUsdsFuturesCommands::NewOrder(args) => new_order(args).await,

        DerivativesTradingUsdsFuturesCommands::PlaceMultipleOrders(args) => {
            place_multiple_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::PositionAdlQuantileEstimation(args) => {
            position_adl_quantile_estimation(args).await
        }

        DerivativesTradingUsdsFuturesCommands::PositionInformationV2(args) => {
            position_information_v2(args).await
        }

        DerivativesTradingUsdsFuturesCommands::PositionInformationV3(args) => {
            position_information_v3(args).await
        }

        DerivativesTradingUsdsFuturesCommands::QueryAlgoOrder(args) => query_algo_order(args).await,

        DerivativesTradingUsdsFuturesCommands::QueryAllAlgoOrders(args) => {
            query_all_algo_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::QueryCurrentOpenOrder(args) => {
            query_current_open_order(args).await
        }

        DerivativesTradingUsdsFuturesCommands::QueryOrder(args) => query_order(args).await,

        DerivativesTradingUsdsFuturesCommands::TestOrder(args) => test_order(args).await,

        DerivativesTradingUsdsFuturesCommands::UsersForceOrders(args) => {
            users_force_orders(args).await
        }

        DerivativesTradingUsdsFuturesCommands::CloseUserDataStream(args) => {
            close_user_data_stream(args).await
        }

        DerivativesTradingUsdsFuturesCommands::KeepaliveUserDataStream(args) => {
            keepalive_user_data_stream(args).await
        }

        DerivativesTradingUsdsFuturesCommands::StartUserDataStream(args) => {
            start_user_data_stream(args).await
        }
    }
}

async fn account_information_v2(args: AccountInformationV2Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountInformationV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountInformationV2Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountInformationV2Params::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_information_v2(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn account_information_v3(args: AccountInformationV3Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountInformationV3Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountInformationV3Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountInformationV3Params::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_information_v3(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn futures_account_balance_v2(args: FuturesAccountBalanceV2Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FuturesAccountBalanceV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FuturesAccountBalanceV2Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => FuturesAccountBalanceV2Params::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.futures_account_balance_v2(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn futures_account_balance_v3(args: FuturesAccountBalanceV3Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FuturesAccountBalanceV3Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FuturesAccountBalanceV3Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => FuturesAccountBalanceV3Params::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.futures_account_balance_v3(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn futures_account_configuration(
    args: FuturesAccountConfigurationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FuturesAccountConfigurationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<FuturesAccountConfigurationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => FuturesAccountConfigurationParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.futures_account_configuration(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn futures_trading_quantitative_rules_indicators(
    args: FuturesTradingQuantitativeRulesIndicatorsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FuturesTradingQuantitativeRulesIndicatorsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FuturesTradingQuantitativeRulesIndicatorsParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => FuturesTradingQuantitativeRulesIndicatorsParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .futures_trading_quantitative_rules_indicators(params)
        .await?;

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

async fn get_current_multi_assets_mode(
    args: GetCurrentMultiAssetsModeArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCurrentMultiAssetsModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetCurrentMultiAssetsModeParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetCurrentMultiAssetsModeParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_current_multi_assets_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_current_position_mode(args: GetCurrentPositionModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCurrentPositionModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetCurrentPositionModeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetCurrentPositionModeParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_current_position_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_download_id_for_futures_order_history(
    mut args: GetDownloadIdForFuturesOrderHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDownloadIdForFuturesOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDownloadIdForFuturesOrderHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
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
                }
                GetDownloadIdForFuturesOrderHistoryParams::builder(
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_download_id_for_futures_order_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_download_id_for_futures_trade_history(
    mut args: GetDownloadIdForFuturesTradeHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDownloadIdForFuturesTradeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDownloadIdForFuturesTradeHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
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
                }
                GetDownloadIdForFuturesTradeHistoryParams::builder(
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_download_id_for_futures_trade_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_download_id_for_futures_transaction_history(
    mut args: GetDownloadIdForFuturesTransactionHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDownloadIdForFuturesTransactionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDownloadIdForFuturesTransactionHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
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
                }
                GetDownloadIdForFuturesTransactionHistoryParams::builder(
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_download_id_for_futures_transaction_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_futures_order_history_download_link_by_id(
    mut args: GetFuturesOrderHistoryDownloadLinkByIdArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFuturesOrderHistoryDownloadLinkByIdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFuturesOrderHistoryDownloadLinkByIdParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.download_id.is_none() {
                        let download_id: String = Input::new()
                            .with_prompt("Please enter the download_id name")
                            .interact_text()?;

                        args.download_id = Some(download_id);
                    }
                }
                GetFuturesOrderHistoryDownloadLinkByIdParams::builder(
                    args.download_id
                        .ok_or_else(|| anyhow::anyhow!("download_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_futures_order_history_download_link_by_id(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_futures_trade_download_link_by_id(
    mut args: GetFuturesTradeDownloadLinkByIdArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFuturesTradeDownloadLinkByIdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFuturesTradeDownloadLinkByIdParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.download_id.is_none() {
                        let download_id: String = Input::new()
                            .with_prompt("Please enter the download_id name")
                            .interact_text()?;

                        args.download_id = Some(download_id);
                    }
                }
                GetFuturesTradeDownloadLinkByIdParams::builder(
                    args.download_id
                        .ok_or_else(|| anyhow::anyhow!("download_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_futures_trade_download_link_by_id(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_futures_transaction_history_download_link_by_id(
    mut args: GetFuturesTransactionHistoryDownloadLinkByIdArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFuturesTransactionHistoryDownloadLinkByIdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFuturesTransactionHistoryDownloadLinkByIdParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.download_id.is_none() {
                        let download_id: String = Input::new()
                            .with_prompt("Please enter the download_id name")
                            .interact_text()?;

                        args.download_id = Some(download_id);
                    }
                }
                GetFuturesTransactionHistoryDownloadLinkByIdParams::builder(
                    args.download_id
                        .ok_or_else(|| anyhow::anyhow!("download_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_futures_transaction_history_download_link_by_id(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_income_history(args: GetIncomeHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetIncomeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetIncomeHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetIncomeHistoryParams::builder()
                .symbol(args.symbol)
                .income_type(args.income_type)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .page(args.page)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_income_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn notional_and_leverage_brackets(
    args: NotionalAndLeverageBracketsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NotionalAndLeverageBracketsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<NotionalAndLeverageBracketsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => NotionalAndLeverageBracketsParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.notional_and_leverage_brackets(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_user_rate_limit(args: QueryUserRateLimitArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUserRateLimitParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryUserRateLimitParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryUserRateLimitParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_user_rate_limit(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn symbol_configuration(args: SymbolConfigurationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SymbolConfigurationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SymbolConfigurationParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => SymbolConfigurationParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.symbol_configuration(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn toggle_bnb_burn_on_futures_trade(
    mut args: ToggleBnbBurnOnFuturesTradeArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ToggleBnbBurnOnFuturesTradeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ToggleBnbBurnOnFuturesTradeParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.fee_burn.is_none() {
                        let fee_burn: String = Input::new()
                            .with_prompt("Please enter the fee_burn name")
                            .interact_text()?;

                        args.fee_burn = Some(fee_burn);
                    }
                }
                ToggleBnbBurnOnFuturesTradeParams::builder(
                    args.fee_burn
                        .ok_or_else(|| anyhow::anyhow!("fee_burn is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.toggle_bnb_burn_on_futures_trade(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn user_commission_rate(mut args: UserCommissionRateArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UserCommissionRateParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UserCommissionRateParams>(json).ok_or_else(|| {
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
                }
                UserCommissionRateParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.user_commission_rate(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn accept_the_offered_quote(mut args: AcceptTheOfferedQuoteArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AcceptTheOfferedQuoteParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AcceptTheOfferedQuoteParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.quote_id.is_none() {
                        let quote_id: String = Input::new()
                            .with_prompt("Please enter the quote_id name")
                            .interact_text()?;

                        args.quote_id = Some(quote_id);
                    }
                }
                AcceptTheOfferedQuoteParams::builder(
                    args.quote_id
                        .ok_or_else(|| anyhow::anyhow!("quote_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.accept_the_offered_quote(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn list_all_convert_pairs(args: ListAllConvertPairsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<ListAllConvertPairsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ListAllConvertPairsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ListAllConvertPairsParams::builder()
                .from_asset(args.from_asset)
                .to_asset(args.to_asset)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.list_all_convert_pairs(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_status(args: OrderStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => OrderStatusParams::builder()
                .order_id(args.order_id)
                .quote_id(args.quote_id)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.order_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn send_quote_request(mut args: SendQuoteRequestArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SendQuoteRequestParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SendQuoteRequestParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.from_asset.is_none() {
                        let from_asset: String = Input::new()
                            .with_prompt("Please enter the from_asset name")
                            .interact_text()?;

                        args.from_asset = Some(from_asset);
                    }
                    if args.to_asset.is_none() {
                        let to_asset: String = Input::new()
                            .with_prompt("Please enter the to_asset name")
                            .interact_text()?;

                        args.to_asset = Some(to_asset);
                    }
                }
                SendQuoteRequestParams::builder(
                    args.from_asset
                        .ok_or_else(|| anyhow::anyhow!("from_asset is required"))?,
                    args.to_asset
                        .ok_or_else(|| anyhow::anyhow!("to_asset is required"))?,
                )
                .from_amount(args.from_amount)
                .to_amount(args.to_amount)
                .valid_time(args.valid_time)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.send_quote_request(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn adl_risk(args: AdlRiskArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<AdlRiskParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AdlRiskParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AdlRiskParams::builder().symbol(args.symbol).build()?,
        },
    };

    // Make the API call
    let response = rest_client.adl_risk(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn asset_index(args: AssetIndexArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<AssetIndexParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AssetIndexParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AssetIndexParams::builder().symbol(args.symbol).build()?,
        },
    };

    // Make the API call
    let response = rest_client.asset_index(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn basis(mut args: BasisArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<BasisParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BasisParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.pair.is_none() {
                        let pair: String = Input::new()
                            .with_prompt("Please enter the pair name")
                            .interact_text()?;

                        args.pair = Some(pair);
                    }
                    if args.contract_type.is_none() {
                        let options = vec![
                            ("PERPETUAL", BasisContractTypeEnum::Perpetual),
                            ("CURRENT_QUARTER", BasisContractTypeEnum::CurrentQuarter),
                            ("NEXT_QUARTER", BasisContractTypeEnum::NextQuarter),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the contract_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.contract_type = Some(selected);
                    }
                    if args.period.is_none() {
                        let options = vec![
                            ("5m", BasisPeriodEnum::Period5m),
                            ("15m", BasisPeriodEnum::Period15m),
                            ("30m", BasisPeriodEnum::Period30m),
                            ("1h", BasisPeriodEnum::Period1h),
                            ("2h", BasisPeriodEnum::Period2h),
                            ("4h", BasisPeriodEnum::Period4h),
                            ("6h", BasisPeriodEnum::Period6h),
                            ("12h", BasisPeriodEnum::Period12h),
                            ("1d", BasisPeriodEnum::Period1d),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the period")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.period = Some(selected);
                    }
                }
                BasisParams::builder(
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
                    args.contract_type
                        .ok_or_else(|| anyhow::anyhow!("contract_type is required"))?,
                    args.period
                        .ok_or_else(|| anyhow::anyhow!("period is required"))?,
                )
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.basis(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn check_server_time(args: CheckServerTimeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.check_server_time().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn composite_index_symbol_information(
    args: CompositeIndexSymbolInformationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<CompositeIndexSymbolInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CompositeIndexSymbolInformationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => CompositeIndexSymbolInformationParams::builder()
                .symbol(args.symbol)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .composite_index_symbol_information(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn compressed_aggregate_trades_list(
    mut args: CompressedAggregateTradesListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<CompressedAggregateTradesListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CompressedAggregateTradesListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                CompressedAggregateTradesListParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .from_id(args.from_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.compressed_aggregate_trades_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn continuous_contract_kline_candlestick_data(
    mut args: ContinuousContractKlineCandlestickDataArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<ContinuousContractKlineCandlestickDataParams>() {
        Some(params) => params,
        None => {
            match args.json {
                Some(json) => read_json_as::<ContinuousContractKlineCandlestickDataParams>(json)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                    })?,
                None => {
                    if args.interactive {
                        if args.pair.is_none() {
                            let pair: String = Input::new()
                                .with_prompt("Please enter the pair name")
                                .interact_text()?;

                            args.pair = Some(pair);
                        }
                        if args.contract_type.is_none() {
                            let options = vec![
                        ("PERPETUAL", ContinuousContractKlineCandlestickDataContractTypeEnum::Perpetual),
                        ("CURRENT_QUARTER", ContinuousContractKlineCandlestickDataContractTypeEnum::CurrentQuarter),
                        ("NEXT_QUARTER", ContinuousContractKlineCandlestickDataContractTypeEnum::NextQuarter),
                        ("TRADIFI_PERPETUAL", ContinuousContractKlineCandlestickDataContractTypeEnum::TradifiPerpetual),
                    ];

                            let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                            let selected = Select::new()
                                .with_prompt("Please select the contract_type")
                                .items(&labels)
                                .default(0)
                                .interact()?;

                            let selected = options[selected].1.clone();

                            println!("Selected option: {:?}", selected);

                            args.contract_type = Some(selected);
                        }
                        if args.interval.is_none() {
                            let options =
                                vec![
                        ("1m", ContinuousContractKlineCandlestickDataIntervalEnum::Interval1m),
                        ("3m", ContinuousContractKlineCandlestickDataIntervalEnum::Interval3m),
                        ("5m", ContinuousContractKlineCandlestickDataIntervalEnum::Interval5m),
                        ("15m", ContinuousContractKlineCandlestickDataIntervalEnum::Interval15m),
                        ("30m", ContinuousContractKlineCandlestickDataIntervalEnum::Interval30m),
                        ("1h", ContinuousContractKlineCandlestickDataIntervalEnum::Interval1h),
                        ("2h", ContinuousContractKlineCandlestickDataIntervalEnum::Interval2h),
                        ("4h", ContinuousContractKlineCandlestickDataIntervalEnum::Interval4h),
                        ("6h", ContinuousContractKlineCandlestickDataIntervalEnum::Interval6h),
                        ("8h", ContinuousContractKlineCandlestickDataIntervalEnum::Interval8h),
                        ("12h", ContinuousContractKlineCandlestickDataIntervalEnum::Interval12h),
                        ("1d", ContinuousContractKlineCandlestickDataIntervalEnum::Interval1d),
                        ("3d", ContinuousContractKlineCandlestickDataIntervalEnum::Interval3d),
                        ("1w", ContinuousContractKlineCandlestickDataIntervalEnum::Interval1w),
                        ("1M", ContinuousContractKlineCandlestickDataIntervalEnum::Interval1m),
                    ];

                            let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                            let selected = Select::new()
                                .with_prompt("Please select the interval")
                                .items(&labels)
                                .default(0)
                                .interact()?;

                            let selected = options[selected].1.clone();

                            println!("Selected option: {:?}", selected);

                            args.interval = Some(selected);
                        }
                    }
                    ContinuousContractKlineCandlestickDataParams::builder(
                        args.pair
                            .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
                        args.contract_type
                            .ok_or_else(|| anyhow::anyhow!("contract_type is required"))?,
                        args.interval
                            .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                    )
                    .start_time(args.start_time)
                    .end_time(args.end_time)
                    .limit(args.limit)
                    .build()?
                }
            }
        }
    };

    // Make the API call
    let response = rest_client
        .continuous_contract_kline_candlestick_data(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn exchange_information(args: ExchangeInformationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.exchange_information().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_funding_rate_history(args: GetFundingRateHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetFundingRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFundingRateHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetFundingRateHistoryParams::builder()
                .symbol(args.symbol)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_funding_rate_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_funding_rate_info(args: GetFundingRateInfoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.get_funding_rate_info().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn index_price_kline_candlestick_data(
    mut args: IndexPriceKlineCandlestickDataArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<IndexPriceKlineCandlestickDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<IndexPriceKlineCandlestickDataParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.pair.is_none() {
                        let pair: String = Input::new()
                            .with_prompt("Please enter the pair name")
                            .interact_text()?;

                        args.pair = Some(pair);
                    }
                    if args.interval.is_none() {
                        let options = vec![
                            ("1m", IndexPriceKlineCandlestickDataIntervalEnum::Interval1m),
                            ("3m", IndexPriceKlineCandlestickDataIntervalEnum::Interval3m),
                            ("5m", IndexPriceKlineCandlestickDataIntervalEnum::Interval5m),
                            (
                                "15m",
                                IndexPriceKlineCandlestickDataIntervalEnum::Interval15m,
                            ),
                            (
                                "30m",
                                IndexPriceKlineCandlestickDataIntervalEnum::Interval30m,
                            ),
                            ("1h", IndexPriceKlineCandlestickDataIntervalEnum::Interval1h),
                            ("2h", IndexPriceKlineCandlestickDataIntervalEnum::Interval2h),
                            ("4h", IndexPriceKlineCandlestickDataIntervalEnum::Interval4h),
                            ("6h", IndexPriceKlineCandlestickDataIntervalEnum::Interval6h),
                            ("8h", IndexPriceKlineCandlestickDataIntervalEnum::Interval8h),
                            (
                                "12h",
                                IndexPriceKlineCandlestickDataIntervalEnum::Interval12h,
                            ),
                            ("1d", IndexPriceKlineCandlestickDataIntervalEnum::Interval1d),
                            ("3d", IndexPriceKlineCandlestickDataIntervalEnum::Interval3d),
                            ("1w", IndexPriceKlineCandlestickDataIntervalEnum::Interval1w),
                            ("1M", IndexPriceKlineCandlestickDataIntervalEnum::Interval1m),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the interval")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.interval = Some(selected);
                    }
                }
                IndexPriceKlineCandlestickDataParams::builder(
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
                    args.interval
                        .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .index_price_kline_candlestick_data(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn kline_candlestick_data(mut args: KlineCandlestickDataArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<KlineCandlestickDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<KlineCandlestickDataParams>(json).ok_or_else(|| {
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
                    if args.interval.is_none() {
                        let options = vec![
                            ("1m", KlineCandlestickDataIntervalEnum::Interval1m),
                            ("3m", KlineCandlestickDataIntervalEnum::Interval3m),
                            ("5m", KlineCandlestickDataIntervalEnum::Interval5m),
                            ("15m", KlineCandlestickDataIntervalEnum::Interval15m),
                            ("30m", KlineCandlestickDataIntervalEnum::Interval30m),
                            ("1h", KlineCandlestickDataIntervalEnum::Interval1h),
                            ("2h", KlineCandlestickDataIntervalEnum::Interval2h),
                            ("4h", KlineCandlestickDataIntervalEnum::Interval4h),
                            ("6h", KlineCandlestickDataIntervalEnum::Interval6h),
                            ("8h", KlineCandlestickDataIntervalEnum::Interval8h),
                            ("12h", KlineCandlestickDataIntervalEnum::Interval12h),
                            ("1d", KlineCandlestickDataIntervalEnum::Interval1d),
                            ("3d", KlineCandlestickDataIntervalEnum::Interval3d),
                            ("1w", KlineCandlestickDataIntervalEnum::Interval1w),
                            ("1M", KlineCandlestickDataIntervalEnum::Interval1m),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the interval")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.interval = Some(selected);
                    }
                }
                KlineCandlestickDataParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.interval
                        .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.kline_candlestick_data(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn long_short_ratio(mut args: LongShortRatioArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<LongShortRatioParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<LongShortRatioParams>(json).ok_or_else(|| {
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
                    if args.period.is_none() {
                        let options = vec![
                            ("5m", LongShortRatioPeriodEnum::Period5m),
                            ("15m", LongShortRatioPeriodEnum::Period15m),
                            ("30m", LongShortRatioPeriodEnum::Period30m),
                            ("1h", LongShortRatioPeriodEnum::Period1h),
                            ("2h", LongShortRatioPeriodEnum::Period2h),
                            ("4h", LongShortRatioPeriodEnum::Period4h),
                            ("6h", LongShortRatioPeriodEnum::Period6h),
                            ("12h", LongShortRatioPeriodEnum::Period12h),
                            ("1d", LongShortRatioPeriodEnum::Period1d),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the period")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.period = Some(selected);
                    }
                }
                LongShortRatioParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.period
                        .ok_or_else(|| anyhow::anyhow!("period is required"))?,
                )
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.long_short_ratio(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn mark_price(args: MarkPriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<MarkPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarkPriceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => MarkPriceParams::builder().symbol(args.symbol).build()?,
        },
    };

    // Make the API call
    let response = rest_client.mark_price(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn mark_price_kline_candlestick_data(
    mut args: MarkPriceKlineCandlestickDataArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<MarkPriceKlineCandlestickDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<MarkPriceKlineCandlestickDataParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.interval.is_none() {
                        let options = vec![
                            ("1m", MarkPriceKlineCandlestickDataIntervalEnum::Interval1m),
                            ("3m", MarkPriceKlineCandlestickDataIntervalEnum::Interval3m),
                            ("5m", MarkPriceKlineCandlestickDataIntervalEnum::Interval5m),
                            (
                                "15m",
                                MarkPriceKlineCandlestickDataIntervalEnum::Interval15m,
                            ),
                            (
                                "30m",
                                MarkPriceKlineCandlestickDataIntervalEnum::Interval30m,
                            ),
                            ("1h", MarkPriceKlineCandlestickDataIntervalEnum::Interval1h),
                            ("2h", MarkPriceKlineCandlestickDataIntervalEnum::Interval2h),
                            ("4h", MarkPriceKlineCandlestickDataIntervalEnum::Interval4h),
                            ("6h", MarkPriceKlineCandlestickDataIntervalEnum::Interval6h),
                            ("8h", MarkPriceKlineCandlestickDataIntervalEnum::Interval8h),
                            (
                                "12h",
                                MarkPriceKlineCandlestickDataIntervalEnum::Interval12h,
                            ),
                            ("1d", MarkPriceKlineCandlestickDataIntervalEnum::Interval1d),
                            ("3d", MarkPriceKlineCandlestickDataIntervalEnum::Interval3d),
                            ("1w", MarkPriceKlineCandlestickDataIntervalEnum::Interval1w),
                            ("1M", MarkPriceKlineCandlestickDataIntervalEnum::Interval1m),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the interval")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.interval = Some(selected);
                    }
                }
                MarkPriceKlineCandlestickDataParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.interval
                        .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .mark_price_kline_candlestick_data(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn old_trades_lookup(mut args: OldTradesLookupArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<OldTradesLookupParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OldTradesLookupParams>(json).ok_or_else(|| {
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
                }
                OldTradesLookupParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .limit(args.limit)
                .from_id(args.from_id)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.old_trades_lookup(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn open_interest(mut args: OpenInterestArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<OpenInterestParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OpenInterestParams>(json).ok_or_else(|| {
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
                }
                OpenInterestParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.open_interest(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn open_interest_statistics(mut args: OpenInterestStatisticsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<OpenInterestStatisticsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OpenInterestStatisticsParams>(json).ok_or_else(|| {
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
                    if args.period.is_none() {
                        let options = vec![
                            ("5m", OpenInterestStatisticsPeriodEnum::Period5m),
                            ("15m", OpenInterestStatisticsPeriodEnum::Period15m),
                            ("30m", OpenInterestStatisticsPeriodEnum::Period30m),
                            ("1h", OpenInterestStatisticsPeriodEnum::Period1h),
                            ("2h", OpenInterestStatisticsPeriodEnum::Period2h),
                            ("4h", OpenInterestStatisticsPeriodEnum::Period4h),
                            ("6h", OpenInterestStatisticsPeriodEnum::Period6h),
                            ("12h", OpenInterestStatisticsPeriodEnum::Period12h),
                            ("1d", OpenInterestStatisticsPeriodEnum::Period1d),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the period")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.period = Some(selected);
                    }
                }
                OpenInterestStatisticsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.period
                        .ok_or_else(|| anyhow::anyhow!("period is required"))?,
                )
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.open_interest_statistics(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_book(mut args: OrderBookArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<OrderBookParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderBookParams>(json).ok_or_else(|| {
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
                }
                OrderBookParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_book(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn premium_index_kline_data(mut args: PremiumIndexKlineDataArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<PremiumIndexKlineDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PremiumIndexKlineDataParams>(json).ok_or_else(|| {
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
                    if args.interval.is_none() {
                        let options = vec![
                            ("1m", PremiumIndexKlineDataIntervalEnum::Interval1m),
                            ("3m", PremiumIndexKlineDataIntervalEnum::Interval3m),
                            ("5m", PremiumIndexKlineDataIntervalEnum::Interval5m),
                            ("15m", PremiumIndexKlineDataIntervalEnum::Interval15m),
                            ("30m", PremiumIndexKlineDataIntervalEnum::Interval30m),
                            ("1h", PremiumIndexKlineDataIntervalEnum::Interval1h),
                            ("2h", PremiumIndexKlineDataIntervalEnum::Interval2h),
                            ("4h", PremiumIndexKlineDataIntervalEnum::Interval4h),
                            ("6h", PremiumIndexKlineDataIntervalEnum::Interval6h),
                            ("8h", PremiumIndexKlineDataIntervalEnum::Interval8h),
                            ("12h", PremiumIndexKlineDataIntervalEnum::Interval12h),
                            ("1d", PremiumIndexKlineDataIntervalEnum::Interval1d),
                            ("3d", PremiumIndexKlineDataIntervalEnum::Interval3d),
                            ("1w", PremiumIndexKlineDataIntervalEnum::Interval1w),
                            ("1M", PremiumIndexKlineDataIntervalEnum::Interval1m),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the interval")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.interval = Some(selected);
                    }
                }
                PremiumIndexKlineDataParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.interval
                        .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.premium_index_kline_data(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn quarterly_contract_settlement_price(
    mut args: QuarterlyContractSettlementPriceArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<QuarterlyContractSettlementPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QuarterlyContractSettlementPriceParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.pair.is_none() {
                        let pair: String = Input::new()
                            .with_prompt("Please enter the pair name")
                            .interact_text()?;

                        args.pair = Some(pair);
                    }
                }
                QuarterlyContractSettlementPriceParams::builder(
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .quarterly_contract_settlement_price(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_index_price_constituents(
    mut args: QueryIndexPriceConstituentsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<QueryIndexPriceConstituentsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryIndexPriceConstituentsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                QueryIndexPriceConstituentsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_index_price_constituents(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_insurance_fund_balance_snapshot(
    args: QueryInsuranceFundBalanceSnapshotArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<QueryInsuranceFundBalanceSnapshotParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryInsuranceFundBalanceSnapshotParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryInsuranceFundBalanceSnapshotParams::builder()
                .symbol(args.symbol)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_insurance_fund_balance_snapshot(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn recent_trades_list(mut args: RecentTradesListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<RecentTradesListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RecentTradesListParams>(json).ok_or_else(|| {
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
                }
                RecentTradesListParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.recent_trades_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn rpi_order_book(mut args: RpiOrderBookArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<RpiOrderBookParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RpiOrderBookParams>(json).ok_or_else(|| {
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
                }
                RpiOrderBookParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.rpi_order_book(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn symbol_order_book_ticker(args: SymbolOrderBookTickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<SymbolOrderBookTickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SymbolOrderBookTickerParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => SymbolOrderBookTickerParams::builder()
                .symbol(args.symbol)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.symbol_order_book_ticker(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn symbol_price_ticker(args: SymbolPriceTickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<SymbolPriceTickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SymbolPriceTickerParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => SymbolPriceTickerParams::builder()
                .symbol(args.symbol)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.symbol_price_ticker(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn symbol_price_ticker_v2(args: SymbolPriceTickerV2Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<SymbolPriceTickerV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SymbolPriceTickerV2Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => SymbolPriceTickerV2Params::builder()
                .symbol(args.symbol)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.symbol_price_ticker_v2(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn taker_buy_sell_volume(mut args: TakerBuySellVolumeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<TakerBuySellVolumeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TakerBuySellVolumeParams>(json).ok_or_else(|| {
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
                    if args.period.is_none() {
                        let options = vec![
                            ("5m", TakerBuySellVolumePeriodEnum::Period5m),
                            ("15m", TakerBuySellVolumePeriodEnum::Period15m),
                            ("30m", TakerBuySellVolumePeriodEnum::Period30m),
                            ("1h", TakerBuySellVolumePeriodEnum::Period1h),
                            ("2h", TakerBuySellVolumePeriodEnum::Period2h),
                            ("4h", TakerBuySellVolumePeriodEnum::Period4h),
                            ("6h", TakerBuySellVolumePeriodEnum::Period6h),
                            ("12h", TakerBuySellVolumePeriodEnum::Period12h),
                            ("1d", TakerBuySellVolumePeriodEnum::Period1d),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the period")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.period = Some(selected);
                    }
                }
                TakerBuySellVolumeParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.period
                        .ok_or_else(|| anyhow::anyhow!("period is required"))?,
                )
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.taker_buy_sell_volume(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn test_connectivity(args: TestConnectivityArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.test_connectivity().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn ticker24hr_price_change_statistics(
    args: Ticker24hrPriceChangeStatisticsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<Ticker24hrPriceChangeStatisticsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<Ticker24hrPriceChangeStatisticsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => Ticker24hrPriceChangeStatisticsParams::builder()
                .symbol(args.symbol)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .ticker24hr_price_change_statistics(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn top_trader_long_short_ratio_accounts(
    mut args: TopTraderLongShortRatioAccountsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<TopTraderLongShortRatioAccountsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<TopTraderLongShortRatioAccountsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.period.is_none() {
                        let options = vec![
                            ("5m", TopTraderLongShortRatioAccountsPeriodEnum::Period5m),
                            ("15m", TopTraderLongShortRatioAccountsPeriodEnum::Period15m),
                            ("30m", TopTraderLongShortRatioAccountsPeriodEnum::Period30m),
                            ("1h", TopTraderLongShortRatioAccountsPeriodEnum::Period1h),
                            ("2h", TopTraderLongShortRatioAccountsPeriodEnum::Period2h),
                            ("4h", TopTraderLongShortRatioAccountsPeriodEnum::Period4h),
                            ("6h", TopTraderLongShortRatioAccountsPeriodEnum::Period6h),
                            ("12h", TopTraderLongShortRatioAccountsPeriodEnum::Period12h),
                            ("1d", TopTraderLongShortRatioAccountsPeriodEnum::Period1d),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the period")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.period = Some(selected);
                    }
                }
                TopTraderLongShortRatioAccountsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.period
                        .ok_or_else(|| anyhow::anyhow!("period is required"))?,
                )
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .top_trader_long_short_ratio_accounts(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn top_trader_long_short_ratio_positions(
    mut args: TopTraderLongShortRatioPositionsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<TopTraderLongShortRatioPositionsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<TopTraderLongShortRatioPositionsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.period.is_none() {
                        let options = vec![
                            ("5m", TopTraderLongShortRatioPositionsPeriodEnum::Period5m),
                            ("15m", TopTraderLongShortRatioPositionsPeriodEnum::Period15m),
                            ("30m", TopTraderLongShortRatioPositionsPeriodEnum::Period30m),
                            ("1h", TopTraderLongShortRatioPositionsPeriodEnum::Period1h),
                            ("2h", TopTraderLongShortRatioPositionsPeriodEnum::Period2h),
                            ("4h", TopTraderLongShortRatioPositionsPeriodEnum::Period4h),
                            ("6h", TopTraderLongShortRatioPositionsPeriodEnum::Period6h),
                            ("12h", TopTraderLongShortRatioPositionsPeriodEnum::Period12h),
                            ("1d", TopTraderLongShortRatioPositionsPeriodEnum::Period1d),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the period")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.period = Some(selected);
                    }
                }
                TopTraderLongShortRatioPositionsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.period
                        .ok_or_else(|| anyhow::anyhow!("period is required"))?,
                )
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .top_trader_long_short_ratio_positions(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn trading_schedule(args: TradingScheduleArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.trading_schedule().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn classic_portfolio_margin_account_information(
    mut args: ClassicPortfolioMarginAccountInformationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ClassicPortfolioMarginAccountInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ClassicPortfolioMarginAccountInformationParams>(json)
                .ok_or_else(|| {
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
                }
                ClassicPortfolioMarginAccountInformationParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .classic_portfolio_margin_account_information(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn account_trade_list(mut args: AccountTradeListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountTradeListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountTradeListParams>(json).ok_or_else(|| {
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
                }
                AccountTradeListParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
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
    let response = rest_client.account_trade_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn all_orders(mut args: AllOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AllOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllOrdersParams>(json).ok_or_else(|| {
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
                }
                AllOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
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
    let response = rest_client.all_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn auto_cancel_all_open_orders(mut args: AutoCancelAllOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AutoCancelAllOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AutoCancelAllOpenOrdersParams>(json).ok_or_else(|| {
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
                    if args.countdown_time.is_none() {
                        let countdown_time: i64 = Input::new()
                            .with_prompt("Please enter the countdown_time name")
                            .interact_text()?;

                        args.countdown_time = Some(countdown_time);
                    }
                }
                AutoCancelAllOpenOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.countdown_time
                        .ok_or_else(|| anyhow::anyhow!("countdown_time is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.auto_cancel_all_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_algo_order(args: CancelAlgoOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAlgoOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAlgoOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => CancelAlgoOrderParams::builder()
                .algo_id(args.algo_id)
                .client_algo_id(args.client_algo_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.cancel_algo_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_all_algo_open_orders(mut args: CancelAllAlgoOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllAlgoOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAllAlgoOpenOrdersParams>(json).ok_or_else(|| {
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
                }
                CancelAllAlgoOpenOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_all_algo_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_all_open_orders(mut args: CancelAllOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAllOpenOrdersParams>(json).ok_or_else(|| {
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
                }
                CancelAllOpenOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_all_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_multiple_orders(mut args: CancelMultipleOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelMultipleOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelMultipleOrdersParams>(json).ok_or_else(|| {
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
                }
                CancelMultipleOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_id_list(args.order_id_list)
                .orig_client_order_id_list(args.orig_client_order_id_list)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_multiple_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_order(mut args: CancelOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelOrderParams>(json).ok_or_else(|| {
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
                }
                CancelOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_initial_leverage(mut args: ChangeInitialLeverageArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeInitialLeverageParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ChangeInitialLeverageParams>(json).ok_or_else(|| {
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
                    if args.leverage.is_none() {
                        let leverage: i64 = Input::new()
                            .with_prompt("Please enter the leverage name")
                            .interact_text()?;

                        args.leverage = Some(leverage);
                    }
                }
                ChangeInitialLeverageParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.leverage
                        .ok_or_else(|| anyhow::anyhow!("leverage is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_initial_leverage(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_margin_type(mut args: ChangeMarginTypeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeMarginTypeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ChangeMarginTypeParams>(json).ok_or_else(|| {
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
                    if args.margin_type.is_none() {
                        let options = vec![
                            ("ISOLATED", ChangeMarginTypeMarginTypeEnum::Isolated),
                            ("CROSSED", ChangeMarginTypeMarginTypeEnum::Crossed),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the margin_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.margin_type = Some(selected);
                    }
                }
                ChangeMarginTypeParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.margin_type
                        .ok_or_else(|| anyhow::anyhow!("margin_type is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_margin_type(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_multi_assets_mode(mut args: ChangeMultiAssetsModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeMultiAssetsModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ChangeMultiAssetsModeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.multi_assets_margin.is_none() {
                        let multi_assets_margin: String = Input::new()
                            .with_prompt("Please enter the multi_assets_margin name")
                            .interact_text()?;

                        args.multi_assets_margin = Some(multi_assets_margin);
                    }
                }
                ChangeMultiAssetsModeParams::builder(
                    args.multi_assets_margin
                        .ok_or_else(|| anyhow::anyhow!("multi_assets_margin is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_multi_assets_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_position_mode(mut args: ChangePositionModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangePositionModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ChangePositionModeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.dual_side_position.is_none() {
                        let dual_side_position: String = Input::new()
                            .with_prompt("Please enter the dual_side_position name")
                            .interact_text()?;

                        args.dual_side_position = Some(dual_side_position);
                    }
                }
                ChangePositionModeParams::builder(
                    args.dual_side_position
                        .ok_or_else(|| anyhow::anyhow!("dual_side_position is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_position_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn current_all_algo_open_orders(
    args: CurrentAllAlgoOpenOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CurrentAllAlgoOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CurrentAllAlgoOpenOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => CurrentAllAlgoOpenOrdersParams::builder()
                .algo_type(args.algo_type)
                .symbol(args.symbol)
                .algo_id(args.algo_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.current_all_algo_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn current_all_open_orders(args: CurrentAllOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CurrentAllOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CurrentAllOpenOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => CurrentAllOpenOrdersParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.current_all_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn futures_tradfi_perps_contract(
    args: FuturesTradfiPerpsContractArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FuturesTradfiPerpsContractParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<FuturesTradfiPerpsContractParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => FuturesTradfiPerpsContractParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.futures_tradfi_perps_contract(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_order_modify_history(mut args: GetOrderModifyHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOrderModifyHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOrderModifyHistoryParams>(json).ok_or_else(|| {
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
                }
                GetOrderModifyHistoryParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_order_modify_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_position_margin_change_history(
    mut args: GetPositionMarginChangeHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetPositionMarginChangeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetPositionMarginChangeHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                GetPositionMarginChangeHistoryParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .r#type(args.r#type)
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
        .get_position_margin_change_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn modify_isolated_position_margin(
    mut args: ModifyIsolatedPositionMarginArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ModifyIsolatedPositionMarginParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ModifyIsolatedPositionMarginParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.r#type.is_none() {
                        let r#type: i32 = Input::new()
                            .with_prompt("Please enter the r#type name")
                            .interact_text()?;

                        args.r#type = Some(r#type);
                    }
                }
                ModifyIsolatedPositionMarginParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .position_side(args.position_side)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.modify_isolated_position_margin(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn modify_multiple_orders(mut args: ModifyMultipleOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ModifyMultipleOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ModifyMultipleOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.batch_orders.is_none() {
                        let batch_orders: String = Input::new()
                            .with_prompt("Please enter the batch_orders name")
                            .interact_text()?;

                        args.batch_orders = Some(batch_orders);
                    }
                }
                ModifyMultipleOrdersParams::builder(serde_json::from_str::<
                    Vec<models::ModifyMultipleOrdersBatchOrdersParameterInner>,
                >(
                    &args
                        .batch_orders
                        .ok_or_else(|| anyhow::anyhow!("batch_orders is required"))?,
                )?)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.modify_multiple_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn modify_order(mut args: ModifyOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ModifyOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ModifyOrderParams>(json).ok_or_else(|| {
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
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", ModifyOrderSideEnum::Buy),
                            ("SELL", ModifyOrderSideEnum::Sell),
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
                            .with_prompt("Please enter the quantity name")
                            .interact_text()?;

                        args.quantity = Some(quantity);
                    }
                    if args.price.is_none() {
                        let price: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the price name")
                            .interact_text()?;

                        args.price = Some(price);
                    }
                }
                ModifyOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                    args.price
                        .ok_or_else(|| anyhow::anyhow!("price is required"))?,
                )
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .price_match(args.price_match)
                .modify_id(args.modify_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.modify_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_algo_order(mut args: NewAlgoOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewAlgoOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewAlgoOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo_type.is_none() {
                        let options = vec![("CONDITIONAL", NewAlgoOrderAlgoTypeEnum::Conditional)];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the algo_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.algo_type = Some(selected);
                    }
                    if args.symbol.is_none() {
                        let symbol: String = Input::new()
                            .with_prompt("Please enter the symbol name")
                            .interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", NewAlgoOrderSideEnum::Buy),
                            ("SELL", NewAlgoOrderSideEnum::Sell),
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
                            ("LIMIT", NewAlgoOrderTypeEnum::Limit),
                            ("MARKET", NewAlgoOrderTypeEnum::Market),
                            ("STOP", NewAlgoOrderTypeEnum::Stop),
                            ("STOP_MARKET", NewAlgoOrderTypeEnum::StopMarket),
                            ("TAKE_PROFIT", NewAlgoOrderTypeEnum::TakeProfit),
                            ("TAKE_PROFIT_MARKET", NewAlgoOrderTypeEnum::TakeProfitMarket),
                            (
                                "TRAILING_STOP_MARKET",
                                NewAlgoOrderTypeEnum::TrailingStopMarket,
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
                NewAlgoOrderParams::builder(
                    args.algo_type
                        .ok_or_else(|| anyhow::anyhow!("algo_type is required"))?,
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .position_side(args.position_side)
                .time_in_force(args.time_in_force)
                .quantity(args.quantity)
                .price(args.price)
                .trigger_price(args.trigger_price)
                .working_type(args.working_type)
                .price_match(args.price_match)
                .close_position(args.close_position)
                .price_protect(args.price_protect)
                .reduce_only(args.reduce_only)
                .activate_price(args.activate_price)
                .callback_rate(args.callback_rate)
                .client_algo_id(args.client_algo_id)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .good_till_date(args.good_till_date)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.new_algo_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_order(mut args: NewOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewOrderParams>(json).ok_or_else(|| {
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
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", NewOrderSideEnum::Buy),
                            ("SELL", NewOrderSideEnum::Sell),
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
                            ("LIMIT", NewOrderTypeEnum::Limit),
                            ("MARKET", NewOrderTypeEnum::Market),
                            ("STOP", NewOrderTypeEnum::Stop),
                            ("STOP_MARKET", NewOrderTypeEnum::StopMarket),
                            ("TAKE_PROFIT", NewOrderTypeEnum::TakeProfit),
                            ("TAKE_PROFIT_MARKET", NewOrderTypeEnum::TakeProfitMarket),
                            ("TRAILING_STOP_MARKET", NewOrderTypeEnum::TrailingStopMarket),
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
                NewOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .position_side(args.position_side)
                .time_in_force(args.time_in_force)
                .reduce_only(args.reduce_only)
                .quantity(args.quantity)
                .price(args.price)
                .new_client_order_id(args.new_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .price_match(args.price_match)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .good_till_date(args.good_till_date)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.new_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn place_multiple_orders(mut args: PlaceMultipleOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PlaceMultipleOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PlaceMultipleOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.batch_orders.is_none() {
                        let batch_orders: String = Input::new()
                            .with_prompt("Please enter the batch_orders name")
                            .interact_text()?;

                        args.batch_orders = Some(batch_orders);
                    }
                }
                PlaceMultipleOrdersParams::builder(serde_json::from_str::<
                    Vec<models::PlaceMultipleOrdersBatchOrdersParameterInner>,
                >(
                    &args
                        .batch_orders
                        .ok_or_else(|| anyhow::anyhow!("batch_orders is required"))?,
                )?)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.place_multiple_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn position_adl_quantile_estimation(
    args: PositionAdlQuantileEstimationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PositionAdlQuantileEstimationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<PositionAdlQuantileEstimationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => PositionAdlQuantileEstimationParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.position_adl_quantile_estimation(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn position_information_v2(args: PositionInformationV2Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PositionInformationV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PositionInformationV2Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => PositionInformationV2Params::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.position_information_v2(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn position_information_v3(args: PositionInformationV3Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PositionInformationV3Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PositionInformationV3Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => PositionInformationV3Params::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.position_information_v3(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_algo_order(args: QueryAlgoOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAlgoOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryAlgoOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryAlgoOrderParams::builder()
                .algo_id(args.algo_id)
                .client_algo_id(args.client_algo_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_algo_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_algo_orders(mut args: QueryAllAlgoOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllAlgoOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryAllAlgoOrdersParams>(json).ok_or_else(|| {
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
                }
                QueryAllAlgoOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .algo_id(args.algo_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_all_algo_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_open_order(mut args: QueryCurrentOpenOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentOpenOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCurrentOpenOrderParams>(json).ok_or_else(|| {
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
                }
                QueryCurrentOpenOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_current_open_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_order(mut args: QueryOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryOrderParams>(json).ok_or_else(|| {
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
                }
                QueryOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn test_order(mut args: TestOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TestOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TestOrderParams>(json).ok_or_else(|| {
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
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", TestOrderSideEnum::Buy),
                            ("SELL", TestOrderSideEnum::Sell),
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
                            ("LIMIT", TestOrderTypeEnum::Limit),
                            ("MARKET", TestOrderTypeEnum::Market),
                            ("STOP", TestOrderTypeEnum::Stop),
                            ("STOP_MARKET", TestOrderTypeEnum::StopMarket),
                            ("TAKE_PROFIT", TestOrderTypeEnum::TakeProfit),
                            ("TAKE_PROFIT_MARKET", TestOrderTypeEnum::TakeProfitMarket),
                            (
                                "TRAILING_STOP_MARKET",
                                TestOrderTypeEnum::TrailingStopMarket,
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
                TestOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .position_side(args.position_side)
                .reduce_only(args.reduce_only)
                .quantity(args.quantity)
                .price(args.price)
                .new_client_order_id(args.new_client_order_id)
                .stop_price(args.stop_price)
                .close_position(args.close_position)
                .activation_price(args.activation_price)
                .callback_rate(args.callback_rate)
                .time_in_force(args.time_in_force)
                .working_type(args.working_type)
                .price_protect(args.price_protect)
                .new_order_resp_type(args.new_order_resp_type)
                .price_match(args.price_match)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .good_till_date(args.good_till_date)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.test_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn users_force_orders(args: UsersForceOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UsersForceOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UsersForceOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => UsersForceOrdersParams::builder()
                .symbol(args.symbol)
                .auto_close_type(args.auto_close_type)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.users_force_orders(params).await?;

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

async fn keepalive_user_data_stream(args: KeepaliveUserDataStreamArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.keepalive_user_data_stream().await?;

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
