use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::{
    DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL,
    DERIVATIVES_TRADING_COIN_FUTURES_REST_API_TESTNET_URL,
};
use binance_sdk::derivatives_trading_coin_futures::DerivativesTradingCoinFuturesRestApi;
use binance_sdk::derivatives_trading_coin_futures::rest_api::{self as models, *};
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("derivatives-trading-coin-futures");

    let client_config =
        get_client_configuration(profile, "derivatives-trading-coin-futures").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "testnet" | "demo" => DERIVATIVES_TRADING_COIN_FUTURES_REST_API_TESTNET_URL.to_string(),
        "prod" => DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL.to_string(),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid api env, valid values: testnet, demo, prod",
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

    Ok(DerivativesTradingCoinFuturesRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct AccountInformationArgs {
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
struct FuturesAccountBalanceArgs {
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Income type."#, long)]
    income_type: Option<GetIncomeHistoryIncomeTypeEnum>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Page number"#, long)]
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
struct NotionalBracketForPairArgs {
    #[arg(help = r#""#, long)]
    pair: Option<String>,
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
struct NotionalBracketForSymbolArgs {
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
struct BasisArgs {
    #[arg(help = r#"Pair."#, long)]
    pair: Option<String>,
    #[arg(help = r#"Contract type."#, long)]
    contract_type: Option<BasisContractTypeEnum>,
    #[arg(help = r#"Period interval."#, long)]
    period: Option<BasisPeriodEnum>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
        help = r#"After CM migration, accepts both CM and UM pair values."#,
        long
    )]
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    contract_type: Option<ContinuousContractKlineCandlestickDataContractTypeEnum>,
    #[arg(help = r#"Interval"#, long)]
    interval: Option<ContinuousContractKlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
struct GetFundingRateHistoryOfPerpetualFuturesArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Timestamp in ms to get funding rate from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding rate until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
struct IndexPriceAndMarkPriceArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
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
struct IndexPriceKlineCandlestickDataArgs {
    #[arg(
        help = r#"After CM migration, accepts both CM and UM pair values."#,
        long
    )]
    pair: Option<String>,
    #[arg(help = r#"Interval"#, long)]
    interval: Option<IndexPriceKlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
    #[arg(help = r#"After CM migration, accepts both CM and UM symbols."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Interval"#, long)]
    interval: Option<KlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
    #[arg(help = r#"BTCUSD"#, long)]
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    period: Option<LongShortRatioPeriodEnum>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
struct MarkPriceKlineCandlestickDataArgs {
    #[arg(help = r#"After CM migration, accepts both CM and UM symbols."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Interval"#, long)]
    interval: Option<MarkPriceKlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
struct OpenInterestStatisticsArgs {
    #[arg(help = r#""#, long)]
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    contract_type: Option<OpenInterestStatisticsContractTypeEnum>,
    #[arg(help = r#""#, long)]
    period: Option<OpenInterestStatisticsPeriodEnum>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Valid limits:[5, 10, 20, 50, 100, 500, 1000]."#, long)]
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
    #[arg(help = r#"After CM migration, accepts both CM and UM symbols."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Interval"#, long)]
    interval: Option<PremiumIndexKlineDataIntervalEnum>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryIndexPriceConstituentsArgs {
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Symbol"#, long)]
    pair: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SymbolPriceTickerArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Pair"#, long)]
    pair: Option<String>,
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
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    contract_type: Option<TakerBuySellVolumeContractTypeEnum>,
    #[arg(help = r#""#, long)]
    period: Option<TakerBuySellVolumePeriodEnum>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Pair"#, long)]
    pair: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TopTraderLongShortRatioAccountsArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    period: Option<TopTraderLongShortRatioAccountsPeriodEnum>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    period: Option<TopTraderLongShortRatioPositionsPeriodEnum>,
    #[arg(help = r#"Maximum number of records to return."#, long)]
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
struct AccountTradeListArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"pair"#, long)]
    pair: Option<String>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<String>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(
        help = r#"Trade id to fetch from. Default gets most recent trades."#,
        long
    )]
    from_id: Option<i64>,
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
struct AllOrdersArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Pair"#, long)]
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
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
struct AutoCancelAllOpenOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"countdown time, 1000 for 1 second. 0 to cancel the timer"#,
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
struct CancelAllOpenOrdersArgs {
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
struct CancelMultipleOrdersArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order IDs to cancel."#, long)]
    order_id_list: Option<Vec<i64>>,
    #[arg(help = r#"Original client order IDs to cancel."#, long)]
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"target initial leverage: int from 1 to 125"#, long)]
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
    #[arg(help = r#"Symbol"#, long)]
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
struct CurrentAllOpenOrdersArgs {
    #[arg(
        help = r#"Symbol. **After CM migration, an invalid `symbol` returns `-1121` (previously a silent `200`).**"#,
        long
    )]
    symbol: Option<String>,
    #[arg(help = r#"Pair"#, long)]
    pair: Option<String>,
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
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
struct GetPositionMarginChangeHistoryArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"1: Add position margin,2: Reduce position margin"#, long)]
    r#type: Option<i64>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
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
struct ModifyIsolatedPositionMarginArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Margin asset"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"1: Add position margin,2: Reduce position margin"#, long)]
    r#type: Option<i64>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent with Hedge Mode."#,
        long
    )]
    position_side: Option<ModifyIsolatedPositionMarginPositionSideEnum>,
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<ModifyOrderSideEnum>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"Order quantity, cannot be sent with `closePosition=true`. **After CM migration, this parameter becomes mandatory** (must be sent together with `price`)."#,
        long
    )]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Order price. **After CM migration, this parameter becomes mandatory** (must be sent together with `quantity`)."#,
        long
    )]
    price: Option<rust_decimal::Decimal>,
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
struct NewOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<NewOrderSideEnum>,
    #[arg(
        help = r#"**After CM migration, stop-type values (`STOP`, `STOP_MARKET`, `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`, `TRAILING_STOP_MARKET`) are no longer accepted by this endpoint and will return `-4120`. Use the new `/dapi/v1/algoOrder` endpoint instead.**"#,
        long
    )]
    r#type: Option<NewOrderTypeEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<NewOrderPositionSideEnum>,
    #[arg(
        help = r#""true" or "false". Cannot be sent in Hedge
Mode; cannot be sent with `closePosition`=`true`(Close-All)"#,
        long
    )]
    reduce_only: Option<NewOrderReduceOnlyEnum>,
    #[arg(
        help = r#"quantity measured by contract number, Cannot be sent with `closePosition`=`true`"#,
        long
    )]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Order price"#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if
not sent. Can only be string following the rule:
`^[\.A-Z\:/a-z0-9_-]{1,36}$`"#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(
        help = r#"Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders."#,
        long
    )]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"`true`, `false`；Close-All,used with `STOP_MARKET` or `TAKE_PROFIT_MARKET`."#,
        long
    )]
    close_position: Option<String>,
    #[arg(
        help = r#"Used with `TRAILING_STOP_MARKET` orders, default as the
latest price(supporting different `workingType`)"#,
        long
    )]
    activation_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Used with `TRAILING_STOP_MARKET` orders, min 0.1, max 10 where 1 for 1%"#,
        long
    )]
    callback_rate: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewOrderTimeInForceEnum>,
    #[arg(
        help = r#"'stopPrice triggered by: "MARK_PRICE", "CONTRACT_PRICE"."#,
        long
    )]
    working_type: Option<NewOrderWorkingTypeEnum>,
    #[arg(
        help = r#""true" or "false". Used with
`STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET`
orders."#,
        long
    )]
    price_protect: Option<NewOrderPriceProtectEnum>,
    #[arg(help = r#""#, long)]
    new_order_resp_type: Option<NewOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order;
Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<NewOrderPriceMatchEnum>,
    #[arg(
        help = r#"`EXPIRE_TAKER`:expire taker order when STP triggers/
`EXPIRE_MAKER`:expire taker order when STP triggers/
`EXPIRE_BOTH`:expire both orders when STP triggers"#,
        long
    )]
    self_trade_prevention_mode: Option<NewOrderSelfTradePreventionModeEnum>,
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
struct PositionInformationArgs {
    #[arg(help = r#""#, long)]
    margin_asset: Option<String>,
    #[arg(help = r#""#, long)]
    pair: Option<String>,
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
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
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
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
struct UsersForceOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    auto_close_type: Option<UsersForceOrdersAutoCloseTypeEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
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
pub enum DerivativesTradingCoinFuturesCommands {
    #[command(
        about = decode_selected_entities(r#"Get current account information.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- for One-way Mode user, the "positions" will only show the "BOTH" positions
- for Hedge Mode user, the "positions" will show "BOTH", "LONG", and "SHORT" positions."#, false),
    )]
    AccountInformation(AccountInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Check futures account balance

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    FuturesAccountBalance(FuturesAccountBalanceArgs),
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
- Request Limitation is 8 times per month, shared by front end download page and rest api
- This endpoint uses the IP rate limit bucket and costs 1000 weight per call. The maximum is 2 calls per minute; the 3rd call within the same minute will trigger a ban.
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForFuturesOrderHistory(GetDownloadIdForFuturesOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get download id for futures trade history

Weight(IP): 1000

Security Type: USER_DATA

Notes:
- Request Limitation is 8 times per month, shared by front end download page and rest api
- This endpoint uses the IP rate limit bucket and costs 1000 weight per call. The maximum is 2 calls per minute; the 3rd call within the same minute will trigger a ban.
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForFuturesTradeHistory(GetDownloadIdForFuturesTradeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get download id for futures transaction history

Weight(IP): 1000

Security Type: USER_DATA

Notes:
- Request Limitation is 8 times per month, shared by front end download page and rest api
- This endpoint uses the IP rate limit bucket and costs 1000 weight per call. The maximum is 2 calls per minute; the 3rd call within the same minute will trigger a ban.
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForFuturesTransactionHistory(GetDownloadIdForFuturesTransactionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get futures order history download link by Id

Weight(IP): 5

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetFuturesOrderHistoryDownloadLinkById(GetFuturesOrderHistoryDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Get futures trade download link by Id

Weight(IP): 5

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetFuturesTradeDownloadLinkById(GetFuturesTradeDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Get futures transaction history download link by Id

Weight(IP): 5

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetFuturesTransactionHistoryDownloadLinkById(GetFuturesTransactionHistoryDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Get income history

Weight(IP): 20

Security Type: USER_DATA

Notes:
- If `incomeType ` is not sent, all kinds of flow will be returned
- "trandId" is unique in the same "incomeType" for a user
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetIncomeHistory(GetIncomeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"**Not recommended to continue using this v1 endpoint**

Get the pair's default notional bracket list, may return ambiguous
values when there have been multiple different `symbol` brackets under
the `pair`, suggest using the following `GET /dapi/v2/leverageBracket`
query instead to get the specific `symbol` notional bracket list.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    NotionalBracketForPair(NotionalBracketForPairArgs),
    #[command(
        about = decode_selected_entities(r#"Get the symbol's notional bracket list.

Weight: 1 (after CM migration: 1 with `symbol` / 2 without `symbol`)

Security Type: USER_DATA"#, false),
    )]
    NotionalBracketForSymbol(NotionalBracketForSymbolArgs),
    #[command(
        about = decode_selected_entities(r#"Query user commission rate

Weight(IP): 20

Security Type: USER_DATA"#, false),
    )]
    UserCommissionRate(UserCommissionRateArgs),
    #[command(
        about = decode_selected_entities(r#"Query basis

Weight(IP): 1

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
        about = decode_selected_entities(r#"Get compressed, aggregate trades. Market trades that fill in 100ms with
the same price and the same taking side will have the quantity
aggregated.

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
        about = decode_selected_entities(r#"Kline/candlestick bars for a specific contract type. Klines are uniquely identified by their open time.

Weight: based on parameter `LIMIT`

| LIMIT | weight |
| --- | --- |
| [1,100) | 1 |
| [100, 500) | 2 |
| [500, 1000] | 5 |
| > 1000 | 10 |

Notes:
- The difference between `startTime` and `endTime` can only be up to 200 days
- Between `startTime` and `endTime`, the most recent `limit` data from `endTime` will be returned:
  - If `startTime` and `endTime` are not sent, current timestamp will be set as `endTime`, and the most recent data will be returned.
  - If `startTime` is sent only, the timestamp of 200 days after `startTime` will be set as `endTime`(up to the current time)
  - If `endTime` is sent only, the timestamp of 200 days before `endTime` will be set as `startTime`"#, false),
    )]
    ContinuousContractKlineCandlestickData(ContinuousContractKlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Current exchange trading rules and symbol information

Weight(IP): 1"#, false),
    )]
    ExchangeInformation(ExchangeInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Get Funding Rate History of Perpetual Futures

Weight(IP): 1

Notes:
- empty array will be returned for delivery symbols."#, false),
    )]
    GetFundingRateHistoryOfPerpetualFutures(GetFundingRateHistoryOfPerpetualFuturesArgs),
    #[command(
        about = decode_selected_entities(r#"Query funding rate info for symbols that had FundingRateCap/FundingRateFloor/fundingIntervalHours adjustment"#, false),
    )]
    GetFundingRateInfo(GetFundingRateInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Query index price and mark price

Weight(IP): 10"#, false),
    )]
    IndexPriceAndMarkPrice(IndexPriceAndMarkPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for the index price of a pair. Klines are uniquely identified by their open time.

Weight: Based on parameter `LIMIT`

| LIMIT | weight |
| --- | --- |
| [1,100) | 1 |
| [100, 500) | 2 |
| [500, 1000] | 5 |
| > 1000 | 10 |

Notes:
- The difference between `startTime` and `endTime` can only be up to 200 days
- Between `startTime` and `endTime`, the most recent `limit` data from `endTime` will be returned:
  - If `startTime` and `endTime` are not sent, current timestamp will be set as `endTime`, and the most recent data will be returned.
  - If `startTime` is sent only, the timestamp of 200 days after `startTime` will be set as `endTime`(up to the current time)
  - If `endTime` is sent only, the timestamp of 200 days before `endTime` will be set as `startTime`"#, false),
    )]
    IndexPriceKlineCandlestickData(IndexPriceKlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.

Weight: Based on parameter `LIMIT`

| LIMIT | weight |
| --- | --- |
| [1,100) | 1 |
| [100, 500) | 2 |
| [500, 1000] | 5 |
| > 1000 | 10 |

Notes:
- The difference between `startTime` and `endTime` can only be up to 200 days
- Between `startTime` and `endTime`, the most recent `limit` data from `endTime` will be returned:
  - If `startTime` and `endTime` are not sent, current timestamp will be set as `endTime`, and the most recent data will be returned.
  - If `startTime` is sent only, the timestamp of 200 days after `startTime` will be set as `endTime`(up to the current time)
  - If `endTime` is sent only, the timestamp of 200 days before `endTime` will be set as `startTime`"#, false),
    )]
    KlineCandlestickData(KlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Query symbol Long/Short Ratio

Weight(IP): 1

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available."#, false),
    )]
    LongShortRatio(LongShortRatioArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for the mark price of a symbol. Klines are uniquely identified by their open time.

Weight: Based on parameter `LIMIT`

| LIMIT | weight |
| --- | --- |
| [1,100) | 1 |
| [100, 500) | 2 |
| [500, 1000] | 5 |
| > 1000 | 10 |

Notes:
- The difference between `startTime` and `endTime` can only be up to 200 days
- Between `startTime` and `endTime`, the most recent `limit` data from `endTime` will be returned:
  - If `startTime` and `endTime` are not sent, current timestamp will be set as `endTime`, and the most recent data will be returned.
  - If `startTime` is sent only, the timestamp of 200 days after `startTime` will be set as `endTime`(up to the current time)
  - If `endTime` is sent only, the timestamp of 200 days before `endTime` will be set as `startTime`"#, false),
    )]
    MarkPriceKlineCandlestickData(MarkPriceKlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Get older market historical trades.

Weight(IP): 200

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
        about = decode_selected_entities(r#"Query open interest stats

Weight(IP): 1

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available."#, false),
    )]
    OpenInterestStatistics(OpenInterestStatisticsArgs),
    #[command(
        about = decode_selected_entities(r#"Query orderbook on specific symbol

Weight: Adjusted based on the limit:

| Limit | Weight |
| ------------ | ------------ |
| 5, 10, 20, 50 | 2 |
| 100 | 5 |
| 500 | 10 |
| 1000 | 20 |"#, false),
    )]
    OrderBook(OrderBookArgs),
    #[command(
        about = decode_selected_entities(r#"Premium index kline bars of a symbol. Klines are uniquely identified by their open time.

Weight: Based on parameter `LIMIT`

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
        about = decode_selected_entities(r#"Query index price constituents

Weight(IP): 1"#, false),
    )]
    QueryIndexPriceConstituents(QueryIndexPriceConstituentsArgs),
    #[command(
        about = decode_selected_entities(r#"Get recent market trades

Weight(IP): 5

Notes:
- Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won't be returned."#, false),
    )]
    RecentTradesList(RecentTradesListArgs),
    #[command(
        about = decode_selected_entities(r#"Best price/qty on the order book for a symbol or symbols.

Weight: **2** for a single symbol, **5** when the symbol parameter is omitted

Notes:
- Symbol and pair cannot be sent together
- If a pair is sent,tickers for all symbols of the pair will be returned
- If either a pair or symbol is sent, tickers for all symbols of all pairs will be returned"#, false),
    )]
    SymbolOrderBookTicker(SymbolOrderBookTickerArgs),
    #[command(
        about = decode_selected_entities(r#"Latest price for a symbol or symbols.

Weight: **1** for a single symbol, **2** when the symbol parameter is omitted

Notes:
- Symbol and pair cannot be sent together
- If a pair is sent,tickers for all symbols of the pair will be returned
- If either a pair or symbol is sent, tickers for all symbols of all pairs will be returned"#, false),
    )]
    SymbolPriceTicker(SymbolPriceTickerArgs),
    #[command(
        about = decode_selected_entities(r#"Taker Buy Volume: the total volume of buy orders filled by takers within
the period.

Taker Sell Volume: the total volume of sell orders filled by takers
within the period.

Weight(IP): 1

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available."#, false),
    )]
    TakerBuySellVolume(TakerBuySellVolumeArgs),
    #[command(
        about = decode_selected_entities(r#"Test connectivity to the Rest API.

Weight(IP): 1"#, false),
    )]
    TestConnectivity(TestConnectivityArgs),
    #[command(
        about = decode_selected_entities(r#"24 hour rolling window price change statistics.

Weight: **1** for a single symbol, **40** when the symbol parameter is omitted
**Careful** when accessing this with no symbol.

Notes:
- Symbol and pair cannot be sent together
- If a pair is sent,tickers for all symbols of the pair will be returned
- If either a pair or symbol is sent, tickers for all symbols of all pairs will be returned"#, false),
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

Weight(IP): 1

Security Type: Accounts

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available."#, false),
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

Weight(IP): 1

Security Type: Positions

Notes:
- If startTime and endTime are not sent, the most recent data is returned.
- Only the data of the latest 30 days is available."#, false),
    )]
    TopTraderLongShortRatioPositions(TopTraderLongShortRatioPositionsArgs),
    #[command(
        about = decode_selected_entities(r#"Get trades for a specific account and symbol.

Weight: **20** with symbol，**40** with pair (after CM migration: **5** flat)

Security Type: USER_DATA

Notes:
- Either symbol or pair must be sent
- Symbol and pair cannot be sent together
- Pair and fromId cannot be sent together
- OrderId can only be sent together with symbol
- If a pair is sent,tickers for all symbols of the pair will be returned
- The parameter `fromId` cannot be sent with `startTime` or `endTime`
- If startTime and endTime are both not sent, then the last 7 days' data will be returned.
- The time between startTime and endTime cannot be longer than 7 days."#, false),
    )]
    AccountTradeList(AccountTradeListArgs),
    #[command(
        about = decode_selected_entities(r#"Get all account orders; active, canceled, or filled.

* These orders will not be found:
  * order status is CANCELED or EXPIRED AND order has NO filled trade AND created time + 3 days < current time
  * order create time + 90 days < current time

Weight: **20** with symbol, **40** with pair (after CM migration: **5** flat)

Security Type: USER_DATA

Notes:
- Either `symbol` or `pair` must be sent.
- `pair` can't be sent with `orderId`
- If `orderId` is set, it will get orders >= that `orderId`. Otherwise most recent orders are returned.
- If orderId is set, it will get orders >= that orderId. Otherwise most recent orders are returned.
- The query time period must be less then 7 days( default as the recent 7 days)."#, false),
    )]
    AllOrders(AllOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel all open orders of the specified symbol at the end of the
specified countdown. This rest endpoint means to ensure your open orders
are canceled in case of an outage. The endpoint should be called
repeatedly as heartbeats so that the existing countdown time can be
canceled and repalced by a new one. The system will check all countdowns
**approximately every 10 milliseconds**, so please note that sufficient
redundancy should be considered when using this function. We do not
recommend setting the countdown time to be too precise or too small.


* Example usage:

Call this endpoint at 30s intervals with an countdownTime of 120000
(120s).

If this endpoint is not called within 120 seconds, all your orders of
the specified symbol will be automatically canceled.

If this endpoint is called with an countdownTime of 0, the countdown
timer will be stopped.

Weight(IP): 10

Security Type: TRADE"#, false),
    )]
    AutoCancelAllOpenOrders(AutoCancelAllOpenOrdersArgs),
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
        about = decode_selected_entities(r#"Change user's initial leverage in the specific symbol market.

For Hedge Mode, LONG and SHORT positions of one symbol use the same
initial leverage and share a total notional value.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeInitialLeverage(ChangeInitialLeverageArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's margin type in the specific symbol market.For Hedge Mode,
LONG and SHORT positions of one symbol use the same margin type.

With ISOLATED margin type, margins of the LONG and SHORT positions are
isolated from each other.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeMarginType(ChangeMarginTypeArgs),
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
        about = decode_selected_entities(r#"Get all open orders on a symbol. **Careful** when accessing this with no symbol.

Weight(IP): null

Weight: **1** for a single symbol, **40** for mutltiple symbols

Security Type: USER_DATA"#, false),
    )]
    CurrentAllOpenOrders(CurrentAllOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get order modification history

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `orderId` or `origClientOrderId` must be sent, and the `orderId` will prevail if both are sent.
- Order modify history longer than 3 month is not avaliable"#, false),
    )]
    GetOrderModifyHistory(GetOrderModifyHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get position margin change history

Weight(IP): 1

Security Type: TRADE"#, false),
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
        about = decode_selected_entities(r#"Modify Multiple Orders

Weight(IP): 5

Security Type: TRADE

Notes:
- Parameter rules are same with `Modify Order`
- Batch modify orders are processed concurrently, and the order of matching is not guaranteed.
- The order of returned contents for batch modify orders is the same as the order of the order list.
- One order can only be modfied for less than 10000 times
- `modifyId` is an optional user-defined identifier passed through as-is; the server does not validate its uniqueness. If omitted, it is not included in the response."#, false),
    )]
    ModifyMultipleOrders(ModifyMultipleOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent, and the `orderId` will prevail if both are sent.
- Either `quantity` or `price` must be sent. *(After CM migration, both `quantity` and `price` are required.)*
- When the new `quantity` or `price` doesn't satisfy PRICE_FILTER / PERCENT_FILTER / LOT_SIZE, amendment will be rejected and the order will stay as it is.
- However the order will be cancelled by the amendment in the following situations:
  - when the order is in partially filled status and the new `quantity` <= `executedQty`
  - When the order is `GTX` and the new price will cause it to be executed immediately
- One order can only be modfied for less than 10000 times"#, false),
    )]
    ModifyOrder(ModifyOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a new order.

Weight: 1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M)
0 on IP rate limit(x-mbx-used-weight-1m)

Security Type: TRADE

Notes:
- Additional mandatory parameters based on `type`:
  - Order with type `STOP`, parameter `timeInForce` can be sent ( default `GTC`).
  - Order with type `TAKE_PROFIT`, parameter `timeInForce` can be sent ( default `GTC`).
  - Condition orders will be triggered when:
  - If parameter`priceProtect`is sent as true:
  - when price reaches the `stopPrice` ，the difference rate between "MARK_PRICE" and "CONTRACT_PRICE" cannot be larger than the "triggerProtect" of the symbol
  - "triggerProtect" of a symbol can be got from `GET /dapi/v1/exchangeInfo`
  - `STOP`, `STOP_MARKET`:
  - BUY: latest price ("MARK_PRICE" or "CONTRACT_PRICE") >= `stopPrice`
  - SELL: latest price ("MARK_PRICE" or "CONTRACT_PRICE")
  -`TAKE_PROFIT`, `TAKE_PROFIT_MARKET`:
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
  - If triggered,**close all** current long position( if `SELL`) or current short position( if `BUY`).
  - Cannot be used with `quantity` parameter
  - Cannot be used with `reduceOnly` parameter
  - In Hedge Mode,cannot be used with `BUY` orders in `LONG` position side. and cannot be used with `SELL` orders in `SHORT` position side

  - `selfTradePreventionMode` is only effective when `timeInForce` set to `IOC` or `GTC`."#, false),
    )]
    NewOrder(NewOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Place multiple orders

* Parameter rules are same with `New Order`
* Batch orders are processed concurrently, and the order of matching is not guaranteed.
* The order of returned contents for batch orders is the same as the order of the order list.

Weight(IP): 5

Security Type: TRADE

Notes:
- `batchOrders` must be a JSON array of order parameter objects.
- Example:
  `/dapi/v1/batchOrders?batchOrders=[{"type":"LIMIT","timeInForce":"GTC","symbol":"BTCUSD_PERP","side":"BUY","price":"10001","quantity":"1"}]`"#, false),
    )]
    PlaceMultipleOrders(PlaceMultipleOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query position ADL quantile estimation

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
        about = decode_selected_entities(r#"Get current account information.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- If neither `marginAsset` nor `pair` is sent, positions of all symbols with `TRADING` status will be returned.
- for One-way Mode user, the response will only show the "BOTH" positions
- for Hedge Mode user, the response will show "BOTH", "LONG", and "SHORT" positions. **Note** > Please use with user data stream `ACCOUNT_UPDATE` to meet your timeliness and accuracy needs.
- Please use with user data stream ACCOUNT_UPDATE to meet your timeliness and accuracy needs."#, false),
    )]
    PositionInformation(PositionInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Query Current Open Order

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
  * order status is CANCELED or EXPIRED AND order has NO filled trade AND created time + 3 days < current time
  * order create time + 90 days < current time

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `orderId` or `origClientOrderId` must be sent."#, false),
    )]
    QueryOrder(QueryOrderArgs),
    #[command(
        about = decode_selected_entities(r#"User's Force Orders

Weight: **20** (after CM migration: **20** with symbol / **50** without symbol)

Security Type: USER_DATA

Notes:
- If "autoCloseType" is not sent, orders with both of the types will be returned
- Only support querying data in the past 90 days"#, false),
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
will close after 60 minutes.

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

pub async fn handle_derivatives_trading_coin_futures_command(
    command: DerivativesTradingCoinFuturesCommands,
) -> anyhow::Result<()> {
    match command {
        DerivativesTradingCoinFuturesCommands::AccountInformation(args) => {
            account_information(args).await
        }

        DerivativesTradingCoinFuturesCommands::FuturesAccountBalance(args) => {
            futures_account_balance(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetCurrentPositionMode(args) => {
            get_current_position_mode(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetDownloadIdForFuturesOrderHistory(args) => {
            get_download_id_for_futures_order_history(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetDownloadIdForFuturesTradeHistory(args) => {
            get_download_id_for_futures_trade_history(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetDownloadIdForFuturesTransactionHistory(args) => {
            get_download_id_for_futures_transaction_history(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetFuturesOrderHistoryDownloadLinkById(args) => {
            get_futures_order_history_download_link_by_id(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetFuturesTradeDownloadLinkById(args) => {
            get_futures_trade_download_link_by_id(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetFuturesTransactionHistoryDownloadLinkById(
            args,
        ) => get_futures_transaction_history_download_link_by_id(args).await,

        DerivativesTradingCoinFuturesCommands::GetIncomeHistory(args) => {
            get_income_history(args).await
        }

        DerivativesTradingCoinFuturesCommands::NotionalBracketForPair(args) => {
            notional_bracket_for_pair(args).await
        }

        DerivativesTradingCoinFuturesCommands::NotionalBracketForSymbol(args) => {
            notional_bracket_for_symbol(args).await
        }

        DerivativesTradingCoinFuturesCommands::UserCommissionRate(args) => {
            user_commission_rate(args).await
        }

        DerivativesTradingCoinFuturesCommands::Basis(args) => basis(args).await,

        DerivativesTradingCoinFuturesCommands::CheckServerTime(args) => {
            check_server_time(args).await
        }

        DerivativesTradingCoinFuturesCommands::CompressedAggregateTradesList(args) => {
            compressed_aggregate_trades_list(args).await
        }

        DerivativesTradingCoinFuturesCommands::ContinuousContractKlineCandlestickData(args) => {
            continuous_contract_kline_candlestick_data(args).await
        }

        DerivativesTradingCoinFuturesCommands::ExchangeInformation(args) => {
            exchange_information(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetFundingRateHistoryOfPerpetualFutures(args) => {
            get_funding_rate_history_of_perpetual_futures(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetFundingRateInfo(args) => {
            get_funding_rate_info(args).await
        }

        DerivativesTradingCoinFuturesCommands::IndexPriceAndMarkPrice(args) => {
            index_price_and_mark_price(args).await
        }

        DerivativesTradingCoinFuturesCommands::IndexPriceKlineCandlestickData(args) => {
            index_price_kline_candlestick_data(args).await
        }

        DerivativesTradingCoinFuturesCommands::KlineCandlestickData(args) => {
            kline_candlestick_data(args).await
        }

        DerivativesTradingCoinFuturesCommands::LongShortRatio(args) => long_short_ratio(args).await,

        DerivativesTradingCoinFuturesCommands::MarkPriceKlineCandlestickData(args) => {
            mark_price_kline_candlestick_data(args).await
        }

        DerivativesTradingCoinFuturesCommands::OldTradesLookup(args) => {
            old_trades_lookup(args).await
        }

        DerivativesTradingCoinFuturesCommands::OpenInterest(args) => open_interest(args).await,

        DerivativesTradingCoinFuturesCommands::OpenInterestStatistics(args) => {
            open_interest_statistics(args).await
        }

        DerivativesTradingCoinFuturesCommands::OrderBook(args) => order_book(args).await,

        DerivativesTradingCoinFuturesCommands::PremiumIndexKlineData(args) => {
            premium_index_kline_data(args).await
        }

        DerivativesTradingCoinFuturesCommands::QueryIndexPriceConstituents(args) => {
            query_index_price_constituents(args).await
        }

        DerivativesTradingCoinFuturesCommands::RecentTradesList(args) => {
            recent_trades_list(args).await
        }

        DerivativesTradingCoinFuturesCommands::SymbolOrderBookTicker(args) => {
            symbol_order_book_ticker(args).await
        }

        DerivativesTradingCoinFuturesCommands::SymbolPriceTicker(args) => {
            symbol_price_ticker(args).await
        }

        DerivativesTradingCoinFuturesCommands::TakerBuySellVolume(args) => {
            taker_buy_sell_volume(args).await
        }

        DerivativesTradingCoinFuturesCommands::TestConnectivity(args) => {
            test_connectivity(args).await
        }

        DerivativesTradingCoinFuturesCommands::Ticker24hrPriceChangeStatistics(args) => {
            ticker24hr_price_change_statistics(args).await
        }

        DerivativesTradingCoinFuturesCommands::TopTraderLongShortRatioAccounts(args) => {
            top_trader_long_short_ratio_accounts(args).await
        }

        DerivativesTradingCoinFuturesCommands::TopTraderLongShortRatioPositions(args) => {
            top_trader_long_short_ratio_positions(args).await
        }

        DerivativesTradingCoinFuturesCommands::AccountTradeList(args) => {
            account_trade_list(args).await
        }

        DerivativesTradingCoinFuturesCommands::AllOrders(args) => all_orders(args).await,

        DerivativesTradingCoinFuturesCommands::AutoCancelAllOpenOrders(args) => {
            auto_cancel_all_open_orders(args).await
        }

        DerivativesTradingCoinFuturesCommands::CancelAllOpenOrders(args) => {
            cancel_all_open_orders(args).await
        }

        DerivativesTradingCoinFuturesCommands::CancelMultipleOrders(args) => {
            cancel_multiple_orders(args).await
        }

        DerivativesTradingCoinFuturesCommands::CancelOrder(args) => cancel_order(args).await,

        DerivativesTradingCoinFuturesCommands::ChangeInitialLeverage(args) => {
            change_initial_leverage(args).await
        }

        DerivativesTradingCoinFuturesCommands::ChangeMarginType(args) => {
            change_margin_type(args).await
        }

        DerivativesTradingCoinFuturesCommands::ChangePositionMode(args) => {
            change_position_mode(args).await
        }

        DerivativesTradingCoinFuturesCommands::CurrentAllOpenOrders(args) => {
            current_all_open_orders(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetOrderModifyHistory(args) => {
            get_order_modify_history(args).await
        }

        DerivativesTradingCoinFuturesCommands::GetPositionMarginChangeHistory(args) => {
            get_position_margin_change_history(args).await
        }

        DerivativesTradingCoinFuturesCommands::ModifyIsolatedPositionMargin(args) => {
            modify_isolated_position_margin(args).await
        }

        DerivativesTradingCoinFuturesCommands::ModifyMultipleOrders(args) => {
            modify_multiple_orders(args).await
        }

        DerivativesTradingCoinFuturesCommands::ModifyOrder(args) => modify_order(args).await,

        DerivativesTradingCoinFuturesCommands::NewOrder(args) => new_order(args).await,

        DerivativesTradingCoinFuturesCommands::PlaceMultipleOrders(args) => {
            place_multiple_orders(args).await
        }

        DerivativesTradingCoinFuturesCommands::PositionAdlQuantileEstimation(args) => {
            position_adl_quantile_estimation(args).await
        }

        DerivativesTradingCoinFuturesCommands::PositionInformation(args) => {
            position_information(args).await
        }

        DerivativesTradingCoinFuturesCommands::QueryCurrentOpenOrder(args) => {
            query_current_open_order(args).await
        }

        DerivativesTradingCoinFuturesCommands::QueryOrder(args) => query_order(args).await,

        DerivativesTradingCoinFuturesCommands::UsersForceOrders(args) => {
            users_force_orders(args).await
        }

        DerivativesTradingCoinFuturesCommands::CloseUserDataStream(args) => {
            close_user_data_stream(args).await
        }

        DerivativesTradingCoinFuturesCommands::KeepaliveUserDataStream(args) => {
            keepalive_user_data_stream(args).await
        }

        DerivativesTradingCoinFuturesCommands::StartUserDataStream(args) => {
            start_user_data_stream(args).await
        }
    }
}

async fn account_information(args: AccountInformationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountInformationParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountInformationParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_information(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn futures_account_balance(args: FuturesAccountBalanceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FuturesAccountBalanceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FuturesAccountBalanceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => FuturesAccountBalanceParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.futures_account_balance(params).await?;

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
                            .with_prompt("Input start_time:")
                            .interact_text()?;

                        args.start_time = Some(start_time);
                    }
                    if args.end_time.is_none() {
                        let end_time: i64 = Input::new()
                            .with_prompt("Input end_time:")
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
                            .with_prompt("Input start_time:")
                            .interact_text()?;

                        args.start_time = Some(start_time);
                    }
                    if args.end_time.is_none() {
                        let end_time: i64 = Input::new()
                            .with_prompt("Input end_time:")
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
                            .with_prompt("Input start_time:")
                            .interact_text()?;

                        args.start_time = Some(start_time);
                    }
                    if args.end_time.is_none() {
                        let end_time: i64 = Input::new()
                            .with_prompt("Input end_time:")
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
                            .with_prompt("Input download_id:")
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
                            .with_prompt("Input download_id:")
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
                            .with_prompt("Input download_id:")
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

async fn notional_bracket_for_pair(args: NotionalBracketForPairArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NotionalBracketForPairParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NotionalBracketForPairParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => NotionalBracketForPairParams::builder()
                .pair(args.pair)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.notional_bracket_for_pair(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn notional_bracket_for_symbol(args: NotionalBracketForSymbolArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NotionalBracketForSymbolParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<NotionalBracketForSymbolParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => NotionalBracketForSymbolParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.notional_bracket_for_symbol(params).await?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                            let pair: String =
                                Input::new().with_prompt("Input pair:").interact_text()?;

                            args.pair = Some(pair);
                        }
                        if args.contract_type.is_none() {
                            let options = vec![
                        ("PERPETUAL", ContinuousContractKlineCandlestickDataContractTypeEnum::Perpetual),
                        ("CURRENT_QUARTER", ContinuousContractKlineCandlestickDataContractTypeEnum::CurrentQuarter),
                        ("NEXT_QUARTER", ContinuousContractKlineCandlestickDataContractTypeEnum::NextQuarter),
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
                        ("1M", ContinuousContractKlineCandlestickDataIntervalEnum::Interval1M),
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

async fn get_funding_rate_history_of_perpetual_futures(
    mut args: GetFundingRateHistoryOfPerpetualFuturesArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetFundingRateHistoryOfPerpetualFuturesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFundingRateHistoryOfPerpetualFuturesParams>(json)
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
                GetFundingRateHistoryOfPerpetualFuturesParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
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
        .get_funding_rate_history_of_perpetual_futures(params)
        .await?;

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

async fn index_price_and_mark_price(args: IndexPriceAndMarkPriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<IndexPriceAndMarkPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<IndexPriceAndMarkPriceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => IndexPriceAndMarkPriceParams::builder()
                .symbol(args.symbol)
                .pair(args.pair)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.index_price_and_mark_price(params).await?;

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
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

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
                            ("1M", IndexPriceKlineCandlestickDataIntervalEnum::Interval1M),
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                            ("1M", KlineCandlestickDataIntervalEnum::Interval1M),
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
                    if args.pair.is_none() {
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

                        args.pair = Some(pair);
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
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                            ("1M", MarkPriceKlineCandlestickDataIntervalEnum::Interval1M),
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                    if args.pair.is_none() {
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

                        args.pair = Some(pair);
                    }
                    if args.contract_type.is_none() {
                        let options = vec![
                            ("ALL", OpenInterestStatisticsContractTypeEnum::All),
                            (
                                "PERPETUAL",
                                OpenInterestStatisticsContractTypeEnum::Perpetual,
                            ),
                            (
                                "CURRENT_QUARTER",
                                OpenInterestStatisticsContractTypeEnum::CurrentQuarter,
                            ),
                            (
                                "NEXT_QUARTER",
                                OpenInterestStatisticsContractTypeEnum::NextQuarter,
                            ),
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                            ("1M", PremiumIndexKlineDataIntervalEnum::Interval1M),
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                .pair(args.pair)
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
                .pair(args.pair)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.symbol_price_ticker(params).await?;

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
                    if args.pair.is_none() {
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

                        args.pair = Some(pair);
                    }
                    if args.contract_type.is_none() {
                        let options = vec![
                            ("ALL", TakerBuySellVolumeContractTypeEnum::All),
                            ("PERPETUAL", TakerBuySellVolumeContractTypeEnum::Perpetual),
                            (
                                "CURRENT_QUARTER",
                                TakerBuySellVolumeContractTypeEnum::CurrentQuarter,
                            ),
                            (
                                "NEXT_QUARTER",
                                TakerBuySellVolumeContractTypeEnum::NextQuarter,
                            ),
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
                .pair(args.pair)
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                    if args.pair.is_none() {
                        let pair: String =
                            Input::new().with_prompt("Input pair:").interact_text()?;

                        args.pair = Some(pair);
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
                    args.pair
                        .ok_or_else(|| anyhow::anyhow!("pair is required"))?,
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

async fn account_trade_list(args: AccountTradeListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountTradeListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountTradeListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountTradeListParams::builder()
                .symbol(args.symbol)
                .pair(args.pair)
                .order_id(args.order_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .from_id(args.from_id)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_trade_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn all_orders(args: AllOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AllOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllOrdersParams::builder()
                .symbol(args.symbol)
                .pair(args.pair)
                .order_id(args.order_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.countdown_time.is_none() {
                        let countdown_time: i64 = Input::new()
                            .with_prompt("Input countdown_time:")
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.leverage.is_none() {
                        let leverage: i64 = Input::new()
                            .with_prompt("Input leverage:")
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                            .with_prompt("Input dual_side_position:")
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
                .pair(args.pair)
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.r#type.is_none() {
                        let r#type: i64 =
                            Input::new().with_prompt("Input r#type:").interact_text()?;

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
                            .with_prompt("Input batch_orders:")
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                }
                ModifyOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                )
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .quantity(args.quantity)
                .price(args.price)
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                            .with_prompt("Input batch_orders:")
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

async fn position_information(args: PositionInformationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PositionInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PositionInformationParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => PositionInformationParams::builder()
                .margin_asset(args.margin_asset)
                .pair(args.pair)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.position_information(params).await?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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
