use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::{
    SPOT_REST_API_DEMO_URL, SPOT_REST_API_PROD_URL, SPOT_REST_API_TESTNET_URL,
};
use binance_sdk::spot::SpotRestApi;
use binance_sdk::spot::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("spot");

    let client_config = get_client_configuration(profile, "spot").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "testnet" => SPOT_REST_API_TESTNET_URL.to_string(),
        "demo" => SPOT_REST_API_DEMO_URL.to_string(),
        "prod" => SPOT_REST_API_PROD_URL.to_string(),
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

    Ok(SpotRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct AccountCommissionArgs {
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
struct AllOrderListArgs {
    #[arg(
        help = r#"If supplied, neither startTime or endTime can be provided"#,
        long
    )]
    from_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
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
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. <br> Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetAccountArgs {
    #[arg(help = r#"When set to `true`, emits only the non-zero balances of an account."#, long, num_args = 0..=1, default_missing_value = "true")]
    omit_zero_balances: Option<bool>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetOpenOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetOrderListArgs {
    #[arg(
        help = r#"Query order list by `orderListId`. `orderListId` or `origClientOrderId` must be provided."#,
        long
    )]
    order_list_id: Option<i64>,
    #[arg(
        help = r#"Query order list by `listClientOrderId`. `orderListId` or `origClientOrderId` must be provided."#,
        long
    )]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MyAllocationsArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    from_allocation_id: Option<i32>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MyFiltersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MyPreventedMatchesArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    prevented_match_id: Option<i64>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    from_prevented_match_id: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MyTradesArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"This can only be used in combination with `symbol`."#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(
        help = r#"TradeId to fetch from. Default gets most recent trades."#,
        long
    )]
    from_id: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OpenOrderListArgs {
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderAmendmentsArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    from_execution_id: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RateLimitOrderArgs {
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ExchangeInfoArgs {
    #[arg(
        help = r#"Example: curl -X GET "https://api.binance.com/api/v3/exchangeInfo?symbol=BNBBTC""#,
        long
    )]
    symbol: Option<String>,
    #[arg(
        help = r#"Examples: curl -X GET "https://api.binance.com/api/v3/exchangeInfo?symbols=%5B%22BNBBTC%22,%22BTCUSDT%22%5D" or curl -g -X GET 'https://api.binance.com/api/v3/exchangeInfo?symbols=["BTCUSDT","BNBBTC"]'"#,
        long
    )]
    symbols: Option<Vec<String>>,
    #[arg(
        help = r#"Examples: curl -X GET "https://api.binance.com/api/v3/exchangeInfo?permissions=SPOT"

curl -X GET "https://api.binance.com/api/v3/exchangeInfo?permissions=%5B%22MARGIN%22%2C%22LEVERAGED%22%5D"
or
curl -g -X GET 'https://api.binance.com/api/v3/exchangeInfo?permissions=["MARGIN","LEVERAGED"]'"#,
        long
    )]
    permissions: Option<ExchangeInfoPermissionsEnum>,
    #[arg(help = r#"Controls whether the content of the `permissionSets` field is populated or not."#, long, num_args = 0..=1, default_missing_value = "true")]
    show_permission_sets: Option<bool>,
    #[arg(
        help = r#"Filters for symbols that have this `tradingStatus`. Cannot be used in combination with `symbols` or `symbol`."#,
        long
    )]
    symbol_status: Option<ExchangeInfoSymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ExecutionRulesArgs {
    #[arg(help = r#"Query for specified symbol."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Query for multiple symbols."#, long)]
    symbols: Option<Vec<String>>,
    #[arg(help = r#"Query for all symbols with the specified status."#, long)]
    symbol_status: Option<ExecutionRulesSymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct PingArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TimeArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct AggTradesArgs {
    #[arg(help = r#""#, long)]
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
    limit: Option<i32>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct AvgPriceArgs {
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
struct DepthArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"If limit > 5000, only 5000 entries will be returned."#, long)]
    limit: Option<i32>,
    #[arg(
        help = r#"Filters for symbols that have this `tradingStatus`.
A status mismatch returns error `-1220 SYMBOL_DOES_NOT_MATCH_STATUS`."#,
        long
    )]
    symbol_status: Option<DepthSymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetTradesArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct HistoricalBlockTradesArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Block trade ID to fetch from"#, long)]
    from_id: Option<i64>,
    #[arg(help = r#"Default: 500; Maximum: 1000"#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct HistoricalTradesArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
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
struct KlinesArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    interval: Option<KlinesIntervalEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Default: 0 (UTC)"#, long)]
    time_zone: Option<String>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ReferencePriceArgs {
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
struct ReferencePriceCalculationArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Supported values: `TRADING`, `HALT`, `BREAK`"#, long)]
    symbol_status: Option<ReferencePriceCalculationSymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TickerArgs {
    #[arg(help = r#"Either `symbol` or `symbols` must be provided"#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Either `symbol` or `symbols` must be provided

Examples of accepted format for the `symbols` parameter: ["BTCUSDT","BNBUSDT"] or %5B%22BTCUSDT%22,%22BNBUSDT%22%5D

The maximum number of symbols allowed in a request is 100."#,
        long
    )]
    symbols: Option<Vec<String>>,
    #[arg(
        help = r#"Units cannot be combined (e.g. `1d2h` is not allowed)."#,
        long
    )]
    window_size: Option<TickerWindowSizeEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<TickerTypeEnum>,
    #[arg(help = r#""#, long)]
    symbol_status: Option<TickerSymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct Ticker24hrArgs {
    #[arg(help = r#"Either `symbol` or `symbols` must be provided"#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Either `symbol` or `symbols` must be provided

Examples of accepted format for the `symbols` parameter: ["BTCUSDT","BNBUSDT"] or %5B%22BTCUSDT%22,%22BNBUSDT%22%5D

The maximum number of symbols allowed in a request is 100."#,
        long
    )]
    symbols: Option<Vec<String>>,
    #[arg(help = r#""#, long)]
    r#type: Option<Ticker24hrTypeEnum>,
    #[arg(help = r#""#, long)]
    symbol_status: Option<Ticker24hrSymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TickerBookTickerArgs {
    #[arg(
        help = r#"Parameter symbol and symbols cannot be used in combination.
If neither parameter is sent, `bookTickers` for all symbols will be returned in an array."#,
        long
    )]
    symbol: Option<String>,
    #[arg(
        help = r#"Parameter symbol and symbols cannot be used in combination.
If neither parameter is sent, `bookTickers` for all symbols will be returned in an array.
Examples of accepted format for the symbols parameter: ["BTCUSDT","BNBUSDT"] or %5B%22BTCUSDT%22,%22BNBUSDT%22%5D"#,
        long
    )]
    symbols: Option<Vec<String>>,
    #[arg(
        help = r#"Filters for symbols that have this `tradingStatus`.
For a single symbol, a status mismatch returns error `-1220 SYMBOL_DOES_NOT_MATCH_STATUS`.
For multiple or all symbols, non-matching ones are simply excluded from the response."#,
        long
    )]
    symbol_status: Option<TickerBookTickerSymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TickerPriceArgs {
    #[arg(
        help = r#"Parameter symbol and symbols cannot be used in combination.
If neither parameter is sent, prices for all symbols will be returned in an array."#,
        long
    )]
    symbol: Option<String>,
    #[arg(
        help = r#"Parameter symbol and symbols cannot be used in combination.
If neither parameter is sent, prices for all symbols will be returned in an array.
Examples of accepted format for the symbols parameter: ["BTCUSDT","BNBUSDT"] or %5B%22BTCUSDT%22,%22BNBUSDT%22%5D"#,
        long
    )]
    symbols: Option<Vec<String>>,
    #[arg(
        help = r#"Filters for symbols that have this `tradingStatus`.
For a single symbol, a status mismatch returns error `-1220 SYMBOL_DOES_NOT_MATCH_STATUS`.
For multiple or all symbols, non-matching ones are simply excluded from the response."#,
        long
    )]
    symbol_status: Option<TickerPriceSymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TickerTradingDayArgs {
    #[arg(help = r#"Either `symbol` or `symbols` must be provided."#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Either `symbol` or `symbols` must be provided.
Examples of accepted format for the `symbols` parameter: ["BTCUSDT","BNBUSDT"] or %5B%22BTCUSDT%22,%22BNBUSDT%22%5D.
The maximum number of `symbols` allowed in a request is 100."#,
        long
    )]
    symbols: Option<Vec<String>>,
    #[arg(help = r#"Default: 0 (UTC)"#, long)]
    time_zone: Option<String>,
    #[arg(help = r#""#, long)]
    r#type: Option<TickerTradingDayTypeEnum>,
    #[arg(
        help = r#"Filters for symbols that have this `tradingStatus`.
For a single symbol, a status mismatch returns error `-1220 SYMBOL_DOES_NOT_MATCH_STATUS`.
For multiple symbols, non-matching ones are simply excluded from the response."#,
        long
    )]
    symbol_status: Option<TickerTradingDaySymbolStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct UiKlinesArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    interval: Option<UiKlinesIntervalEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Default: 0 (UTC)"#, long)]
    time_zone: Option<String>,
    #[arg(help = r#""#, long)]
    limit: Option<i32>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct DeleteOpenOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct DeleteOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"Used to uniquely identify this cancel. Automatically generated by default."#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(
        help = r#"Supported values: <br>`ONLY_NEW` - Cancel will succeed if the order status is `NEW`.<br> `ONLY_PARTIALLY_FILLED` - Cancel will succeed if order status is `PARTIALLY_FILLED`."#,
        long
    )]
    cancel_restrictions: Option<DeleteOrderCancelRestrictionsEnum>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct DeleteOrderListArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Either `orderListId` or `listClientOrderId` must be provided"#,
        long
    )]
    order_list_id: Option<i64>,
    #[arg(
        help = r#"Either `orderListId` or `listClientOrderId` must be provided"#,
        long
    )]
    list_client_order_id: Option<String>,
    #[arg(
        help = r#"Used to uniquely identify this cancel. Automatically generated by default."#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
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
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#side) for supported values."#,
        long
    )]
    side: Option<NewOrderSideEnum>,
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#ordertypes) for supported values."#,
        long
    )]
    r#type: Option<NewOrderTypeEnum>,
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#timeinforce) for supported values."#,
        long
    )]
    time_in_force: Option<NewOrderTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    quote_order_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent.<br/> Orders with the same `newClientOrderID` can be accepted only when the previous one is filled, otherwise the order will be rejected."#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#"The value cannot be less than `1000000`."#, long)]
    strategy_type: Option<i32>,
    #[arg(
        help = r#"Used with `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders."#,
        long
    )]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"See Trailing Stop order FAQ"#, long)]
    trailing_delta: Option<i64>,
    #[arg(
        help = r#"Used with `LIMIT`, `STOP_LOSS_LIMIT`, and `TAKE_PROFIT_LIMIT` to create an iceberg order."#,
        long
    )]
    iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"`MARKET` and `LIMIT` order types default to `FULL`, all other orders default to `ACK`."#,
        long
    )]
    new_order_resp_type: Option<NewOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol."#,
        long
    )]
    self_trade_prevention_mode: Option<NewOrderSelfTradePreventionModeEnum>,
    #[arg(help = r#"See Pegged Orders Info"#, long)]
    peg_price_type: Option<NewOrderPegPriceTypeEnum>,
    #[arg(
        help = r#"Price level to peg the price to (max: 100). See Pegged Orders Info"#,
        long
    )]
    peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Only `PRICE_LEVEL` is supported. See Pegged Orders Info"#,
        long
    )]
    peg_offset_type: Option<NewOrderPegOffsetTypeEnum>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderAmendKeepPriorityArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"`newQty` must be greater than 0 and less than the order's quantity."#,
        long
    )]
    new_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#"`orderId` or `origClientOrderId` must be sent"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"`orderId` or `origClientOrderId` must be sent"#, long)]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"The new client order ID for the order after being amended. <br> If not sent, one will be randomly generated. <br> It is possible to reuse the current clientOrderId by sending it as the `newClientOrderId`."#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. <br> Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderCancelReplaceArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<OrderCancelReplaceSideEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<OrderCancelReplaceTypeEnum>,
    #[arg(
        help = r#"The allowed values are: <br/> `STOP_ON_FAILURE` - If the cancel request fails, the new order placement will not be attempted. <br/> `ALLOW_FAILURE` - new order placement will be attempted even if cancel request fails."#,
        long
    )]
    cancel_replace_mode: Option<OrderCancelReplaceCancelReplaceModeEnum>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<OrderCancelReplaceTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    quote_order_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Used to uniquely identify this cancel. Automatically generated by default."#,
        long
    )]
    cancel_new_client_order_id: Option<String>,
    #[arg(
        help = r#"Either `cancelOrderId` or `cancelOrigClientOrderId` must be sent. <br></br> If both `cancelOrderId` and `cancelOrigClientOrderId` parameters are provided, the `cancelOrderId` is searched first, then the `cancelOrigClientOrderId` from that result is checked against that order. <br></br> If both conditions are not met the request will be rejected."#,
        long
    )]
    cancel_orig_client_order_id: Option<String>,
    #[arg(
        help = r#"Either `cancelOrderId` or `cancelOrigClientOrderId` must be sent. <br></br>If both `cancelOrderId` and `cancelOrigClientOrderId` parameters are provided, the `cancelOrderId` is searched first, then the `cancelOrigClientOrderId` from that result is checked against that order. <br></br>If both conditions are not met the request will be rejected."#,
        long
    )]
    cancel_order_id: Option<i64>,
    #[arg(help = r#"Used to identify the new order."#, long)]
    new_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#"The value cannot be less than `1000000`."#, long)]
    strategy_type: Option<i32>,
    #[arg(help = r#""#, long)]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"See [Trailing Stop order FAQ](/products/spot/faqs/trailing-stop-faq)"#,
        long
    )]
    trailing_delta: Option<i64>,
    #[arg(help = r#""#, long)]
    iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Allowed values: <br/> `ACK`, `RESULT`, `FULL` <br/> `MARKET` and `LIMIT` orders types default to `FULL`; all other orders default to `ACK`"#,
        long
    )]
    new_order_resp_type: Option<OrderCancelReplaceNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. The possible supported values are: [STP Modes](/products/spot/enums#stpmodes)."#,
        long
    )]
    self_trade_prevention_mode: Option<OrderCancelReplaceSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"Supported values: <br>`ONLY_NEW` - Cancel will succeed if the order status is `NEW`.<br> `ONLY_PARTIALLY_FILLED ` - Cancel will succeed if order status is `PARTIALLY_FILLED`."#,
        long
    )]
    cancel_restrictions: Option<OrderCancelReplaceCancelRestrictionsEnum>,
    #[arg(
        help = r#"Supported values: <br> `DO_NOTHING` (default)- will only attempt to cancel the order if account has not exceeded the unfilled order rate limit<br> `CANCEL_ONLY` - will always cancel the order"#,
        long
    )]
    order_rate_limit_exceeded_mode: Option<OrderCancelReplaceOrderRateLimitExceededModeEnum>,
    #[arg(help = r#"`PRIMARY_PEG` or `MARKET_PEG` <br> See Pegged Orders"#, long)]
    peg_price_type: Option<OrderCancelReplacePegPriceTypeEnum>,
    #[arg(
        help = r#"Price level to peg the price to (max: 100) <br> See Pegged Orders"#,
        long
    )]
    peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Only `PRICE_LEVEL` is supported <br> See Pegged Orders."#,
        long
    )]
    peg_offset_type: Option<OrderCancelReplacePegOffsetTypeEnum>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderListOcoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<OrderListOcoSideEnum>,
    #[arg(help = r#"Quantity for both orders of the order list."#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    above_type: Option<OrderListOcoAboveTypeEnum>,
    #[arg(
        help = r#"Supported values: `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`"#,
        long
    )]
    below_type: Option<OrderListOcoBelowTypeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open order lists. Automatically generated if not sent. A new order list with the same `listClientOrderId` is accepted only when the previous one is filled or completely expired. `listClientOrderId` is distinct from the `aboveClientOrderId` and the `belowClientOrderId`."#,
        long
    )]
    list_client_order_id: Option<String>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the above order. Automatically generated if not sent."#,
        long
    )]
    above_client_order_id: Option<String>,
    #[arg(
        help = r#"Note that this can only be used if `aboveTimeInForce` is `GTC`."#,
        long
    )]
    above_iceberg_qty: Option<i64>,
    #[arg(
        help = r#"Can be used if `aboveType` is `STOP_LOSS_LIMIT`, `LIMIT_MAKER`, or `TAKE_PROFIT_LIMIT` to specify the limit price."#,
        long
    )]
    above_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Can be used if `aboveType` is `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`. Either `aboveStopPrice` or `aboveTrailingDelta` or both, must be specified."#,
        long
    )]
    above_stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"See [Trailing Stop order FAQ](/products/spot/faqs/trailing-stop-faq)"#,
        long
    )]
    above_trailing_delta: Option<i64>,
    #[arg(
        help = r#"Required if `aboveType` is `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT`."#,
        long
    )]
    above_time_in_force: Option<OrderListOcoAboveTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the above order within an order strategy."#,
        long
    )]
    above_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the above order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    above_strategy_type: Option<i32>,
    #[arg(
        help = r#"`PRIMARY_PEG` or `MARKET_PEG`. See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    above_peg_price_type: Option<OrderListOcoAbovePegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    above_peg_offset_type: Option<OrderListOcoAbovePegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    above_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the below order. Automatically generated if not sent."#,
        long
    )]
    below_client_order_id: Option<String>,
    #[arg(
        help = r#"Note that this can only be used if `belowTimeInForce` is `GTC`."#,
        long
    )]
    below_iceberg_qty: Option<i64>,
    #[arg(
        help = r#"Can be used if `belowType` is `STOP_LOSS_LIMIT`, `LIMIT_MAKER`, or `TAKE_PROFIT_LIMIT` to specify the limit price."#,
        long
    )]
    below_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Can be used if `belowType` is `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`. Either `belowStopPrice` or `belowTrailingDelta` or both, must be specified."#,
        long
    )]
    below_stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"See [Trailing Stop order FAQ](/products/spot/faqs/trailing-stop-faq)"#,
        long
    )]
    below_trailing_delta: Option<i64>,
    #[arg(
        help = r#"Required if `belowType` is `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT`."#,
        long
    )]
    below_time_in_force: Option<OrderListOcoBelowTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the below order within an order strategy."#,
        long
    )]
    below_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the below order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    below_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    below_peg_price_type: Option<OrderListOcoBelowPegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    below_peg_offset_type: Option<OrderListOcoBelowPegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    below_peg_offset_value: Option<i32>,
    #[arg(help = r#"Select response format: `ACK`, `RESULT`, `FULL`."#, long)]
    new_order_resp_type: Option<OrderListOcoNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. Supported values: [STP Modes](/products/spot/enums#stpmodes)"#,
        long
    )]
    self_trade_prevention_mode: Option<OrderListOcoSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderListOpoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Supported values: `LIMIT`, `LIMIT_MAKER`"#, long)]
    working_type: Option<OrderListOpoWorkingTypeEnum>,
    #[arg(
        help = r#"Supported values: [Order Side](/products/spot/enums#side)"#,
        long
    )]
    working_side: Option<OrderListOpoWorkingSideEnum>,
    #[arg(help = r#"Price for the working order."#, long)]
    working_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Sets the quantity for the working order."#, long)]
    working_quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Order Types](/products/spot/enums#ordertypes). Note that `MARKET` orders using `quoteOrderQty` are not supported."#,
        long
    )]
    pending_type: Option<OrderListOpoPendingTypeEnum>,
    #[arg(
        help = r#"Supported values: [Order Side](/products/spot/enums#side)"#,
        long
    )]
    pending_side: Option<OrderListOpoPendingSideEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open order lists. Automatically generated if not sent. A new order list with the same `listClientOrderId` is accepted only when the previous one is filled or completely expired. `listClientOrderId` is distinct from the `workingClientOrderId` and the `pendingClientOrderId`."#,
        long
    )]
    list_client_order_id: Option<String>,
    #[arg(
        help = r#"Format of the JSON response. Supported values: [Order Response Type](/products/spot/enums#orderresponsetype)"#,
        long
    )]
    new_order_resp_type: Option<OrderListOpoNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. Supported values: [STP Modes](/products/spot/enums#stpmodes)"#,
        long
    )]
    self_trade_prevention_mode: Option<OrderListOpoSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the working order. Automatically generated if not sent."#,
        long
    )]
    working_client_order_id: Option<String>,
    #[arg(
        help = r#"This can only be used if `workingTimeInForce` is `GTC`, or if `workingType` is `LIMIT_MAKER`."#,
        long
    )]
    working_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Time In Force](/products/spot/enums#timeinforce)"#,
        long
    )]
    working_time_in_force: Option<OrderListOpoWorkingTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the working order within an order strategy."#,
        long
    )]
    working_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the working order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    working_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    working_peg_price_type: Option<OrderListOpoWorkingPegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    working_peg_offset_type: Option<OrderListOpoWorkingPegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    working_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the pending order. Automatically generated if not sent."#,
        long
    )]
    pending_client_order_id: Option<String>,
    #[arg(help = r#"Price for the pending order."#, long)]
    pending_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Stop price for the pending order."#, long)]
    pending_stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Trailing delta for the pending order."#, long)]
    pending_trailing_delta: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `pendingTimeInForce` is `GTC` or if `pendingType` is `LIMIT_MAKER`."#,
        long
    )]
    pending_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Time In Force](/products/spot/enums#timeinforce)"#,
        long
    )]
    pending_time_in_force: Option<OrderListOpoPendingTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending order within an order strategy."#,
        long
    )]
    pending_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    pending_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    pending_peg_price_type: Option<OrderListOpoPendingPegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_peg_offset_type: Option<OrderListOpoPendingPegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderListOpocoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    working_type: Option<OrderListOpocoWorkingTypeEnum>,
    #[arg(
        help = r#"Supported values: [Order Side](/products/spot/enums#side)"#,
        long
    )]
    working_side: Option<OrderListOpocoWorkingSideEnum>,
    #[arg(help = r#"Price for the working order."#, long)]
    working_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Sets the quantity for the working order."#, long)]
    working_quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Order Side](/products/spot/enums#side)"#,
        long
    )]
    pending_side: Option<OrderListOpocoPendingSideEnum>,
    #[arg(
        help = r#"Supported values: `STOP_LOSS_LIMIT`, `STOP_LOSS`, `LIMIT_MAKER`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`"#,
        long
    )]
    pending_above_type: Option<OrderListOpocoPendingAboveTypeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open order lists. Automatically generated if not sent. A new order list with the same `listClientOrderId` is accepted only when the previous one is filled or completely expired. `listClientOrderId` is distinct from the `workingClientOrderId` and the `pendingClientOrderId`."#,
        long
    )]
    list_client_order_id: Option<String>,
    #[arg(
        help = r#"Format of the JSON response. Supported values: [Order Response Type](/products/spot/enums#orderresponsetype)"#,
        long
    )]
    new_order_resp_type: Option<OrderListOpocoNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. Supported values: [STP Modes](/products/spot/enums#stpmodes)"#,
        long
    )]
    self_trade_prevention_mode: Option<OrderListOpocoSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the working order. Automatically generated if not sent."#,
        long
    )]
    working_client_order_id: Option<String>,
    #[arg(
        help = r#"This can only be used if `workingTimeInForce` is `GTC`, or if `workingType` is `LIMIT_MAKER`."#,
        long
    )]
    working_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Time In Force](/products/spot/enums#timeinforce)"#,
        long
    )]
    working_time_in_force: Option<OrderListOpocoWorkingTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the working order within an order strategy."#,
        long
    )]
    working_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the working order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    working_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    working_peg_price_type: Option<OrderListOpocoWorkingPegPriceTypeEnum>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    working_peg_offset_type: Option<OrderListOpocoWorkingPegOffsetTypeEnum>,
    #[arg(
        help = r#"Price level for pegging (max: 100). See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    working_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the pending above order. Automatically generated if not sent."#,
        long
    )]
    pending_above_client_order_id: Option<String>,
    #[arg(
        help = r#"Can be used if `pendingAboveType` is `STOP_LOSS_LIMIT`, `LIMIT_MAKER`, or `TAKE_PROFIT_LIMIT` to specify the limit price."#,
        long
    )]
    pending_above_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Can be used if `pendingAboveType` is `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`."#,
        long
    )]
    pending_above_stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"See [Trailing Stop order FAQ](/products/spot/faqs/trailing-stop-faq)"#,
        long
    )]
    pending_above_trailing_delta: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `pendingAboveTimeInForce` is `GTC` or `pendingAboveType` is `LIMIT_MAKER`."#,
        long
    )]
    pending_above_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Required if `pendingAboveType` is `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT`."#,
        long
    )]
    pending_above_time_in_force: Option<OrderListOpocoPendingAboveTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending above order within an order strategy."#,
        long
    )]
    pending_above_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending above order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    pending_above_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    pending_above_peg_price_type: Option<OrderListOpocoPendingAbovePegPriceTypeEnum>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    pending_above_peg_offset_type: Option<OrderListOpocoPendingAbovePegOffsetTypeEnum>,
    #[arg(
        help = r#"Price level for pegging (max: 100). See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    pending_above_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Supported values: `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`"#,
        long
    )]
    pending_below_type: Option<OrderListOpocoPendingBelowTypeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the pending below order. Automatically generated if not sent."#,
        long
    )]
    pending_below_client_order_id: Option<String>,
    #[arg(
        help = r#"Can be used if `pendingBelowType` is `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT` to specify the limit price."#,
        long
    )]
    pending_below_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Can be used if `pendingBelowType` is `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`. Either `pendingBelowStopPrice` or `pendingBelowTrailingDelta` or both, must be specified."#,
        long
    )]
    pending_below_stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"See [Trailing Stop order FAQ](/products/spot/faqs/trailing-stop-faq)"#,
        long
    )]
    pending_below_trailing_delta: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `pendingBelowTimeInForce` is `GTC` or `pendingBelowType` is `LIMIT_MAKER`."#,
        long
    )]
    pending_below_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Time In Force](/products/spot/enums#timeinforce)"#,
        long
    )]
    pending_below_time_in_force: Option<OrderListOpocoPendingBelowTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending below order within an order strategy."#,
        long
    )]
    pending_below_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending below order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    pending_below_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    pending_below_peg_price_type: Option<OrderListOpocoPendingBelowPegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_below_peg_offset_type: Option<OrderListOpocoPendingBelowPegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_below_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderListOtoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Supported values: `LIMIT`, `LIMIT_MAKER`"#, long)]
    working_type: Option<OrderListOtoWorkingTypeEnum>,
    #[arg(
        help = r#"Supported values: [Order Side](/products/spot/enums#side)"#,
        long
    )]
    working_side: Option<OrderListOtoWorkingSideEnum>,
    #[arg(help = r#""#, long)]
    working_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Sets the quantity for the working order."#, long)]
    working_quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Order Types](/products/spot/enums#ordertypes). Note that `MARKET` orders using `quoteOrderQty` are not supported."#,
        long
    )]
    pending_type: Option<OrderListOtoPendingTypeEnum>,
    #[arg(
        help = r#"Supported values: [Order Side](/products/spot/enums#side)"#,
        long
    )]
    pending_side: Option<OrderListOtoPendingSideEnum>,
    #[arg(help = r#"Sets the quantity for the pending order."#, long)]
    pending_quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Arbitrary unique ID among open order lists. Automatically generated if not sent. A new order list with the same `listClientOrderId` is accepted only when the previous one is filled or completely expired. `listClientOrderId` is distinct from the `workingClientOrderId` and the `pendingClientOrderId`."#,
        long
    )]
    list_client_order_id: Option<String>,
    #[arg(
        help = r#"Format of the JSON response. Supported values: [Order Response Type](/products/spot/enums#orderresponsetype)"#,
        long
    )]
    new_order_resp_type: Option<OrderListOtoNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. Supported values: [STP Modes](/products/spot/enums#stpmodes)"#,
        long
    )]
    self_trade_prevention_mode: Option<OrderListOtoSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the working order. Automatically generated if not sent."#,
        long
    )]
    working_client_order_id: Option<String>,
    #[arg(
        help = r#"This can only be used if `workingTimeInForce` is `GTC`, or if `workingType` is `LIMIT_MAKER`."#,
        long
    )]
    working_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Time In Force](/products/spot/enums#timeinforce)"#,
        long
    )]
    working_time_in_force: Option<OrderListOtoWorkingTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the working order within an order strategy."#,
        long
    )]
    working_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the working order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    working_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    working_peg_price_type: Option<OrderListOtoWorkingPegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    working_peg_offset_type: Option<OrderListOtoWorkingPegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    working_peg_offset_value: Option<i32>,
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
        help = r#"This can only be used if `pendingTimeInForce` is `GTC` or if `pendingType` is `LIMIT_MAKER`."#,
        long
    )]
    pending_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Time In Force](/products/spot/enums#timeinforce)"#,
        long
    )]
    pending_time_in_force: Option<OrderListOtoPendingTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending order within an order strategy."#,
        long
    )]
    pending_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    pending_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    pending_peg_price_type: Option<OrderListOtoPendingPegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_peg_offset_type: Option<OrderListOtoPendingPegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderListOtocoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Supported values: `LIMIT`, `LIMIT_MAKER`"#, long)]
    working_type: Option<OrderListOtocoWorkingTypeEnum>,
    #[arg(
        help = r#"Supported values: [Order Side](/products/spot/enums#side)"#,
        long
    )]
    working_side: Option<OrderListOtocoWorkingSideEnum>,
    #[arg(help = r#""#, long)]
    working_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Sets the quantity for the working order."#, long)]
    working_quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Order Side](/products/spot/enums#side)"#,
        long
    )]
    pending_side: Option<OrderListOtocoPendingSideEnum>,
    #[arg(help = r#"Sets the quantity for the pending orders."#, long)]
    pending_quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: `STOP_LOSS_LIMIT`, `STOP_LOSS`, `LIMIT_MAKER`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`"#,
        long
    )]
    pending_above_type: Option<OrderListOtocoPendingAboveTypeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open order lists. Automatically generated if not sent. A new order list with the same `listClientOrderId` is accepted only when the previous one is filled or completely expired. `listClientOrderId` is distinct from the `workingClientOrderId` and the `pendingClientOrderId`."#,
        long
    )]
    list_client_order_id: Option<String>,
    #[arg(
        help = r#"Format of the JSON response. Supported values: [Order Response Type](/products/spot/enums#orderresponsetype)"#,
        long
    )]
    new_order_resp_type: Option<OrderListOtocoNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. Supported values: [STP Modes](/products/spot/enums#stpmodes)"#,
        long
    )]
    self_trade_prevention_mode: Option<OrderListOtocoSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the working order. Automatically generated if not sent."#,
        long
    )]
    working_client_order_id: Option<String>,
    #[arg(
        help = r#"This can only be used if `workingTimeInForce` is `GTC`, or if `workingType` is `LIMIT_MAKER`."#,
        long
    )]
    working_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Supported values: [Time In Force](/products/spot/enums#timeinforce)"#,
        long
    )]
    working_time_in_force: Option<OrderListOtocoWorkingTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the working order within an order strategy."#,
        long
    )]
    working_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the working order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    working_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    working_peg_price_type: Option<OrderListOtocoWorkingPegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    working_peg_offset_type: Option<OrderListOtocoWorkingPegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    working_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the pending above order. Automatically generated if not sent."#,
        long
    )]
    pending_above_client_order_id: Option<String>,
    #[arg(
        help = r#"Can be used if `pendingAboveType` is `STOP_LOSS_LIMIT`, `LIMIT_MAKER`, or `TAKE_PROFIT_LIMIT` to specify the limit price."#,
        long
    )]
    pending_above_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Can be used if `pendingAboveType` is `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`."#,
        long
    )]
    pending_above_stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"See [Trailing Stop order FAQ](/products/spot/faqs/trailing-stop-faq)"#,
        long
    )]
    pending_above_trailing_delta: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `pendingAboveTimeInForce` is `GTC` or if `pendingAboveType` is `LIMIT_MAKER`."#,
        long
    )]
    pending_above_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Required if `pendingAboveType` is `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT`."#,
        long
    )]
    pending_above_time_in_force: Option<OrderListOtocoPendingAboveTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending above order within an order strategy."#,
        long
    )]
    pending_above_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending above order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    pending_above_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    pending_above_peg_price_type: Option<OrderListOtocoPendingAbovePegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_above_peg_offset_type: Option<OrderListOtocoPendingAbovePegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_above_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Supported values: `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`"#,
        long
    )]
    pending_below_type: Option<OrderListOtocoPendingBelowTypeEnum>,
    #[arg(
        help = r#"Arbitrary unique ID among open orders for the pending below order. Automatically generated if not sent."#,
        long
    )]
    pending_below_client_order_id: Option<String>,
    #[arg(
        help = r#"Can be used if `pendingBelowType` is `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT` to specify the limit price."#,
        long
    )]
    pending_below_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Can be used if `pendingBelowType` is `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, `TAKE_PROFIT_LIMIT`. Either `pendingBelowStopPrice` or `pendingBelowTrailingDelta` or both, must be specified."#,
        long
    )]
    pending_below_stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"See [Trailing Stop order FAQ](/products/spot/faqs/trailing-stop-faq)"#,
        long
    )]
    pending_below_trailing_delta: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"This can only be used if `pendingBelowTimeInForce` is `GTC`, or if `pendingBelowType` is `LIMIT_MAKER`."#,
        long
    )]
    pending_below_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Required if `pendingBelowType` is `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT`."#,
        long
    )]
    pending_below_time_in_force: Option<OrderListOtocoPendingBelowTimeInForceEnum>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending below order within an order strategy."#,
        long
    )]
    pending_below_strategy_id: Option<i64>,
    #[arg(
        help = r#"Arbitrary numeric value identifying the pending below order strategy. Values smaller than `1000000` are reserved and cannot be used."#,
        long
    )]
    pending_below_strategy_type: Option<i32>,
    #[arg(
        help = r#"See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    pending_below_peg_price_type: Option<OrderListOtocoPendingBelowPegPriceTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_below_peg_offset_type: Option<OrderListOtocoPendingBelowPegOffsetTypeEnum>,
    #[arg(help = r#""#, long)]
    pending_below_peg_offset_value: Option<i32>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderOcoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<OrderOcoSideEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"A unique Id for the entire orderList"#, long)]
    list_client_order_id: Option<String>,
    #[arg(help = r#"A unique Id for the limit order"#, long)]
    limit_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    limit_strategy_id: Option<i64>,
    #[arg(help = r#"The value cannot be less than `1000000`."#, long)]
    limit_strategy_type: Option<i32>,
    #[arg(help = r#"Used to make the `LIMIT_MAKER` leg an iceberg order."#, long)]
    limit_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    trailing_delta: Option<i64>,
    #[arg(help = r#"A unique Id for the stop loss/stop loss limit leg"#, long)]
    stop_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    stop_strategy_id: Option<i64>,
    #[arg(help = r#"The value cannot be less than `1000000`."#, long)]
    stop_strategy_type: Option<i32>,
    #[arg(help = r#"If provided, `stopLimitTimeInForce` is required."#, long)]
    stop_limit_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Used with `STOP_LOSS_LIMIT` leg to make an iceberg order."#,
        long
    )]
    stop_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Valid values are `GTC`/`FOK`/`IOC`"#, long)]
    stop_limit_time_in_force: Option<OrderOcoStopLimitTimeInForceEnum>,
    #[arg(
        help = r#"Format of the JSON response. Supported values: [Order Response Type](/products/spot/enums#orderresponsetype)"#,
        long
    )]
    new_order_resp_type: Option<OrderOcoNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed values are dependent on what is configured on the symbol. Supported values: [STP Modes](/products/spot/enums#stpmodes)"#,
        long
    )]
    self_trade_prevention_mode: Option<OrderOcoSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. <br> Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderTestArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#side) for supported values."#,
        long
    )]
    side: Option<OrderTestSideEnum>,
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#ordertypes) for supported values."#,
        long
    )]
    r#type: Option<OrderTestTypeEnum>,
    #[arg(help = r#"Default: `false` <br> See [Commissions FAQ](/products/spot/faqs/commission_faq#test-order-diferences) to learn more."#, long, num_args = 0..=1, default_missing_value = "true")]
    compute_commission_rates: Option<bool>,
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#timeinforce) for supported values."#,
        long
    )]
    time_in_force: Option<OrderTestTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    quote_order_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Orders with the same `newClientOrderID` can be accepted only when the previous one is filled, otherwise the order will be rejected."#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#"The value cannot be less than `1000000`."#, long)]
    strategy_type: Option<i32>,
    #[arg(
        help = r#"Used with `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT`, and `TAKE_PROFIT_LIMIT` orders."#,
        long
    )]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"See [Trailing Stop order FAQ](/products/spot/faqs/trailing-stop-faq)"#,
        long
    )]
    trailing_delta: Option<i64>,
    #[arg(
        help = r#"Used with `LIMIT`, `STOP_LOSS_LIMIT`, and `TAKE_PROFIT_LIMIT` to create an iceberg order."#,
        long
    )]
    iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Set the response JSON. `ACK`, `RESULT`, or `FULL`; `MARKET` and `LIMIT` order types default to `FULL`, all other orders default to `ACK`."#,
        long
    )]
    new_order_resp_type: Option<OrderTestNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. Supported values: [STP Modes](/products/spot/enums#stpmodes)"#,
        long
    )]
    self_trade_prevention_mode: Option<OrderTestSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"`PRIMARY_PEG` or `MARKET_PEG`. See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    peg_price_type: Option<OrderTestPegPriceTypeEnum>,
    #[arg(
        help = r#"Price level for pegging (max: 100). See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    peg_offset_value: Option<i32>,
    #[arg(
        help = r#"Only `PRICE_LEVEL` is supported. See [Pegged Orders](/products/spot/faqs/pegged_orders)"#,
        long
    )]
    peg_offset_type: Option<OrderTestPegOffsetTypeEnum>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SorOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<SorOrderSideEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<SorOrderTypeEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<SorOrderTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent.<br/> Orders with the same `newClientOrderID` can be accepted only when the previous one is filled, otherwise the order will be rejected."#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#"The value cannot be less than `1000000`."#, long)]
    strategy_type: Option<i32>,
    #[arg(help = r#"Used with `LIMIT` to create an iceberg order."#, long)]
    iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Set the response JSON. `ACK`, `RESULT`, or `FULL`. Default to `FULL`"#,
        long
    )]
    new_order_resp_type: Option<SorOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. The possible supported values are: [STP Modes](/products/spot/enums#stpmodes)."#,
        long
    )]
    self_trade_prevention_mode: Option<SorOrderSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. <br> Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SorOrderTestArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#side) for supported values."#,
        long
    )]
    side: Option<SorOrderTestSideEnum>,
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#ordertypes) for supported values."#,
        long
    )]
    r#type: Option<SorOrderTestTypeEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    compute_commission_rates: Option<bool>,
    #[arg(
        help = r#"Please see [Enums](/products/spot/enums#timeinforce) for supported values."#,
        long
    )]
    time_in_force: Option<SorOrderTestTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Orders with the same `newClientOrderID` can be accepted only when the previous one is filled, otherwise the order will be rejected."#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#"The value cannot be less than `1000000`."#, long)]
    strategy_type: Option<i32>,
    #[arg(help = r#"Used with `LIMIT` to create an iceberg order."#, long)]
    iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Set the response JSON. `ACK`, `RESULT`, or `FULL`. Default to `FULL`."#,
        long
    )]
    new_order_resp_type: Option<SorOrderTestNewOrderRespTypeEnum>,
    #[arg(
        help = r#"The allowed enums is dependent on what is configured on the symbol. Supported values: [STP Modes](/products/spot/enums#stpmodes)"#,
        long
    )]
    self_trade_prevention_mode: Option<SorOrderTestSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"The value cannot be greater than `60000`. Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified."#,
        long
    )]
    recv_window: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum SpotCommands {
    #[command(
        about = decode_selected_entities(r#"Get current account commission rates.

Weight(IP): 20

Security Type: USER_DATA

Notes:
**Data Source:** Database"#, false),
    )]
    AccountCommission(AccountCommissionArgs),
    #[command(
        about = decode_selected_entities(r#"Retrieves all order lists based on provided optional parameters.

Note that the time between `startTime` and `endTime` can't be longer
than 24 hours.

Weight(IP): 20

Security Type: USER_DATA

Notes:
**Data Source:** Database"#, false),
    )]
    AllOrderList(AllOrderListArgs),
    #[command(
        about = decode_selected_entities(r#"Get all account orders; active, canceled, or filled.

Weight(IP): 20

Security Type: USER_DATA

Notes:
**Data Source:** Database

- If `orderId` is set, it will get orders >= that `orderId`. Otherwise most recent orders are returned.
- For some historical orders `cummulativeQuoteQty` will be < 0, meaning the data is not available at this time.
- If `startTime` and/or `endTime` provided, `orderId` is not required.
- The time between `startTime` and `endTime` can't be longer than 24 hours."#, false),
    )]
    AllOrders(AllOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get current account information.

Weight(IP): 20

Security Type: USER_DATA

Notes:
**Data Source:** Memory => Database"#, false),
    )]
    GetAccount(GetAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Get all open orders on a symbol. **Careful** when accessing this with no symbol.

Weight: 6 for a single symbol; 80 when the symbol parameter is omitted

Security Type: USER_DATA

Notes:
**Data Source:** Memory => Database

- If the symbol is not sent, orders for all symbols will be returned in an array."#, false),
    )]
    GetOpenOrders(GetOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Check an order's status.

Weight(IP): 4

Security Type: USER_DATA

Notes:
**Data Source:** Memory => Database

- Either `orderId` or `origClientOrderId` must be sent.
- If both `orderId` and `origClientOrderId` are provided, the `orderId` is searched first, then the `origClientOrderId` from that result is checked against that order. If both conditions are not met the request will be rejected.
- For some historical orders `cummulativeQuoteQty` will be < 0, meaning the data is not available at this time."#, false),
    )]
    GetOrder(GetOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Retrieves a specific order list based on provided optional parameters.

Weight(IP): 4

Security Type: USER_DATA

Notes:
**Data Source:** Database"#, false),
    )]
    GetOrderList(GetOrderListArgs),
    #[command(
        about = decode_selected_entities(r#"Retrieves allocations resulting from SOR order placement.

Weight(IP): 20

Security Type: USER_DATA

Notes:
**Data Source:** Database"

Supported parameter combinations:

Parameters                                  | Response |
------------------------------------------- | -------- |
`symbol`                                    | allocations from oldest to newest |
`symbol` + `startTime`                      | oldest allocations since `startTime` |
`symbol` + `endTime`                        | newest allocations until `endTime` |
`symbol` + `startTime` + `endTime`          | allocations within the time range |
`symbol` + `fromAllocationId`               | allocations by allocation ID |
`symbol` + `orderId`                        | allocations related to an order starting with oldest |
`symbol` + `orderId` + `fromAllocationId`   | allocations related to an order by allocation ID |

**Note:** The time between `startTime` and `endTime` can't be longer than 24 hours."#, false),
    )]
    MyAllocations(MyAllocationsArgs),
    #[command(
        about = decode_selected_entities(r#"Retrieves the list of filters relevant to an account on a given symbol. This is the only endpoint that shows if an account has `MAX_ASSET` filters applied to it.

Weight(IP): 40

Security Type: USER_DATA

Notes:
**Data Source:** Memory"#, false),
    )]
    MyFilters(MyFiltersArgs),
    #[command(
        about = decode_selected_entities(r#"Displays the list of orders that were expired due to STP.

These are the combinations supported:
  - `symbol` + `preventedMatchId`
  - `symbol` + `orderId`
  - `symbol` + `orderId` + `fromPreventedMatchId` (`limit` will default to 500)
  - `symbol` + `orderId` + `fromPreventedMatchId` + `limit`

Weight: Case                            | Weight
----                            | -----
If `symbol` is invalid          | 2
Querying by `preventedMatchId`  | 2
Querying by `orderId`           | 20

Security Type: USER_DATA

Notes:
**Data Source:** Database"#, false),
    )]
    MyPreventedMatches(MyPreventedMatchesArgs),
    #[command(
        about = decode_selected_entities(r#"Get trades for a specific account and symbol.

Weight: Condition| Weight|
---| ---
|Without orderId|20|
|With orderId|5|

Security Type: USER_DATA

Notes:
**Data Source:** Memory => Database

**Notes:**:
- If `fromId` is set, it will get trades >= that `fromId`. Otherwise most recent trades are returned.
- The time between `startTime` and `endTime` can't be longer than 24 hours.
- These are the supported combinations of all parameters:
  - `symbol`
  - `symbol` + `orderId`
  - `symbol` + `startTime`
  - `symbol` + `endTime`
  - `symbol` + `fromId`
  - `symbol` + `startTime` + `endTime`
  - `symbol`+ `orderId` + `fromId`"#, false),
    )]
    MyTrades(MyTradesArgs),
    #[command(
        about = decode_selected_entities(r#"Query Open Order lists

Weight(IP): 6

Security Type: USER_DATA

Notes:
**Data Source:** Memory -> Database"#, false),
    )]
    OpenOrderList(OpenOrderListArgs),
    #[command(
        about = decode_selected_entities(r#"Queries all amendments of a single order.

Weight(IP): 4

Security Type: USER_DATA

Notes:
**Data Source:** Database"#, false),
    )]
    OrderAmendments(OrderAmendmentsArgs),
    #[command(
        about = decode_selected_entities(r#"Displays the user's unfilled order count for all intervals.

Weight(IP): 40

Security Type: USER_DATA

Notes:
**Data Source:** Memory"#, false),
    )]
    RateLimitOrder(RateLimitOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Current exchange trading rules and symbol information

Weight(IP): 20

Security Type: NONE

Notes:
**Data Source:** Memory

**Notes:**
* If the value provided to `symbol` or `symbols` do not exist, the endpoint will throw an error saying the symbol is invalid.
* All parameters are optional.
* `permissions` can support single or multiple values (e.g. `SPOT`, `["MARGIN","LEVERAGED"]`). This cannot be used in combination with `symbol` or `symbols`.
* If `permissions` parameter not provided, all symbols that have either `SPOT`, `MARGIN`, or `LEVERAGED` permission will be exposed.
  * To display symbols with any permission you need to specify them explicitly in `permissions`: (e.g. `["SPOT","MARGIN",...]`.). See Account and Symbol Permissions for the full list.

**Examples of Symbol Permissions Interpretation from the Response:**

* `[["A","B"]]` means you may place an order if your account has either permission "A" **or** permission "B".
* `[["A"],["B"]]` means you can place an order if your account has permission "A" **and** permission "B".
* `[["A"],["B","C"]]` means you can place an order if your account has permission "A" **and** permission "B" or permission "C". (Inclusive or is applied here, not exclusive or, so your account may have both permission "B" and permission "C".)"#, false),
    )]
    ExchangeInfo(ExchangeInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Query execution rules for symbols.

Weight: Parameter | Weight
--- | ---
`symbol` | 2
`symbols` | 2 for each `symbol`, capped at a max of 40
`symbolStatus` | 40
None | 40

Security Type: NONE

Notes:
**Data Source:** Memory

**Note:**: No combination of multiple parameters is allowed."#, false),
    )]
    ExecutionRules(ExecutionRulesArgs),
    #[command(
        about = decode_selected_entities(r#"Test connectivity to the Rest API.

Weight(IP): 1

Security Type: NONE"#, false),
    )]
    Ping(PingArgs),
    #[command(
        about = decode_selected_entities(r#"Test connectivity to the Rest API and get the current server time.

Weight(IP): 1

Security Type: NONE"#, false),
    )]
    Time(TimeArgs),
    #[command(
        about = decode_selected_entities(r#"Get compressed, aggregate trades. Trades that fill at the time, from the same taker order, with the same price will have the quantity aggregated.

Weight(IP): 4

Security Type: NONE

Notes:
**Data Source:** Database

- If fromId, startTime, and endTime are not sent, the most recent aggregate trades will be returned."#, false),
    )]
    AggTrades(AggTradesArgs),
    #[command(
        about = decode_selected_entities(r#"Current average price for a symbol.

Weight(IP): 2

Security Type: NONE

Notes:
**Data Source:** Memory"#, false),
    )]
    AvgPrice(AvgPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Order book

Weight: Adjusted based on the limit:

|Limit|Request Weight
------|-------
1-100|  5
101-500| 25
501-1000| 50
1001-5000| 250

Security Type: NONE

Notes:
**Data Source:** Memory"#, false),
    )]
    Depth(DepthArgs),
    #[command(
        about = decode_selected_entities(r#"Get recent trades.

Weight(IP): 25

Security Type: NONE

Notes:
**Data Source:** Memory"#, false),
    )]
    GetTrades(GetTradesArgs),
    #[command(
        about = decode_selected_entities(r#"Get block trades.

Weight(IP): 25

Security Type: MARKET_DATA

Notes:
- Data Source: Database"#, false),
    )]
    HistoricalBlockTrades(HistoricalBlockTradesArgs),
    #[command(
        about = decode_selected_entities(r#"Get older trades.

Weight(IP): 25

Security Type: NONE

Notes:
**Data Source:** Database"#, false),
    )]
    HistoricalTrades(HistoricalTradesArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for a symbol.
Klines are uniquely identified by their open time.

Weight(IP): 2

Security Type: NONE

Notes:
**Data Source:** Database

Supported kline intervals (case-sensitive):

Interval  | `interval` value
--------- | ----------------
seconds   | `1s`
minutes   | `1m`, `3m`, `5m`, `15m`, `30m`
hours     | `1h`, `2h`, `4h`, `6h`, `8h`, `12h`
days      | `1d`, `3d`
weeks     | `1w`
months    | `1M`

**Notes:**

* If `startTime` and `endTime` are not sent, the most recent klines are returned.
* Supported values for `timeZone`:
  * Hours and minutes (e.g. `-1:00`, `05:45`)
  * Only hours (e.g. `0`, `8`, `4`)
  * Accepted range is strictly [-12:00 to +14:00] inclusive
* If `timeZone` provided, kline intervals are interpreted in that timezone instead of UTC.
* Note that `startTime` and `endTime` are always interpreted in UTC, regardless of `timeZone`."#, false),
    )]
    Klines(KlinesArgs),
    #[command(
        about = decode_selected_entities(r#"Query the reference price for a symbol.

Weight(IP): 2

Security Type: NONE

Notes:
**Data Source:** Memory"#, false),
    )]
    ReferencePrice(ReferencePriceArgs),
    #[command(
        about = decode_selected_entities(r#"Describes how reference price is calculated for a given symbol.

Weight(IP): 2

Security Type: NONE

Notes:
**Data Source:** Memory"#, false),
    )]
    ReferencePriceCalculation(ReferencePriceCalculationArgs),
    #[command(
        about = decode_selected_entities(r#"**Note:** This endpoint differs from `GET /api/v3/ticker/24hr`.

The statistical time range of this endpoint can be up to 59999ms longer
than the requested `windowSize`.

`openTime` starts at the beginning of a minute, while the end time is
the current time. Therefore, the actual interval can be up to 59999ms
longer than the requested window.

For example, if `closeTime` is 1641287867099 (January 04, 2022
09:17:47:099 UTC) and `windowSize` is `1d`, then `openTime` is
1641201420000 (January 3, 2022, 09:17:00 UTC).

Weight: 4 for each requested symbol regardless of windowSize.

The weight for this request will cap at 200 once the number of `symbols` in the request is more than 50.

Security Type: NONE

Notes:
**Data Source:** Database"#, false),
    )]
    Ticker(TickerArgs),
    #[command(
        about = decode_selected_entities(r#"24 hour rolling window price change statistics. **Careful** when accessing this with no symbol.

Weight: <table>
  <thead>
      <tr>
          <th>Parameter</th>
          <th>Symbols Provided</th>
          <th>Weight</th>
      </tr>
  </thead>
  <tbody>
      <tr>
          <td rowspan="2">symbol</td>
          <td>1</td>
          <td>2</td>
      </tr>
      <tr>
          <td>symbol parameter is omitted</td>
          <td>80</td>
      </tr>
      <tr>
          <td rowspan="4">symbols</td>
          <td>1-20</td>
          <td>2</td>
      </tr>
      <tr>
          <td>21-100</td>
          <td>40</td>
      </tr>
      <tr>
          <td>101 or more</td>
          <td>80</td>
      </tr>
      <tr>
          <td>symbols parameter is omitted</td>
          <td>80</td>
      </tr>
  </tbody>
</table>

Security Type: NONE

Notes:
**Data Source:** Memory"#, false),
    )]
    Ticker24hr(Ticker24hrArgs),
    #[command(
        about = decode_selected_entities(r#"Best price/qty on the order book for a symbol or symbols.

Weight: |Parameter|Symbols Provided|Weight|
|---|---|---|
|symbol| 1 |2|
| |omitted| 4|
|symbols| Any |4|

Security Type: NONE

Notes:
**Data Source:** Memory"#, false),
    )]
    TickerBookTicker(TickerBookTickerArgs),
    #[command(
        about = decode_selected_entities(r#"Latest price for a symbol or symbols.

Weight: |Parameter|Symbols Provided|Weight|
|---|---|---|
|symbol| 1 |2|
| |omitted| 4|
|symbols| Any |4|

Security Type: NONE

Notes:
**Data Source:** Memory"#, false),
    )]
    TickerPrice(TickerPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Price change statistics for a trading day.

Weight: 4 for each requested symbol. The weight for this request will cap at 200 once the number of symbols in the request is more than 50.

Security Type: NONE

Notes:
**Data Source:** Database

**Notes:**:
  - Supported values for `timeZone`:
    - Hours and minutes (e.g. `-1:00`, `05:45`)
    - Only hours (e.g. `0`, `8`, `4`)"#, false),
    )]
    TickerTradingDay(TickerTradingDayArgs),
    #[command(
        about = decode_selected_entities(r#"The request is similar to klines having the same parameters and
response.

`uiKlines` return modified kline data, optimized for presentation of
candlestick charts.

Weight(IP): 2

Security Type: NONE

Notes:
**Data Source:** Database

- If `startTime` and `endTime` are not sent, the most recent klines are returned.
- Supported values for `timeZone`:
  - Hours and minutes (e.g. `-1:00`, `05:45`)
  - Only hours (e.g. `0`, `8`, `4`)
  - Accepted range is strictly [-12:00 to +14:00] inclusive
- If `timeZone` provided, kline intervals are interpreted in that timezone instead of UTC.
- Note that `startTime` and `endTime` are always interpreted in UTC, regardless of `timeZone`."#, false),
    )]
    UiKlines(UiKlinesArgs),
    #[command(
        about = decode_selected_entities(r#"Cancels all active orders on a symbol.
This includes orders that are part of an order list.

Weight(IP): 1

Security Type: TRADE

Notes:
**Data Source:** Matching Engine"#, false),
    )]
    DeleteOpenOrders(DeleteOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an active order.

Weight(IP): 1

Security Type: TRADE

Notes:
**Data Source:** Matching Engine

- Either `orderId` or `origClientOrderId` must be sent.
- If both `orderId` and `origClientOrderId` are provided, the `orderId` is searched first, then the `origClientOrderId` from that result is checked against that order. If both conditions are not met the request will be rejected.
- The performance for canceling an order (single cancel or as part of a cancel-replace) is always better when only `orderId` is sent. Sending `origClientOrderId` or both `orderId` + `origClientOrderId` will be slower."#, false),
    )]
    DeleteOrder(DeleteOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an entire Order list

Weight(IP): 1

Security Type: TRADE

Notes:
**Data Source:** Matching Engine

**Notes:**
  - Canceling an individual order from an order list will cancel the entire order list.
  - If both orderListId and listClientOrderId parameters are provided, the orderListId is searched first, then the listClientOrderId from that result is checked against that order. If both conditions are not met the request will be rejected."#, false),
    )]
    DeleteOrderList(DeleteOrderListArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a new order.

This adds 1 order to the `EXCHANGE_MAX_ORDERS` filter and the `MAX_NUM_ORDERS` filter.

Weight(IP): 1

Unfilled Order Count: 1

Security Type: TRADE

Notes:
**Data Source:** Matching Engine

Some additional mandatory parameters based on order `type`:

Type | Additional mandatory parameters | Additional Information
------------ | ------------| ------
`LIMIT` | `timeInForce`, `quantity`, `price`|
`MARKET` | `quantity` or `quoteOrderQty`| `MARKET` orders using the `quantity` field specifies the amount of the `base asset` the user wants to buy or sell at the market price. <br/> E.g. MARKET order on BTCUSDT will specify how much BTC the user is buying or selling. <br/><br/> `MARKET` orders using `quoteOrderQty` specifies the amount the user wants to spend (when buying) or receive (when selling) the `quote` asset; the correct `quantity` will be determined based on the market liquidity and `quoteOrderQty`. <br/> E.g. Using the symbol BTCUSDT: <br/> `BUY` side, the order will buy as many BTC as `quoteOrderQty` USDT can. <br/> `SELL` side, the order will sell as much BTC needed to receive `quoteOrderQty` USDT.
`STOP_LOSS` | `quantity`, `stopPrice` or `trailingDelta`| This will execute a `MARKET` order when the conditions are met. (e.g. `stopPrice` is met or `trailingDelta` is activated)
`STOP_LOSS_LIMIT` | `timeInForce`, `quantity`,  `price`, `stopPrice` or `trailingDelta`
`TAKE_PROFIT` | `quantity`, `stopPrice` or `trailingDelta` | This will execute a `MARKET` order when the conditions are met. (e.g. `stopPrice` is met or `trailingDelta` is activated)
`TAKE_PROFIT_LIMIT` | `timeInForce`, `quantity`, `price`, `stopPrice` or `trailingDelta` |
`LIMIT_MAKER` | `quantity`, `price`| This is a `LIMIT` order that will be rejected if the order immediately matches and trades as a taker. <br/> This is also known as a POST-ONLY order.


Notes on using parameters for Pegged Orders:
* These parameters are allowed for `LIMIT`, `LIMIT_MAKER`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT_LIMIT` orders.
* If `pegPriceType` is specified, `price` becomes optional. Otherwise, it is still mandatory.
* `pegPriceType=PRIMARY_PEG` means the primary peg, that is the best price on the same side of the order book as your order.
* `pegPriceType=MARKET_PEG` means the market peg, that is the best price on the opposite side of the order book from your order.
* Use `pegOffsetType` and `pegOffsetValue` to request a price level other than the best one. These parameters must be specified together.

Other info:
* Any `LIMIT` or `LIMIT_MAKER` type order can be made an iceberg order by sending an `icebergQty`.
* Any order with an `icebergQty` MUST have `timeInForce` set to `GTC`.
* For `STOP_LOSS`, `STOP_LOSS_LIMIT`, `TAKE_PROFIT_LIMIT` and `TAKE_PROFIT` orders, `trailingDelta` can be combined with `stopPrice`.
* `MARKET` orders using `quoteOrderQty` will not break `LOT_SIZE` filter rules; the order will execute a `quantity` that will have the notional value as close as possible to `quoteOrderQty`. Trigger order price rules against market price for both MARKET and LIMIT versions:
  * Price above market price: `STOP_LOSS` `BUY`, `TAKE_PROFIT` `SELL`
  * Price below market price: `STOP_LOSS` `SELL`, `TAKE_PROFIT` `BUY`"#, false),
    )]
    NewOrder(NewOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Reduce the quantity of an existing open order.

This adds 0 orders to the `EXCHANGE_MAX_ORDERS` filter and the `MAX_NUM_ORDERS` filter.

Read Order Amend Keep Priority FAQ to learn more.

Weight(IP): 4

Unfilled Order Count: 0

Security Type: TRADE

Notes:
**Data Source:** Matching Engine"#, false),
    )]
    OrderAmendKeepPriority(OrderAmendKeepPriorityArgs),
    #[command(
        about = decode_selected_entities(r#"- Cancels an existing order and places a new order on the same symbol.
- Filters and Order Count are evaluated before the processing of the cancellation and order placement occurs.
- A new order that was not attempted (i.e. when `newOrderResult: NOT_ATTEMPTED`), will still increase the unfilled order count by 1.
- You can only cancel an individual order from an orderList using this endpoint, but the result is the same as canceling the entire orderList.

Weight(IP): 1

Unfilled Order Count: 1

Security Type: TRADE

Notes:
**Data Source:** Matching Engine

Similar to `POST /api/v3/order`, additional mandatory parameters are determined by `type`.
Response format varies depending on whether the processing of the message succeeded, partially succeeded, or failed.

<table>
  <thead>
      <tr>
          <th colspan=3 align=left>Request</th>
          <th colspan=3 align=left>Response</th>
      </tr>
      <tr>
          <th><code>cancelReplaceMode</code></th>
          <th><code>orderRateLimitExceededMode</code></th>
          <th>Unfilled Order Count</th>
          <th><code>cancelResult</code></th>
          <th><code>newOrderResult</code></th>
          <th><code>status</code></th>
      </tr>
  </thead>
  <tbody>
      <tr>
          <td rowspan="11"><code>STOP_ON_FAILURE</code></td>
          <td rowspan="6"><code>DO_NOTHING</code></td>
          <td rowspan="3">Within Limits</td>
          <td>✅ <code>SUCCESS</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right><code>200</code></td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>➖ <code>NOT_ATTEMPTED</code></td>
          <td align=right><code>400</code></td>
      </tr>
      <tr>
          <td>✅ <code>SUCCESS</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>409</code></td>
      </tr>
      <tr>
          <td rowspan="3">Exceeds Limits</td>
          <td>✅ <code>SUCCESS</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right>N/A</td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>➖ <code>NOT_ATTEMPTED</code></td>
          <td align=right>N/A</td>
      </tr>
      <tr>
          <td>✅ <code>SUCCESS</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right>N/A</td>
      </tr>
      <tr>
          <td rowspan="5"><code>CANCEL_ONLY</code></td>
          <td rowspan="3">Within Limits</td>
          <td>✅ <code>SUCCESS</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right><code>200</code></td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>➖ <code>NOT_ATTEMPTED</code></td>
          <td align=right><code>400</code></td>
      </tr>
      <tr>
          <td>✅ <code>SUCCESS</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>409</code></td>
      </tr>
      <tr>
          <td rowspan="2">Exceeds Limits</td>
          <td>❌ <code>FAILURE</code></td>
          <td>➖ <code>NOT_ATTEMPTED</code></td>
          <td align=right><code>429</code></td>
      </tr>
      <tr>
          <td>✅ <code>SUCCESS</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>429</code></td>
      </tr>
      <tr>
          <td rowspan="16"><code>ALLOW_FAILURE</code></td>
          <td rowspan="8"><code>DO_NOTHING</code></td>
          <td rowspan="4">Within Limits</td>
          <td>✅ <code>SUCCESS</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right><code>200</code></td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>400</code></td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right><code>409</code></td>
      </tr>
      <tr>
          <td>✅ <code>SUCCESS</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>409</code></td>
      </tr>
      <tr>
      <td rowspan="4">Exceeds Limits</td>
          <td>✅ <code>SUCCESS</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right>N/A</td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right>N/A</td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right>N/A</td>
      </tr>
      <tr>
          <td>✅ <code>SUCCESS</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right>N/A</td>
      </tr>
      <tr>
          <td rowspan="8"><CODE>CANCEL_ONLY</CODE></td>
          <td rowspan="4">Within Limits</td>
          <td>✅ <code>SUCCESS</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right><code>200</code></td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>400</code></td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right><code>409</code></td>
      </tr>
      <tr>
          <td>✅ <code>SUCCESS</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>409</code></td>
      </tr>
      <tr>
          <td rowspan="4">Exceeds Limits</td>
          <td>✅ <code>SUCCESS</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right><code>N/A</code></td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>400</code></td>
      </tr>
      <tr>
          <td>❌ <code>FAILURE</code></td>
          <td>✅ <code>SUCCESS</code></td>
          <td align=right>N/A</td>
      </tr>
      <tr>
          <td>✅ <code>SUCCESS</code></td>
          <td>❌ <code>FAILURE</code></td>
          <td align=right><code>409</code></td>
      </tr>
  </tbody>
</table>

**Notes:**
  - The performance for canceling an order (single cancel or as part of a cancel-replace) is always better when only `orderId` is sent. Sending `origClientOrderId` or both `orderId` + `origClientOrderId` will be slower."#, false),
    )]
    OrderCancelReplace(OrderCancelReplaceArgs),
    #[command(
        about = decode_selected_entities(r#"Send in an one-cancels-the-other (OCO) pair, where activation of one
order immediately cancels the other.

- An OCO has 2 orders called the **above order** and **below order**.
- One of the orders must be a `LIMIT_MAKER/TAKE_PROFIT/TAKE_PROFIT_LIMIT` order and the other must be `STOP_LOSS` or `STOP_LOSS_LIMIT` order.
- Price restrictions
  - If the OCO is on the `SELL` side:
    - `LIMIT_MAKER/TAKE_PROFIT_LIMIT` `price` > Last Traded Price >  `STOP_LOSS/STOP_LOSS_LIMIT` `stopPrice`
    - `TAKE_PROFIT stopPrice` > Last Traded Price > `STOP_LOSS/STOP_LOSS_LIMIT stopPrice`
  - If the OCO is on the `BUY` side:
    - `LIMIT_MAKER/TAKE_PROFIT_LIMIT price` < Last Traded Price < `stopPrice`
    - `TAKE_PROFIT stopPrice` < Last Traded Price < `STOP_LOSS/STOP_LOSS_LIMIT stopPrice` * OCOs add **2 orders** to the `EXCHANGE_MAX_ORDERS` filter and the `MAX_NUM_ORDERS` filter.
- OCOs add 2 orders to the `EXCHANGE_MAX_ORDERS` filter and the `MAX_NUM_ORDERS` filter.

Weight(IP): 1

Unfilled Order Count: 2

Security Type: TRADE

Notes:
**Data Source:** Matching Engine"#, false),
    )]
    OrderListOco(OrderListOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Place an [OPO](/products/spot/faqs/opo).

- OPOs add 2 orders to the `EXCHANGE_MAX_NUM_ORDERS`` filter and `MAX_NUM_ORDERS`` filter.

Weight(IP): 1

Unfilled Order Count: 2

Security Type: TRADE

Notes:
**Data Source:** Matching Engine"#, false),
    )]
    OrderListOpo(OrderListOpoArgs),
    #[command(
        about = decode_selected_entities(r#"Place an [OPOCO](/products/spot/faqs/opo).

Weight(IP): 1

Unfilled Order Count: 3

Security Type: TRADE

Notes:
**Data Source:** Matching Engine"#, false),
    )]
    OrderListOpoco(OrderListOpocoArgs),
    #[command(
        about = decode_selected_entities(r#"Place an OTO.

- An OTO (One-Triggers-the-Other) is an order list comprised of 2 orders.
- The first order is called the **working order** and must be `LIMIT` or `LIMIT_MAKER`. Initially, only the working order goes on the order book.
- The second order is called the **pending order**. It can be any order type except for `MARKET` orders using parameter `quoteOrderQty`. The pending order is only placed on the order book when the working order gets **fully filled**.
- If either the working order or the pending order is cancelled individually, the other order in the order list will also be canceled or expired.
- When the order list is placed, if the working order gets **immediately fully filled**, the placement response will show the working order as `FILLED` but the pending order will still appear as `PENDING_NEW`. You need to query the status of the pending order again to see its updated status.
- OTOs add **2 orders** to the `EXCHANGE_MAX_NUM_ORDERS` filter and `MAX_NUM_ORDERS` filter.

Weight(IP): 1

Unfilled Order Count: 2

Security Type: TRADE

Notes:
**Data Source:** Matching Engine

**Mandatory parameters based on `pendingType` or `workingType`**

Depending on the `pendingType` or `workingType`, some optional parameters will become mandatory.

|Type                                                  |Additional mandatory parameters|Additional information|
|----                                                  |----                           |------
|`workingType` = `LIMIT`                               |`workingTimeInForce`           |
|`pendingType` = `LIMIT`                                |`pendingPrice`, `pendingTimeInForce`          |
|`pendingType` = `STOP_LOSS` or `TAKE_PROFIT`           |`pendingStopPrice` and/or `pendingTrailingDelta`|
|`pendingType` = `STOP_LOSS_LIMIT` or `TAKE_PROFIT_LIMIT`|`pendingPrice`, `pendingStopPrice` and/or `pendingTrailingDelta`, `pendingTimeInForce`|"#, false),
    )]
    OrderListOto(OrderListOtoArgs),
    #[command(
        about = decode_selected_entities(r#"Place an OTOCO.

- An OTOCO (One-Triggers-One-Cancels-the-Other) is an order list comprised of 3 orders.
- The first order is called the **working order** and must be `LIMIT` or `LIMIT_MAKER`. Initially, only the working order goes on the order book.
  - The behavior of the working order is the same as the [OTO](#order-list-oto).
- OTOCO has 2 pending orders (pending above and pending below), forming an OCO pair. The pending orders are only placed on the order book when the working order gets **fully filled**.
  - The rules of the pending above and pending below follow the same rules as the [Order list OCO](#order-list-oco).
- OTOCOs add **3 orders** to the `EXCHANGE_MAX_NUM_ORDERS` filter and `MAX_NUM_ORDERS` filter.

Weight(IP): 1

Unfilled Order Count: 3

Security Type: TRADE

Notes:
**Data Source:** Matching Engine

**Mandatory parameters based on `pendingAboveType`, `pendingBelowType` or `workingType`**

Depending on the `pendingAboveType`/`pendingBelowType` or `workingType`, some optional parameters will become mandatory.

|Type                                                       |Additional mandatory parameters|Additional information|
|----                                                       |----                           |------
|`workingType` = `LIMIT`                                    |`workingTimeInForce`           |
|`pendingAboveType`= `LIMIT_MAKER`                                |`pendingAbovePrice`     |
|`pendingAboveType` = `STOP_LOSS/TAKE_PROFIT`        |`pendingAboveStopPrice` and/or `pendingAboveTrailingDelta`|
|`pendingAboveType=STOP_LOSS_LIMIT/TAKE_PROFIT_LIMIT` |`pendingAbovePrice`, `pendingAboveStopPrice` and/or `pendingAboveTrailingDelta`, `pendingAboveTimeInForce`|
|`pendingBelowType`= `LIMIT_MAKER`                                |`pendingBelowPrice`          |
|`pendingBelowType= STOP_LOSS/TAKE_PROFIT`         |`pendingBelowStopPrice` and/or `pendingBelowTrailingDelta`|
|`pendingBelowType=STOP_LOSS_LIMIT/TAKE_PROFIT_LIMIT` |`pendingBelowPrice`, `pendingBelowStopPrice` and/or `pendingBelowTrailingDelta`, `pendingBelowTimeInForce`|"#, false),
    )]
    OrderListOtoco(OrderListOtocoArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a new OCO.

- Price Restrictions:
  - `SELL`: Limit Price > Last Price > Stop Price
  - `BUY`: Limit Price < Last Price < Stop Price
- Quantity Restrictions:
  - Both legs must have the same quantity.
  - `ICEBERG` quantities however do not have to be the same
- `OCO` adds **2 orders** to the `EXCHANGE_MAX_ORDERS` filter and the `MAX_NUM_ORDERS` filter.

Weight(IP): 1

Unfilled Order Count: 2

Security Type: TRADE

Notes:
**Data Source:** Matching Engine"#, false),
    )]
    OrderOco(OrderOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Test new order creation and signature/recvWindow long.

Creates and validates a new order but does not send it into the matching
engine.

Weight: |Condition|Weight|
|---|---|
|Without `computeCommissionRates`|1|
|With `computeCommissionRates`|20|

Security Type: TRADE

Notes:
**Data Source:** Memory"#, false),
    )]
    OrderTest(OrderTestArgs),
    #[command(
        about = decode_selected_entities(r#"Places an order using smart order routing (SOR).

This adds 1 order to the `EXCHANGE_MAX_ORDERS` filter and the `MAX_NUM_ORDERS` filter.

Read [SOR FAQ](/products/spot/faqs/sor_faq) to learn more.

Weight(IP): 1

Unfilled Order Count: 1

Security Type: TRADE

Notes:
**Data Source:** Matching Engine

**Note:** `POST /api/v3/sor/order` only supports `LIMIT` and `MARKET` orders. `quoteOrderQty` is not supported."#, false),
    )]
    SorOrder(SorOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Test new order creation and signature/recvWindow using smart order routing (SOR).
Creates and validates a new order but does not send it into the matching engine.

Weight: |Condition|Weight|
|---|---|
|Without `computeCommissionRates`|1|
|With `computeCommissionRates`|20|

Security Type: TRADE

Notes:
**Data Source:** Memory"#, false),
    )]
    SorOrderTest(SorOrderTestArgs),
}

pub async fn handle_spot_command(command: SpotCommands) -> anyhow::Result<()> {
    match command {
        SpotCommands::AccountCommission(args) => account_commission(args).await,

        SpotCommands::AllOrderList(args) => all_order_list(args).await,

        SpotCommands::AllOrders(args) => all_orders(args).await,

        SpotCommands::GetAccount(args) => get_account(args).await,

        SpotCommands::GetOpenOrders(args) => get_open_orders(args).await,

        SpotCommands::GetOrder(args) => get_order(args).await,

        SpotCommands::GetOrderList(args) => get_order_list(args).await,

        SpotCommands::MyAllocations(args) => my_allocations(args).await,

        SpotCommands::MyFilters(args) => my_filters(args).await,

        SpotCommands::MyPreventedMatches(args) => my_prevented_matches(args).await,

        SpotCommands::MyTrades(args) => my_trades(args).await,

        SpotCommands::OpenOrderList(args) => open_order_list(args).await,

        SpotCommands::OrderAmendments(args) => order_amendments(args).await,

        SpotCommands::RateLimitOrder(args) => rate_limit_order(args).await,

        SpotCommands::ExchangeInfo(args) => exchange_info(args).await,

        SpotCommands::ExecutionRules(args) => execution_rules(args).await,

        SpotCommands::Ping(args) => ping(args).await,

        SpotCommands::Time(args) => time(args).await,

        SpotCommands::AggTrades(args) => agg_trades(args).await,

        SpotCommands::AvgPrice(args) => avg_price(args).await,

        SpotCommands::Depth(args) => depth(args).await,

        SpotCommands::GetTrades(args) => get_trades(args).await,

        SpotCommands::HistoricalBlockTrades(args) => historical_block_trades(args).await,

        SpotCommands::HistoricalTrades(args) => historical_trades(args).await,

        SpotCommands::Klines(args) => klines(args).await,

        SpotCommands::ReferencePrice(args) => reference_price(args).await,

        SpotCommands::ReferencePriceCalculation(args) => reference_price_calculation(args).await,

        SpotCommands::Ticker(args) => ticker(args).await,

        SpotCommands::Ticker24hr(args) => ticker24hr(args).await,

        SpotCommands::TickerBookTicker(args) => ticker_book_ticker(args).await,

        SpotCommands::TickerPrice(args) => ticker_price(args).await,

        SpotCommands::TickerTradingDay(args) => ticker_trading_day(args).await,

        SpotCommands::UiKlines(args) => ui_klines(args).await,

        SpotCommands::DeleteOpenOrders(args) => delete_open_orders(args).await,

        SpotCommands::DeleteOrder(args) => delete_order(args).await,

        SpotCommands::DeleteOrderList(args) => delete_order_list(args).await,

        SpotCommands::NewOrder(args) => new_order(args).await,

        SpotCommands::OrderAmendKeepPriority(args) => order_amend_keep_priority(args).await,

        SpotCommands::OrderCancelReplace(args) => order_cancel_replace(args).await,

        SpotCommands::OrderListOco(args) => order_list_oco(args).await,

        SpotCommands::OrderListOpo(args) => order_list_opo(args).await,

        SpotCommands::OrderListOpoco(args) => order_list_opoco(args).await,

        SpotCommands::OrderListOto(args) => order_list_oto(args).await,

        SpotCommands::OrderListOtoco(args) => order_list_otoco(args).await,

        SpotCommands::OrderOco(args) => order_oco(args).await,

        SpotCommands::OrderTest(args) => order_test(args).await,

        SpotCommands::SorOrder(args) => sor_order(args).await,

        SpotCommands::SorOrderTest(args) => sor_order_test(args).await,
    }
}

async fn account_commission(mut args: AccountCommissionArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountCommissionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountCommissionParams>(json).ok_or_else(|| {
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
                AccountCommissionParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.account_commission(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn all_order_list(args: AllOrderListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AllOrderListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllOrderListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllOrderListParams::builder()
                .from_id(args.from_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.all_order_list(params).await?;

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

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

async fn get_account(args: GetAccountArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetAccountParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetAccountParams::builder()
                .omit_zero_balances(args.omit_zero_balances)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_account(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_open_orders(args: GetOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOpenOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetOpenOrdersParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_order(mut args: GetOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOrderParams>(json).ok_or_else(|| {
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
                GetOrderParams::builder(
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
    let response = rest_client.get_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_order_list(args: GetOrderListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOrderListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOrderListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetOrderListParams::builder()
                .order_list_id(args.order_list_id)
                .orig_client_order_id(args.orig_client_order_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_order_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn my_allocations(mut args: MyAllocationsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MyAllocationsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MyAllocationsParams>(json).ok_or_else(|| {
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
                MyAllocationsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .from_allocation_id(args.from_allocation_id)
                .limit(args.limit)
                .order_id(args.order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.my_allocations(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn my_filters(mut args: MyFiltersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MyFiltersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MyFiltersParams>(json).ok_or_else(|| {
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
                MyFiltersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.my_filters(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn my_prevented_matches(mut args: MyPreventedMatchesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MyPreventedMatchesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MyPreventedMatchesParams>(json).ok_or_else(|| {
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
                MyPreventedMatchesParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .prevented_match_id(args.prevented_match_id)
                .order_id(args.order_id)
                .from_prevented_match_id(args.from_prevented_match_id)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.my_prevented_matches(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn my_trades(mut args: MyTradesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MyTradesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MyTradesParams>(json).ok_or_else(|| {
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
                MyTradesParams::builder(
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
    let response = rest_client.my_trades(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn open_order_list(args: OpenOrderListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OpenOrderListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OpenOrderListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => OpenOrderListParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.open_order_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_amendments(mut args: OrderAmendmentsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderAmendmentsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderAmendmentsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.order_id.is_none() {
                        let order_id: i64 = Input::new()
                            .with_prompt("Input order_id:")
                            .interact_text()?;

                        args.order_id = Some(order_id);
                    }
                }
                OrderAmendmentsParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.order_id
                        .ok_or_else(|| anyhow::anyhow!("order_id is required"))?,
                )
                .from_execution_id(args.from_execution_id)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_amendments(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn rate_limit_order(args: RateLimitOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RateLimitOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RateLimitOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => RateLimitOrderParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.rate_limit_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn exchange_info(args: ExchangeInfoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<ExchangeInfoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ExchangeInfoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ExchangeInfoParams::builder()
                .symbol(args.symbol)
                .symbols(args.symbols)
                .permissions(args.permissions)
                .show_permission_sets(args.show_permission_sets)
                .symbol_status(args.symbol_status)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.exchange_info(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn execution_rules(args: ExecutionRulesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<ExecutionRulesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ExecutionRulesParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ExecutionRulesParams::builder()
                .symbol(args.symbol)
                .symbols(args.symbols)
                .symbol_status(args.symbol_status)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.execution_rules(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn ping(args: PingArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.ping().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn time(args: TimeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.time().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn agg_trades(mut args: AggTradesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<AggTradesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AggTradesParams>(json).ok_or_else(|| {
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
                AggTradesParams::builder(
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
    let response = rest_client.agg_trades(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn avg_price(mut args: AvgPriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<AvgPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AvgPriceParams>(json).ok_or_else(|| {
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
                AvgPriceParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.avg_price(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn depth(mut args: DepthArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<DepthParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DepthParams>(json).ok_or_else(|| {
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
                DepthParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .limit(args.limit)
                .symbol_status(args.symbol_status)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.depth(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_trades(mut args: GetTradesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetTradesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetTradesParams>(json).ok_or_else(|| {
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
                GetTradesParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_trades(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn historical_block_trades(mut args: HistoricalBlockTradesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<HistoricalBlockTradesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<HistoricalBlockTradesParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.from_id.is_none() {
                        let from_id: i64 =
                            Input::new().with_prompt("Input from_id:").interact_text()?;

                        args.from_id = Some(from_id);
                    }
                }
                HistoricalBlockTradesParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.from_id
                        .ok_or_else(|| anyhow::anyhow!("from_id is required"))?,
                )
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.historical_block_trades(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn historical_trades(mut args: HistoricalTradesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<HistoricalTradesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<HistoricalTradesParams>(json).ok_or_else(|| {
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
                HistoricalTradesParams::builder(
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
    let response = rest_client.historical_trades(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn klines(mut args: KlinesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<KlinesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<KlinesParams>(json).ok_or_else(|| {
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
                            ("1s", KlinesIntervalEnum::Interval1s),
                            ("1m", KlinesIntervalEnum::Interval1m),
                            ("3m", KlinesIntervalEnum::Interval3m),
                            ("5m", KlinesIntervalEnum::Interval5m),
                            ("15m", KlinesIntervalEnum::Interval15m),
                            ("30m", KlinesIntervalEnum::Interval30m),
                            ("1h", KlinesIntervalEnum::Interval1h),
                            ("2h", KlinesIntervalEnum::Interval2h),
                            ("4h", KlinesIntervalEnum::Interval4h),
                            ("6h", KlinesIntervalEnum::Interval6h),
                            ("8h", KlinesIntervalEnum::Interval8h),
                            ("12h", KlinesIntervalEnum::Interval12h),
                            ("1d", KlinesIntervalEnum::Interval1d),
                            ("3d", KlinesIntervalEnum::Interval3d),
                            ("1w", KlinesIntervalEnum::Interval1w),
                            ("1M", KlinesIntervalEnum::Interval1M),
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
                KlinesParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.interval
                        .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .time_zone(args.time_zone)
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.klines(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn reference_price(mut args: ReferencePriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<ReferencePriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ReferencePriceParams>(json).ok_or_else(|| {
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
                ReferencePriceParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.reference_price(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn reference_price_calculation(
    mut args: ReferencePriceCalculationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<ReferencePriceCalculationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ReferencePriceCalculationParams>(json).ok_or_else(|| {
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
                ReferencePriceCalculationParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .symbol_status(args.symbol_status)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.reference_price_calculation(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn ticker(args: TickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<TickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TickerParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => TickerParams::builder()
                .symbol(args.symbol)
                .symbols(args.symbols)
                .window_size(args.window_size)
                .r#type(args.r#type)
                .symbol_status(args.symbol_status)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.ticker(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn ticker24hr(args: Ticker24hrArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<Ticker24hrParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<Ticker24hrParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => Ticker24hrParams::builder()
                .symbol(args.symbol)
                .symbols(args.symbols)
                .r#type(args.r#type)
                .symbol_status(args.symbol_status)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.ticker24hr(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn ticker_book_ticker(args: TickerBookTickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<TickerBookTickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TickerBookTickerParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => TickerBookTickerParams::builder()
                .symbol(args.symbol)
                .symbols(args.symbols)
                .symbol_status(args.symbol_status)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.ticker_book_ticker(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn ticker_price(args: TickerPriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<TickerPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TickerPriceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => TickerPriceParams::builder()
                .symbol(args.symbol)
                .symbols(args.symbols)
                .symbol_status(args.symbol_status)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.ticker_price(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn ticker_trading_day(args: TickerTradingDayArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<TickerTradingDayParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TickerTradingDayParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => TickerTradingDayParams::builder()
                .symbol(args.symbol)
                .symbols(args.symbols)
                .time_zone(args.time_zone)
                .r#type(args.r#type)
                .symbol_status(args.symbol_status)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.ticker_trading_day(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn ui_klines(mut args: UiKlinesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<UiKlinesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UiKlinesParams>(json).ok_or_else(|| {
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
                            ("1s", UiKlinesIntervalEnum::Interval1s),
                            ("1m", UiKlinesIntervalEnum::Interval1m),
                            ("3m", UiKlinesIntervalEnum::Interval3m),
                            ("5m", UiKlinesIntervalEnum::Interval5m),
                            ("15m", UiKlinesIntervalEnum::Interval15m),
                            ("30m", UiKlinesIntervalEnum::Interval30m),
                            ("1h", UiKlinesIntervalEnum::Interval1h),
                            ("2h", UiKlinesIntervalEnum::Interval2h),
                            ("4h", UiKlinesIntervalEnum::Interval4h),
                            ("6h", UiKlinesIntervalEnum::Interval6h),
                            ("8h", UiKlinesIntervalEnum::Interval8h),
                            ("12h", UiKlinesIntervalEnum::Interval12h),
                            ("1d", UiKlinesIntervalEnum::Interval1d),
                            ("3d", UiKlinesIntervalEnum::Interval3d),
                            ("1w", UiKlinesIntervalEnum::Interval1w),
                            ("1M", UiKlinesIntervalEnum::Interval1M),
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
                UiKlinesParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.interval
                        .ok_or_else(|| anyhow::anyhow!("interval is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .time_zone(args.time_zone)
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.ui_klines(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn delete_open_orders(mut args: DeleteOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DeleteOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DeleteOpenOrdersParams>(json).ok_or_else(|| {
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
                DeleteOpenOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.delete_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn delete_order(mut args: DeleteOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DeleteOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DeleteOrderParams>(json).ok_or_else(|| {
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
                DeleteOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .new_client_order_id(args.new_client_order_id)
                .cancel_restrictions(args.cancel_restrictions)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.delete_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn delete_order_list(mut args: DeleteOrderListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DeleteOrderListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DeleteOrderListParams>(json).ok_or_else(|| {
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
                DeleteOrderListParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_list_id(args.order_list_id)
                .list_client_order_id(args.list_client_order_id)
                .new_client_order_id(args.new_client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.delete_order_list(params).await?;

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
                            ("MARKET", NewOrderTypeEnum::Market),
                            ("LIMIT", NewOrderTypeEnum::Limit),
                            ("STOP_LOSS", NewOrderTypeEnum::StopLoss),
                            ("STOP_LOSS_LIMIT", NewOrderTypeEnum::StopLossLimit),
                            ("TAKE_PROFIT", NewOrderTypeEnum::TakeProfit),
                            ("TAKE_PROFIT_LIMIT", NewOrderTypeEnum::TakeProfitLimit),
                            ("LIMIT_MAKER", NewOrderTypeEnum::LimitMaker),
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
                .time_in_force(args.time_in_force)
                .quantity(args.quantity)
                .quote_order_qty(args.quote_order_qty)
                .price(args.price)
                .new_client_order_id(args.new_client_order_id)
                .strategy_id(args.strategy_id)
                .strategy_type(args.strategy_type)
                .stop_price(args.stop_price)
                .trailing_delta(args.trailing_delta)
                .iceberg_qty(args.iceberg_qty)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .peg_price_type(args.peg_price_type)
                .peg_offset_value(args.peg_offset_value)
                .peg_offset_type(args.peg_offset_type)
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

async fn order_amend_keep_priority(mut args: OrderAmendKeepPriorityArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderAmendKeepPriorityParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderAmendKeepPriorityParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.symbol.is_none() {
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.new_qty.is_none() {
                        let new_qty: rust_decimal::Decimal =
                            Input::new().with_prompt("Input new_qty:").interact_text()?;

                        args.new_qty = Some(new_qty);
                    }
                }
                OrderAmendKeepPriorityParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.new_qty
                        .ok_or_else(|| anyhow::anyhow!("new_qty is required"))?,
                )
                .order_id(args.order_id)
                .orig_client_order_id(args.orig_client_order_id)
                .new_client_order_id(args.new_client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_amend_keep_priority(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_cancel_replace(mut args: OrderCancelReplaceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderCancelReplaceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderCancelReplaceParams>(json).ok_or_else(|| {
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
                            ("BUY", OrderCancelReplaceSideEnum::Buy),
                            ("SELL", OrderCancelReplaceSideEnum::Sell),
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
                            ("MARKET", OrderCancelReplaceTypeEnum::Market),
                            ("LIMIT", OrderCancelReplaceTypeEnum::Limit),
                            ("STOP_LOSS", OrderCancelReplaceTypeEnum::StopLoss),
                            ("STOP_LOSS_LIMIT", OrderCancelReplaceTypeEnum::StopLossLimit),
                            ("TAKE_PROFIT", OrderCancelReplaceTypeEnum::TakeProfit),
                            (
                                "TAKE_PROFIT_LIMIT",
                                OrderCancelReplaceTypeEnum::TakeProfitLimit,
                            ),
                            ("LIMIT_MAKER", OrderCancelReplaceTypeEnum::LimitMaker),
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
                    if args.cancel_replace_mode.is_none() {
                        let options = vec![
                            (
                                "STOP_ON_FAILURE",
                                OrderCancelReplaceCancelReplaceModeEnum::StopOnFailure,
                            ),
                            (
                                "ALLOW_FAILURE",
                                OrderCancelReplaceCancelReplaceModeEnum::AllowFailure,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the cancel_replace_mode")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.cancel_replace_mode = Some(selected);
                    }
                }
                OrderCancelReplaceParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                    args.cancel_replace_mode
                        .ok_or_else(|| anyhow::anyhow!("cancel_replace_mode is required"))?,
                )
                .time_in_force(args.time_in_force)
                .quantity(args.quantity)
                .quote_order_qty(args.quote_order_qty)
                .price(args.price)
                .cancel_new_client_order_id(args.cancel_new_client_order_id)
                .cancel_orig_client_order_id(args.cancel_orig_client_order_id)
                .cancel_order_id(args.cancel_order_id)
                .new_client_order_id(args.new_client_order_id)
                .strategy_id(args.strategy_id)
                .strategy_type(args.strategy_type)
                .stop_price(args.stop_price)
                .trailing_delta(args.trailing_delta)
                .iceberg_qty(args.iceberg_qty)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .cancel_restrictions(args.cancel_restrictions)
                .order_rate_limit_exceeded_mode(args.order_rate_limit_exceeded_mode)
                .peg_price_type(args.peg_price_type)
                .peg_offset_value(args.peg_offset_value)
                .peg_offset_type(args.peg_offset_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_cancel_replace(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_list_oco(mut args: OrderListOcoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderListOcoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderListOcoParams>(json).ok_or_else(|| {
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
                            ("BUY", OrderListOcoSideEnum::Buy),
                            ("SELL", OrderListOcoSideEnum::Sell),
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
                    if args.above_type.is_none() {
                        let options = vec![
                            ("STOP_LOSS_LIMIT", OrderListOcoAboveTypeEnum::StopLossLimit),
                            ("STOP_LOSS", OrderListOcoAboveTypeEnum::StopLoss),
                            ("LIMIT_MAKER", OrderListOcoAboveTypeEnum::LimitMaker),
                            ("TAKE_PROFIT", OrderListOcoAboveTypeEnum::TakeProfit),
                            (
                                "TAKE_PROFIT_LIMIT",
                                OrderListOcoAboveTypeEnum::TakeProfitLimit,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the above_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.above_type = Some(selected);
                    }
                    if args.below_type.is_none() {
                        let options = vec![
                            ("STOP_LOSS", OrderListOcoBelowTypeEnum::StopLoss),
                            ("STOP_LOSS_LIMIT", OrderListOcoBelowTypeEnum::StopLossLimit),
                            ("TAKE_PROFIT", OrderListOcoBelowTypeEnum::TakeProfit),
                            (
                                "TAKE_PROFIT_LIMIT",
                                OrderListOcoBelowTypeEnum::TakeProfitLimit,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the below_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.below_type = Some(selected);
                    }
                }
                OrderListOcoParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                    args.above_type
                        .ok_or_else(|| anyhow::anyhow!("above_type is required"))?,
                    args.below_type
                        .ok_or_else(|| anyhow::anyhow!("below_type is required"))?,
                )
                .list_client_order_id(args.list_client_order_id)
                .above_client_order_id(args.above_client_order_id)
                .above_iceberg_qty(args.above_iceberg_qty)
                .above_price(args.above_price)
                .above_stop_price(args.above_stop_price)
                .above_trailing_delta(args.above_trailing_delta)
                .above_time_in_force(args.above_time_in_force)
                .above_strategy_id(args.above_strategy_id)
                .above_strategy_type(args.above_strategy_type)
                .above_peg_price_type(args.above_peg_price_type)
                .above_peg_offset_type(args.above_peg_offset_type)
                .above_peg_offset_value(args.above_peg_offset_value)
                .below_client_order_id(args.below_client_order_id)
                .below_iceberg_qty(args.below_iceberg_qty)
                .below_price(args.below_price)
                .below_stop_price(args.below_stop_price)
                .below_trailing_delta(args.below_trailing_delta)
                .below_time_in_force(args.below_time_in_force)
                .below_strategy_id(args.below_strategy_id)
                .below_strategy_type(args.below_strategy_type)
                .below_peg_price_type(args.below_peg_price_type)
                .below_peg_offset_type(args.below_peg_offset_type)
                .below_peg_offset_value(args.below_peg_offset_value)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_list_oco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_list_opo(mut args: OrderListOpoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderListOpoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderListOpoParams>(json).ok_or_else(|| {
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
                            ("LIMIT", OrderListOpoWorkingTypeEnum::Limit),
                            ("LIMIT_MAKER", OrderListOpoWorkingTypeEnum::LimitMaker),
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
                            ("BUY", OrderListOpoWorkingSideEnum::Buy),
                            ("SELL", OrderListOpoWorkingSideEnum::Sell),
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
                    if args.pending_type.is_none() {
                        let options = vec![
                            ("LIMIT", OrderListOpoPendingTypeEnum::Limit),
                            ("MARKET", OrderListOpoPendingTypeEnum::Market),
                            ("STOP_LOSS", OrderListOpoPendingTypeEnum::StopLoss),
                            (
                                "STOP_LOSS_LIMIT",
                                OrderListOpoPendingTypeEnum::StopLossLimit,
                            ),
                            ("TAKE_PROFIT", OrderListOpoPendingTypeEnum::TakeProfit),
                            (
                                "TAKE_PROFIT_LIMIT",
                                OrderListOpoPendingTypeEnum::TakeProfitLimit,
                            ),
                            ("LIMIT_MAKER", OrderListOpoPendingTypeEnum::LimitMaker),
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
                            ("BUY", OrderListOpoPendingSideEnum::Buy),
                            ("SELL", OrderListOpoPendingSideEnum::Sell),
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
                }
                OrderListOpoParams::builder(
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
                    args.pending_type
                        .ok_or_else(|| anyhow::anyhow!("pending_type is required"))?,
                    args.pending_side
                        .ok_or_else(|| anyhow::anyhow!("pending_side is required"))?,
                )
                .list_client_order_id(args.list_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .working_client_order_id(args.working_client_order_id)
                .working_iceberg_qty(args.working_iceberg_qty)
                .working_time_in_force(args.working_time_in_force)
                .working_strategy_id(args.working_strategy_id)
                .working_strategy_type(args.working_strategy_type)
                .working_peg_price_type(args.working_peg_price_type)
                .working_peg_offset_type(args.working_peg_offset_type)
                .working_peg_offset_value(args.working_peg_offset_value)
                .pending_client_order_id(args.pending_client_order_id)
                .pending_price(args.pending_price)
                .pending_stop_price(args.pending_stop_price)
                .pending_trailing_delta(args.pending_trailing_delta)
                .pending_iceberg_qty(args.pending_iceberg_qty)
                .pending_time_in_force(args.pending_time_in_force)
                .pending_strategy_id(args.pending_strategy_id)
                .pending_strategy_type(args.pending_strategy_type)
                .pending_peg_price_type(args.pending_peg_price_type)
                .pending_peg_offset_type(args.pending_peg_offset_type)
                .pending_peg_offset_value(args.pending_peg_offset_value)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_list_opo(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_list_opoco(mut args: OrderListOpocoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderListOpocoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderListOpocoParams>(json).ok_or_else(|| {
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
                            ("LIMIT", OrderListOpocoWorkingTypeEnum::Limit),
                            ("LIMIT_MAKER", OrderListOpocoWorkingTypeEnum::LimitMaker),
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
                            ("BUY", OrderListOpocoWorkingSideEnum::Buy),
                            ("SELL", OrderListOpocoWorkingSideEnum::Sell),
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
                            ("BUY", OrderListOpocoPendingSideEnum::Buy),
                            ("SELL", OrderListOpocoPendingSideEnum::Sell),
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
                    if args.pending_above_type.is_none() {
                        let options = vec![
                            (
                                "STOP_LOSS_LIMIT",
                                OrderListOpocoPendingAboveTypeEnum::StopLossLimit,
                            ),
                            ("STOP_LOSS", OrderListOpocoPendingAboveTypeEnum::StopLoss),
                            (
                                "LIMIT_MAKER",
                                OrderListOpocoPendingAboveTypeEnum::LimitMaker,
                            ),
                            (
                                "TAKE_PROFIT",
                                OrderListOpocoPendingAboveTypeEnum::TakeProfit,
                            ),
                            (
                                "TAKE_PROFIT_LIMIT",
                                OrderListOpocoPendingAboveTypeEnum::TakeProfitLimit,
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
                OrderListOpocoParams::builder(
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
                    args.pending_above_type
                        .ok_or_else(|| anyhow::anyhow!("pending_above_type is required"))?,
                )
                .list_client_order_id(args.list_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .working_client_order_id(args.working_client_order_id)
                .working_iceberg_qty(args.working_iceberg_qty)
                .working_time_in_force(args.working_time_in_force)
                .working_strategy_id(args.working_strategy_id)
                .working_strategy_type(args.working_strategy_type)
                .working_peg_price_type(args.working_peg_price_type)
                .working_peg_offset_type(args.working_peg_offset_type)
                .working_peg_offset_value(args.working_peg_offset_value)
                .pending_above_client_order_id(args.pending_above_client_order_id)
                .pending_above_price(args.pending_above_price)
                .pending_above_stop_price(args.pending_above_stop_price)
                .pending_above_trailing_delta(args.pending_above_trailing_delta)
                .pending_above_iceberg_qty(args.pending_above_iceberg_qty)
                .pending_above_time_in_force(args.pending_above_time_in_force)
                .pending_above_strategy_id(args.pending_above_strategy_id)
                .pending_above_strategy_type(args.pending_above_strategy_type)
                .pending_above_peg_price_type(args.pending_above_peg_price_type)
                .pending_above_peg_offset_type(args.pending_above_peg_offset_type)
                .pending_above_peg_offset_value(args.pending_above_peg_offset_value)
                .pending_below_type(args.pending_below_type)
                .pending_below_client_order_id(args.pending_below_client_order_id)
                .pending_below_price(args.pending_below_price)
                .pending_below_stop_price(args.pending_below_stop_price)
                .pending_below_trailing_delta(args.pending_below_trailing_delta)
                .pending_below_iceberg_qty(args.pending_below_iceberg_qty)
                .pending_below_time_in_force(args.pending_below_time_in_force)
                .pending_below_strategy_id(args.pending_below_strategy_id)
                .pending_below_strategy_type(args.pending_below_strategy_type)
                .pending_below_peg_price_type(args.pending_below_peg_price_type)
                .pending_below_peg_offset_type(args.pending_below_peg_offset_type)
                .pending_below_peg_offset_value(args.pending_below_peg_offset_value)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_list_opoco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_list_oto(mut args: OrderListOtoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderListOtoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderListOtoParams>(json).ok_or_else(|| {
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
                            ("LIMIT", OrderListOtoWorkingTypeEnum::Limit),
                            ("LIMIT_MAKER", OrderListOtoWorkingTypeEnum::LimitMaker),
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
                            ("BUY", OrderListOtoWorkingSideEnum::Buy),
                            ("SELL", OrderListOtoWorkingSideEnum::Sell),
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
                    if args.pending_type.is_none() {
                        let options = vec![
                            ("LIMIT", OrderListOtoPendingTypeEnum::Limit),
                            ("MARKET", OrderListOtoPendingTypeEnum::Market),
                            ("STOP_LOSS", OrderListOtoPendingTypeEnum::StopLoss),
                            (
                                "STOP_LOSS_LIMIT",
                                OrderListOtoPendingTypeEnum::StopLossLimit,
                            ),
                            ("TAKE_PROFIT", OrderListOtoPendingTypeEnum::TakeProfit),
                            (
                                "TAKE_PROFIT_LIMIT",
                                OrderListOtoPendingTypeEnum::TakeProfitLimit,
                            ),
                            ("LIMIT_MAKER", OrderListOtoPendingTypeEnum::LimitMaker),
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
                            ("BUY", OrderListOtoPendingSideEnum::Buy),
                            ("SELL", OrderListOtoPendingSideEnum::Sell),
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
                OrderListOtoParams::builder(
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
                    args.pending_type
                        .ok_or_else(|| anyhow::anyhow!("pending_type is required"))?,
                    args.pending_side
                        .ok_or_else(|| anyhow::anyhow!("pending_side is required"))?,
                    args.pending_quantity
                        .ok_or_else(|| anyhow::anyhow!("pending_quantity is required"))?,
                )
                .list_client_order_id(args.list_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .working_client_order_id(args.working_client_order_id)
                .working_iceberg_qty(args.working_iceberg_qty)
                .working_time_in_force(args.working_time_in_force)
                .working_strategy_id(args.working_strategy_id)
                .working_strategy_type(args.working_strategy_type)
                .working_peg_price_type(args.working_peg_price_type)
                .working_peg_offset_type(args.working_peg_offset_type)
                .working_peg_offset_value(args.working_peg_offset_value)
                .pending_client_order_id(args.pending_client_order_id)
                .pending_price(args.pending_price)
                .pending_stop_price(args.pending_stop_price)
                .pending_trailing_delta(args.pending_trailing_delta)
                .pending_iceberg_qty(args.pending_iceberg_qty)
                .pending_time_in_force(args.pending_time_in_force)
                .pending_strategy_id(args.pending_strategy_id)
                .pending_strategy_type(args.pending_strategy_type)
                .pending_peg_price_type(args.pending_peg_price_type)
                .pending_peg_offset_type(args.pending_peg_offset_type)
                .pending_peg_offset_value(args.pending_peg_offset_value)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_list_oto(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_list_otoco(mut args: OrderListOtocoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderListOtocoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderListOtocoParams>(json).ok_or_else(|| {
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
                            ("LIMIT", OrderListOtocoWorkingTypeEnum::Limit),
                            ("LIMIT_MAKER", OrderListOtocoWorkingTypeEnum::LimitMaker),
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
                            ("BUY", OrderListOtocoWorkingSideEnum::Buy),
                            ("SELL", OrderListOtocoWorkingSideEnum::Sell),
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
                            ("BUY", OrderListOtocoPendingSideEnum::Buy),
                            ("SELL", OrderListOtocoPendingSideEnum::Sell),
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
                                "STOP_LOSS_LIMIT",
                                OrderListOtocoPendingAboveTypeEnum::StopLossLimit,
                            ),
                            ("STOP_LOSS", OrderListOtocoPendingAboveTypeEnum::StopLoss),
                            (
                                "LIMIT_MAKER",
                                OrderListOtocoPendingAboveTypeEnum::LimitMaker,
                            ),
                            (
                                "TAKE_PROFIT",
                                OrderListOtocoPendingAboveTypeEnum::TakeProfit,
                            ),
                            (
                                "TAKE_PROFIT_LIMIT",
                                OrderListOtocoPendingAboveTypeEnum::TakeProfitLimit,
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
                OrderListOtocoParams::builder(
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
                .list_client_order_id(args.list_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .working_client_order_id(args.working_client_order_id)
                .working_iceberg_qty(args.working_iceberg_qty)
                .working_time_in_force(args.working_time_in_force)
                .working_strategy_id(args.working_strategy_id)
                .working_strategy_type(args.working_strategy_type)
                .working_peg_price_type(args.working_peg_price_type)
                .working_peg_offset_type(args.working_peg_offset_type)
                .working_peg_offset_value(args.working_peg_offset_value)
                .pending_above_client_order_id(args.pending_above_client_order_id)
                .pending_above_price(args.pending_above_price)
                .pending_above_stop_price(args.pending_above_stop_price)
                .pending_above_trailing_delta(args.pending_above_trailing_delta)
                .pending_above_iceberg_qty(args.pending_above_iceberg_qty)
                .pending_above_time_in_force(args.pending_above_time_in_force)
                .pending_above_strategy_id(args.pending_above_strategy_id)
                .pending_above_strategy_type(args.pending_above_strategy_type)
                .pending_above_peg_price_type(args.pending_above_peg_price_type)
                .pending_above_peg_offset_type(args.pending_above_peg_offset_type)
                .pending_above_peg_offset_value(args.pending_above_peg_offset_value)
                .pending_below_type(args.pending_below_type)
                .pending_below_client_order_id(args.pending_below_client_order_id)
                .pending_below_price(args.pending_below_price)
                .pending_below_stop_price(args.pending_below_stop_price)
                .pending_below_trailing_delta(args.pending_below_trailing_delta)
                .pending_below_iceberg_qty(args.pending_below_iceberg_qty)
                .pending_below_time_in_force(args.pending_below_time_in_force)
                .pending_below_strategy_id(args.pending_below_strategy_id)
                .pending_below_strategy_type(args.pending_below_strategy_type)
                .pending_below_peg_price_type(args.pending_below_peg_price_type)
                .pending_below_peg_offset_type(args.pending_below_peg_offset_type)
                .pending_below_peg_offset_value(args.pending_below_peg_offset_value)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_list_otoco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_oco(mut args: OrderOcoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderOcoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderOcoParams>(json).ok_or_else(|| {
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
                            ("BUY", OrderOcoSideEnum::Buy),
                            ("SELL", OrderOcoSideEnum::Sell),
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
                OrderOcoParams::builder(
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
                .list_client_order_id(args.list_client_order_id)
                .limit_client_order_id(args.limit_client_order_id)
                .limit_strategy_id(args.limit_strategy_id)
                .limit_strategy_type(args.limit_strategy_type)
                .limit_iceberg_qty(args.limit_iceberg_qty)
                .trailing_delta(args.trailing_delta)
                .stop_client_order_id(args.stop_client_order_id)
                .stop_strategy_id(args.stop_strategy_id)
                .stop_strategy_type(args.stop_strategy_type)
                .stop_limit_price(args.stop_limit_price)
                .stop_iceberg_qty(args.stop_iceberg_qty)
                .stop_limit_time_in_force(args.stop_limit_time_in_force)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_oco(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn order_test(mut args: OrderTestArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OrderTestParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OrderTestParams>(json).ok_or_else(|| {
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
                            ("BUY", OrderTestSideEnum::Buy),
                            ("SELL", OrderTestSideEnum::Sell),
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
                            ("MARKET", OrderTestTypeEnum::Market),
                            ("LIMIT", OrderTestTypeEnum::Limit),
                            ("STOP_LOSS", OrderTestTypeEnum::StopLoss),
                            ("STOP_LOSS_LIMIT", OrderTestTypeEnum::StopLossLimit),
                            ("TAKE_PROFIT", OrderTestTypeEnum::TakeProfit),
                            ("TAKE_PROFIT_LIMIT", OrderTestTypeEnum::TakeProfitLimit),
                            ("LIMIT_MAKER", OrderTestTypeEnum::LimitMaker),
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
                OrderTestParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .compute_commission_rates(args.compute_commission_rates)
                .time_in_force(args.time_in_force)
                .quantity(args.quantity)
                .quote_order_qty(args.quote_order_qty)
                .price(args.price)
                .new_client_order_id(args.new_client_order_id)
                .strategy_id(args.strategy_id)
                .strategy_type(args.strategy_type)
                .stop_price(args.stop_price)
                .trailing_delta(args.trailing_delta)
                .iceberg_qty(args.iceberg_qty)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .peg_price_type(args.peg_price_type)
                .peg_offset_value(args.peg_offset_value)
                .peg_offset_type(args.peg_offset_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.order_test(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn sor_order(mut args: SorOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SorOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SorOrderParams>(json).ok_or_else(|| {
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
                            ("BUY", SorOrderSideEnum::Buy),
                            ("SELL", SorOrderSideEnum::Sell),
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
                            ("MARKET", SorOrderTypeEnum::Market),
                            ("LIMIT", SorOrderTypeEnum::Limit),
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
                    if args.quantity.is_none() {
                        let quantity: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input quantity:")
                            .interact_text()?;

                        args.quantity = Some(quantity);
                    }
                }
                SorOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                )
                .time_in_force(args.time_in_force)
                .price(args.price)
                .new_client_order_id(args.new_client_order_id)
                .strategy_id(args.strategy_id)
                .strategy_type(args.strategy_type)
                .iceberg_qty(args.iceberg_qty)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.sor_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn sor_order_test(mut args: SorOrderTestArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SorOrderTestParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SorOrderTestParams>(json).ok_or_else(|| {
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
                            ("BUY", SorOrderTestSideEnum::Buy),
                            ("SELL", SorOrderTestSideEnum::Sell),
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
                            ("MARKET", SorOrderTestTypeEnum::Market),
                            ("LIMIT", SorOrderTestTypeEnum::Limit),
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
                    if args.quantity.is_none() {
                        let quantity: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input quantity:")
                            .interact_text()?;

                        args.quantity = Some(quantity);
                    }
                }
                SorOrderTestParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                )
                .compute_commission_rates(args.compute_commission_rates)
                .time_in_force(args.time_in_force)
                .price(args.price)
                .new_client_order_id(args.new_client_order_id)
                .strategy_id(args.strategy_id)
                .strategy_type(args.strategy_type)
                .iceberg_qty(args.iceberg_qty)
                .new_order_resp_type(args.new_order_resp_type)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.sor_order_test(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
