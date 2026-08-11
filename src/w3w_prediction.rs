use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::W3W_PREDICTION_REST_API_PROD_URL;
use binance_sdk::w3w_prediction::W3WPredictionRestApi;
use binance_sdk::w3w_prediction::rest_api::{self as models, *};
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("w3w-prediction");

    let client_config = get_client_configuration(profile, "w3w-prediction").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => W3W_PREDICTION_REST_API_PROD_URL.to_string(),
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

    Ok(W3WPredictionRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct GetMarketDetailArgs {
    #[arg(help = r#"Market topic ID. Must be > 0"#, long)]
    market_topic_id: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ListPredictionCategoriesArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ListPredictionMarketsArgs {
    #[arg(help = r#"Level-1 category filter"#, long)]
    l1_category: Option<String>,
    #[arg(help = r#"Level-2 category filter"#, long)]
    l2_category: Option<String>,
    #[arg(
        help = r#"Sort field. Enum: `RECOMMENDED`, `VOLUME`, `PARTICIPANTS`, `CREATED_TIME`, `END_DATE`"#,
        long
    )]
    sort_by: Option<ListPredictionMarketsSortByEnum>,
    #[arg(help = r#"Sort direction. Enum: `ASC`, `DESC`"#, long)]
    order_by: Option<ListPredictionMarketsOrderByEnum>,
    #[arg(help = r#"Pagination offset. Default `0`"#, long)]
    offset: Option<i32>,
    #[arg(help = r#"Page size. Default `20`, range 1–100"#, long)]
    limit: Option<i32>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MarketSearchArgs {
    #[arg(help = r#"Search keyword. Not blank"#, long)]
    query: Option<String>,
    #[arg(
        help = r#"Max number of results to return. Default `20`, range 1–50"#,
        long
    )]
    top_k: Option<i32>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryLastTradePriceArgs {
    #[arg(help = r#"Market ID. Must be > 0"#, long)]
    market_id: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryOrderBookArgs {
    #[arg(help = r#"Vendor identifier (e.g. `predict_fun`)"#, long)]
    vendor: Option<String>,
    #[arg(help = r#"Market ID. Must be > 0"#, long)]
    market_id: Option<i64>,
    #[arg(help = r#"Prediction outcome token ID"#, long)]
    token_id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetPositionByTokenArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Prediction outcome token ID"#, long)]
    token_id: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryPnLArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Filter by prediction token ID"#, long)]
    token_id: Option<String>,
    #[arg(help = r#"Filter by market ID. Must be > 0"#, long)]
    market_id: Option<i64>,
    #[arg(help = r#"Filter by market topic ID. Must be > 0"#, long)]
    market_topic_id: Option<i64>,
    #[arg(help = r#"If `true`, return only active (unresolved) positions"#, long, num_args = 0..=1, default_missing_value = "true")]
    active_only: Option<bool>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryPositionsArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(
        help = r#"Position status tab. Values from `PositionQueryType`. Default `ONGOING`"#,
        long
    )]
    tab: Option<String>,
    #[arg(help = r#"Pagination offset. Default `0`"#, long)]
    offset: Option<i32>,
    #[arg(help = r#"Page size. Default `20`, range 1–100"#, long)]
    limit: Option<i32>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryPositionsByFilterArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Filter by market topic ID"#, long)]
    market_topic_id: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QuerySettledPositionHistoryArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Filter by level-1 category"#, long)]
    l1_category: Option<String>,
    #[arg(help = r#"Settlement result filter"#, long)]
    result: Option<i32>,
    #[arg(
        help = r#"Start date. Format: `yyyy-MM-dd`. Must be ≤ `endDate`"#,
        long
    )]
    start_date: Option<String>,
    #[arg(
        help = r#"End date. Format: `yyyy-MM-dd`. Must be ≥ `startDate`"#,
        long
    )]
    end_date: Option<String>,
    #[arg(help = r#"Pagination offset. Default `0`"#, long)]
    offset: Option<i32>,
    #[arg(help = r#"Page size. Default `20`, range 1–100"#, long)]
    limit: Option<i32>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct BatchRedeemArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Wallet ID"#, long)]
    wallet_id: Option<String>,
    #[arg(
        help = r#"List of prediction token IDs to redeem. Not empty. Example: `tokenIds=112233&tokenIds=112234`"#,
        long
    )]
    token_ids: Option<String>,
    #[arg(help = r#"Chain ID. Default `56` (BSC)"#, long)]
    chain_id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetRedeemStatusArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Redeem transaction hash"#, long)]
    tx_hash: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct BatchCancelOrdersArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Wallet ID"#, long)]
    wallet_id: Option<String>,
    #[arg(help = r#"List of orders to cancel (index `i` starts from 0)"#, long)]
    cancel_info_list: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetQuoteArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Prediction outcome token ID"#, long)]
    token_id: Option<String>,
    #[arg(help = r#"Trade direction. Enum: `BUY`, `SELL`"#, long)]
    side: Option<GetQuoteSideEnum>,
    #[arg(
        help = r#"Input amount in wei (18 decimals). Must be > 0. For `MARKET` orders, minimum is approximately 1.5 USDT (varies by market depth). Example: `1000000000000000000` = 1 USDT"#,
        long
    )]
    amount_in: Option<String>,
    #[arg(help = r#"Order type. Enum: `MARKET`, `LIMIT`"#, long)]
    order_type: Option<GetQuoteOrderTypeEnum>,
    #[arg(help = r#"Slippage tolerance in basis points. Range 1–10000"#, long)]
    slippage_bps: Option<i32>,
    #[arg(
        help = r#"Limit price. Required when `orderType=LIMIT`. Must be > 0"#,
        long
    )]
    price_limit: Option<String>,
    #[arg(help = r#"Chain ID. Default `56` (BSC)"#, long)]
    chain_id: Option<String>,
    #[arg(
        help = r#"Fee rate in basis points. Default `200`, range 1–10000"#,
        long
    )]
    fee_rate_bps: Option<i32>,
    #[arg(help = r#"Funding source. Enum: `MPC`, `CEX`. Default `MPC`"#, long)]
    funding_source: Option<GetQuoteFundingSourceEnum>,
    #[arg(
        help = r#"Auto-transfer amount before order (wei). Must be > 0 if provided"#,
        long
    )]
    fund_transfer_amount: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct PlaceOrderArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Wallet ID"#, long)]
    wallet_id: Option<String>,
    #[arg(help = r#"Quote ID obtained from `Get Quote`"#, long)]
    quote_id: Option<String>,
    #[arg(
        help = r#"Must match `orderType`: `FOK` for `MARKET`, `GTC` for `LIMIT`"#,
        long
    )]
    time_in_force: Option<String>,
    #[arg(help = r#"Payment account type. Enum: `SPOT`, `FUNDING`"#, long)]
    account_type: Option<PlaceOrderAccountTypeEnum>,
    #[arg(help = r#"Order type. Enum: `MARKET`, `LIMIT`"#, long)]
    order_type: Option<PlaceOrderOrderTypeEnum>,
    #[arg(help = r#"Slippage tolerance in basis points. Range 1–10000"#, long)]
    slippage_bps: Option<i32>,
    #[arg(
        help = r#"Limit price. Required when `orderType=LIMIT`. Must be > 0"#,
        long
    )]
    price_limit: Option<String>,
    #[arg(help = r#"Funding source. Enum: `MPC`, `CEX`. Default `MPC`"#, long)]
    funding_source: Option<PlaceOrderFundingSourceEnum>,
    #[arg(
        help = r#"Auto-transfer amount before order (wei). Must be > 0 if provided"#,
        long
    )]
    fund_transfer_amount: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryActiveOrdersArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Filter by trade side. Enum: `BUY`, `SELL`"#, long)]
    trade_side: Option<QueryActiveOrdersTradeSideEnum>,
    #[arg(help = r#"Filter by level-1 category"#, long)]
    l1_category: Option<String>,
    #[arg(help = r#"Filter by market ID"#, long)]
    market_id: Option<i64>,
    #[arg(help = r#"Pagination offset. Default `0`"#, long)]
    offset: Option<i32>,
    #[arg(help = r#"Page size. Default `20`, range 1–100"#, long)]
    limit: Option<i32>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryOrderHistoryArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Filter by level-1 category"#, long)]
    l1_category: Option<String>,
    #[arg(help = r#"Filter by order type. Enum: `MARKET`, `LIMIT`"#, long)]
    order_type: Option<QueryOrderHistoryOrderTypeEnum>,
    #[arg(help = r#"Filter by order status"#, long)]
    status: Option<String>,
    #[arg(
        help = r#"Start date. Format: `yyyy-MM-dd`. Must be ≤ `endDate`"#,
        long
    )]
    start_date: Option<String>,
    #[arg(
        help = r#"End date. Format: `yyyy-MM-dd`. Must be ≥ `startDate`"#,
        long
    )]
    end_date: Option<String>,
    #[arg(help = r#"Pagination offset. Default `0`"#, long)]
    offset: Option<i32>,
    #[arg(help = r#"Page size. Default `20`, range 1–100"#, long)]
    limit: Option<i32>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CreateInboundTransferArgs {
    #[arg(help = r#"Wallet ID"#, long)]
    wallet_id: Option<String>,
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(
        help = r#"Transfer amount in wei (18 decimals). Must be > 0. Example: `1000000000000000000` = 1 USDT"#,
        long
    )]
    from_token_amount: Option<String>,
    #[arg(help = r#"Destination CEX account. Enum: `SPOT`, `FUNDING`"#, long)]
    account_type: Option<CreateInboundTransferAccountTypeEnum>,
    #[arg(help = r#"Source token symbol. Default `USDT`"#, long)]
    from_token: Option<String>,
    #[arg(help = r#"Destination token symbol. Default `USDT`"#, long)]
    to_token: Option<String>,
    #[arg(help = r#"Chain ID. Default `56` (BSC)"#, long)]
    chain_id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CreateOutboundTransferArgs {
    #[arg(help = r#"Wallet ID"#, long)]
    wallet_id: Option<String>,
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(
        help = r#"Transfer amount in wei (18 decimals). Must be > 0. Example: `1000000000000000000` = 1 USDT"#,
        long
    )]
    from_token_amount: Option<String>,
    #[arg(help = r#"Source CEX account. Enum: `SPOT`, `FUNDING`"#, long)]
    account_type: Option<CreateOutboundTransferAccountTypeEnum>,
    #[arg(
        help = r#"Business source. Enum: `USER_TRANSFER`, `PREDICTION_BUY`"#,
        long
    )]
    source_biz: Option<CreateOutboundTransferSourceBizEnum>,
    #[arg(help = r#"Source token symbol. Default `USDT`"#, long)]
    from_token: Option<String>,
    #[arg(help = r#"Destination token symbol. Default `USDT`"#, long)]
    to_token: Option<String>,
    #[arg(help = r#"Chain ID. Default `56` (BSC)"#, long)]
    chain_id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryTransferListArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(
        help = r#"Start date. Format: `yyyy-MM-dd`. Must be ≤ `endDate`"#,
        long
    )]
    start_date: Option<String>,
    #[arg(
        help = r#"End date. Format: `yyyy-MM-dd`. Must be ≥ `startDate`"#,
        long
    )]
    end_date: Option<String>,
    #[arg(help = r#"Filter by token symbol (e.g. `USDT`)"#, long)]
    token_symbol: Option<String>,
    #[arg(help = r#"Filter by direction. Enum: `INBOUND`, `OUTBOUND`"#, long)]
    direction: Option<QueryTransferListDirectionEnum>,
    #[arg(help = r#"Pagination offset. Default `0`"#, long)]
    offset: Option<i32>,
    #[arg(help = r#"Page size. Default `20`, range 1–100"#, long)]
    limit: Option<i32>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryTransferStatusArgs {
    #[arg(help = r#"Transfer ID returned from outbound/inbound transfer"#, long)]
    transfer_id: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetPortfolioArgs {
    #[arg(help = r#"User's prediction wallet address"#, long)]
    wallet_address: Option<String>,
    #[arg(help = r#"Filter by prediction token ID"#, long)]
    token_id: Option<String>,
    #[arg(help = r#"Filter by market ID. Must be > 0"#, long)]
    market_id: Option<i64>,
    #[arg(help = r#"Filter by market topic ID. Must be > 0"#, long)]
    market_topic_id: Option<i64>,
    #[arg(help = r#"If `true`, return only active (unresolved) positions"#, long, num_args = 0..=1, default_missing_value = "true")]
    active_only: Option<bool>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetQuotaStatusArgs {
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ListPredictionWalletsArgs {
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryPaymentOptionBalancesArgs {
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum W3WPredictionCommands {
    #[command(
        about = decode_selected_entities(r#"Get full details for a specific prediction market topic, including variant data and timeline.

Weight(IP): 200"#, false),
    )]
    GetMarketDetail(GetMarketDetailArgs),
    #[command(
        about = decode_selected_entities(r#"Get all available prediction market categories (L1 and L2).

Weight(IP): 200"#, false),
    )]
    ListPredictionCategories(ListPredictionCategoriesArgs),
    #[command(
        about = decode_selected_entities(r#"Get a paginated list of prediction market topics, with optional category and sort filters.

Weight(IP): 200"#, false),
    )]
    ListPredictionMarkets(ListPredictionMarketsArgs),
    #[command(
        about = decode_selected_entities(r#"Semantic search for prediction market topics by keyword.

Weight(IP): 200"#, false),
    )]
    MarketSearch(MarketSearchArgs),
    #[command(
        about = decode_selected_entities(r#"Get the most recent trade price for a prediction market.

Weight(IP): 200"#, false),
    )]
    QueryLastTradePrice(QueryLastTradePriceArgs),
    #[command(
        about = decode_selected_entities(r#"Get the current order book (bids and asks) for a specific prediction market outcome token.

Weight(IP): 200"#, false),
    )]
    QueryOrderBook(QueryOrderBookArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's position detail for a specific prediction token.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    GetPositionByToken(GetPositionByTokenArgs),
    #[command(
        about = decode_selected_entities(r#"Query profit and loss records for the authenticated user's prediction positions. When `tokenId` is provided, returns a single record in `pnl`; otherwise returns a list in `pnlList`.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryPnL(QueryPnLArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's prediction token positions with portfolio summary and tab-based filtering.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryPositions(QueryPositionsArgs),
    #[command(
        about = decode_selected_entities(r#"Get prediction positions filtered by wallet address and/or market topic ID. Both parameters are optional.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryPositionsByFilter(QueryPositionsByFilterArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's settled (resolved) prediction position history with optional filters.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QuerySettledPositionHistory(QuerySettledPositionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem one or more settled prediction tokens on-chain to claim winnings. Requires SAS authorization.

Weight(IP): 200

Security Type: TRADE"#, false),
    )]
    BatchRedeem(BatchRedeemArgs),
    #[command(
        about = decode_selected_entities(r#"Query the on-chain transaction status of a previously submitted redeem request.

Weight(IP): 200

Security Type: USER_DATA

Response Notes:
- Status values:

  | Value       | Description                                  |
  | ----------- | -------------------------------------------- |
  | `PENDING`   | Transaction submitted, awaiting confirmation |
  | `CONFIRMED` | Transaction confirmed on-chain               |
  | `FAILED`    | Transaction failed                           |
  | `NOT_FOUND` | Transaction hash not found                   |"#, false),
    )]
    GetRedeemStatus(GetRedeemStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel one or more active prediction orders in a single request. Requires SAS authorization.

**Known Issue — Bracket Encoding Incompatibility:**
This endpoint uses indexed bracket notation (`cancelInfoList[0].orderId`). Binance SAPI signature verification runs over the **raw, unencoded** canonical string. However, mainstream HTTP libraries (Python `requests`, Java `HttpURLConnection`/`URI`, Go `net/url`, Node.js `url`) automatically percent-encode `[` → `%5B` and `]` → `%5D`, producing a signature mismatch with error `-1022 Signature for this request is not valid`. Postman is unaffected because it does not encode keys.

**Workarounds** (use low-level HTTP APIs that do not normalize URLs):
- **Python:** use `http.client` (stdlib) and hand-build the body string.
- **Java:** use `HttpURLConnection` and write the raw body bytes directly.
- **Go:** use `strings.NewReader` with a hand-built body instead of `url.Values.Encode()`.

Weight(IP): 200

Security Type: TRADE

Notes:
- Use dot notation for nested list fields: `cancelInfoList[0].orderId`, `cancelInfoList[1].orderId`, etc.
- `vendor` does not need to be supplied. The server automatically sets the correct vendor (`predict_fun`) for every item in the batch."#, false),
    )]
    BatchCancelOrders(BatchCancelOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get a price quote for a prediction order. The returned `quoteId` must be used in the subsequent Place Order request.

Weight(IP): 200

Security Type: TRADE

Response Notes:
- `feeAmount` is a string because it is denominated in wei (18 decimals) and may exceed JavaScript's safe integer range. `feeDiscountBps` is also a string to allow fractional basis-point values in the future. `feeRateBps` and `slippageBps` are integers and will never exceed safe integer bounds.
- **MARKET order minimum amount:** For `MARKET` orders, `amountIn` must be at least approximately **1.5 USDT** (in wei: `1500000000000000000`). The exact minimum varies by market liquidity. If the amount is too small, the server returns `-9000 Your order amount is too small`. This limit does **not** apply to `LIMIT` orders."#, false),
    )]
    GetQuote(GetQuoteArgs),
    #[command(
        about = decode_selected_entities(r#"Place a prediction order using a previously obtained quote. Requires SAS authorization.

Weight(IP): 200

Security Type: TRADE

Notes:
- Validation rules:

  | orderType | timeInForce   | priceLimit            |
  | --------- | ------------- | --------------------- |
  | `MARKET`  | Must be `FOK` | Not required          |
  | `LIMIT`   | Must be `GTC` | Required, must be > 0 |"#, false),
    )]
    PlaceOrder(PlaceOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Get active (open) prediction orders for the authenticated user.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryActiveOrders(QueryActiveOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get historical prediction orders (all statuses) for the authenticated user, with optional filters.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryOrderHistory(QueryOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Transfer funds from the prediction wallet back to the user's CEX account (SPOT or FUNDING). Requires SAS authorization.

⚠️ **SAS Authorization Required:** This endpoint enforces SAS (Self-Authorization Service) authorization. If SAS is not enabled for the wallet, the request will be rejected with `-31003 SAS authorization required`. Enable SAS for your wallet before calling this endpoint.

Weight(IP): 200

Security Type: TRADE"#, false),
    )]
    CreateInboundTransfer(CreateInboundTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Transfer funds from the user's CEX account (SPOT or FUNDING) into the prediction wallet. Requires SAS authorization.

Weight(IP): 200

Security Type: TRADE"#, false),
    )]
    CreateOutboundTransfer(CreateOutboundTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's prediction wallet transfer history within a date range.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryTransferList(QueryTransferListArgs),
    #[command(
        about = decode_selected_entities(r#"Query the current status of a prediction wallet transfer by transfer ID.

**`status` values:** Terminal states are `COMPLETED` and `FAILED`. Intermediate states are `PROCESSING` and `PENDING`. **Do not** poll for `SUCCESS` — it is not a valid terminal state.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryTransferStatus(QueryTransferStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's prediction portfolio overview including active positions count, aggregated PnL, and full position list.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    GetPortfolio(GetPortfolioArgs),
    #[command(
        about = decode_selected_entities(r#"Query the current user's daily trading quota limit and remaining allowance for prediction markets.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    GetQuotaStatus(GetQuotaStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get all prediction wallets registered for the authenticated user.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    ListPredictionWallets(ListPredictionWalletsArgs),
    #[command(
        about = decode_selected_entities(r#"Get available balances for each payment option that can be used for prediction trading.

Weight(IP): 200

Security Type: USER_DATA"#, false),
    )]
    QueryPaymentOptionBalances(QueryPaymentOptionBalancesArgs),
}

pub async fn handle_w3w_prediction_command(command: W3WPredictionCommands) -> anyhow::Result<()> {
    match command {
        W3WPredictionCommands::GetMarketDetail(args) => get_market_detail(args).await,

        W3WPredictionCommands::ListPredictionCategories(args) => {
            list_prediction_categories(args).await
        }

        W3WPredictionCommands::ListPredictionMarkets(args) => list_prediction_markets(args).await,

        W3WPredictionCommands::MarketSearch(args) => market_search(args).await,

        W3WPredictionCommands::QueryLastTradePrice(args) => query_last_trade_price(args).await,

        W3WPredictionCommands::QueryOrderBook(args) => query_order_book(args).await,

        W3WPredictionCommands::GetPositionByToken(args) => get_position_by_token(args).await,

        W3WPredictionCommands::QueryPnL(args) => query_pn_l(args).await,

        W3WPredictionCommands::QueryPositions(args) => query_positions(args).await,

        W3WPredictionCommands::QueryPositionsByFilter(args) => {
            query_positions_by_filter(args).await
        }

        W3WPredictionCommands::QuerySettledPositionHistory(args) => {
            query_settled_position_history(args).await
        }

        W3WPredictionCommands::BatchRedeem(args) => batch_redeem(args).await,

        W3WPredictionCommands::GetRedeemStatus(args) => get_redeem_status(args).await,

        W3WPredictionCommands::BatchCancelOrders(args) => batch_cancel_orders(args).await,

        W3WPredictionCommands::GetQuote(args) => get_quote(args).await,

        W3WPredictionCommands::PlaceOrder(args) => place_order(args).await,

        W3WPredictionCommands::QueryActiveOrders(args) => query_active_orders(args).await,

        W3WPredictionCommands::QueryOrderHistory(args) => query_order_history(args).await,

        W3WPredictionCommands::CreateInboundTransfer(args) => create_inbound_transfer(args).await,

        W3WPredictionCommands::CreateOutboundTransfer(args) => create_outbound_transfer(args).await,

        W3WPredictionCommands::QueryTransferList(args) => query_transfer_list(args).await,

        W3WPredictionCommands::QueryTransferStatus(args) => query_transfer_status(args).await,

        W3WPredictionCommands::GetPortfolio(args) => get_portfolio(args).await,

        W3WPredictionCommands::GetQuotaStatus(args) => get_quota_status(args).await,

        W3WPredictionCommands::ListPredictionWallets(args) => list_prediction_wallets(args).await,

        W3WPredictionCommands::QueryPaymentOptionBalances(args) => {
            query_payment_option_balances(args).await
        }
    }
}

async fn get_market_detail(mut args: GetMarketDetailArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetMarketDetailParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetMarketDetailParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.market_topic_id.is_none() {
                        let market_topic_id: i64 = Input::new()
                            .with_prompt("Input market_topic_id:")
                            .interact_text()?;

                        args.market_topic_id = Some(market_topic_id);
                    }
                }
                GetMarketDetailParams::builder(
                    args.market_topic_id
                        .ok_or_else(|| anyhow::anyhow!("market_topic_id is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_market_detail(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn list_prediction_categories(args: ListPredictionCategoriesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    // Make the API call
    let response = rest_client.list_prediction_categories().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn list_prediction_markets(args: ListPredictionMarketsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ListPredictionMarketsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ListPredictionMarketsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ListPredictionMarketsParams::builder()
                .l1_category(args.l1_category)
                .l2_category(args.l2_category)
                .sort_by(args.sort_by)
                .order_by(args.order_by)
                .offset(args.offset)
                .limit(args.limit)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.list_prediction_markets(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn market_search(mut args: MarketSearchArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MarketSearchParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MarketSearchParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.query.is_none() {
                        let query: String =
                            Input::new().with_prompt("Input query:").interact_text()?;

                        args.query = Some(query);
                    }
                }
                MarketSearchParams::builder(
                    args.query
                        .ok_or_else(|| anyhow::anyhow!("query is required"))?,
                )
                .top_k(args.top_k)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.market_search(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_last_trade_price(mut args: QueryLastTradePriceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryLastTradePriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryLastTradePriceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.market_id.is_none() {
                        let market_id: i64 = Input::new()
                            .with_prompt("Input market_id:")
                            .interact_text()?;

                        args.market_id = Some(market_id);
                    }
                }
                QueryLastTradePriceParams::builder(
                    args.market_id
                        .ok_or_else(|| anyhow::anyhow!("market_id is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_last_trade_price(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_order_book(mut args: QueryOrderBookArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryOrderBookParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryOrderBookParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.vendor.is_none() {
                        let vendor: String =
                            Input::new().with_prompt("Input vendor:").interact_text()?;

                        args.vendor = Some(vendor);
                    }
                    if args.market_id.is_none() {
                        let market_id: i64 = Input::new()
                            .with_prompt("Input market_id:")
                            .interact_text()?;

                        args.market_id = Some(market_id);
                    }
                    if args.token_id.is_none() {
                        let token_id: String = Input::new()
                            .with_prompt("Input token_id:")
                            .interact_text()?;

                        args.token_id = Some(token_id);
                    }
                }
                QueryOrderBookParams::builder(
                    args.vendor
                        .ok_or_else(|| anyhow::anyhow!("vendor is required"))?,
                    args.market_id
                        .ok_or_else(|| anyhow::anyhow!("market_id is required"))?,
                    args.token_id
                        .ok_or_else(|| anyhow::anyhow!("token_id is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_order_book(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_position_by_token(mut args: GetPositionByTokenArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetPositionByTokenParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetPositionByTokenParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.token_id.is_none() {
                        let token_id: String = Input::new()
                            .with_prompt("Input token_id:")
                            .interact_text()?;

                        args.token_id = Some(token_id);
                    }
                }
                GetPositionByTokenParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.token_id
                        .ok_or_else(|| anyhow::anyhow!("token_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_position_by_token(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_pn_l(mut args: QueryPnLArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryPnLParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryPnLParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                }
                QueryPnLParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                )
                .token_id(args.token_id)
                .market_id(args.market_id)
                .market_topic_id(args.market_topic_id)
                .active_only(args.active_only)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_pn_l(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_positions(mut args: QueryPositionsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryPositionsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryPositionsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                }
                QueryPositionsParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                )
                .tab(args.tab)
                .offset(args.offset)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_positions(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_positions_by_filter(args: QueryPositionsByFilterArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryPositionsByFilterParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryPositionsByFilterParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryPositionsByFilterParams::builder()
                .wallet_address(args.wallet_address)
                .market_topic_id(args.market_topic_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_positions_by_filter(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_settled_position_history(
    mut args: QuerySettledPositionHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySettledPositionHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QuerySettledPositionHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                }
                QuerySettledPositionHistoryParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                )
                .l1_category(args.l1_category)
                .result(args.result)
                .start_date(args.start_date)
                .end_date(args.end_date)
                .offset(args.offset)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_settled_position_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn batch_redeem(mut args: BatchRedeemArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<BatchRedeemParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BatchRedeemParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.wallet_id.is_none() {
                        let wallet_id: String = Input::new()
                            .with_prompt("Input wallet_id:")
                            .interact_text()?;

                        args.wallet_id = Some(wallet_id);
                    }
                    if args.token_ids.is_none() {
                        let token_ids: String = Input::new()
                            .with_prompt("Input token_ids:")
                            .interact_text()?;

                        args.token_ids = Some(token_ids);
                    }
                }
                BatchRedeemParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.wallet_id
                        .ok_or_else(|| anyhow::anyhow!("wallet_id is required"))?,
                    serde_json::from_str::<Vec<String>>(
                        &args
                            .token_ids
                            .ok_or_else(|| anyhow::anyhow!("token_ids is required"))?,
                    )?,
                )
                .chain_id(args.chain_id)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.batch_redeem(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_redeem_status(mut args: GetRedeemStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRedeemStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetRedeemStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.tx_hash.is_none() {
                        let tx_hash: String =
                            Input::new().with_prompt("Input tx_hash:").interact_text()?;

                        args.tx_hash = Some(tx_hash);
                    }
                }
                GetRedeemStatusParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.tx_hash
                        .ok_or_else(|| anyhow::anyhow!("tx_hash is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_redeem_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn batch_cancel_orders(mut args: BatchCancelOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<BatchCancelOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BatchCancelOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.wallet_id.is_none() {
                        let wallet_id: String = Input::new()
                            .with_prompt("Input wallet_id:")
                            .interact_text()?;

                        args.wallet_id = Some(wallet_id);
                    }
                }
                BatchCancelOrdersParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.wallet_id
                        .ok_or_else(|| anyhow::anyhow!("wallet_id is required"))?,
                )
                .cancel_info_list(serde_json::from_str::<
                    Vec<models::BatchCancelOrdersCancelInfoListParameterInner>,
                >(
                    &args
                        .cancel_info_list
                        .ok_or_else(|| anyhow::anyhow!("cancel_info_list is required"))?,
                )?)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.batch_cancel_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_quote(mut args: GetQuoteArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetQuoteParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetQuoteParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.token_id.is_none() {
                        let token_id: String = Input::new()
                            .with_prompt("Input token_id:")
                            .interact_text()?;

                        args.token_id = Some(token_id);
                    }
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", GetQuoteSideEnum::Buy),
                            ("SELL", GetQuoteSideEnum::Sell),
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
                    if args.amount_in.is_none() {
                        let amount_in: String = Input::new()
                            .with_prompt("Input amount_in:")
                            .interact_text()?;

                        args.amount_in = Some(amount_in);
                    }
                    if args.order_type.is_none() {
                        let options = vec![
                            ("MARKET", GetQuoteOrderTypeEnum::Market),
                            ("LIMIT", GetQuoteOrderTypeEnum::Limit),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the order_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.order_type = Some(selected);
                    }
                    if args.slippage_bps.is_none() {
                        let slippage_bps: i32 = Input::new()
                            .with_prompt("Input slippage_bps:")
                            .interact_text()?;

                        args.slippage_bps = Some(slippage_bps);
                    }
                }
                GetQuoteParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.token_id
                        .ok_or_else(|| anyhow::anyhow!("token_id is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.amount_in
                        .ok_or_else(|| anyhow::anyhow!("amount_in is required"))?,
                    args.order_type
                        .ok_or_else(|| anyhow::anyhow!("order_type is required"))?,
                    args.slippage_bps
                        .ok_or_else(|| anyhow::anyhow!("slippage_bps is required"))?,
                )
                .price_limit(args.price_limit)
                .chain_id(args.chain_id)
                .fee_rate_bps(args.fee_rate_bps)
                .funding_source(args.funding_source)
                .fund_transfer_amount(args.fund_transfer_amount)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_quote(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn place_order(mut args: PlaceOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PlaceOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PlaceOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.wallet_id.is_none() {
                        let wallet_id: String = Input::new()
                            .with_prompt("Input wallet_id:")
                            .interact_text()?;

                        args.wallet_id = Some(wallet_id);
                    }
                    if args.quote_id.is_none() {
                        let quote_id: String = Input::new()
                            .with_prompt("Input quote_id:")
                            .interact_text()?;

                        args.quote_id = Some(quote_id);
                    }
                    if args.time_in_force.is_none() {
                        let time_in_force: String = Input::new()
                            .with_prompt("Input time_in_force:")
                            .interact_text()?;

                        args.time_in_force = Some(time_in_force);
                    }
                    if args.account_type.is_none() {
                        let options = vec![
                            ("SPOT", PlaceOrderAccountTypeEnum::Spot),
                            ("FUNDING", PlaceOrderAccountTypeEnum::Funding),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the account_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.account_type = Some(selected);
                    }
                    if args.order_type.is_none() {
                        let options = vec![
                            ("MARKET", PlaceOrderOrderTypeEnum::Market),
                            ("LIMIT", PlaceOrderOrderTypeEnum::Limit),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the order_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.order_type = Some(selected);
                    }
                    if args.slippage_bps.is_none() {
                        let slippage_bps: i32 = Input::new()
                            .with_prompt("Input slippage_bps:")
                            .interact_text()?;

                        args.slippage_bps = Some(slippage_bps);
                    }
                }
                PlaceOrderParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.wallet_id
                        .ok_or_else(|| anyhow::anyhow!("wallet_id is required"))?,
                    args.quote_id
                        .ok_or_else(|| anyhow::anyhow!("quote_id is required"))?,
                    args.time_in_force
                        .ok_or_else(|| anyhow::anyhow!("time_in_force is required"))?,
                    args.account_type
                        .ok_or_else(|| anyhow::anyhow!("account_type is required"))?,
                    args.order_type
                        .ok_or_else(|| anyhow::anyhow!("order_type is required"))?,
                    args.slippage_bps
                        .ok_or_else(|| anyhow::anyhow!("slippage_bps is required"))?,
                )
                .price_limit(args.price_limit)
                .funding_source(args.funding_source)
                .fund_transfer_amount(args.fund_transfer_amount)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.place_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_active_orders(mut args: QueryActiveOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryActiveOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryActiveOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                }
                QueryActiveOrdersParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                )
                .trade_side(args.trade_side)
                .l1_category(args.l1_category)
                .market_id(args.market_id)
                .offset(args.offset)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_active_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_order_history(mut args: QueryOrderHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryOrderHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                }
                QueryOrderHistoryParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                )
                .l1_category(args.l1_category)
                .order_type(args.order_type)
                .status(args.status)
                .start_date(args.start_date)
                .end_date(args.end_date)
                .offset(args.offset)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_order_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn create_inbound_transfer(mut args: CreateInboundTransferArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CreateInboundTransferParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CreateInboundTransferParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_id.is_none() {
                        let wallet_id: String = Input::new()
                            .with_prompt("Input wallet_id:")
                            .interact_text()?;

                        args.wallet_id = Some(wallet_id);
                    }
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.from_token_amount.is_none() {
                        let from_token_amount: String = Input::new()
                            .with_prompt("Input from_token_amount:")
                            .interact_text()?;

                        args.from_token_amount = Some(from_token_amount);
                    }
                    if args.account_type.is_none() {
                        let options = vec![
                            ("SPOT", CreateInboundTransferAccountTypeEnum::Spot),
                            ("FUNDING", CreateInboundTransferAccountTypeEnum::Funding),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the account_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.account_type = Some(selected);
                    }
                }
                CreateInboundTransferParams::builder(
                    args.wallet_id
                        .ok_or_else(|| anyhow::anyhow!("wallet_id is required"))?,
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.from_token_amount
                        .ok_or_else(|| anyhow::anyhow!("from_token_amount is required"))?,
                    args.account_type
                        .ok_or_else(|| anyhow::anyhow!("account_type is required"))?,
                )
                .from_token(args.from_token)
                .to_token(args.to_token)
                .chain_id(args.chain_id)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.create_inbound_transfer(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn create_outbound_transfer(mut args: CreateOutboundTransferArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CreateOutboundTransferParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CreateOutboundTransferParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_id.is_none() {
                        let wallet_id: String = Input::new()
                            .with_prompt("Input wallet_id:")
                            .interact_text()?;

                        args.wallet_id = Some(wallet_id);
                    }
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.from_token_amount.is_none() {
                        let from_token_amount: String = Input::new()
                            .with_prompt("Input from_token_amount:")
                            .interact_text()?;

                        args.from_token_amount = Some(from_token_amount);
                    }
                    if args.account_type.is_none() {
                        let options = vec![
                            ("SPOT", CreateOutboundTransferAccountTypeEnum::Spot),
                            ("FUNDING", CreateOutboundTransferAccountTypeEnum::Funding),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the account_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.account_type = Some(selected);
                    }
                    if args.source_biz.is_none() {
                        let options = vec![
                            (
                                "USER_TRANSFER",
                                CreateOutboundTransferSourceBizEnum::UserTransfer,
                            ),
                            (
                                "PREDICTION_BUY",
                                CreateOutboundTransferSourceBizEnum::PredictionBuy,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the source_biz")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.source_biz = Some(selected);
                    }
                }
                CreateOutboundTransferParams::builder(
                    args.wallet_id
                        .ok_or_else(|| anyhow::anyhow!("wallet_id is required"))?,
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.from_token_amount
                        .ok_or_else(|| anyhow::anyhow!("from_token_amount is required"))?,
                    args.account_type
                        .ok_or_else(|| anyhow::anyhow!("account_type is required"))?,
                    args.source_biz
                        .ok_or_else(|| anyhow::anyhow!("source_biz is required"))?,
                )
                .from_token(args.from_token)
                .to_token(args.to_token)
                .chain_id(args.chain_id)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.create_outbound_transfer(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_transfer_list(mut args: QueryTransferListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryTransferListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryTransferListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                    if args.start_date.is_none() {
                        let start_date: String = Input::new()
                            .with_prompt("Input start_date:")
                            .interact_text()?;

                        args.start_date = Some(start_date);
                    }
                    if args.end_date.is_none() {
                        let end_date: String = Input::new()
                            .with_prompt("Input end_date:")
                            .interact_text()?;

                        args.end_date = Some(end_date);
                    }
                }
                QueryTransferListParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                    args.start_date
                        .ok_or_else(|| anyhow::anyhow!("start_date is required"))?,
                    args.end_date
                        .ok_or_else(|| anyhow::anyhow!("end_date is required"))?,
                )
                .token_symbol(args.token_symbol)
                .direction(args.direction)
                .offset(args.offset)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_transfer_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_transfer_status(mut args: QueryTransferStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryTransferStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryTransferStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.transfer_id.is_none() {
                        let transfer_id: String = Input::new()
                            .with_prompt("Input transfer_id:")
                            .interact_text()?;

                        args.transfer_id = Some(transfer_id);
                    }
                }
                QueryTransferStatusParams::builder(
                    args.transfer_id
                        .ok_or_else(|| anyhow::anyhow!("transfer_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_transfer_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_portfolio(mut args: GetPortfolioArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetPortfolioParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetPortfolioParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.wallet_address.is_none() {
                        let wallet_address: String = Input::new()
                            .with_prompt("Input wallet_address:")
                            .interact_text()?;

                        args.wallet_address = Some(wallet_address);
                    }
                }
                GetPortfolioParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet_address is required"))?,
                )
                .token_id(args.token_id)
                .market_id(args.market_id)
                .market_topic_id(args.market_topic_id)
                .active_only(args.active_only)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_portfolio(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_quota_status(args: GetQuotaStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetQuotaStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetQuotaStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetQuotaStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_quota_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn list_prediction_wallets(args: ListPredictionWalletsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ListPredictionWalletsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ListPredictionWalletsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ListPredictionWalletsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.list_prediction_wallets(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_payment_option_balances(args: QueryPaymentOptionBalancesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryPaymentOptionBalancesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryPaymentOptionBalancesParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => QueryPaymentOptionBalancesParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_payment_option_balances(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
