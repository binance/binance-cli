use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::{
    DERIVATIVES_TRADING_PORTFOLIO_MARGIN_REST_API_PROD_URL,
    DERIVATIVES_TRADING_PORTFOLIO_MARGIN_REST_API_TESTNET_URL,
};
use binance_sdk::derivatives_trading_portfolio_margin::DerivativesTradingPortfolioMarginRestApi;
use binance_sdk::derivatives_trading_portfolio_margin::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("derivatives-trading-portfolio-margin");

    let client_config =
        get_client_configuration(profile, "derivatives-trading-portfolio-margin").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "testnet" | "demo" => DERIVATIVES_TRADING_PORTFOLIO_MARGIN_REST_API_TESTNET_URL.to_string(),
        "prod" => DERIVATIVES_TRADING_PORTFOLIO_MARGIN_REST_API_PROD_URL.to_string(),
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

    Ok(DerivativesTradingPortfolioMarginRestApi::from_config(
        rest_conf,
    ))
}

#[derive(Args, Debug)]
struct AccountBalanceArgs {
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
struct BnbTransferArgs {
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    transfer_side: Option<BnbTransferTransferSideEnum>,
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
struct ChangeAutoRepayFuturesStatusArgs {
    #[arg(
        help = r#"`false` for turn off the auto-repay futures negative balance function"#,
        long
    )]
    auto_repay: Option<ChangeAutoRepayFuturesStatusAutoRepayEnum>,
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
struct ChangeCmInitialLeverageArgs {
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
struct ChangeCmPositionModeArgs {
    #[arg(help = r#""true": Hedge Mode; "false": One-way Mode"#, long)]
    dual_side_position: Option<ChangeCmPositionModeDualSidePositionEnum>,
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
struct ChangeUmInitialLeverageArgs {
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
struct ChangeUmPositionModeArgs {
    #[arg(help = r#""true": Hedge Mode; "false": One-way Mode"#, long)]
    dual_side_position: Option<ChangeUmPositionModeDualSidePositionEnum>,
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
struct CmNotionalAndLeverageBracketsArgs {
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
struct FundAutoCollectionArgs {
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
struct FundCollectionByAssetArgs {
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
struct GetAutoRepayFuturesStatusArgs {
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
struct GetCmAccountDetailArgs {
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
struct GetCmCurrentPositionModeArgs {
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
struct GetCmIncomeHistoryArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    income_type: Option<GetCmIncomeHistoryIncomeTypeEnum>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct GetDownloadIdForUmFuturesOrderHistoryArgs {
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
struct GetDownloadIdForUmFuturesTradeHistoryArgs {
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
struct GetDownloadIdForUmFuturesTransactionHistoryArgs {
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
struct GetMarginBorrowLoanInterestHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current page number."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    size: Option<i64>,
    #[arg(
        help = r#"Set to true to query archived data from 6 months ago."#,
        long
    )]
    archived: Option<GetMarginBorrowLoanInterestHistoryArchivedEnum>,
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
struct GetUmAccountDetailArgs {
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
struct GetUmAccountDetailV2Args {
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
struct GetUmCurrentPositionModeArgs {
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
struct GetUmFuturesOrderDownloadLinkByIdArgs {
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
struct GetUmFuturesTradeDownloadLinkByIdArgs {
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
struct GetUmFuturesTransactionDownloadLinkByIdArgs {
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
struct GetUmIncomeHistoryArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Income type."#, long)]
    income_type: Option<GetUmIncomeHistoryIncomeTypeEnum>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Page number."#, long)]
    page: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct GetUserCommissionRateForCmArgs {
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
struct GetUserCommissionRateForUmArgs {
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
struct MarginMaxBorrowArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
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
struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsArgs {
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
struct QueryCmPositionInformationArgs {
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
struct QueryMarginLoanRecordArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"the `tranId` in `POST/papi/v1/marginLoan`"#, long)]
    tx_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current page number."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    size: Option<i64>,
    #[arg(
        help = r#"Set to true to query archived data from 6 months ago."#,
        long
    )]
    archived: Option<QueryMarginLoanRecordArchivedEnum>,
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
struct QueryMarginMaxWithdrawArgs {
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
struct QueryMarginRepayRecordArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"the `tranId` in `POST /papi/v1/repayLoan`"#, long)]
    tx_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current page number."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    size: Option<i64>,
    #[arg(
        help = r#"Set to true to query archived data from 6 months ago."#,
        long
    )]
    archived: Option<QueryMarginRepayRecordArchivedEnum>,
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
struct QueryPortfolioMarginNegativeBalanceInterestHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct QueryUmPositionInformationArgs {
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
struct QueryUserNegativeBalanceAutoExchangeRecordArgs {
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
struct RepayFuturesNegativeBalanceArgs {
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
struct UmFuturesAccountConfigurationArgs {
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
struct UmFuturesSymbolConfigurationArgs {
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
struct UmNotionalAndLeverageBracketsArgs {
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
struct TestConnectivityArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelAllCmOpenConditionalOrdersArgs {
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
struct CancelAllCmOpenOrdersArgs {
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
struct CancelAllUmAlgoOpenOrdersArgs {
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
struct CancelAllUmOpenConditionalOrdersArgs {
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
struct CancelAllUmOpenOrdersArgs {
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
struct CancelCmConditionalOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#""#, long)]
    new_client_strategy_id: Option<String>,
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
struct CancelCmOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
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
struct CancelMarginAccountAllOpenOrdersOnASymbolArgs {
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
struct CancelMarginAccountOcoOrdersArgs {
    #[arg(help = r#"Symbol"#, long)]
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
    #[arg(help = r#"Used to uniquely identify this cancel request."#, long)]
    new_client_order_id: Option<String>,
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
struct CancelMarginAccountOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
    #[arg(help = r#"Used to uniquely identify this cancel request."#, long)]
    new_client_order_id: Option<String>,
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
struct CancelUmAlgoOrderArgs {
    #[arg(help = r#"Algo order ID"#, long)]
    algo_id: Option<i64>,
    #[arg(help = r#"Client algo order ID"#, long)]
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
struct CancelUmConditionalOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#""#, long)]
    new_client_strategy_id: Option<String>,
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
struct CancelUmOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
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
struct CmAccountTradeListArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    pair: Option<String>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Trade ID to fetch from."#, long)]
    from_id: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct CmPositionAdlQuantileEstimationArgs {
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
struct GetUmFuturesBnbBurnStatusArgs {
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
struct MarginAccountBorrowArgs {
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
struct MarginAccountNewOcoArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"See enum definitions: order side"#, long)]
    side: Option<MarginAccountNewOcoSideEnum>,
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
    limit_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#"A unique Id for the stop loss/stop loss limit leg"#, long)]
    stop_client_order_id: Option<String>,
    #[arg(help = r#"If provided, stopLimitTimeInForce is required."#, long)]
    stop_limit_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    stop_iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Valid values are `GTC/FOK/IOC`"#, long)]
    stop_limit_time_in_force: Option<MarginAccountNewOcoStopLimitTimeInForceEnum>,
    #[arg(help = r#"Set the response JSON."#, long)]
    new_order_resp_type: Option<MarginAccountNewOcoNewOrderRespTypeEnum>,
    #[arg(
        help = r#"NO_SIDE_EFFECT, MARGIN_BUY, AUTO_REPAY; default NO_SIDE_EFFECT."#,
        long
    )]
    side_effect_type: Option<MarginAccountNewOcoSideEffectTypeEnum>,
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
struct MarginAccountRepayArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
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
struct MarginAccountRepayDebtArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<String>,
    #[arg(
        help = r#"Specific asset list to repay debt; Can be added in batch, separated by commas"#,
        long
    )]
    specify_repay_assets: Option<String>,
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
struct MarginAccountTradeListArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Trade ID to fetch from."#, long)]
    from_id: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Value cannot be greater than 60000"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ModifyCmOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<ModifyCmOrderSideEnum>,
    #[arg(help = r#"Order quantity"#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Order price"#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; can be set to `OPPONENT`/ `OPPONENT_5`/ `OPPONENT_10`/ `OPPONENT_20`: /`QUEUE`/ `QUEUE_5`/ `QUEUE_10`/ `QUEUE_20`; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<ModifyCmOrderPriceMatchEnum>,
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
struct ModifyUmOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<ModifyUmOrderSideEnum>,
    #[arg(help = r#"Order quantity"#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Order price"#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
    orig_client_order_id: Option<String>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; can be set to `OPPONENT`/ `OPPONENT_5`/ `OPPONENT_10`/ `OPPONENT_20`: /`QUEUE`/ `QUEUE_5`/ `QUEUE_10`/ `QUEUE_20`; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<ModifyUmOrderPriceMatchEnum>,
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
struct NewCmConditionalOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Side"#, long)]
    side: Option<NewCmConditionalOrderSideEnum>,
    #[arg(help = r#""#, long)]
    strategy_type: Option<NewCmConditionalOrderStrategyTypeEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<NewCmConditionalOrderPositionSideEnum>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewCmConditionalOrderTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#""true" or "false". default "false". Cannot be sent in Hedge Mode"#,
        long
    )]
    reduce_only: Option<String>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"stopPrice triggered by: "MARK_PRICE", "CONTRACT_PRICE". Default "CONTRACT_PRICE""#,
        long
    )]
    working_type: Option<NewCmConditionalOrderWorkingTypeEnum>,
    #[arg(
        help = r#""true" or "false", default "false". Used with `STOP`/`STOP_MARKET` or `TAKE_PROFIT`/`TAKE_PROFIT_MARKET` orders"#,
        long
    )]
    price_protect: Option<NewCmConditionalOrderPriceProtectEnum>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,36}$`"#,
        long
    )]
    new_client_strategy_id: Option<String>,
    #[arg(
        help = r#"Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders."#,
        long
    )]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Used with `TRAILING_STOP_MARKET` orders."#, long)]
    activation_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Used with `TRAILING_STOP_MARKET` orders."#, long)]
    callback_rate: Option<rust_decimal::Decimal>,
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
struct NewCmOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Side"#, long)]
    side: Option<NewCmOrderSideEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<NewCmOrderTypeEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<NewCmOrderPositionSideEnum>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewCmOrderTimeInForceEnum>,
    #[arg(help = r#"Place amount"#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#""true" or "false". Cannot be sent in Hedge Mode."#, long)]
    reduce_only: Option<NewCmOrderReduceOnlyEnum>,
    #[arg(help = r#"Order price"#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; can be set to `OPPONENT`/ `OPPONENT_5`/ `OPPONENT_10`/ `OPPONENT_20`: /`QUEUE`/ `QUEUE_5`/ `QUEUE_10`/ `QUEUE_20`; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<NewCmOrderPriceMatchEnum>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,32}$`"#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(help = r#""ACK", "RESULT", default "ACK""#, long)]
    new_order_resp_type: Option<NewCmOrderNewOrderRespTypeEnum>,
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
struct NewMarginOrderArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<NewMarginOrderSideEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<NewMarginOrderTypeEnum>,
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
    #[arg(help = r#"Set the response JSON. ACK, RESULT, or FULL."#, long)]
    new_order_resp_type: Option<NewMarginOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"Used with `LIMIT`, `STOP_LOSS_LIMIT`, and `TAKE_PROFIT_LIMIT` to create an iceberg order"#,
        long
    )]
    iceberg_qty: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    side_effect_type: Option<NewMarginOrderSideEffectTypeEnum>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewMarginOrderTimeInForceEnum>,
    #[arg(
        help = r#"`NONE`:No STP / `EXPIRE_TAKER`:expire taker order when STP triggers/ `EXPIRE_MAKER`:expire taker order when STP triggers/ `EXPIRE_BOTH`:expire both orders when STP triggers"#,
        long
    )]
    self_trade_prevention_mode: Option<NewMarginOrderSelfTradePreventionModeEnum>,
    #[arg(help = r#"Only when MARGIN_BUY or AUTO_BORROW_REPAY order takes effect, true means that the debt generated by the order needs to be repaid after the order is cancelled."#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_repay_at_cancel: Option<bool>,
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
struct NewUmAlgoOrderArgs {
    #[arg(help = r#"Only support `CONDITIONAL`"#, long)]
    algo_type: Option<NewUmAlgoOrderAlgoTypeEnum>,
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<NewUmAlgoOrderSideEnum>,
    #[arg(help = r#"Conditional order type"#, long)]
    r#type: Option<NewUmAlgoOrderTypeEnum>,
    #[arg(help = r#"Order quantity"#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode; `LONG` or `SHORT` for Hedge Mode"#,
        long
    )]
    position_side: Option<NewUmAlgoOrderPositionSideEnum>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewUmAlgoOrderTimeInForceEnum>,
    #[arg(help = r#"Order price"#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Trigger price"#, long)]
    trigger_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Trigger price type. Default `CONTRACT_PRICE`"#, long)]
    working_type: Option<NewUmAlgoOrderWorkingTypeEnum>,
    #[arg(help = r#"Can't be passed together with `price`"#, long)]
    price_match: Option<NewUmAlgoOrderPriceMatchEnum>,
    #[arg(help = r#"Price protection. Default `false`"#, long)]
    price_protect: Option<NewUmAlgoOrderPriceProtectEnum>,
    #[arg(help = r#"Cannot be sent in Hedge Mode"#, long)]
    reduce_only: Option<NewUmAlgoOrderReduceOnlyEnum>,
    #[arg(
        help = r#"Used with `TRAILING_STOP_MARKET`, default as latest price"#,
        long
    )]
    activate_price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Used with `TRAILING_STOP_MARKET`, min 0.1, max 10 (1 = 1%)"#,
        long
    )]
    callback_rate: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Unique id among open orders. Auto-generated if not sent"#,
        long
    )]
    client_algo_id: Option<String>,
    #[arg(help = r#""#, long)]
    new_order_resp_type: Option<NewUmAlgoOrderNewOrderRespTypeEnum>,
    #[arg(help = r#""#, long)]
    self_trade_prevention_mode: Option<NewUmAlgoOrderSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"Order cancel time for `GTD` timeInForce, mandatory when timeInForce is `GTD`"#,
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
struct NewUmConditionalOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<NewUmConditionalOrderSideEnum>,
    #[arg(help = r#""#, long)]
    strategy_type: Option<NewUmConditionalOrderStrategyTypeEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<NewUmConditionalOrderPositionSideEnum>,
    #[arg(help = r#""#, long)]
    time_in_force: Option<NewUmConditionalOrderTimeInForceEnum>,
    #[arg(help = r#""#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#""true" or "false". Cannot be sent in Hedge Mode ; cannot be sent with `closePosition`=`true`"#,
        long
    )]
    reduce_only: Option<NewUmConditionalOrderReduceOnlyEnum>,
    #[arg(help = r#""#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"stopPrice triggered by: "MARK_PRICE", "CONTRACT_PRICE". Default "CONTRACT_PRICE""#,
        long
    )]
    working_type: Option<NewUmConditionalOrderWorkingTypeEnum>,
    #[arg(
        help = r#""true" or "false". Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders"#,
        long
    )]
    price_protect: Option<NewUmConditionalOrderPriceProtectEnum>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,32}$`"#,
        long
    )]
    new_client_strategy_id: Option<String>,
    #[arg(
        help = r#"Used with `STOP/STOP_MARKET` or `TAKE_PROFIT/TAKE_PROFIT_MARKET` orders."#,
        long
    )]
    stop_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Used with `TRAILING_STOP_MARKET` orders."#, long)]
    activation_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Used with `TRAILING_STOP_MARKET` orders."#, long)]
    callback_rate: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; can be set to `OPPONENT`/ `OPPONENT_5`/ `OPPONENT_10`/ `OPPONENT_20`: /`QUEUE`/ `QUEUE_5`/ `QUEUE_10`/ `QUEUE_20`; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<NewUmConditionalOrderPriceMatchEnum>,
    #[arg(
        help = r#"`NONE`:No STP / `EXPIRE_TAKER`:expire taker order when STP triggers/ `EXPIRE_MAKER`:expire taker order when STP triggers/ `EXPIRE_BOTH`:expire both orders when STP triggers"#,
        long
    )]
    self_trade_prevention_mode: Option<NewUmConditionalOrderSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"order cancel time for timeInForce `GTD`, mandatory when `timeInforce` set to `GTD`; order the timestamp only retains second-level precision, ms part will be ignored; The goodTillDate timestamp must be greater than the current time plus 600 seconds and smaller than 253402300799000Mode. It must be sent in Hedge Mode."#,
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
struct NewUmOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<NewUmOrderSideEnum>,
    #[arg(help = r#""#, long)]
    r#type: Option<NewUmOrderTypeEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<NewUmOrderPositionSideEnum>,
    #[arg(help = r#"Valid values"#, long)]
    time_in_force: Option<NewUmOrderTimeInForceEnum>,
    #[arg(help = r#"Place amount"#, long)]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#""true" or "false". default "false". Cannot be sent in Hedge Mode ."#,
        long
    )]
    reduce_only: Option<NewUmOrderReduceOnlyEnum>,
    #[arg(help = r#"Order price"#, long)]
    price: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"A unique id among open orders. Automatically generated if not sent. Can only be string following the rule: `^[\.A-Z\:/a-z0-9_-]{1,32}$`"#,
        long
    )]
    new_client_order_id: Option<String>,
    #[arg(help = r#"`ACK`, `RESULT`, default `ACK`"#, long)]
    new_order_resp_type: Option<NewUmOrderNewOrderRespTypeEnum>,
    #[arg(
        help = r#"only avaliable for `LIMIT`/`STOP`/`TAKE_PROFIT` order; can be set to `OPPONENT`/ `OPPONENT_5`/ `OPPONENT_10`/ `OPPONENT_20`: /`QUEUE`/ `QUEUE_5`/ `QUEUE_10`/ `QUEUE_20`; Can't be passed together with `price`"#,
        long
    )]
    price_match: Option<NewUmOrderPriceMatchEnum>,
    #[arg(
        help = r#"`NONE`:No STP / `EXPIRE_TAKER`:expire taker order when STP triggers/ `EXPIRE_MAKER`:expire taker order when STP triggers/ `EXPIRE_BOTH`:expire both orders when STP triggers"#,
        long
    )]
    self_trade_prevention_mode: Option<NewUmOrderSelfTradePreventionModeEnum>,
    #[arg(
        help = r#"order cancel time for timeInForce `GTD`, mandatory when `timeInforce` set to `GTD`; order the timestamp only retains second-level precision, ms part will be ignored; The goodTillDate timestamp must be greater than the current time plus 600 seconds and smaller than 253402300799000Mode. It must be sent in Hedge Mode."#,
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
struct QueryAllCmConditionalOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct QueryAllCmOrdersArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    pair: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct QueryAllCurrentCmOpenConditionalOrdersArgs {
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
struct QueryAllCurrentCmOpenOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
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
struct QueryAllCurrentUmOpenAlgoOrdersArgs {
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
struct QueryAllCurrentUmOpenConditionalOrdersArgs {
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
struct QueryAllCurrentUmOpenOrdersArgs {
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
struct QueryAllMarginAccountOrdersArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Value cannot be greater than 60000"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryAllUmConditionalOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct QueryAllUmOrdersArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct QueryCmConditionalOrderHistoryArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#""#, long)]
    new_client_strategy_id: Option<String>,
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
struct QueryCmModifyOrderHistoryArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
    orig_client_order_id: Option<String>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct QueryCmOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
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
struct QueryCurrentCmOpenConditionalOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#""#, long)]
    new_client_strategy_id: Option<String>,
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
struct QueryCurrentCmOpenOrderArgs {
    #[arg(help = r#"Trading pair."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID."#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"User-defined order ID."#, long)]
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
struct QueryCurrentMarginOpenOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Value cannot be greater than 60000"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryCurrentUmOpenAlgoOrderArgs {
    #[arg(help = r#"Algo order ID"#, long)]
    algo_id: Option<i64>,
    #[arg(help = r#"Client algo order ID"#, long)]
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
struct QueryCurrentUmOpenConditionalOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#""#, long)]
    new_client_strategy_id: Option<String>,
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
struct QueryCurrentUmOpenOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
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
struct QueryMarginAccountOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    orig_client_order_id: Option<String>,
    #[arg(help = r#"Value cannot be greater than 60000"#, long)]
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
    #[arg(help = r#"Trade ID to fetch from."#, long)]
    from_id: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Value cannot be greater than 60000"#, long)]
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
    #[arg(
        help = r#"Either `orderListId` or `listClientOrderId` must be provided"#,
        long
    )]
    order_list_id: Option<i64>,
    #[arg(
        help = r#"`orderListId` or `listClientOrderId` must be provided."#,
        long
    )]
    orig_client_order_id: Option<String>,
    #[arg(help = r#"Value cannot be greater than 60000"#, long)]
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
    #[arg(help = r#"Value cannot be greater than 60000"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryUmAlgoOrderHistoryArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Only return orders >= this algoId"#, long)]
    algo_id: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Default 500; max 1000"#, long)]
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
struct QueryUmConditionalOrderHistoryArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    strategy_id: Option<i64>,
    #[arg(help = r#""#, long)]
    new_client_strategy_id: Option<String>,
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
struct QueryUmModifyOrderHistoryArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order ID"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"Client order ID"#, long)]
    orig_client_order_id: Option<String>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct QueryUmOrderArgs {
    #[arg(help = r#"Symbol"#, long)]
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
struct QueryUsersCmForceOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"`LIQUIDATION` for liquidation orders, `ADL` for ADL orders."#,
        long
    )]
    auto_close_type: Option<QueryUsersCmForceOrdersAutoCloseTypeEnum>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    limit: Option<i64>,
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
struct QueryUsersMarginForceOrdersArgs {
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current page number."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct QueryUsersUmForceOrdersArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
    #[arg(
        help = r#"`LIQUIDATION` for liquidation orders, `ADL` for ADL orders."#,
        long
    )]
    auto_close_type: Option<QueryUsersUmForceOrdersAutoCloseTypeEnum>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    limit: Option<i64>,
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
struct ToggleBnbBurnOnUmFuturesTradeArgs {
    #[arg(help = r#""true": Fee Discount On; "false": Fee Discount Off"#, long)]
    fee_burn: Option<ToggleBnbBurnOnUmFuturesTradeFeeBurnEnum>,
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
struct UmAccountTradeListArgs {
    #[arg(help = r#"Symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Timestamp in ms to get funding from INCLUSIVE."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Timestamp in ms to get funding until INCLUSIVE."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Trade ID to fetch from."#, long)]
    from_id: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
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
struct UmPositionAdlQuantileEstimationArgs {
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
pub enum DerivativesTradingPortfolioMarginCommands {
    #[command(
        about = decode_selected_entities(r#"Query account balance

Weight(IP): 20

Security Type: USER_DATA"#, false),
    )]
    AccountBalance(AccountBalanceArgs),
    #[command(
        about = decode_selected_entities(r#"Query account information

Weight(IP): 20

Security Type: USER_DATA"#, false),
    )]
    AccountInformation(AccountInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Transfer BNB in and out of UM

Weight(IP): 750

Security Type: TRADE

Notes:
- The endpoint can only be called 10 times per 10 minutes in a rolling manner"#, false),
    )]
    BnbTransfer(BnbTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Change Auto-repay-futures Status

Weight(IP): 750

Security Type: TRADE"#, false),
    )]
    ChangeAutoRepayFuturesStatus(ChangeAutoRepayFuturesStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's initial leverage of specific symbol in CM.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeCmInitialLeverage(ChangeCmInitialLeverageArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's position mode (Hedge Mode or One-way Mode ) on EVERY symbol in CM

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeCmPositionMode(ChangeCmPositionModeArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's initial leverage of specific symbol in UM.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeUmInitialLeverage(ChangeUmInitialLeverageArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's position mode (Hedge Mode or One-way Mode ) on EVERY symbol in UM

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    ChangeUmPositionMode(ChangeUmPositionModeArgs),
    #[command(
        about = decode_selected_entities(r#"Query CM notional and leverage brackets

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    CmNotionalAndLeverageBrackets(CmNotionalAndLeverageBracketsArgs),
    #[command(
        about = decode_selected_entities(r#"Fund collection for Portfolio Margin

Weight(IP): 750

Security Type: TRADE

Notes:
- BNB assets will not be auto-collected.
- Rolling window endpoint can be called at most 500 times per hour."#, false),
    )]
    FundAutoCollection(FundAutoCollectionArgs),
    #[command(
        about = decode_selected_entities(r#"Transfers specific asset from Futures Account to Margin account

Weight(IP): 30

Security Type: TRADE

Notes:
- The BNB transfer is not be supported"#, false),
    )]
    FundCollectionByAsset(FundCollectionByAssetArgs),
    #[command(
        about = decode_selected_entities(r#"Query Auto-repay-futures Status

Weight(IP): 30

Security Type: USER_DATA"#, false),
    )]
    GetAutoRepayFuturesStatus(GetAutoRepayFuturesStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get current CM account asset and position information.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    GetCmAccountDetail(GetCmAccountDetailArgs),
    #[command(
        about = decode_selected_entities(r#"Get user's position mode (Hedge Mode or One-way Mode ) on EVERY symbol in CM

Weight(IP): 30

Security Type: USER_DATA"#, false),
    )]
    GetCmCurrentPositionMode(GetCmCurrentPositionModeArgs),
    #[command(
        about = decode_selected_entities(r#"Get CM Income History.

Weight(IP): 30

Security Type: USER_DATA

Notes:
- If `incomeType` is not sent, all kinds of flow will be returned
- "trandId" is unique in the same "incomeType" for a user
- The interval between `startTime` and `endTime` can not exceed 200 days:
  - If `startTime` and `endTime` are not sent, the last 200 days will be returned"#, false),
    )]
    GetCmIncomeHistory(GetCmIncomeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get download id for UM futures order history

Weight(IP): 1500

Security Type: USER_DATA

Notes:
- Request Limitation is 10 times per month, shared by front end download page and rest api
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForUmFuturesOrderHistory(GetDownloadIdForUmFuturesOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get download id for UM futures trade history

Weight(IP): 1500

Security Type: USER_DATA

Notes:
- Request Limitation is 5 times per month, shared by front end download page and rest api
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForUmFuturesTradeHistory(GetDownloadIdForUmFuturesTradeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get download id for UM futures transaction history

Weight(IP): 1500

Security Type: USER_DATA

Notes:
- Request Limitation is 5 times per month, shared by front end download page and rest api
- The time between `startTime` and `endTime` can not be longer than 1 year"#, false),
    )]
    GetDownloadIdForUmFuturesTransactionHistory(GetDownloadIdForUmFuturesTransactionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Margin Borrow/Loan Interest History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Response in descending order
- The max interval between startTime and endTime is 30 days. It is a MUST to ensure data correctness.
- If `startTime` and `endTime` not sent, return records of the last 7 days by default
- If `startTime` is sent and `endTime` is not sent, the records from `startTime` to the present will be returned; if `startTime` is more than 30 days ago, the records of the past 30 days will be returned.
- If `startTime` is not sent and `endTime` is sent, the records of the 7 days before `endTime` is returned.
- Type in response has 5 enums:
  - `PERIODIC` interest charged per hour
  - `ON_BORROW` first interest charged on borrow
  - `PERIODIC_CONVERTED` interest charged per hour converted into BNB
  - `ON_BORROW_CONVERTED` first interest charged on borrow converted into BNB
  - `PORTFOLIO` Portfolio Margin negative balance daily interest"#, false),
    )]
    GetMarginBorrowLoanInterestHistory(GetMarginBorrowLoanInterestHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get current UM account asset and position information.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    GetUmAccountDetail(GetUmAccountDetailArgs),
    #[command(
        about = decode_selected_entities(r#"Get current UM account asset and position information.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    GetUmAccountDetailV2(GetUmAccountDetailV2Args),
    #[command(
        about = decode_selected_entities(r#"Get user's position mode (Hedge Mode or One-way Mode ) on EVERY symbol in UM

Weight(IP): 30

Security Type: USER_DATA"#, false),
    )]
    GetUmCurrentPositionMode(GetUmCurrentPositionModeArgs),
    #[command(
        about = decode_selected_entities(r#"Get UM futures order download link by Id

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetUmFuturesOrderDownloadLinkById(GetUmFuturesOrderDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Get UM futures trade download link by Id

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetUmFuturesTradeDownloadLinkById(GetUmFuturesTradeDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Get UM futures Transaction download link by Id

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Download link expiration: 7 days"#, false),
    )]
    GetUmFuturesTransactionDownloadLinkById(GetUmFuturesTransactionDownloadLinkByIdArgs),
    #[command(
        about = decode_selected_entities(r#"Get UM Income History.

Weight(IP): 30

Security Type: USER_DATA

Notes:
- If neither `startTime` nor `endTime` is sent, the recent 7-day data will be returned.
- If `incomeType` is not sent, all kinds of flow will be returned
- "trandId" is unique in the same incomeType for a user
- Income history only contains data for the last three months"#, false),
    )]
    GetUmIncomeHistory(GetUmIncomeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get User Commission Rate for CM

Weight(IP): 20

Security Type: USER_DATA"#, false),
    )]
    GetUserCommissionRateForCm(GetUserCommissionRateForCmArgs),
    #[command(
        about = decode_selected_entities(r#"Get User Commission Rate for UM

Weight(IP): 20

Security Type: USER_DATA"#, false),
    )]
    GetUserCommissionRateForUm(GetUserCommissionRateForUmArgs),
    #[command(
        about = decode_selected_entities(r#"Query margin max borrow

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    MarginMaxBorrow(MarginMaxBorrowArgs),
    #[command(
        about = decode_selected_entities(r#"Portfolio Margin UM Trading Quantitative Rules Indicators

Weight: - 1 for a single `symbol`
- 10 when `symbol` is omitted

Security Type: USER_DATA"#, false),
    )]
    PortfolioMarginUmTradingQuantitativeRulesIndicators(
        PortfolioMarginUmTradingQuantitativeRulesIndicatorsArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Get current CM position information.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- If neither `marginAsset` nor `pair` is sent, positions of all symbols with `TRADING` status will be returned.
- for One-way Mode user, the response will only show the "BOTH" positions
- for Hedge Mode user, the response will show "LONG", and "SHORT" positions. **Note**
- Please use with user data stream `ACCOUNT_UPDATE` to meet your timeliness and accuracy needs."#, false),
    )]
    QueryCmPositionInformation(QueryCmPositionInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Query margin loan record

Weight(IP): 10

Security Type: USER_DATA

Notes:
- txId or startTime must be sent. txId takes precedence.
- Response in descending order
- The max interval between `startTime` and `endTime` is 30 days.
- If `startTime` and `endTime` not sent, return records of the last 7 days by default
- Set `archived` to `true` to query data from 6 months ago"#, false),
    )]
    QueryMarginLoanRecord(QueryMarginLoanRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Max Withdraw

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    QueryMarginMaxWithdraw(QueryMarginMaxWithdrawArgs),
    #[command(
        about = decode_selected_entities(r#"Query margin repay record.

Weight(IP): 10

Security Type: USER_DATA

Notes:
- txId or startTime must be sent. txId takes precedence.
- Response in descending order
- The max interval between `startTime` and `endTime` is 30 days.
- If `startTime` and `endTime` not sent, return records of the last 7 days by default
- Set `archived` to `true` to query data from 6 months ago"#, false),
    )]
    QueryMarginRepayRecord(QueryMarginRepayRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Query interest history of negative balance for portfolio margin.

Weight(IP): 50

Security Type: USER_DATA

Notes:
- Results are returned in descending order.
- The query range cannot exceed 30 days to ensure data correctness.
- If both `startTime` and `endTime` are omitted, the most recent 7 days are returned by default.
- If `startTime` is provided but `endTime` is omitted, records from `startTime` to now are returned; if that exceeds 30 days, only the most recent 30 days are returned.
- If `endTime` is provided but `startTime` is omitted, records from the 7 days before `endTime` are returned."#, false),
    )]
    QueryPortfolioMarginNegativeBalanceInterestHistory(
        QueryPortfolioMarginNegativeBalanceInterestHistoryArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Get current UM position information.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- Please use with account push event `ACCOUNT_UPDATE` for timeliness and accuracy.
- In One-way Mode, only positions with side `BOTH` are shown.
- In Hedge Mode, positions with sides `BOTH`, `LONG`, and `SHORT` are shown."#, false),
    )]
    QueryUmPositionInformation(QueryUmPositionInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Query user negative balance auto exchange record

Weight(IP): 100

Security Type: USER_DATA

Notes:
- Response in descending order
- The max interval between `startTime` and `endTime` is 3 months."#, false),
    )]
    QueryUserNegativeBalanceAutoExchangeRecord(QueryUserNegativeBalanceAutoExchangeRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Query User Rate Limit

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryUserRateLimit(QueryUserRateLimitArgs),
    #[command(
        about = decode_selected_entities(r#"Repay futures Negative Balance

Weight(IP): 750

Security Type: USER_DATA"#, false),
    )]
    RepayFuturesNegativeBalance(RepayFuturesNegativeBalanceArgs),
    #[command(
        about = decode_selected_entities(r#"Query UM Futures account configuration

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    UmFuturesAccountConfiguration(UmFuturesAccountConfigurationArgs),
    #[command(
        about = decode_selected_entities(r#"Get current UM account symbol configuration.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    UmFuturesSymbolConfiguration(UmFuturesSymbolConfigurationArgs),
    #[command(
        about = decode_selected_entities(r#"Query UM notional and leverage brackets

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    UmNotionalAndLeverageBrackets(UmNotionalAndLeverageBracketsArgs),
    #[command(
        about = decode_selected_entities(r#"Test connectivity to the Rest API.

Weight(IP): 1"#, false),
    )]
    TestConnectivity(TestConnectivityArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel All CM Open Conditional Orders

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAllCmOpenConditionalOrders(CancelAllCmOpenConditionalOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel all active LIMIT orders on specific symbol

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAllCmOpenOrders(CancelAllCmOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel All UM Algo Open Orders

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAllUmAlgoOpenOrders(CancelAllUmAlgoOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel All UM Open Conditional Orders

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAllUmOpenConditionalOrders(CancelAllUmOpenConditionalOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel all active LIMIT orders on specific symbol

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAllUmOpenOrders(CancelAllUmOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel CM Conditional Order

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `strategyId` or `newClientStrategyId` must be sent."#, false),
    )]
    CancelCmConditionalOrder(CancelCmConditionalOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an active LIMIT order

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent."#, false),
    )]
    CancelCmOrder(CancelCmOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel Margin Account All Open Orders on a Symbol

Weight(IP): 5

Security Type: TRADE"#, false),
    )]
    CancelMarginAccountAllOpenOrdersOnASymbol(CancelMarginAccountAllOpenOrdersOnASymbolArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel Margin Account OCO Orders

Weight(IP): 2

Security Type: TRADE

Notes:
- Additional notes: Canceling an individual leg will cancel the entire OCO"#, false),
    )]
    CancelMarginAccountOcoOrders(CancelMarginAccountOcoOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel Margin Account Order

Weight(IP): 2

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent."#, false),
    )]
    CancelMarginAccountOrder(CancelMarginAccountOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an active UM algo order

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `algoId` or `clientAlgoId` must be sent."#, false),
    )]
    CancelUmAlgoOrder(CancelUmAlgoOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel UM Conditional Order

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `strategyId` or `newClientStrategyId` must be sent."#, false),
    )]
    CancelUmConditionalOrder(CancelUmConditionalOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an active UM LIMIT order

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent."#, false),
    )]
    CancelUmOrder(CancelUmOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Get trades for a specific account and CM symbol.

Weight: - 20 with `symbol`
- 40 with `pair`

Security Type: USER_DATA

Notes:
- Either `symbol` or `pair` must be sent
- `symbol` and `pair` cannot be sent together
- `pair` and `fromId` cannot be sent together
- `OrderId` can only be sent together with symbol
- If a `pair` is sent, tickers for all symbols of the `pair` will be returned
- The parameter `fromId` cannot be sent with `startTime` or `endTime`
- If `startTime` and `endTime` are both not sent, then the last '24 hours' data will be returned.
- The time between `startTime` and `endTime` cannot be longer than 24 hours."#, false),
    )]
    CmAccountTradeList(CmAccountTradeListArgs),
    #[command(
        about = decode_selected_entities(r#"Query CM Position ADL Quantile Estimation
* Values update every 30s.
* Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
* For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode, "LONG", "SHORT", and "BOTH" will be returned to show the positions' adl quantiles of different position sides.
* If the positions of the symbol are crossed margined in Hedge Mode:
* "HEDGE" as a sign will be returned instead of "BOTH";
* A same value caculated on unrealized pnls on long and short sides' positions will be shown for "LONG" and "SHORT" when there are positions in both of long and short sides.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    CmPositionAdlQuantileEstimation(CmPositionAdlQuantileEstimationArgs),
    #[command(
        about = decode_selected_entities(r#"Sign TradFi-Perps agreement contract

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    FuturesTradfiPerpsContract(FuturesTradfiPerpsContractArgs),
    #[command(
        about = decode_selected_entities(r#"Get user's BNB Fee Discount for UM Futures (Fee Discount On or Fee Discount Off )

Weight(IP): 30

Security Type: USER_DATA"#, false),
    )]
    GetUmFuturesBnbBurnStatus(GetUmFuturesBnbBurnStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Apply for a margin loan.

Weight(IP): 100

Security Type: MARGIN"#, false),
    )]
    MarginAccountBorrow(MarginAccountBorrowArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a new OCO for a margin account

Weight(IP): 1

Security Type: TRADE

Notes:
- Other Info:
  - Price Restrictions:
  - `SELL`: Limit Price > Last Price > Stop Price
  - `BUY`: Limit Price  * Quantity Restrictions:
  - Both legs must have the same quantity
  - `ICEBERG` quantities however do not have to be the same.
  - Order Rate Limit
  - `OCO` counts as 2 orders against the order rate limit."#, false),
    )]
    MarginAccountNewOco(MarginAccountNewOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Repay for a margin loan.

Weight(IP): 100

Security Type: MARGIN"#, false),
    )]
    MarginAccountRepay(MarginAccountRepayArgs),
    #[command(
        about = decode_selected_entities(r#"Repay debt for a margin loan.

Weight(IP): 3000

Security Type: TRADE

Notes:
- The repay asset amount cannot exceed 50000 USD equivalent value for a single request.
- If `amount` is not sent, all the asset loan will be repaid if having enough specific repay assets.
- If `amount` is sent, only the certain amount of the asset loan will be repaid if having enough specific repay assets.
- The system will use the same asset to repay the loan first (if have) no matter whether put the asset in `specifyRepayAssets`"#, false),
    )]
    MarginAccountRepayDebt(MarginAccountRepayDebtArgs),
    #[command(
        about = decode_selected_entities(r#"Margin Account Trade List

Weight(IP): 5

Security Type: USER_DATA

Notes:
- **Note:** * If `fromId` is set, returns orders with id >= `fromId`; otherwise returns recent order history.
- The interval between `startTime` and `endTime` must be less than 24 hours."#, false),
    )]
    MarginAccountTradeList(MarginAccountTradeListArgs),
    #[command(
        about = decode_selected_entities(r#"Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent, and the `orderId` will prevail if both are sent.
- Both `quantity` and `price` must be sent
- When the new `quantity` or `price` doesn't satisfy PRICE_FILTER / PERCENT_FILTER / LOT_SIZE, amendment will be rejected and the order will stay as it is.
- However the order will be cancelled by the amendment in the following situations:
  - when the order is in partially filled status and the new `quantity` <= `executedQty`
  - When the order is `GTX` and the new price will cause it to be executed immediately"#, false),
    )]
    ModifyCmOrder(ModifyCmOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue

Weight(IP): 1

Security Type: TRADE

Notes:
- Either orderId or origClientOrderId must be sent, and the orderId will prevail if both are sent.
- Both quantity and price must be sent
- When the new quantity or price doesn't satisfy PRICE_FILTER / PERCENT_FILTER / LOT_SIZE, amendment will be rejected and the order will stay as it is.
- However the order will be cancelled by the amendment in the following situations:
  - when the order is in partially filled status and the new quantity <= executedQty
  - When the order is GTX and the new price will cause it to be executed immediately
- The amendment keeps the order's original selfTradePreventionMode."#, false),
    )]
    ModifyUmOrder(ModifyUmOrderArgs),
    #[command(
        about = decode_selected_entities(r#"New CM Conditional Order

Weight(IP): 1

Security Type: TRADE

Notes:
- Additional mandatory parameters based on type:
  - Order with type `STOP/TAKE_PROFIT`, parameter `timeInForce` can be sent ( default `GTC`).
  - Condition orders will be triggered when:
  - `STOP`, `STOP_MARKET`:
  - BUY: "MARK_PRICE" >= `stopPrice`
  - SELL: "MARK_PRICE" = `stopPrice`
  - `TRAILING_STOP_MARKET`:
  - BUY: the lowest mark price after order placed ``= the lowest mark price
  - (1 + `callbackRate`)
  - SELL: the highest mark price after order placed >= `activationPrice`, and the latest mark price = `stopPrice`
  - SELL: latest price ("MARK_PRICE" or "CONTRACT_PRICE") = `stopPrice`"#, false),
    )]
    NewCmConditionalOrder(NewCmConditionalOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Place new CM order

Weight(IP): 1

Security Type: TRADE

Notes:
- Additional mandatory parameters based on `type`:
  - If `newOrderRespType` is sent as `RESULT` :
  - `MARKET` order: the final FILLED result of the order will be return directly.
  - `LIMIT` order with special `timeInForce`: the final status result of the order(FILLED or EXPIRED) will be returned directly."#, false),
    )]
    NewCmOrder(NewCmOrderArgs),
    #[command(
        about = decode_selected_entities(r#"New Margin Order

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    NewMarginOrder(NewMarginOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Place new UM conditional order

Weight(IP): 1

Security Type: TRADE

Notes:
- Algo order with type `STOP`, parameter `timeInForce` can be sent (default `GTC`).
- Algo order with type `TAKE_PROFIT`, parameter `timeInForce` can be sent (default `GTC`).
- Condition orders will be triggered when price reaches the `triggerPrice`.
- `STOP`, `STOP_MARKET`: BUY: latest price >= `triggerPrice`; SELL: latest price <= `triggerPrice`.
- `TAKE_PROFIT`, `TAKE_PROFIT_MARKET`: BUY: latest price <= `triggerPrice`; SELL: latest price >= `triggerPrice`.
- `TRAILING_STOP_MARKET`: BUY: lowest price after order placed <= `activatePrice`, and latest price >= lowest price * (1 + `callbackRate`); SELL: highest price after order placed >= `activatePrice`, and latest price <= highest price * (1 - `callbackRate`).
- `selfTradePreventionMode` is only effective when `timeInForce` set to `IOC` or `GTC` or `GTD`."#, false),
    )]
    NewUmAlgoOrder(NewUmAlgoOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Place new UM conditional order

Weight(IP): 1

Security Type: TRADE

Notes:
- Additional mandatory parameters based on type:
  - Order with type `STOP/TAKE_PROFIT`, parameter `timeInForce` can be sent ( default `GTC`).
  - Condition orders will be triggered when:
  - `STOP`, `STOP_MARKET`:
  - BUY: "MARK_PRICE" >= `stopPrice`
  - SELL: "MARK_PRICE" = `stopPrice`
  - `TRAILING_STOP_MARKET`:
  - BUY: the lowest mark price after order placed ``= the lowest mark price
  - (1 + `callbackRate`)
  - SELL: the highest mark price after order placed >= `activationPrice`, and the latest mark price = `stopPrice`
  - SELL: latest price ("MARK_PRICE" or "CONTRACT_PRICE") = `stopPrice`
  - `selfTradePreventionMode` is only effective when `timeInForce` set to `IOC` or `GTC` or `GTD`.
  - In extreme market conditions, timeInForce `GTD` order auto cancel time might be delayed comparing to `goodTillDate`"#, false),
    )]
    NewUmConditionalOrder(NewUmConditionalOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Place new UM order

Weight(IP): 1

Security Type: TRADE

Notes:
- Additional mandatory parameters based on type:
  - If `newOrderRespType` is sent as `RESULT` :
  - `MARKET` order: the final FILLED result of the order will be return directly.
  - `LIMIT` order with special `timeInForce`: the final status result of the order(FILLED or EXPIRED) will be returned directly.
  - `selfTradePreventionMode` is only effective when `timeInForce` set to `IOC` or `GTC` or `GTD`.
  - In extreme market conditions, timeInForce `GTD` order auto cancel time might be delayed comparing to `goodTillDate`"#, false),
    )]
    NewUmOrder(NewUmOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query All CM Conditional Orders

Weight: - 1 for a single `symbol`
- 40 when `symbol` is omitted

Security Type: USER_DATA

Notes:
- These orders will not be found:
  - order strategyStatus is `CANCELED` or `EXPIRED`, **AND**
  - order has NO filled trade, **AND**
  - created time + 7 days  * The query time period must be less than 7 days( default as the recent 7 days)."#, false),
    )]
    QueryAllCmConditionalOrders(QueryAllCmConditionalOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get all account CM orders; active, canceled, or filled.

Weight: - 20 with `symbol`
- 40 with `pair`

Security Type: USER_DATA

Notes:
- Either `symbol` or `pair` must be sent.
- If `orderId` is set, it will get orders >= that orderId. Otherwise most recent orders are returned.
- These orders will not be found:
  - order status is `CANCELED` or `EXPIRED`, **AND**
  - order has NO filled trade, **AND**
  - created time + 3 days < current time"#, false),
    )]
    QueryAllCmOrders(QueryAllCmOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get all open conditional orders on a symbol. **Careful** when accessing this with no symbol.

Weight: - 1 for a single `symbol`
- 40 when `symbol` is omitted

Security Type: USER_DATA

Notes:
- If the symbol is not sent, orders for all symbols will be returned in an array."#, false),
    )]
    QueryAllCurrentCmOpenConditionalOrders(QueryAllCurrentCmOpenConditionalOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get all open orders on a symbol.

* If the symbol is not sent, orders for all symbols will be returned in an array.

Weight: - 1 for a single `symbol`
- 40 when `symbol` is omitted

Security Type: USER_DATA"#, false),
    )]
    QueryAllCurrentCmOpenOrders(QueryAllCurrentCmOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get all UM open algo orders on a symbol. If the symbol is not sent, orders for all symbols will be returned.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Weight: 1 for a single symbol; 40 when the symbol parameter is omitted."#, false),
    )]
    QueryAllCurrentUmOpenAlgoOrders(QueryAllCurrentUmOpenAlgoOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get all open conditional orders on a symbol.

Weight: - 1 for a single `symbol`
- 40 when `symbol` is omitted

Security Type: USER_DATA

Notes:
- If `symbol` is not provided, conditional open orders for all symbols are returned."#, false),
    )]
    QueryAllCurrentUmOpenConditionalOrders(QueryAllCurrentUmOpenConditionalOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get all open orders on a symbol.

Weight: - 1 for a single `symbol`
- 40 when `symbol` is omitted

Security Type: USER_DATA

Notes:
- If the symbol is not sent, orders for all symbols will be returned in an array."#, false),
    )]
    QueryAllCurrentUmOpenOrders(QueryAllCurrentUmOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query All Margin Account Orders

Weight(IP): 100

Security Type: USER_DATA

Notes:
- If `orderId` is set, returns orders with id >= `orderId`; otherwise returns recent order history.
- For some historical orders, `cummulativeQuoteQty < 0` means the data is unavailable at this time."#, false),
    )]
    QueryAllMarginAccountOrders(QueryAllMarginAccountOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query All UM Conditional Orders

Weight: - 1 for a single `symbol`
- 40 when `symbol` is omitted

Security Type: USER_DATA

Notes:
- These orders will not be found:
  - order strategyStatus is `CANCELED` or `EXPIRED`, **AND**
  - order has NO filled trade, **AND**
  - created time + 7 days  * The query time period must be less than 7 days( default as the recent 7 days)."#, false),
    )]
    QueryAllUmConditionalOrders(QueryAllUmConditionalOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get all account UM orders; active, canceled, or filled.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- If `orderId` is set, it will get orders >= that orderId. Otherwise most recent orders are returned.
- The query time period must be less then 7 days( default as the recent 7 days)."#, false),
    )]
    QueryAllUmOrders(QueryAllUmOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query CM Conditional Order History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `strategyId` or `newClientStrategyId` must be sent.
- `NEW` orders will not be found.
- These orders will not be found:
  - order status is `CANCELED` or `EXPIRED`, **AND**
  - order has NO filled trade, **AND**
  - created time + 7 days < current time"#, false),
    )]
    QueryCmConditionalOrderHistory(QueryCmConditionalOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get order modification history

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent, and the `orderId` will prevail if both are sent."#, false),
    )]
    QueryCmModifyOrderHistory(QueryCmModifyOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Check an CM order's status.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `orderId` or `origClientOrderId` must be sent.
- These orders will not be found:
  - order status is `CANCELED` or `EXPIRED`, **AND**
  - order has NO filled trade, **AND**
  - created time + 3 days < current time"#, false),
    )]
    QueryCmOrder(QueryCmOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query Current CM Open Conditional Order

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `strategyId` or `newClientStrategyId` must be sent.
- If the queried order has been triggered, cancelled or expired, the error message "Order does not exist" will be returned."#, false),
    )]
    QueryCurrentCmOpenConditionalOrder(QueryCurrentCmOpenConditionalOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query current CM open order

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `orderId` or `origClientOrderId` must be sent.
- If the queried order has been filled or cancelled, the error message "Order does not exist" will be returned."#, false),
    )]
    QueryCurrentCmOpenOrder(QueryCurrentCmOpenOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query Current Margin Open Order

Weight(IP): 5

Security Type: USER_DATA

Notes:
- If `symbol` is not sent, order records for all symbols are returned.
- When returning all symbols, the request count charged to the rate limiter equals the number of symbols currently trading on the exchange."#, false),
    )]
    QueryCurrentMarginOpenOrder(QueryCurrentMarginOpenOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Check an UM algo order's status. Orders will not be found if: status is CANCELED/EXPIRED with no fills and created 3+ days ago; or created 90+ days ago.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `algoId` or `clientAlgoId` must be sent. `algoId` is self-increment for each specific `symbol`."#, false),
    )]
    QueryCurrentUmOpenAlgoOrder(QueryCurrentUmOpenAlgoOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query Current UM Open Conditional Order

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `strategyId` or `newClientStrategyId` must be sent.
- If the queried order has been `CANCELED`, `TRIGGERED` or `EXPIRED`, the error message "Order does not exist" will be returned."#, false),
    )]
    QueryCurrentUmOpenConditionalOrder(QueryCurrentUmOpenConditionalOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query current UM open order

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `orderId` or `origClientOrderId` must be sent.
- If the queried order has been filled or cancelled, the error message "Order does not exist" will be returned."#, false),
    )]
    QueryCurrentUmOpenOrder(QueryCurrentUmOpenOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Account Order

Weight(IP): 10

Security Type: USER_DATA

Notes:
- Either `orderId` or `origClientOrderId` must be sent.
- For some historical orders, `cummulativeQuoteQty < 0` means the data is unavailable at this time."#, false),
    )]
    QueryMarginAccountOrder(QueryMarginAccountOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query all OCO for a specific margin account based on provided optional parameters

Weight(IP): 100

Security Type: USER_DATA"#, false),
    )]
    QueryMarginAccountsAllOco(QueryMarginAccountsAllOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Retrieves a specific OCO based on provided optional parameters

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    QueryMarginAccountsOco(QueryMarginAccountsOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Query Margin Account's Open OCO

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    QueryMarginAccountsOpenOco(QueryMarginAccountsOpenOcoArgs),
    #[command(
        about = decode_selected_entities(r#"Get all algo orders: ACTIVE, CANCELED, TRIGGERED or FINISHED.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- If `algoId` is set, it will get orders >= that `algoId`. Otherwise most recent orders are returned.
- The query time period must be less than 7 days (default as the recent 7 days)."#, false),
    )]
    QueryUmAlgoOrderHistory(QueryUmAlgoOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query UM Conditional Order History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Either `strategyId` or `newClientStrategyId` must be sent.
- `NEW` orders will not be found.
- These orders will not be found:
  - order status is `CANCELED` or `EXPIRED`, **AND**
  - order has NO filled trade, **AND**
  - created time + 7 days < current time"#, false),
    )]
    QueryUmConditionalOrderHistory(QueryUmConditionalOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get order modification history

Weight(IP): 1

Security Type: TRADE

Notes:
- Either `orderId` or `origClientOrderId` must be sent, and the `orderId` will prevail if both are sent."#, false),
    )]
    QueryUmModifyOrderHistory(QueryUmModifyOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Check an UM order's status.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- These orders will not be found:
  - Either `orderId` or `origClientOrderId` must be sent.
  - order status is `CANCELED` or `EXPIRED`, **AND**
  - order has NO filled trade, **AND**
  - created time + 3 days < current time"#, false),
    )]
    QueryUmOrder(QueryUmOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query User's CM Force Orders

Weight: - 20 with `symbol`
- 50 without `symbol`

Security Type: USER_DATA

Notes:
- If "autoCloseType" is not sent, orders with both of the types will be returned
- If "startTime" is not sent, data within 7 days before "endTime" can be queried"#, false),
    )]
    QueryUsersCmForceOrders(QueryUsersCmForceOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query user's margin force orders

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryUsersMarginForceOrders(QueryUsersMarginForceOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Query User's UM Force Orders

Weight: - 20 with `symbol`
- 50 without `symbol`

Security Type: USER_DATA

Notes:
- If `autoCloseType` is not sent, orders with both of the types will be returned
- If `startTime` is not sent, data within 7 days before `endTime` can be queried"#, false),
    )]
    QueryUsersUmForceOrders(QueryUsersUmForceOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Change user's BNB Fee Discount for UM Futures (Fee Discount On or Fee Discount Off ) on ***EVERY symbol***

Weight(IP): 1

Security Type: TRADE

Notes:
- The BNB would not be collected from UM-PM account to the Portfolio Margin account."#, false),
    )]
    ToggleBnbBurnOnUmFuturesTrade(ToggleBnbBurnOnUmFuturesTradeArgs),
    #[command(
        about = decode_selected_entities(r#"Get trades for a specific account and UM symbol.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are both not sent, then the last '7 days' data will be returned.
- The time between `startTime` and `endTime` cannot be longer than 7 days.
- The parameter `fromId` cannot be sent with `startTime` or `endTime`."#, false),
    )]
    UmAccountTradeList(UmAccountTradeListArgs),
    #[command(
        about = decode_selected_entities(r#"Query UM Position ADL Quantile Estimation

* Values update every 30s.
* Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
* For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode, "LONG", "SHORT", and "BOTH" will be returned to show the positions' adl quantiles of different position sides.
* If the positions of the symbol are crossed margined in Hedge Mode:
* "HEDGE" as a sign will be returned instead of "BOTH";
* A same value caculated on unrealized pnls on long and short sides' positions will be shown for "LONG" and "SHORT" when there are positions in both of long and short sides.

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    UmPositionAdlQuantileEstimation(UmPositionAdlQuantileEstimationArgs),
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

pub async fn handle_derivatives_trading_portfolio_margin_command(
    command: DerivativesTradingPortfolioMarginCommands,
) -> anyhow::Result<()> {
    match command {

          DerivativesTradingPortfolioMarginCommands::AccountBalance (args) => account_balance(args).await,

          DerivativesTradingPortfolioMarginCommands::AccountInformation (args) => account_information(args).await,

          DerivativesTradingPortfolioMarginCommands::BnbTransfer (args) => bnb_transfer(args).await,

          DerivativesTradingPortfolioMarginCommands::ChangeAutoRepayFuturesStatus (args) => change_auto_repay_futures_status(args).await,

          DerivativesTradingPortfolioMarginCommands::ChangeCmInitialLeverage (args) => change_cm_initial_leverage(args).await,

          DerivativesTradingPortfolioMarginCommands::ChangeCmPositionMode (args) => change_cm_position_mode(args).await,

          DerivativesTradingPortfolioMarginCommands::ChangeUmInitialLeverage (args) => change_um_initial_leverage(args).await,

          DerivativesTradingPortfolioMarginCommands::ChangeUmPositionMode (args) => change_um_position_mode(args).await,

          DerivativesTradingPortfolioMarginCommands::CmNotionalAndLeverageBrackets (args) => cm_notional_and_leverage_brackets(args).await,

          DerivativesTradingPortfolioMarginCommands::FundAutoCollection (args) => fund_auto_collection(args).await,

          DerivativesTradingPortfolioMarginCommands::FundCollectionByAsset (args) => fund_collection_by_asset(args).await,

          DerivativesTradingPortfolioMarginCommands::GetAutoRepayFuturesStatus (args) => get_auto_repay_futures_status(args).await,

          DerivativesTradingPortfolioMarginCommands::GetCmAccountDetail (args) => get_cm_account_detail(args).await,

          DerivativesTradingPortfolioMarginCommands::GetCmCurrentPositionMode (args) => get_cm_current_position_mode(args).await,

          DerivativesTradingPortfolioMarginCommands::GetCmIncomeHistory (args) => get_cm_income_history(args).await,

          DerivativesTradingPortfolioMarginCommands::GetDownloadIdForUmFuturesOrderHistory (args) => get_download_id_for_um_futures_order_history(args).await,

          DerivativesTradingPortfolioMarginCommands::GetDownloadIdForUmFuturesTradeHistory (args) => get_download_id_for_um_futures_trade_history(args).await,

          DerivativesTradingPortfolioMarginCommands::GetDownloadIdForUmFuturesTransactionHistory (args) => get_download_id_for_um_futures_transaction_history(args).await,

          DerivativesTradingPortfolioMarginCommands::GetMarginBorrowLoanInterestHistory (args) => get_margin_borrow_loan_interest_history(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUmAccountDetail (args) => get_um_account_detail(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUmAccountDetailV2 (args) => get_um_account_detail_v2(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUmCurrentPositionMode (args) => get_um_current_position_mode(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUmFuturesOrderDownloadLinkById (args) => get_um_futures_order_download_link_by_id(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUmFuturesTradeDownloadLinkById (args) => get_um_futures_trade_download_link_by_id(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUmFuturesTransactionDownloadLinkById (args) => get_um_futures_transaction_download_link_by_id(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUmIncomeHistory (args) => get_um_income_history(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUserCommissionRateForCm (args) => get_user_commission_rate_for_cm(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUserCommissionRateForUm (args) => get_user_commission_rate_for_um(args).await,

          DerivativesTradingPortfolioMarginCommands::MarginMaxBorrow (args) => margin_max_borrow(args).await,

          DerivativesTradingPortfolioMarginCommands::PortfolioMarginUmTradingQuantitativeRulesIndicators (args) => portfolio_margin_um_trading_quantitative_rules_indicators(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCmPositionInformation (args) => query_cm_position_information(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryMarginLoanRecord (args) => query_margin_loan_record(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryMarginMaxWithdraw (args) => query_margin_max_withdraw(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryMarginRepayRecord (args) => query_margin_repay_record(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryPortfolioMarginNegativeBalanceInterestHistory (args) => query_portfolio_margin_negative_balance_interest_history(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUmPositionInformation (args) => query_um_position_information(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUserNegativeBalanceAutoExchangeRecord (args) => query_user_negative_balance_auto_exchange_record(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUserRateLimit (args) => query_user_rate_limit(args).await,

          DerivativesTradingPortfolioMarginCommands::RepayFuturesNegativeBalance (args) => repay_futures_negative_balance(args).await,

          DerivativesTradingPortfolioMarginCommands::UmFuturesAccountConfiguration (args) => um_futures_account_configuration(args).await,

          DerivativesTradingPortfolioMarginCommands::UmFuturesSymbolConfiguration (args) => um_futures_symbol_configuration(args).await,

          DerivativesTradingPortfolioMarginCommands::UmNotionalAndLeverageBrackets (args) => um_notional_and_leverage_brackets(args).await,

          DerivativesTradingPortfolioMarginCommands::TestConnectivity (args) => test_connectivity(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelAllCmOpenConditionalOrders (args) => cancel_all_cm_open_conditional_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelAllCmOpenOrders (args) => cancel_all_cm_open_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelAllUmAlgoOpenOrders (args) => cancel_all_um_algo_open_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelAllUmOpenConditionalOrders (args) => cancel_all_um_open_conditional_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelAllUmOpenOrders (args) => cancel_all_um_open_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelCmConditionalOrder (args) => cancel_cm_conditional_order(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelCmOrder (args) => cancel_cm_order(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelMarginAccountAllOpenOrdersOnASymbol (args) => cancel_margin_account_all_open_orders_on_a_symbol(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelMarginAccountOcoOrders (args) => cancel_margin_account_oco_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelMarginAccountOrder (args) => cancel_margin_account_order(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelUmAlgoOrder (args) => cancel_um_algo_order(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelUmConditionalOrder (args) => cancel_um_conditional_order(args).await,

          DerivativesTradingPortfolioMarginCommands::CancelUmOrder (args) => cancel_um_order(args).await,

          DerivativesTradingPortfolioMarginCommands::CmAccountTradeList (args) => cm_account_trade_list(args).await,

          DerivativesTradingPortfolioMarginCommands::CmPositionAdlQuantileEstimation (args) => cm_position_adl_quantile_estimation(args).await,

          DerivativesTradingPortfolioMarginCommands::FuturesTradfiPerpsContract (args) => futures_tradfi_perps_contract(args).await,

          DerivativesTradingPortfolioMarginCommands::GetUmFuturesBnbBurnStatus (args) => get_um_futures_bnb_burn_status(args).await,

          DerivativesTradingPortfolioMarginCommands::MarginAccountBorrow (args) => margin_account_borrow(args).await,

          DerivativesTradingPortfolioMarginCommands::MarginAccountNewOco (args) => margin_account_new_oco(args).await,

          DerivativesTradingPortfolioMarginCommands::MarginAccountRepay (args) => margin_account_repay(args).await,

          DerivativesTradingPortfolioMarginCommands::MarginAccountRepayDebt (args) => margin_account_repay_debt(args).await,

          DerivativesTradingPortfolioMarginCommands::MarginAccountTradeList (args) => margin_account_trade_list(args).await,

          DerivativesTradingPortfolioMarginCommands::ModifyCmOrder (args) => modify_cm_order(args).await,

          DerivativesTradingPortfolioMarginCommands::ModifyUmOrder (args) => modify_um_order(args).await,

          DerivativesTradingPortfolioMarginCommands::NewCmConditionalOrder (args) => new_cm_conditional_order(args).await,

          DerivativesTradingPortfolioMarginCommands::NewCmOrder (args) => new_cm_order(args).await,

          DerivativesTradingPortfolioMarginCommands::NewMarginOrder (args) => new_margin_order(args).await,

          DerivativesTradingPortfolioMarginCommands::NewUmAlgoOrder (args) => new_um_algo_order(args).await,

          DerivativesTradingPortfolioMarginCommands::NewUmConditionalOrder (args) => new_um_conditional_order(args).await,

          DerivativesTradingPortfolioMarginCommands::NewUmOrder (args) => new_um_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllCmConditionalOrders (args) => query_all_cm_conditional_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllCmOrders (args) => query_all_cm_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllCurrentCmOpenConditionalOrders (args) => query_all_current_cm_open_conditional_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllCurrentCmOpenOrders (args) => query_all_current_cm_open_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllCurrentUmOpenAlgoOrders (args) => query_all_current_um_open_algo_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllCurrentUmOpenConditionalOrders (args) => query_all_current_um_open_conditional_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllCurrentUmOpenOrders (args) => query_all_current_um_open_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllMarginAccountOrders (args) => query_all_margin_account_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllUmConditionalOrders (args) => query_all_um_conditional_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryAllUmOrders (args) => query_all_um_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCmConditionalOrderHistory (args) => query_cm_conditional_order_history(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCmModifyOrderHistory (args) => query_cm_modify_order_history(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCmOrder (args) => query_cm_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCurrentCmOpenConditionalOrder (args) => query_current_cm_open_conditional_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCurrentCmOpenOrder (args) => query_current_cm_open_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCurrentMarginOpenOrder (args) => query_current_margin_open_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCurrentUmOpenAlgoOrder (args) => query_current_um_open_algo_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCurrentUmOpenConditionalOrder (args) => query_current_um_open_conditional_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryCurrentUmOpenOrder (args) => query_current_um_open_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryMarginAccountOrder (args) => query_margin_account_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryMarginAccountsAllOco (args) => query_margin_accounts_all_oco(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryMarginAccountsOco (args) => query_margin_accounts_oco(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryMarginAccountsOpenOco (args) => query_margin_accounts_open_oco(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUmAlgoOrderHistory (args) => query_um_algo_order_history(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUmConditionalOrderHistory (args) => query_um_conditional_order_history(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUmModifyOrderHistory (args) => query_um_modify_order_history(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUmOrder (args) => query_um_order(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUsersCmForceOrders (args) => query_users_cm_force_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUsersMarginForceOrders (args) => query_users_margin_force_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::QueryUsersUmForceOrders (args) => query_users_um_force_orders(args).await,

          DerivativesTradingPortfolioMarginCommands::ToggleBnbBurnOnUmFuturesTrade (args) => toggle_bnb_burn_on_um_futures_trade(args).await,

          DerivativesTradingPortfolioMarginCommands::UmAccountTradeList (args) => um_account_trade_list(args).await,

          DerivativesTradingPortfolioMarginCommands::UmPositionAdlQuantileEstimation (args) => um_position_adl_quantile_estimation(args).await,

          DerivativesTradingPortfolioMarginCommands::CloseUserDataStream (args) => close_user_data_stream(args).await,

          DerivativesTradingPortfolioMarginCommands::KeepaliveUserDataStream (args) => keepalive_user_data_stream(args).await,

          DerivativesTradingPortfolioMarginCommands::StartUserDataStream (args) => start_user_data_stream(args).await,

    }
}

async fn account_balance(args: AccountBalanceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountBalanceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountBalanceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountBalanceParams::builder()
                .asset(args.asset)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_balance(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
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

async fn bnb_transfer(mut args: BnbTransferArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<BnbTransferParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BnbTransferParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.transfer_side.is_none() {
                        let options = vec![
                            ("TO_UM", BnbTransferTransferSideEnum::ToUm),
                            ("FROM_UM", BnbTransferTransferSideEnum::FromUm),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the transfer_side")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.transfer_side = Some(selected);
                    }
                }
                BnbTransferParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    args.transfer_side
                        .ok_or_else(|| anyhow::anyhow!("transfer_side is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.bnb_transfer(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_auto_repay_futures_status(
    mut args: ChangeAutoRepayFuturesStatusArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeAutoRepayFuturesStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ChangeAutoRepayFuturesStatusParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.auto_repay.is_none() {
                        let options = vec![
                            ("true", ChangeAutoRepayFuturesStatusAutoRepayEnum::True),
                            ("false", ChangeAutoRepayFuturesStatusAutoRepayEnum::False),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the auto_repay")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.auto_repay = Some(selected);
                    }
                }
                ChangeAutoRepayFuturesStatusParams::builder(
                    args.auto_repay
                        .ok_or_else(|| anyhow::anyhow!("auto_repay is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_auto_repay_futures_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_cm_initial_leverage(mut args: ChangeCmInitialLeverageArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeCmInitialLeverageParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ChangeCmInitialLeverageParams>(json).ok_or_else(|| {
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
                ChangeCmInitialLeverageParams::builder(
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
    let response = rest_client.change_cm_initial_leverage(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_cm_position_mode(mut args: ChangeCmPositionModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeCmPositionModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ChangeCmPositionModeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.dual_side_position.is_none() {
                        let options = vec![
                            ("true", ChangeCmPositionModeDualSidePositionEnum::True),
                            ("false", ChangeCmPositionModeDualSidePositionEnum::False),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the dual_side_position")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.dual_side_position = Some(selected);
                    }
                }
                ChangeCmPositionModeParams::builder(
                    args.dual_side_position
                        .ok_or_else(|| anyhow::anyhow!("dual_side_position is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_cm_position_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_um_initial_leverage(mut args: ChangeUmInitialLeverageArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeUmInitialLeverageParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ChangeUmInitialLeverageParams>(json).ok_or_else(|| {
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
                ChangeUmInitialLeverageParams::builder(
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
    let response = rest_client.change_um_initial_leverage(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_um_position_mode(mut args: ChangeUmPositionModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeUmPositionModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ChangeUmPositionModeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.dual_side_position.is_none() {
                        let options = vec![
                            ("true", ChangeUmPositionModeDualSidePositionEnum::True),
                            ("false", ChangeUmPositionModeDualSidePositionEnum::False),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the dual_side_position")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.dual_side_position = Some(selected);
                    }
                }
                ChangeUmPositionModeParams::builder(
                    args.dual_side_position
                        .ok_or_else(|| anyhow::anyhow!("dual_side_position is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_um_position_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cm_notional_and_leverage_brackets(
    args: CmNotionalAndLeverageBracketsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CmNotionalAndLeverageBracketsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CmNotionalAndLeverageBracketsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => CmNotionalAndLeverageBracketsParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .cm_notional_and_leverage_brackets(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fund_auto_collection(args: FundAutoCollectionArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FundAutoCollectionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FundAutoCollectionParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => FundAutoCollectionParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.fund_auto_collection(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fund_collection_by_asset(mut args: FundCollectionByAssetArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FundCollectionByAssetParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FundCollectionByAssetParams>(json).ok_or_else(|| {
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
                FundCollectionByAssetParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.fund_collection_by_asset(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_auto_repay_futures_status(args: GetAutoRepayFuturesStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetAutoRepayFuturesStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetAutoRepayFuturesStatusParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetAutoRepayFuturesStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_auto_repay_futures_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_cm_account_detail(args: GetCmAccountDetailArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCmAccountDetailParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetCmAccountDetailParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetCmAccountDetailParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_cm_account_detail(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_cm_current_position_mode(args: GetCmCurrentPositionModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCmCurrentPositionModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetCmCurrentPositionModeParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetCmCurrentPositionModeParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_cm_current_position_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_cm_income_history(args: GetCmIncomeHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCmIncomeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetCmIncomeHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetCmIncomeHistoryParams::builder()
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
    let response = rest_client.get_cm_income_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_download_id_for_um_futures_order_history(
    mut args: GetDownloadIdForUmFuturesOrderHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDownloadIdForUmFuturesOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDownloadIdForUmFuturesOrderHistoryParams>(json)
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
                GetDownloadIdForUmFuturesOrderHistoryParams::builder(
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
        .get_download_id_for_um_futures_order_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_download_id_for_um_futures_trade_history(
    mut args: GetDownloadIdForUmFuturesTradeHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDownloadIdForUmFuturesTradeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDownloadIdForUmFuturesTradeHistoryParams>(json)
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
                GetDownloadIdForUmFuturesTradeHistoryParams::builder(
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
        .get_download_id_for_um_futures_trade_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_download_id_for_um_futures_transaction_history(
    mut args: GetDownloadIdForUmFuturesTransactionHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDownloadIdForUmFuturesTransactionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDownloadIdForUmFuturesTransactionHistoryParams>(json)
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
                GetDownloadIdForUmFuturesTransactionHistoryParams::builder(
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
        .get_download_id_for_um_futures_transaction_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_margin_borrow_loan_interest_history(
    args: GetMarginBorrowLoanInterestHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetMarginBorrowLoanInterestHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetMarginBorrowLoanInterestHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetMarginBorrowLoanInterestHistoryParams::builder()
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .archived(args.archived)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_margin_borrow_loan_interest_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_um_account_detail(args: GetUmAccountDetailArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUmAccountDetailParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetUmAccountDetailParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetUmAccountDetailParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_um_account_detail(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_um_account_detail_v2(args: GetUmAccountDetailV2Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUmAccountDetailV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetUmAccountDetailV2Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetUmAccountDetailV2Params::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_um_account_detail_v2(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_um_current_position_mode(args: GetUmCurrentPositionModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUmCurrentPositionModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetUmCurrentPositionModeParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetUmCurrentPositionModeParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_um_current_position_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_um_futures_order_download_link_by_id(
    mut args: GetUmFuturesOrderDownloadLinkByIdArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUmFuturesOrderDownloadLinkByIdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetUmFuturesOrderDownloadLinkByIdParams>(json)
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
                GetUmFuturesOrderDownloadLinkByIdParams::builder(
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
        .get_um_futures_order_download_link_by_id(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_um_futures_trade_download_link_by_id(
    mut args: GetUmFuturesTradeDownloadLinkByIdArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUmFuturesTradeDownloadLinkByIdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetUmFuturesTradeDownloadLinkByIdParams>(json)
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
                GetUmFuturesTradeDownloadLinkByIdParams::builder(
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
        .get_um_futures_trade_download_link_by_id(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_um_futures_transaction_download_link_by_id(
    mut args: GetUmFuturesTransactionDownloadLinkByIdArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUmFuturesTransactionDownloadLinkByIdParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetUmFuturesTransactionDownloadLinkByIdParams>(json)
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
                GetUmFuturesTransactionDownloadLinkByIdParams::builder(
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
        .get_um_futures_transaction_download_link_by_id(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_um_income_history(args: GetUmIncomeHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUmIncomeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetUmIncomeHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetUmIncomeHistoryParams::builder()
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
    let response = rest_client.get_um_income_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_user_commission_rate_for_cm(
    mut args: GetUserCommissionRateForCmArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUserCommissionRateForCmParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetUserCommissionRateForCmParams>(json).ok_or_else(|| {
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
                GetUserCommissionRateForCmParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_user_commission_rate_for_cm(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_user_commission_rate_for_um(
    mut args: GetUserCommissionRateForUmArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUserCommissionRateForUmParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetUserCommissionRateForUmParams>(json).ok_or_else(|| {
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
                GetUserCommissionRateForUmParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_user_commission_rate_for_um(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_max_borrow(mut args: MarginMaxBorrowArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginMaxBorrowParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginMaxBorrowParams>(json).ok_or_else(|| {
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
                MarginMaxBorrowParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_max_borrow(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn portfolio_margin_um_trading_quantitative_rules_indicators(
    args: PortfolioMarginUmTradingQuantitativeRulesIndicatorsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PortfolioMarginUmTradingQuantitativeRulesIndicatorsParams>()
    {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<PortfolioMarginUmTradingQuantitativeRulesIndicatorsParams>(json)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                    })?
            }
            None => PortfolioMarginUmTradingQuantitativeRulesIndicatorsParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .portfolio_margin_um_trading_quantitative_rules_indicators(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_cm_position_information(args: QueryCmPositionInformationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCmPositionInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryCmPositionInformationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryCmPositionInformationParams::builder()
                .margin_asset(args.margin_asset)
                .pair(args.pair)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_cm_position_information(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_loan_record(mut args: QueryMarginLoanRecordArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginLoanRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryMarginLoanRecordParams>(json).ok_or_else(|| {
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
                QueryMarginLoanRecordParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .tx_id(args.tx_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .archived(args.archived)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_margin_loan_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_max_withdraw(mut args: QueryMarginMaxWithdrawArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginMaxWithdrawParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryMarginMaxWithdrawParams>(json).ok_or_else(|| {
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
                QueryMarginMaxWithdrawParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_margin_max_withdraw(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_repay_record(mut args: QueryMarginRepayRecordArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginRepayRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryMarginRepayRecordParams>(json).ok_or_else(|| {
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
                QueryMarginRepayRecordParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .tx_id(args.tx_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .archived(args.archived)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_margin_repay_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_portfolio_margin_negative_balance_interest_history(
    args: QueryPortfolioMarginNegativeBalanceInterestHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryPortfolioMarginNegativeBalanceInterestHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryPortfolioMarginNegativeBalanceInterestHistoryParams>(json)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                    })?
            }
            None => QueryPortfolioMarginNegativeBalanceInterestHistoryParams::builder()
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_portfolio_margin_negative_balance_interest_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_um_position_information(args: QueryUmPositionInformationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUmPositionInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryUmPositionInformationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryUmPositionInformationParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_um_position_information(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_user_negative_balance_auto_exchange_record(
    mut args: QueryUserNegativeBalanceAutoExchangeRecordArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUserNegativeBalanceAutoExchangeRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryUserNegativeBalanceAutoExchangeRecordParams>(json)
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
                QueryUserNegativeBalanceAutoExchangeRecordParams::builder(
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
        .query_user_negative_balance_auto_exchange_record(params)
        .await?;

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

async fn repay_futures_negative_balance(
    args: RepayFuturesNegativeBalanceArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RepayFuturesNegativeBalanceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<RepayFuturesNegativeBalanceParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => RepayFuturesNegativeBalanceParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.repay_futures_negative_balance(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn um_futures_account_configuration(
    args: UmFuturesAccountConfigurationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UmFuturesAccountConfigurationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<UmFuturesAccountConfigurationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => UmFuturesAccountConfigurationParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.um_futures_account_configuration(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn um_futures_symbol_configuration(
    args: UmFuturesSymbolConfigurationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UmFuturesSymbolConfigurationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<UmFuturesSymbolConfigurationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => UmFuturesSymbolConfigurationParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.um_futures_symbol_configuration(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn um_notional_and_leverage_brackets(
    args: UmNotionalAndLeverageBracketsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UmNotionalAndLeverageBracketsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<UmNotionalAndLeverageBracketsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => UmNotionalAndLeverageBracketsParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .um_notional_and_leverage_brackets(params)
        .await?;

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

async fn cancel_all_cm_open_conditional_orders(
    mut args: CancelAllCmOpenConditionalOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllCmOpenConditionalOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelAllCmOpenConditionalOrdersParams>(json).ok_or_else(|| {
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
                CancelAllCmOpenConditionalOrdersParams::builder(
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
        .cancel_all_cm_open_conditional_orders(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_all_cm_open_orders(mut args: CancelAllCmOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllCmOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAllCmOpenOrdersParams>(json).ok_or_else(|| {
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
                CancelAllCmOpenOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_all_cm_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_all_um_algo_open_orders(
    mut args: CancelAllUmAlgoOpenOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllUmAlgoOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelAllUmAlgoOpenOrdersParams>(json).ok_or_else(|| {
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
                CancelAllUmAlgoOpenOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_all_um_algo_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_all_um_open_conditional_orders(
    mut args: CancelAllUmOpenConditionalOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllUmOpenConditionalOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelAllUmOpenConditionalOrdersParams>(json).ok_or_else(|| {
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
                CancelAllUmOpenConditionalOrdersParams::builder(
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
        .cancel_all_um_open_conditional_orders(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_all_um_open_orders(mut args: CancelAllUmOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllUmOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAllUmOpenOrdersParams>(json).ok_or_else(|| {
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
                CancelAllUmOpenOrdersParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_all_um_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_cm_conditional_order(mut args: CancelCmConditionalOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelCmConditionalOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelCmConditionalOrderParams>(json).ok_or_else(|| {
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
                CancelCmConditionalOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .strategy_id(args.strategy_id)
                .new_client_strategy_id(args.new_client_strategy_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_cm_conditional_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_cm_order(mut args: CancelCmOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelCmOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelCmOrderParams>(json).ok_or_else(|| {
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
                CancelCmOrderParams::builder(
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
    let response = rest_client.cancel_cm_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_margin_account_all_open_orders_on_a_symbol(
    mut args: CancelMarginAccountAllOpenOrdersOnASymbolArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelMarginAccountAllOpenOrdersOnASymbolParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelMarginAccountAllOpenOrdersOnASymbolParams>(json)
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
                CancelMarginAccountAllOpenOrdersOnASymbolParams::builder(
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
        .cancel_margin_account_all_open_orders_on_a_symbol(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_margin_account_oco_orders(
    mut args: CancelMarginAccountOcoOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelMarginAccountOcoOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelMarginAccountOcoOrdersParams>(json).ok_or_else(|| {
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
                CancelMarginAccountOcoOrdersParams::builder(
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
    let response = rest_client.cancel_margin_account_oco_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_margin_account_order(mut args: CancelMarginAccountOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelMarginAccountOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelMarginAccountOrderParams>(json).ok_or_else(|| {
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
                CancelMarginAccountOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
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
    let response = rest_client.cancel_margin_account_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_um_algo_order(args: CancelUmAlgoOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelUmAlgoOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelUmAlgoOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => CancelUmAlgoOrderParams::builder()
                .algo_id(args.algo_id)
                .client_algo_id(args.client_algo_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.cancel_um_algo_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_um_conditional_order(mut args: CancelUmConditionalOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelUmConditionalOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelUmConditionalOrderParams>(json).ok_or_else(|| {
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
                CancelUmConditionalOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .strategy_id(args.strategy_id)
                .new_client_strategy_id(args.new_client_strategy_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_um_conditional_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_um_order(mut args: CancelUmOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelUmOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelUmOrderParams>(json).ok_or_else(|| {
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
                CancelUmOrderParams::builder(
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
    let response = rest_client.cancel_um_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cm_account_trade_list(args: CmAccountTradeListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CmAccountTradeListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CmAccountTradeListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => CmAccountTradeListParams::builder()
                .symbol(args.symbol)
                .pair(args.pair)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .from_id(args.from_id)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.cm_account_trade_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cm_position_adl_quantile_estimation(
    args: CmPositionAdlQuantileEstimationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CmPositionAdlQuantileEstimationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CmPositionAdlQuantileEstimationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => CmPositionAdlQuantileEstimationParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .cm_position_adl_quantile_estimation(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn futures_tradfi_perps_contract(args: FuturesTradfiPerpsContractArgs) -> anyhow::Result<()> {
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

async fn get_um_futures_bnb_burn_status(args: GetUmFuturesBnbBurnStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetUmFuturesBnbBurnStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetUmFuturesBnbBurnStatusParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetUmFuturesBnbBurnStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_um_futures_bnb_burn_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_borrow(mut args: MarginAccountBorrowArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountBorrowParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountBorrowParams>(json).ok_or_else(|| {
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
                MarginAccountBorrowParams::builder(
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
    let response = rest_client.margin_account_borrow(params).await?;

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
                .list_client_order_id(args.list_client_order_id)
                .limit_client_order_id(args.limit_client_order_id)
                .limit_iceberg_qty(args.limit_iceberg_qty)
                .stop_client_order_id(args.stop_client_order_id)
                .stop_limit_price(args.stop_limit_price)
                .stop_iceberg_qty(args.stop_iceberg_qty)
                .stop_limit_time_in_force(args.stop_limit_time_in_force)
                .new_order_resp_type(args.new_order_resp_type)
                .side_effect_type(args.side_effect_type)
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

async fn margin_account_repay(mut args: MarginAccountRepayArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountRepayParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountRepayParams>(json).ok_or_else(|| {
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
                MarginAccountRepayParams::builder(
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
    let response = rest_client.margin_account_repay(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_repay_debt(mut args: MarginAccountRepayDebtArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountRepayDebtParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountRepayDebtParams>(json).ok_or_else(|| {
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
                MarginAccountRepayDebtParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .amount(args.amount)
                .specify_repay_assets(args.specify_repay_assets)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.margin_account_repay_debt(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn margin_account_trade_list(mut args: MarginAccountTradeListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarginAccountTradeListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarginAccountTradeListParams>(json).ok_or_else(|| {
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
                MarginAccountTradeListParams::builder(
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
    let response = rest_client.margin_account_trade_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn modify_cm_order(mut args: ModifyCmOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ModifyCmOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ModifyCmOrderParams>(json).ok_or_else(|| {
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
                            ("BUY", ModifyCmOrderSideEnum::Buy),
                            ("SELL", ModifyCmOrderSideEnum::Sell),
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
                }
                ModifyCmOrderParams::builder(
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
    let response = rest_client.modify_cm_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn modify_um_order(mut args: ModifyUmOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ModifyUmOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ModifyUmOrderParams>(json).ok_or_else(|| {
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
                            ("BUY", ModifyUmOrderSideEnum::Buy),
                            ("SELL", ModifyUmOrderSideEnum::Sell),
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
                }
                ModifyUmOrderParams::builder(
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
    let response = rest_client.modify_um_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_cm_conditional_order(mut args: NewCmConditionalOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewCmConditionalOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewCmConditionalOrderParams>(json).ok_or_else(|| {
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
                            ("BUY", NewCmConditionalOrderSideEnum::Buy),
                            ("SELL", NewCmConditionalOrderSideEnum::Sell),
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
                    if args.strategy_type.is_none() {
                        let options = vec![
                            ("STOP", NewCmConditionalOrderStrategyTypeEnum::Stop),
                            (
                                "STOP_MARKET",
                                NewCmConditionalOrderStrategyTypeEnum::StopMarket,
                            ),
                            (
                                "TAKE_PROFIT",
                                NewCmConditionalOrderStrategyTypeEnum::TakeProfit,
                            ),
                            (
                                "TAKE_PROFIT_MARKET",
                                NewCmConditionalOrderStrategyTypeEnum::TakeProfitMarket,
                            ),
                            (
                                "TRAILING_STOP_MARKET",
                                NewCmConditionalOrderStrategyTypeEnum::TrailingStopMarket,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the strategy_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.strategy_type = Some(selected);
                    }
                }
                NewCmConditionalOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.strategy_type
                        .ok_or_else(|| anyhow::anyhow!("strategy_type is required"))?,
                )
                .position_side(args.position_side)
                .time_in_force(args.time_in_force)
                .quantity(args.quantity)
                .reduce_only(args.reduce_only)
                .price(args.price)
                .working_type(args.working_type)
                .price_protect(args.price_protect)
                .new_client_strategy_id(args.new_client_strategy_id)
                .stop_price(args.stop_price)
                .activation_price(args.activation_price)
                .callback_rate(args.callback_rate)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.new_cm_conditional_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_cm_order(mut args: NewCmOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewCmOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewCmOrderParams>(json).ok_or_else(|| {
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
                            ("BUY", NewCmOrderSideEnum::Buy),
                            ("SELL", NewCmOrderSideEnum::Sell),
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
                            ("LIMIT", NewCmOrderTypeEnum::Limit),
                            ("MARKET", NewCmOrderTypeEnum::Market),
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
                NewCmOrderParams::builder(
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
                .reduce_only(args.reduce_only)
                .price(args.price)
                .price_match(args.price_match)
                .new_client_order_id(args.new_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.new_cm_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_margin_order(mut args: NewMarginOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewMarginOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewMarginOrderParams>(json).ok_or_else(|| {
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
                            ("BUY", NewMarginOrderSideEnum::Buy),
                            ("SELL", NewMarginOrderSideEnum::Sell),
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
                            ("LIMIT", NewMarginOrderTypeEnum::Limit),
                            ("MARKET", NewMarginOrderTypeEnum::Market),
                            ("STOP_LOSS", NewMarginOrderTypeEnum::StopLoss),
                            ("STOP_LOSS_LIMIT", NewMarginOrderTypeEnum::StopLossLimit),
                            ("TAKE_PROFIT", NewMarginOrderTypeEnum::TakeProfit),
                            ("TAKE_PROFIT_LIMIT", NewMarginOrderTypeEnum::TakeProfitLimit),
                            ("LIMIT_MAKER", NewMarginOrderTypeEnum::LimitMaker),
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
                NewMarginOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .quantity(args.quantity)
                .quote_order_qty(args.quote_order_qty)
                .price(args.price)
                .stop_price(args.stop_price)
                .new_client_order_id(args.new_client_order_id)
                .new_order_resp_type(args.new_order_resp_type)
                .iceberg_qty(args.iceberg_qty)
                .side_effect_type(args.side_effect_type)
                .time_in_force(args.time_in_force)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .auto_repay_at_cancel(args.auto_repay_at_cancel)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.new_margin_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_um_algo_order(mut args: NewUmAlgoOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewUmAlgoOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewUmAlgoOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo_type.is_none() {
                        let options =
                            vec![("CONDITIONAL", NewUmAlgoOrderAlgoTypeEnum::Conditional)];

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
                        let symbol: String =
                            Input::new().with_prompt("Input symbol:").interact_text()?;

                        args.symbol = Some(symbol);
                    }
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", NewUmAlgoOrderSideEnum::Buy),
                            ("SELL", NewUmAlgoOrderSideEnum::Sell),
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
                            ("STOP", NewUmAlgoOrderTypeEnum::Stop),
                            ("TAKE_PROFIT", NewUmAlgoOrderTypeEnum::TakeProfit),
                            ("STOP_MARKET", NewUmAlgoOrderTypeEnum::StopMarket),
                            (
                                "TAKE_PROFIT_MARKET",
                                NewUmAlgoOrderTypeEnum::TakeProfitMarket,
                            ),
                            (
                                "TRAILING_STOP_MARKET",
                                NewUmAlgoOrderTypeEnum::TrailingStopMarket,
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
                    if args.quantity.is_none() {
                        let quantity: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input quantity:")
                            .interact_text()?;

                        args.quantity = Some(quantity);
                    }
                }
                NewUmAlgoOrderParams::builder(
                    args.algo_type
                        .ok_or_else(|| anyhow::anyhow!("algo_type is required"))?,
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                )
                .position_side(args.position_side)
                .time_in_force(args.time_in_force)
                .price(args.price)
                .trigger_price(args.trigger_price)
                .working_type(args.working_type)
                .price_match(args.price_match)
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
    let response = rest_client.new_um_algo_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_um_conditional_order(mut args: NewUmConditionalOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewUmConditionalOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewUmConditionalOrderParams>(json).ok_or_else(|| {
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
                            ("BUY", NewUmConditionalOrderSideEnum::Buy),
                            ("SELL", NewUmConditionalOrderSideEnum::Sell),
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
                    if args.strategy_type.is_none() {
                        let options = vec![
                            ("STOP", NewUmConditionalOrderStrategyTypeEnum::Stop),
                            (
                                "STOP_MARKET",
                                NewUmConditionalOrderStrategyTypeEnum::StopMarket,
                            ),
                            (
                                "TAKE_PROFIT",
                                NewUmConditionalOrderStrategyTypeEnum::TakeProfit,
                            ),
                            (
                                "TAKE_PROFIT_MARKET",
                                NewUmConditionalOrderStrategyTypeEnum::TakeProfitMarket,
                            ),
                            (
                                "TRAILING_STOP_MARKET",
                                NewUmConditionalOrderStrategyTypeEnum::TrailingStopMarket,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the strategy_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.strategy_type = Some(selected);
                    }
                }
                NewUmConditionalOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.strategy_type
                        .ok_or_else(|| anyhow::anyhow!("strategy_type is required"))?,
                )
                .position_side(args.position_side)
                .time_in_force(args.time_in_force)
                .quantity(args.quantity)
                .reduce_only(args.reduce_only)
                .price(args.price)
                .working_type(args.working_type)
                .price_protect(args.price_protect)
                .new_client_strategy_id(args.new_client_strategy_id)
                .stop_price(args.stop_price)
                .activation_price(args.activation_price)
                .callback_rate(args.callback_rate)
                .price_match(args.price_match)
                .self_trade_prevention_mode(args.self_trade_prevention_mode)
                .good_till_date(args.good_till_date)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.new_um_conditional_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn new_um_order(mut args: NewUmOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<NewUmOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<NewUmOrderParams>(json).ok_or_else(|| {
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
                            ("BUY", NewUmOrderSideEnum::Buy),
                            ("SELL", NewUmOrderSideEnum::Sell),
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
                            ("LIMIT", NewUmOrderTypeEnum::Limit),
                            ("MARKET", NewUmOrderTypeEnum::Market),
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
                NewUmOrderParams::builder(
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
                .reduce_only(args.reduce_only)
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
    let response = rest_client.new_um_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_cm_conditional_orders(
    args: QueryAllCmConditionalOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllCmConditionalOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryAllCmConditionalOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryAllCmConditionalOrdersParams::builder()
                .symbol(args.symbol)
                .strategy_id(args.strategy_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_all_cm_conditional_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_cm_orders(args: QueryAllCmOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllCmOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryAllCmOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryAllCmOrdersParams::builder()
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
    let response = rest_client.query_all_cm_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_current_cm_open_conditional_orders(
    args: QueryAllCurrentCmOpenConditionalOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllCurrentCmOpenConditionalOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryAllCurrentCmOpenConditionalOrdersParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryAllCurrentCmOpenConditionalOrdersParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_all_current_cm_open_conditional_orders(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_current_cm_open_orders(
    args: QueryAllCurrentCmOpenOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllCurrentCmOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryAllCurrentCmOpenOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryAllCurrentCmOpenOrdersParams::builder()
                .symbol(args.symbol)
                .pair(args.pair)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_all_current_cm_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_current_um_open_algo_orders(
    args: QueryAllCurrentUmOpenAlgoOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllCurrentUmOpenAlgoOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryAllCurrentUmOpenAlgoOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryAllCurrentUmOpenAlgoOrdersParams::builder()
                .algo_type(args.algo_type)
                .symbol(args.symbol)
                .algo_id(args.algo_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_all_current_um_open_algo_orders(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_current_um_open_conditional_orders(
    args: QueryAllCurrentUmOpenConditionalOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllCurrentUmOpenConditionalOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryAllCurrentUmOpenConditionalOrdersParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryAllCurrentUmOpenConditionalOrdersParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_all_current_um_open_conditional_orders(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_current_um_open_orders(
    args: QueryAllCurrentUmOpenOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllCurrentUmOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryAllCurrentUmOpenOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryAllCurrentUmOpenOrdersParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_all_current_um_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_margin_account_orders(
    mut args: QueryAllMarginAccountOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllMarginAccountOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryAllMarginAccountOrdersParams>(json).ok_or_else(|| {
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
                QueryAllMarginAccountOrdersParams::builder(
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
    let response = rest_client.query_all_margin_account_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_um_conditional_orders(
    args: QueryAllUmConditionalOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllUmConditionalOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryAllUmConditionalOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryAllUmConditionalOrdersParams::builder()
                .symbol(args.symbol)
                .strategy_id(args.strategy_id)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_all_um_conditional_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_all_um_orders(mut args: QueryAllUmOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryAllUmOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryAllUmOrdersParams>(json).ok_or_else(|| {
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
                QueryAllUmOrdersParams::builder(
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
    let response = rest_client.query_all_um_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_cm_conditional_order_history(
    mut args: QueryCmConditionalOrderHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCmConditionalOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryCmConditionalOrderHistoryParams>(json).ok_or_else(|| {
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
                QueryCmConditionalOrderHistoryParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .strategy_id(args.strategy_id)
                .new_client_strategy_id(args.new_client_strategy_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_cm_conditional_order_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_cm_modify_order_history(
    mut args: QueryCmModifyOrderHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCmModifyOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryCmModifyOrderHistoryParams>(json).ok_or_else(|| {
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
                QueryCmModifyOrderHistoryParams::builder(
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
    let response = rest_client.query_cm_modify_order_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_cm_order(mut args: QueryCmOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCmOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCmOrderParams>(json).ok_or_else(|| {
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
                QueryCmOrderParams::builder(
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
    let response = rest_client.query_cm_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_cm_open_conditional_order(
    mut args: QueryCurrentCmOpenConditionalOrderArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentCmOpenConditionalOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCurrentCmOpenConditionalOrderParams>(json)
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
                QueryCurrentCmOpenConditionalOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .strategy_id(args.strategy_id)
                .new_client_strategy_id(args.new_client_strategy_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_current_cm_open_conditional_order(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_cm_open_order(mut args: QueryCurrentCmOpenOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentCmOpenOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCurrentCmOpenOrderParams>(json).ok_or_else(|| {
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
                QueryCurrentCmOpenOrderParams::builder(
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
    let response = rest_client.query_current_cm_open_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_margin_open_order(
    mut args: QueryCurrentMarginOpenOrderArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentMarginOpenOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryCurrentMarginOpenOrderParams>(json).ok_or_else(|| {
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
                QueryCurrentMarginOpenOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_current_margin_open_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_um_open_algo_order(
    args: QueryCurrentUmOpenAlgoOrderArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentUmOpenAlgoOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryCurrentUmOpenAlgoOrderParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryCurrentUmOpenAlgoOrderParams::builder()
                .algo_id(args.algo_id)
                .client_algo_id(args.client_algo_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_current_um_open_algo_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_um_open_conditional_order(
    mut args: QueryCurrentUmOpenConditionalOrderArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentUmOpenConditionalOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCurrentUmOpenConditionalOrderParams>(json)
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
                QueryCurrentUmOpenConditionalOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .strategy_id(args.strategy_id)
                .new_client_strategy_id(args.new_client_strategy_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_current_um_open_conditional_order(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_um_open_order(mut args: QueryCurrentUmOpenOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentUmOpenOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCurrentUmOpenOrderParams>(json).ok_or_else(|| {
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
                QueryCurrentUmOpenOrderParams::builder(
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
    let response = rest_client.query_current_um_open_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_margin_account_order(mut args: QueryMarginAccountOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryMarginAccountOrderParams>(json).ok_or_else(|| {
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
                QueryMarginAccountOrderParams::builder(
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
    let response = rest_client.query_margin_account_order(params).await?;

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

async fn query_margin_accounts_oco(args: QueryMarginAccountsOcoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryMarginAccountsOcoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryMarginAccountsOcoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryMarginAccountsOcoParams::builder()
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

async fn query_um_algo_order_history(mut args: QueryUmAlgoOrderHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUmAlgoOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryUmAlgoOrderHistoryParams>(json).ok_or_else(|| {
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
                QueryUmAlgoOrderHistoryParams::builder(
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
    let response = rest_client.query_um_algo_order_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_um_conditional_order_history(
    mut args: QueryUmConditionalOrderHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUmConditionalOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryUmConditionalOrderHistoryParams>(json).ok_or_else(|| {
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
                QueryUmConditionalOrderHistoryParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .strategy_id(args.strategy_id)
                .new_client_strategy_id(args.new_client_strategy_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_um_conditional_order_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_um_modify_order_history(
    mut args: QueryUmModifyOrderHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUmModifyOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryUmModifyOrderHistoryParams>(json).ok_or_else(|| {
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
                QueryUmModifyOrderHistoryParams::builder(
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
    let response = rest_client.query_um_modify_order_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_um_order(mut args: QueryUmOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUmOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryUmOrderParams>(json).ok_or_else(|| {
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
                QueryUmOrderParams::builder(
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
    let response = rest_client.query_um_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_users_cm_force_orders(args: QueryUsersCmForceOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUsersCmForceOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryUsersCmForceOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryUsersCmForceOrdersParams::builder()
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
    let response = rest_client.query_users_cm_force_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_users_margin_force_orders(
    args: QueryUsersMarginForceOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUsersMarginForceOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryUsersMarginForceOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryUsersMarginForceOrdersParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_users_margin_force_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_users_um_force_orders(args: QueryUsersUmForceOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUsersUmForceOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryUsersUmForceOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryUsersUmForceOrdersParams::builder()
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
    let response = rest_client.query_users_um_force_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn toggle_bnb_burn_on_um_futures_trade(
    mut args: ToggleBnbBurnOnUmFuturesTradeArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ToggleBnbBurnOnUmFuturesTradeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ToggleBnbBurnOnUmFuturesTradeParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.fee_burn.is_none() {
                        let options = vec![
                            ("true", ToggleBnbBurnOnUmFuturesTradeFeeBurnEnum::True),
                            ("false", ToggleBnbBurnOnUmFuturesTradeFeeBurnEnum::False),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the fee_burn")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.fee_burn = Some(selected);
                    }
                }
                ToggleBnbBurnOnUmFuturesTradeParams::builder(
                    args.fee_burn
                        .ok_or_else(|| anyhow::anyhow!("fee_burn is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .toggle_bnb_burn_on_um_futures_trade(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn um_account_trade_list(mut args: UmAccountTradeListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UmAccountTradeListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UmAccountTradeListParams>(json).ok_or_else(|| {
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
                UmAccountTradeListParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
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
    let response = rest_client.um_account_trade_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn um_position_adl_quantile_estimation(
    args: UmPositionAdlQuantileEstimationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UmPositionAdlQuantileEstimationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<UmPositionAdlQuantileEstimationParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => UmPositionAdlQuantileEstimationParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .um_position_adl_quantile_estimation(params)
        .await?;

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
