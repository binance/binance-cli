use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::{
    DERIVATIVES_TRADING_OPTIONS_REST_API_PROD_URL, DERIVATIVES_TRADING_OPTIONS_REST_API_TESTNET_URL,
};
use binance_sdk::derivatives_trading_options::DerivativesTradingOptionsRestApi;
use binance_sdk::derivatives_trading_options::rest_api::{self as models, *};
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("derivatives-trading-options");

    let client_config = get_client_configuration(profile, "derivatives-trading-options").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "testnet" | "demo" => DERIVATIVES_TRADING_OPTIONS_REST_API_TESTNET_URL.to_string(),
        "prod" => DERIVATIVES_TRADING_OPTIONS_REST_API_PROD_URL.to_string(),
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

    Ok(DerivativesTradingOptionsRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct AccountFundingFlowArgs {
    #[arg(help = r#"Asset type, only support USDT  as of now"#, long)]
    currency: Option<AccountFundingFlowCurrencyEnum>,
    #[arg(
        help = r#"Return the recordId and subsequent data, the latest data is returned by default"#,
        long
    )]
    record_id: Option<i64>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of result sets returned"#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OptionMarginAccountInformationArgs {
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
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
struct ExchangeInformationArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct HistoricalExerciseRecordsArgs {
    #[arg(help = r#"Underlying asset."#, long)]
    underlying: Option<String>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of result sets returned"#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct IndexPriceArgs {
    #[arg(help = r#"Underlying asset."#, long)]
    underlying: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct KlineCandlestickDataArgs {
    #[arg(help = r#"Option trading pair"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Time interval"#, long)]
    interval: Option<KlineCandlestickDataIntervalEnum>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of result sets returned"#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OpenInterestArgs {
    #[arg(help = r#"Underlying asset."#, long)]
    underlying_asset: Option<String>,
    #[arg(help = r#"expiration date"#, long)]
    expiration: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OptionMarkPriceArgs {
    #[arg(help = r#"Option trading pair"#, long)]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OrderBookArgs {
    #[arg(help = r#"Option trading pair"#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Default:100 Max:1000.Optional value:[10, 20, 50, 100, 500, 1000]"#,
        long
    )]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RecentBlockTradesListArgs {
    #[arg(help = r#"Option trading pair"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Number of records"#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RecentTradesListArgs {
    #[arg(help = r#"Option trading pair"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Number of result sets returned"#, long)]
    limit: Option<i64>,
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
    #[arg(help = r#"Option trading pair"#, long)]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct AcceptBlockTradeOrderArgs {
    #[arg(help = r#""#, long)]
    block_order_matching_key: Option<String>,
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
struct AccountBlockTradeListArgs {
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Underlying asset."#, long)]
    underlying: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelBlockTradeOrderArgs {
    #[arg(help = r#"Block trade matching key."#, long)]
    block_order_matching_key: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ExtendBlockTradeOrderArgs {
    #[arg(help = r#""#, long)]
    block_order_matching_key: Option<String>,
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
struct NewBlockTradeOrderArgs {
    #[arg(help = r#""#, long)]
    liquidity: Option<NewBlockTradeOrderLiquidityEnum>,
    #[arg(
        help = r#"Max 1 (only single leg supported), list of legs parameters in JSON; example: eapi/v1/block/order/create?orders=[{"symbol":"BTC-210115-35000-C", "price":"100","quantity":"0.0002","side":"BUY","type":"LIMIT"}]"#,
        long
    )]
    legs: Option<String>,
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
struct QueryBlockTradeDetailsArgs {
    #[arg(help = r#"Block trade matching key."#, long)]
    block_order_matching_key: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryBlockTradeOrderArgs {
    #[arg(
        help = r#"If specified, returns the specific block trade associated with the blockOrderMatchingKey"#,
        long
    )]
    block_order_matching_key: Option<String>,
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Underlying asset."#, long)]
    underlying: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
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
    underlyings: Option<String>,
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
struct GetAutoCancelAllOpenOrdersArgs {
    #[arg(help = r#"Underlying asset."#, long)]
    underlying: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetMarketMakerProtectionConfigArgs {
    #[arg(help = r#"Underlying asset."#, long)]
    underlying: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ResetMarketMakerProtectionConfigArgs {
    #[arg(help = r#""#, long)]
    underlying: Option<String>,
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
struct SetAutoCancelAllOpenOrdersArgs {
    #[arg(help = r#""#, long)]
    underlying: Option<String>,
    #[arg(
        help = r#"Countdown time in milliseconds (ex. 1,000 for 1 second). 0 to disable the timer. Negative values (ex. -10000) are not accepted. Minimum acceptable value is 5,000"#,
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
struct SetMarketMakerProtectionConfigArgs {
    #[arg(help = r#""#, long)]
    underlying: Option<String>,
    #[arg(help = r#"MMP Interval in milliseconds"#, long)]
    window_time_in_milliseconds: Option<i64>,
    #[arg(
        help = r#"MMP frozen time in milliseconds, if set to 0 manual reset is required"#,
        long
    )]
    frozen_time_in_milliseconds: Option<i64>,
    #[arg(help = r#"quantity limit"#, long)]
    qty_limit: Option<rust_decimal::Decimal>,
    #[arg(help = r#"net delta limit"#, long)]
    delta_limit: Option<rust_decimal::Decimal>,
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
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"Trade id to fetch from. Default gets most recent trades, e.g 4611875134427365376"#,
        long
    )]
    from_id: Option<i64>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of result sets returned."#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelAllOptionOrdersByUnderlyingArgs {
    #[arg(help = r#"Underlying asset."#, long)]
    underlying: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelAllOptionOrdersOnSpecificSymbolArgs {
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelMultipleOptionOrdersArgs {
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID list."#, long)]
    order_ids: Option<Vec<i64>>,
    #[arg(help = r#"Client order ID list."#, long)]
    client_order_ids: Option<Vec<String>>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelOptionOrderArgs {
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID."#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"clientOrderId"#, long)]
    client_order_id: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
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
    #[arg(help = r#""#, long)]
    r#type: Option<NewOrderTypeEnum>,
    #[arg(help = r#"Order Quantity"#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Order Price"#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewOrderTimeInForceEnum>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    reduce_only: Option<bool>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    post_only: Option<bool>,
    #[arg(help = r#""#, long)]
    new_order_resp_type: Option<NewOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"User-defined order ID cannot be repeated in pending orders"#,
        long
    )]
    client_order_id: Option<String>,
    #[arg(help = r#"is market maker protection order"#, long, num_args = 0..=1, default_missing_value = "true")]
    is_mmp: Option<bool>,
    #[arg(help = r#"Self-trade prevention mode"#, long)]
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
struct OptionPositionInformationArgs {
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
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
    #[arg(help = r#"order list. Max 10 orders"#, long)]
    orders: Option<String>,
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
struct QueryCurrentOpenOptionOrdersArgs {
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID."#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryOptionOrderHistoryArgs {
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID."#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of result sets returned"#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QuerySingleOrderArgs {
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID."#, long)]
    order_id: Option<i64>,
    #[arg(
        help = r#"User-defined order ID; cannot be duplicated among open orders."#,
        long
    )]
    client_order_id: Option<String>,
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TradfiOptionsContractArgs {
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
struct UserCommissionArgs {
    #[arg(help = r#"Recv Window."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct UserExerciseRecordArgs {
    #[arg(help = r#"Option trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Start Time, e.g 1593511200000"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End Time, e.g 1593512200000"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of result sets returned."#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Recv Window."#, long)]
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
pub enum DerivativesTradingOptionsCommands {
    #[command(
        about = decode_selected_entities(r#"Query account funding flows.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Only support querying data in the past 3 months"#, false),
    )]
    AccountFundingFlow(AccountFundingFlowArgs),
    #[command(
        about = decode_selected_entities(r#"Get current account information.

Weight(IP): 3

Security Type: USER_DATA"#, false),
    )]
    OptionMarginAccountInformation(OptionMarginAccountInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Test connectivity to the Rest API and get the current server time.

Weight(IP): 1"#, false),
    )]
    CheckServerTime(CheckServerTimeArgs),
    #[command(
        about = decode_selected_entities(r#"Current exchange trading rules and symbol information

Weight(IP): 1"#, false),
    )]
    ExchangeInformation(ExchangeInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Get historical exercise records.

* REALISTIC_VALUE_STRICKEN -> Exercised
* EXTRINSIC_VALUE_EXPIRED -> Expired OTM

Weight(IP): 3"#, false),
    )]
    HistoricalExerciseRecords(HistoricalExerciseRecordsArgs),
    #[command(
        about = decode_selected_entities(r#"Get spot index price for option underlying.

Weight(IP): 1"#, false),
    )]
    IndexPrice(IndexPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Kline/candlestick bars for an option symbol. Klines are uniquely identified by their open time.

Weight(IP): 1

Notes:
- If startTime and endTime are not sent, the most recent klines are returned."#, false),
    )]
    KlineCandlestickData(KlineCandlestickDataArgs),
    #[command(
        about = decode_selected_entities(r#"Get open interest for specific underlying asset on specific expiration date.

Weight(IP): 0"#, false),
    )]
    OpenInterest(OpenInterestArgs),
    #[command(
        about = decode_selected_entities(r#"Option mark price and greek info.

Weight(IP): 5"#, false),
    )]
    OptionMarkPrice(OptionMarkPriceArgs),
    #[command(
        about = decode_selected_entities(r#"Check orderbook depth on specific symbol

Weight: limit         | weight
------------  | ------------
5, 10, 20, 50 | 1
100           | 5
500           | 10
1000          | 20"#, false),
    )]
    OrderBook(OrderBookArgs),
    #[command(
        about = decode_selected_entities(r#"Get recent block trades

Weight(IP): 5"#, false),
    )]
    RecentBlockTradesList(RecentBlockTradesListArgs),
    #[command(
        about = decode_selected_entities(r#"Get recent market trades

Weight(IP): 5"#, false),
    )]
    RecentTradesList(RecentTradesListArgs),
    #[command(
        about = decode_selected_entities(r#"Test connectivity to the Rest API.

Weight(IP): 1"#, false),
    )]
    TestConnectivity(TestConnectivityArgs),
    #[command(
        about = decode_selected_entities(r#"24 hour rolling window price change statistics.

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted"#, false),
    )]
    Ticker24hrPriceChangeStatistics(Ticker24hrPriceChangeStatisticsArgs),
    #[command(
        about = decode_selected_entities(r#"Accept a block trade order

Weight(IP): 5

Security Type: TRADE"#, false),
    )]
    AcceptBlockTradeOrder(AcceptBlockTradeOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Gets block trades for a specific account.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    AccountBlockTradeList(AccountBlockTradeListArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel a block trade order.

Weight(IP): 5

Security Type: TRADE"#, false),
    )]
    CancelBlockTradeOrder(CancelBlockTradeOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Extends a block trade expire time by 30 mins from the current time.

Weight(IP): 5

Security Type: TRADE"#, false),
    )]
    ExtendBlockTradeOrder(ExtendBlockTradeOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a new block trade order.

Weight(IP): 5

Security Type: TRADE"#, false),
    )]
    NewBlockTradeOrder(NewBlockTradeOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query block trade details; returns block trade details from counterparty's perspective.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    QueryBlockTradeDetails(QueryBlockTradeDetailsArgs),
    #[command(
        about = decode_selected_entities(r#"Check block trade order status.

Weight(IP): 5

Security Type: TRADE"#, false),
    )]
    QueryBlockTradeOrder(QueryBlockTradeOrderArgs),
    #[command(
        about = decode_selected_entities(r#"This endpoint resets the time from which the countdown will begin to the time this messaged is received.  It should be called repeatedly as heartbeats.  Multiple heartbeats can be updated at once by specifying the underlying symbols as a list (ex. BTCUSDT,ETHUSDT) in the underlyings parameter.

Weight(IP): 10

Security Type: TRADE

Notes:
- The response will only include underlying symbols where the heartbeat has been successfully updated."#, false),
    )]
    AutoCancelAllOpenOrders(AutoCancelAllOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"This endpoint returns the auto-cancel parameters for each underlying symbol. Note only active auto-cancel parameters will be returned, if countdownTime is set to 0 (ie. countdownTime has been turned off), the underlying symbol and corresponding countdownTime parameter will not be returned in the response.

Weight(IP): 1

Security Type: TRADE

Notes:
- countdownTime = 0 means the function is disabled."#, false),
    )]
    GetAutoCancelAllOpenOrders(GetAutoCancelAllOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get config for MMP.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    GetMarketMakerProtectionConfig(GetMarketMakerProtectionConfigArgs),
    #[command(
        about = decode_selected_entities(r#"Reset MMP, start MMP order again.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ResetMarketMakerProtectionConfig(ResetMarketMakerProtectionConfigArgs),
    #[command(
        about = decode_selected_entities(r#"This endpoint sets the parameters of the auto-cancel feature which cancels all open orders (both market maker protection and non market maker protection order types) of the underlying symbol at the end of the specified countdown time period if no heartbeat message is sent.  After the countdown time period, all open orders will be cancelled and new orders will be rejected with error code -2010 until either a heartbeat message is sent or the auto-cancel feature is turned off by setting countdownTime to 0.

Weight(IP): 1

Security Type: TRADE

Notes:
- This rest endpoint sets up the parameters to cancel your open orders in case of an outage or disconnection.
- Example usage: > Call this endpoint with a countdownTime value of 10000 (10 seconds) to turn on the auto-cancel feature. If the corresponding countdownCancelAllHeartBeat endpoint is not called within 10 seconds with the specified underlying symbol, all open orders of the specified symbol will be automatically canceled. If this endpoint is called with an countdownTime of 0, the countdown timer will be stopped.
- The system will check all countdowns approximately every 100 milliseconds, **please note that sufficient redundancy should be considered when using this function**. We do not recommend setting the countdown time to be too precise or too small."#, false),
    )]
    SetAutoCancelAllOpenOrders(SetAutoCancelAllOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Set config for MMP. Market Maker Protection(MMP) is a set of protection mechanism for option market maker, this mechanism is able to prevent mass trading in short period time. Once market maker's account branches the threshold, the Market Maker Protection will be triggered. When Market Maker Protection triggers, all the current MMP orders will be canceled, new MMP orders will be rejected. Market maker can use this time to reevaluate market and modify order price.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    SetMarketMakerProtectionConfig(SetMarketMakerProtectionConfigArgs),
    #[command(
        about = decode_selected_entities(r#"Get trades for a specific account and symbol.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    AccountTradeList(AccountTradeListArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel all active orders on specified underlying.

Weight(IP): 5

Security Type: TRADE"#, false),
    )]
    CancelAllOptionOrdersByUnderlying(CancelAllOptionOrdersByUnderlyingArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel all active order on a symbol.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAllOptionOrdersOnSpecificSymbol(CancelAllOptionOrdersOnSpecificSymbolArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel multiple orders.

Weight(IP): 5

Security Type: TRADE

Notes:
- At least one instance of `orderId` and `clientOrderId` must be sent."#, false),
    )]
    CancelMultipleOptionOrders(CancelMultipleOptionOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an active order.

Weight(IP): 1

Security Type: TRADE

Notes:
- At least one instance of `orderId` and `clientOrderId` must be sent."#, false),
    )]
    CancelOptionOrder(CancelOptionOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Send a new order.

Weight(IP): 0

Security Type: TRADE

Notes:
Some parameters are mandatory depending on the order type as follows:

Type | Mandatory parameters
------------ | ------------
LIMIT | timeInForce, quantity, price"#, false),
    )]
    NewOrder(NewOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Get current position information.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    OptionPositionInformation(OptionPositionInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Send multiple option orders.

Weight(IP): 5

Security Type: TRADE

Notes:
Some parameters are mandatory depending on the order type as follows:

Type | Mandatory parameters
------------ | ------------
LIMIT | timeInForce, quantity, price

- Parameter rules are same with New Order
- Batch orders are processed concurrently, and the order of matching is not guaranteed."#, false),
    )]
    PlaceMultipleOrders(PlaceMultipleOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query current all open orders, status: ACCEPTED PARTIALLY_FILLED

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted

Security Type: USER_DATA"#, false),
    )]
    QueryCurrentOpenOptionOrders(QueryCurrentOpenOptionOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query all finished orders within 5 days, finished status: CANCELLED FILLED REJECTED.

Weight(IP): 3

Security Type: TRADE"#, false),
    )]
    QueryOptionOrderHistory(QueryOptionOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Check an order status.

* These orders will not be found:
  * order status is `CANCELED` or `REJECTED`, **AND**
  * order has NO filled trade, **AND**
  * created time + 3 days < current time

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderId` or `clientOrderId ` must be sent."#, false),
    )]
    QuerySingleOrder(QuerySingleOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Sign TradFi Options agreement contract

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    TradfiOptionsContract(TradfiOptionsContractArgs),
    #[command(
        about = decode_selected_entities(r#"Get account commission.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    UserCommission(UserCommissionArgs),
    #[command(
        about = decode_selected_entities(r#"Get account exercise records.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    UserExerciseRecord(UserExerciseRecordArgs),
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
        about = decode_selected_entities(r#"Start a new user data stream. The stream will close after 60 minutes unless a keepalive is sent. If the account has an active `listenKey`, that `listenKey` will be returned and its validity will be extended for 60 minutes.

Weight(IP): 1

Security Type: USER_STREAM"#, false),
    )]
    StartUserDataStream(StartUserDataStreamArgs),
}

pub async fn handle_derivatives_trading_options_command(
    command: DerivativesTradingOptionsCommands,
) -> anyhow::Result<()> {
    match command {
        DerivativesTradingOptionsCommands::AccountFundingFlow(args) => {
            account_funding_flow(args).await
        }

        DerivativesTradingOptionsCommands::OptionMarginAccountInformation(args) => {
            option_margin_account_information(args).await
        }

        DerivativesTradingOptionsCommands::CheckServerTime(args) => check_server_time(args).await,

        DerivativesTradingOptionsCommands::ExchangeInformation(args) => {
            exchange_information(args).await
        }

        DerivativesTradingOptionsCommands::HistoricalExerciseRecords(args) => {
            historical_exercise_records(args).await
        }

        DerivativesTradingOptionsCommands::IndexPrice(args) => index_price(args).await,

        DerivativesTradingOptionsCommands::KlineCandlestickData(args) => {
            kline_candlestick_data(args).await
        }

        DerivativesTradingOptionsCommands::OpenInterest(args) => open_interest(args).await,

        DerivativesTradingOptionsCommands::OptionMarkPrice(args) => option_mark_price(args).await,

        DerivativesTradingOptionsCommands::OrderBook(args) => order_book(args).await,

        DerivativesTradingOptionsCommands::RecentBlockTradesList(args) => {
            recent_block_trades_list(args).await
        }

        DerivativesTradingOptionsCommands::RecentTradesList(args) => recent_trades_list(args).await,

        DerivativesTradingOptionsCommands::TestConnectivity(args) => test_connectivity(args).await,

        DerivativesTradingOptionsCommands::Ticker24hrPriceChangeStatistics(args) => {
            ticker24hr_price_change_statistics(args).await
        }

        DerivativesTradingOptionsCommands::AcceptBlockTradeOrder(args) => {
            accept_block_trade_order(args).await
        }

        DerivativesTradingOptionsCommands::AccountBlockTradeList(args) => {
            account_block_trade_list(args).await
        }

        DerivativesTradingOptionsCommands::CancelBlockTradeOrder(args) => {
            cancel_block_trade_order(args).await
        }

        DerivativesTradingOptionsCommands::ExtendBlockTradeOrder(args) => {
            extend_block_trade_order(args).await
        }

        DerivativesTradingOptionsCommands::NewBlockTradeOrder(args) => {
            new_block_trade_order(args).await
        }

        DerivativesTradingOptionsCommands::QueryBlockTradeDetails(args) => {
            query_block_trade_details(args).await
        }

        DerivativesTradingOptionsCommands::QueryBlockTradeOrder(args) => {
            query_block_trade_order(args).await
        }

        DerivativesTradingOptionsCommands::AutoCancelAllOpenOrders(args) => {
            auto_cancel_all_open_orders(args).await
        }

        DerivativesTradingOptionsCommands::GetAutoCancelAllOpenOrders(args) => {
            get_auto_cancel_all_open_orders(args).await
        }

        DerivativesTradingOptionsCommands::GetMarketMakerProtectionConfig(args) => {
            get_market_maker_protection_config(args).await
        }

        DerivativesTradingOptionsCommands::ResetMarketMakerProtectionConfig(args) => {
            reset_market_maker_protection_config(args).await
        }

        DerivativesTradingOptionsCommands::SetAutoCancelAllOpenOrders(args) => {
            set_auto_cancel_all_open_orders(args).await
        }

        DerivativesTradingOptionsCommands::SetMarketMakerProtectionConfig(args) => {
            set_market_maker_protection_config(args).await
        }

        DerivativesTradingOptionsCommands::AccountTradeList(args) => account_trade_list(args).await,

        DerivativesTradingOptionsCommands::CancelAllOptionOrdersByUnderlying(args) => {
            cancel_all_option_orders_by_underlying(args).await
        }

        DerivativesTradingOptionsCommands::CancelAllOptionOrdersOnSpecificSymbol(args) => {
            cancel_all_option_orders_on_specific_symbol(args).await
        }

        DerivativesTradingOptionsCommands::CancelMultipleOptionOrders(args) => {
            cancel_multiple_option_orders(args).await
        }

        DerivativesTradingOptionsCommands::CancelOptionOrder(args) => {
            cancel_option_order(args).await
        }

        DerivativesTradingOptionsCommands::NewOrder(args) => new_order(args).await,

        DerivativesTradingOptionsCommands::OptionPositionInformation(args) => {
            option_position_information(args).await
        }

        DerivativesTradingOptionsCommands::PlaceMultipleOrders(args) => {
            place_multiple_orders(args).await
        }

        DerivativesTradingOptionsCommands::QueryCurrentOpenOptionOrders(args) => {
            query_current_open_option_orders(args).await
        }

        DerivativesTradingOptionsCommands::QueryOptionOrderHistory(args) => {
            query_option_order_history(args).await
        }

        DerivativesTradingOptionsCommands::QuerySingleOrder(args) => query_single_order(args).await,

        DerivativesTradingOptionsCommands::TradfiOptionsContract(args) => {
            tradfi_options_contract(args).await
        }

        DerivativesTradingOptionsCommands::UserCommission(args) => user_commission(args).await,

        DerivativesTradingOptionsCommands::UserExerciseRecord(args) => {
            user_exercise_record(args).await
        }

        DerivativesTradingOptionsCommands::CloseUserDataStream(args) => {
            close_user_data_stream(args).await
        }

        DerivativesTradingOptionsCommands::KeepaliveUserDataStream(args) => {
            keepalive_user_data_stream(args).await
        }

        DerivativesTradingOptionsCommands::StartUserDataStream(args) => {
            start_user_data_stream(args).await
        }
    }
}

async fn account_funding_flow(mut args: AccountFundingFlowArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountFundingFlowParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountFundingFlowParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.currency.is_none() {
                        let options = vec![("USDT", AccountFundingFlowCurrencyEnum::Usdt)];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the currency")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.currency = Some(selected);
                    }
                }
                AccountFundingFlowParams::builder(
                    args.currency
                        .ok_or_else(|| anyhow::anyhow!("currency is required"))?,
                )
                .record_id(args.record_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.account_funding_flow(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn option_margin_account_information(
    args: OptionMarginAccountInformationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OptionMarginAccountInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<OptionMarginAccountInformationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => OptionMarginAccountInformationParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .option_margin_account_information(params)
        .await?;

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

async fn exchange_information(args: ExchangeInformationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.exchange_information().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn historical_exercise_records(args: HistoricalExerciseRecordsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<HistoricalExerciseRecordsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<HistoricalExerciseRecordsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => HistoricalExerciseRecordsParams::builder()
                .underlying(args.underlying)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.historical_exercise_records(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn index_price(mut args: IndexPriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<IndexPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<IndexPriceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.underlying.is_none() {
                        let underlying: String = Input::new()
                            .with_prompt("Input underlying:")
                            .interact_text()?;

                        args.underlying = Some(underlying);
                    }
                }
                IndexPriceParams::builder(
                    args.underlying
                        .ok_or_else(|| anyhow::anyhow!("underlying is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.index_price(params).await?;

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
                    if args.underlying_asset.is_none() {
                        let underlying_asset: String = Input::new()
                            .with_prompt("Input underlying_asset:")
                            .interact_text()?;

                        args.underlying_asset = Some(underlying_asset);
                    }
                    if args.expiration.is_none() {
                        let expiration: String = Input::new()
                            .with_prompt("Input expiration:")
                            .interact_text()?;

                        args.expiration = Some(expiration);
                    }
                }
                OpenInterestParams::builder(
                    args.underlying_asset
                        .ok_or_else(|| anyhow::anyhow!("underlying_asset is required"))?,
                    args.expiration
                        .ok_or_else(|| anyhow::anyhow!("expiration is required"))?,
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

async fn option_mark_price(args: OptionMarkPriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<OptionMarkPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<OptionMarkPriceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => OptionMarkPriceParams::builder()
                .symbol(args.symbol)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.option_mark_price(params).await?;

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

async fn recent_block_trades_list(args: RecentBlockTradesListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<RecentBlockTradesListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RecentBlockTradesListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => RecentBlockTradesListParams::builder()
                .symbol(args.symbol)
                .limit(args.limit)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.recent_block_trades_list(params).await?;

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

async fn accept_block_trade_order(mut args: AcceptBlockTradeOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AcceptBlockTradeOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AcceptBlockTradeOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.block_order_matching_key.is_none() {
                        let block_order_matching_key: String = Input::new()
                            .with_prompt("Input block_order_matching_key:")
                            .interact_text()?;

                        args.block_order_matching_key = Some(block_order_matching_key);
                    }
                }
                AcceptBlockTradeOrderParams::builder(
                    args.block_order_matching_key
                        .ok_or_else(|| anyhow::anyhow!("block_order_matching_key is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.accept_block_trade_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn account_block_trade_list(args: AccountBlockTradeListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountBlockTradeListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountBlockTradeListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountBlockTradeListParams::builder()
                .end_time(args.end_time)
                .start_time(args.start_time)
                .underlying(args.underlying)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_block_trade_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_block_trade_order(mut args: CancelBlockTradeOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelBlockTradeOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelBlockTradeOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.block_order_matching_key.is_none() {
                        let block_order_matching_key: String = Input::new()
                            .with_prompt("Input block_order_matching_key:")
                            .interact_text()?;

                        args.block_order_matching_key = Some(block_order_matching_key);
                    }
                }
                CancelBlockTradeOrderParams::builder(
                    args.block_order_matching_key
                        .ok_or_else(|| anyhow::anyhow!("block_order_matching_key is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_block_trade_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn extend_block_trade_order(mut args: ExtendBlockTradeOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ExtendBlockTradeOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ExtendBlockTradeOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.block_order_matching_key.is_none() {
                        let block_order_matching_key: String = Input::new()
                            .with_prompt("Input block_order_matching_key:")
                            .interact_text()?;

                        args.block_order_matching_key = Some(block_order_matching_key);
                    }
                }
                ExtendBlockTradeOrderParams::builder(
                    args.block_order_matching_key
                        .ok_or_else(|| anyhow::anyhow!("block_order_matching_key is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.extend_block_trade_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_block_trade_order(mut args: NewBlockTradeOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewBlockTradeOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewBlockTradeOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.liquidity.is_none() {
                        let options = vec![
                            ("MAKER", NewBlockTradeOrderLiquidityEnum::Maker),
                            ("TAKER", NewBlockTradeOrderLiquidityEnum::Taker),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the liquidity")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.liquidity = Some(selected);
                    }
                    if args.legs.is_none() {
                        let legs: String =
                            Input::new().with_prompt("Input legs:").interact_text()?;

                        args.legs = Some(legs);
                    }
                }
                NewBlockTradeOrderParams::builder(
                    args.liquidity
                        .ok_or_else(|| anyhow::anyhow!("liquidity is required"))?,
                    serde_json::from_str::<Vec<models::NewBlockTradeOrderLegsParameterInner>>(
                        &args
                            .legs
                            .ok_or_else(|| anyhow::anyhow!("legs is required"))?,
                    )?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.new_block_trade_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_block_trade_details(mut args: QueryBlockTradeDetailsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryBlockTradeDetailsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryBlockTradeDetailsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.block_order_matching_key.is_none() {
                        let block_order_matching_key: String = Input::new()
                            .with_prompt("Input block_order_matching_key:")
                            .interact_text()?;

                        args.block_order_matching_key = Some(block_order_matching_key);
                    }
                }
                QueryBlockTradeDetailsParams::builder(
                    args.block_order_matching_key
                        .ok_or_else(|| anyhow::anyhow!("block_order_matching_key is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_block_trade_details(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_block_trade_order(args: QueryBlockTradeOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryBlockTradeOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryBlockTradeOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryBlockTradeOrderParams::builder()
                .block_order_matching_key(args.block_order_matching_key)
                .end_time(args.end_time)
                .start_time(args.start_time)
                .underlying(args.underlying)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_block_trade_order(params).await?;

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
                    if args.underlyings.is_none() {
                        let underlyings: String = Input::new()
                            .with_prompt("Input underlyings:")
                            .interact_text()?;

                        args.underlyings = Some(underlyings);
                    }
                }
                AutoCancelAllOpenOrdersParams::builder(
                    args.underlyings
                        .ok_or_else(|| anyhow::anyhow!("underlyings is required"))?,
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

async fn get_auto_cancel_all_open_orders(
    args: GetAutoCancelAllOpenOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetAutoCancelAllOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetAutoCancelAllOpenOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetAutoCancelAllOpenOrdersParams::builder()
                .underlying(args.underlying)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_auto_cancel_all_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_market_maker_protection_config(
    mut args: GetMarketMakerProtectionConfigArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetMarketMakerProtectionConfigParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetMarketMakerProtectionConfigParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.underlying.is_none() {
                        let underlying: String = Input::new()
                            .with_prompt("Input underlying:")
                            .interact_text()?;

                        args.underlying = Some(underlying);
                    }
                }
                GetMarketMakerProtectionConfigParams::builder(
                    args.underlying
                        .ok_or_else(|| anyhow::anyhow!("underlying is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_market_maker_protection_config(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn reset_market_maker_protection_config(
    mut args: ResetMarketMakerProtectionConfigArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ResetMarketMakerProtectionConfigParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ResetMarketMakerProtectionConfigParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.underlying.is_none() {
                        let underlying: String = Input::new()
                            .with_prompt("Input underlying:")
                            .interact_text()?;

                        args.underlying = Some(underlying);
                    }
                }
                ResetMarketMakerProtectionConfigParams::builder(
                    args.underlying
                        .ok_or_else(|| anyhow::anyhow!("underlying is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .reset_market_maker_protection_config(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_auto_cancel_all_open_orders(
    mut args: SetAutoCancelAllOpenOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetAutoCancelAllOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SetAutoCancelAllOpenOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.underlying.is_none() {
                        let underlying: String = Input::new()
                            .with_prompt("Input underlying:")
                            .interact_text()?;

                        args.underlying = Some(underlying);
                    }
                    if args.countdown_time.is_none() {
                        let countdown_time: i64 = Input::new()
                            .with_prompt("Input countdown_time:")
                            .interact_text()?;

                        args.countdown_time = Some(countdown_time);
                    }
                }
                SetAutoCancelAllOpenOrdersParams::builder(
                    args.underlying
                        .ok_or_else(|| anyhow::anyhow!("underlying is required"))?,
                    args.countdown_time
                        .ok_or_else(|| anyhow::anyhow!("countdown_time is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.set_auto_cancel_all_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_market_maker_protection_config(
    mut args: SetMarketMakerProtectionConfigArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetMarketMakerProtectionConfigParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SetMarketMakerProtectionConfigParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.underlying.is_none() {
                        let underlying: String = Input::new()
                            .with_prompt("Input underlying:")
                            .interact_text()?;

                        args.underlying = Some(underlying);
                    }
                    if args.window_time_in_milliseconds.is_none() {
                        let window_time_in_milliseconds: i64 = Input::new()
                            .with_prompt("Input window_time_in_milliseconds:")
                            .interact_text()?;

                        args.window_time_in_milliseconds = Some(window_time_in_milliseconds);
                    }
                    if args.frozen_time_in_milliseconds.is_none() {
                        let frozen_time_in_milliseconds: i64 = Input::new()
                            .with_prompt("Input frozen_time_in_milliseconds:")
                            .interact_text()?;

                        args.frozen_time_in_milliseconds = Some(frozen_time_in_milliseconds);
                    }
                    if args.qty_limit.is_none() {
                        let qty_limit: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input qty_limit:")
                            .interact_text()?;

                        args.qty_limit = Some(qty_limit);
                    }
                    if args.delta_limit.is_none() {
                        let delta_limit: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input delta_limit:")
                            .interact_text()?;

                        args.delta_limit = Some(delta_limit);
                    }
                }
                SetMarketMakerProtectionConfigParams::builder(
                    args.underlying
                        .ok_or_else(|| anyhow::anyhow!("underlying is required"))?,
                    args.window_time_in_milliseconds.ok_or_else(|| {
                        anyhow::anyhow!("window_time_in_milliseconds is required")
                    })?,
                    args.frozen_time_in_milliseconds.ok_or_else(|| {
                        anyhow::anyhow!("frozen_time_in_milliseconds is required")
                    })?,
                    args.qty_limit
                        .ok_or_else(|| anyhow::anyhow!("qty_limit is required"))?,
                    args.delta_limit
                        .ok_or_else(|| anyhow::anyhow!("delta_limit is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .set_market_maker_protection_config(params)
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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                }
                AccountTradeListParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .from_id(args.from_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
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

async fn cancel_all_option_orders_by_underlying(
    mut args: CancelAllOptionOrdersByUnderlyingArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllOptionOrdersByUnderlyingParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAllOptionOrdersByUnderlyingParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.underlying.is_none() {
                        let underlying: String = Input::new()
                            .with_prompt("Input underlying:")
                            .interact_text()?;

                        args.underlying = Some(underlying);
                    }
                }
                CancelAllOptionOrdersByUnderlyingParams::builder(
                    args.underlying
                        .ok_or_else(|| anyhow::anyhow!("underlying is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .cancel_all_option_orders_by_underlying(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_all_option_orders_on_specific_symbol(
    mut args: CancelAllOptionOrdersOnSpecificSymbolArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllOptionOrdersOnSpecificSymbolParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAllOptionOrdersOnSpecificSymbolParams>(json)
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
                CancelAllOptionOrdersOnSpecificSymbolParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .cancel_all_option_orders_on_specific_symbol(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_multiple_option_orders(
    mut args: CancelMultipleOptionOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelMultipleOptionOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelMultipleOptionOrdersParams>(json).ok_or_else(|| {
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
                CancelMultipleOptionOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_ids(args.order_ids)
                .client_order_ids(args.client_order_ids)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_multiple_option_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_option_order(mut args: CancelOptionOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelOptionOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelOptionOrderParams>(json).ok_or_else(|| {
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
                CancelOptionOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_id(args.order_id)
                .client_order_id(args.client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_option_order(params).await?;

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
                        let options = vec![("LIMIT", NewOrderTypeEnum::Limit)];

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
                NewOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                )
                .price(args.price)
                .time_in_force(args.time_in_force)
                .reduce_only(args.reduce_only)
                .post_only(args.post_only)
                .new_order_resp_type(args.new_order_resp_type)
                .client_order_id(args.client_order_id)
                .is_mmp(args.is_mmp)
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

async fn option_position_information(args: OptionPositionInformationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OptionPositionInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<OptionPositionInformationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => OptionPositionInformationParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.option_position_information(params).await?;

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
                    if args.orders.is_none() {
                        let orders: String =
                            Input::new().with_prompt("Input orders:").interact_text()?;

                        args.orders = Some(orders);
                    }
                }
                PlaceMultipleOrdersParams::builder(serde_json::from_str::<
                    Vec<models::PlaceMultipleOrdersOrdersParameterInner>,
                >(
                    &args
                        .orders
                        .ok_or_else(|| anyhow::anyhow!("orders is required"))?,
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

async fn query_current_open_option_orders(
    args: QueryCurrentOpenOptionOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentOpenOptionOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryCurrentOpenOptionOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryCurrentOpenOptionOrdersParams::builder()
                .symbol(args.symbol)
                .order_id(args.order_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_current_open_option_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_option_order_history(mut args: QueryOptionOrderHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryOptionOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryOptionOrderHistoryParams>(json).ok_or_else(|| {
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
                QueryOptionOrderHistoryParams::builder(
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
    let response = rest_client.query_option_order_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_single_order(mut args: QuerySingleOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySingleOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySingleOrderParams>(json).ok_or_else(|| {
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
                QuerySingleOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .order_id(args.order_id)
                .client_order_id(args.client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_single_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn tradfi_options_contract(args: TradfiOptionsContractArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TradfiOptionsContractParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TradfiOptionsContractParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => TradfiOptionsContractParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.tradfi_options_contract(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn user_commission(args: UserCommissionArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UserCommissionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UserCommissionParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => UserCommissionParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.user_commission(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn user_exercise_record(args: UserExerciseRecordArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UserExerciseRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UserExerciseRecordParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => UserExerciseRecordParams::builder()
                .symbol(args.symbol)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.user_exercise_record(params).await?;

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
