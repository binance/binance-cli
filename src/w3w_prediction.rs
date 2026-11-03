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

    let mut builder = ConfigurationRestApi::builder()
        .base_path(base_path)
        .timeout(crate::utils::rest_timeout_ms()?);

    if is_signed {
        builder = builder
            .api_key(client_config.api_key)
            .api_secret(client_config.api_secret);

        if client_config.private_key.is_some() {
            builder = builder.private_key(PrivateKey::File(client_config.private_key.unwrap()));
        }
    }

    let rest_conf = builder.build().map_err(Error::other)?;

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
struct CreateOtcBlocktradeArgs {
    #[arg(help = r#"PredictFun market id"#, long)]
    market_id: Option<String>,
    #[arg(help = r#"ERC-1155 outcome token id"#, long)]
    token_id: Option<String>,
    #[arg(help = r#"Trade side. Enum: `BUY` (BID), `SELL` (ASK)"#, long)]
    side: Option<CreateOtcBlocktradeSideEnum>,
    #[arg(help = r#"Maker amount in wei. BID：USDT; ASK：shares"#, long)]
    maker_amount: Option<String>,
    #[arg(help = r#"Taker amount in wei. BID：shares; ASK：USDT"#, long)]
    taker_amount: Option<String>,
    #[arg(help = r#"Price per share (decimal ether, e.g. `0.65`)"#, long)]
    price_per_share: Option<String>,
    #[arg(help = r#"Expiration timestamp in seconds (GTD order)"#, long)]
    expiration: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct FulfilOtcBlocktradeArgs {
    #[arg(help = r#"Maker order id (returned by maker create)"#, long)]
    order_id: Option<String>,
    #[arg(
        help = r#"One-time fulfilment token the maker shared out-of-band"#,
        long
    )]
    secret_token: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetOtcBlocktradeDetailArgs {
    #[arg(help = r#"Maker order id (returned by create)"#, long)]
    order_id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetOtcBlocktradeEventsArgs {
    #[arg(help = r#"Page size"#, long)]
    first: Option<i32>,
    #[arg(help = r#"Pagination cursor"#, long)]
    after: Option<String>,
    #[arg(
        help = r#"Filter by event types (e.g. `["MATCH_SUCCESS","EXPIRE"]`)"#,
        long
    )]
    event_types: Option<Vec<String>>,
    #[arg(help = r#"Filter by market id"#, long)]
    market_id: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetOtcReservedBalancesArgs {
    #[arg(help = r#"Assets to query (max 50)"#, long)]
    assets: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ListOtcBlocktradesArgs {
    #[arg(help = r#"Page size"#, long)]
    first: Option<i32>,
    #[arg(help = r#"Pagination cursor (from previous response)"#, long)]
    after: Option<String>,
    #[arg(
        help = r#"Filter by status. Enum: `OPEN`, `FULFILLED`, `MATCHED`, `CANCELLED`, `EXPIRED`, `FAILED`"#,
        long
    )]
    status: Option<ListOtcBlocktradesStatusEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct PreviewOtcBlocktradeArgs {
    #[arg(
        help = r#"One-time fulfilment token the maker shared out-of-band"#,
        long
    )]
    secret_token: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RemoveOtcBlocktradesArgs {
    #[arg(
        help = r#"Order ids to remove (max 100). Must be the `orderId` returned by Create OTC Blocktrade"#,
        long
    )]
    order_ids: Option<String>,
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
        help = r#"Input amount in wei (18 decimals). Must be > 0. The minimum is set by the server and may change, see the response notes. Example: `1000000000000000000` = 1 USDT"#,
        long
    )]
    amount_in: Option<String>,
    #[arg(help = r#"Order type. Enum: `MARKET`, `LIMIT`"#, long)]
    order_type: Option<GetQuoteOrderTypeEnum>,
    #[arg(help = r#"Slippage tolerance in basis points. Range 1–10000"#, long)]
    slippage_bps: Option<i32>,
    #[arg(
        help = r#"Limit price. Required when `orderType=LIMIT`. Must be > 0. A price that is not on the price tick of the market (see `decimalPrecision` in Get Market Detail) is truncated down instead of being rejected, unless the result is 0"#,
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
    #[arg(
        help = r#"User's prediction wallet address. Must be a valid address owned by the calling UID — a well-formed address not owned by the UID and a malformed (non-address) value both return the same generic `-3026`. An empty string instead returns `-1102` naming the field."#,
        long
    )]
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
    #[arg(
        help = r#"Payment account type. Enum: `SPOT`, `FUNDING`. This only determines the settlement/reference account — it does not control which balance is debited. See `fundingSource` below for that."#,
        long
    )]
    account_type: Option<PlaceOrderAccountTypeEnum>,
    #[arg(
        help = r#"Order type. Enum: `MARKET`, `LIMIT` only. Do not combine with `timeInForce` (e.g. `LIMIT_GTC` is not a valid value) — set `timeInForce` separately per the Validation Rules table below."#,
        long
    )]
    order_type: Option<PlaceOrderOrderTypeEnum>,
    #[arg(help = r#"Slippage tolerance in basis points. Range 1–10000"#, long)]
    slippage_bps: Option<i32>,
    #[arg(
        help = r#"Limit price. Required when `orderType=LIMIT`, must be > 0. Omitting it when `orderType=LIMIT` returns a generic `-3026` (no field name in the message)."#,
        long
    )]
    price_limit: Option<String>,
    #[arg(
        help = r#"Funding source. Enum: `MPC`, `CEX`. Default `MPC`. This determines which balance is actually debited, independent of `accountType`."#,
        long
    )]
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
struct ApplyMmDepositArgs {
    #[arg(help = r#"Source token symbol (e.g. `USDT`)"#, long)]
    from_token: Option<String>,
    #[arg(
        help = r#"Source token amount in WEI (18 decimals). Example: `1000000000000000000` = 1 USDT"#,
        long
    )]
    from_token_amount: Option<String>,
    #[arg(help = r#"Target token symbol (e.g. `USDT`)"#, long)]
    to_token: Option<String>,
    #[arg(help = r#"Target CEX account type. Enum: `SPOT`, `FUNDING`"#, long)]
    account_type: Option<ApplyMmDepositAccountTypeEnum>,
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
struct ApplyMmWithdrawArgs {
    #[arg(help = r#"Coin to withdraw (e.g. `USDT`)"#, long)]
    coin: Option<String>,
    #[arg(help = r#"Network (e.g. `BEP20`)"#, long)]
    network: Option<String>,
    #[arg(help = r#"Amount to withdraw (must be > 0)"#, long)]
    amount: Option<String>,
    #[arg(help = r#"Client withdraw order id (idempotency key)"#, long)]
    withdraw_order_id: Option<String>,
    #[arg(
        help = r#"Source CEX account type. Enum: `0` (SPOT), `1` (FUNDING). Default `0`. Must be `0` or `1`; any other value is rejected"#,
        long
    )]
    wallet_type: Option<ApplyMmWithdrawWalletTypeEnum>,
    #[arg(help = r#"Remark for the withdraw"#, long)]
    name: Option<String>,
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

Weight(IP): 1

Response Notes:
- `status` is a case-sensitive string. `REGISTERED`: registered and not yet resolved, this value alone does not guarantee that trading is available. `RESOLVED`: the outcome has been determined, it does not mean that the user has already claimed the payout. `PAUSED`: trading is temporarily suspended and may resume. `CLOSED`: closed for display, for example the end time has passed or the market was removed, it does not guarantee that trading is closed.
- `markets[].tradingStatus` shows whether new orders may be attempted. `OPEN`: new orders may be attempted, this is the only value that allows new orders. `MATCHING_NOT_ENABLED`: order matching is not enabled. `CANCEL_ONLY`: only cancellation is allowed. `CLOSED`: trading has stopped. Use `markets[].tradingStatus`, not `status`, to decide whether to place an order, because a market whose `status` is `CLOSED` can still have `tradingStatus` `OPEN`. A topic can contain several markets and an order is placed on one market, so only the `tradingStatus` of that market matters. A successful quote does not guarantee that an order is accepted or filled.
- `outcomes[].winner` is returned only after the market is resolved. There is no separate cancelled, invalid or refunded status. A resolved market can pay more than one outcome, in that case several outcomes have `winner` set to `true` and `price` is the settled value per share, for example `0.5` for each of two outcomes."#, false),
    )]
    GetMarketDetail(GetMarketDetailArgs),
    #[command(
        about = decode_selected_entities(r#"Get all available prediction market categories (L1 and L2).

Weight(IP): 1"#, false),
    )]
    ListPredictionCategories(ListPredictionCategoriesArgs),
    #[command(
        about = decode_selected_entities(r#"Get a paginated list of prediction market topics, with optional category and sort filters.

Weight(IP): 1

Response Notes:
- For the values of `status` and `markets[].tradingStatus`, see Get Market Detail."#, false),
    )]
    ListPredictionMarkets(ListPredictionMarketsArgs),
    #[command(
        about = decode_selected_entities(r#"Semantic search for prediction market topics by keyword.

Weight(IP): 1"#, false),
    )]
    MarketSearch(MarketSearchArgs),
    #[command(
        about = decode_selected_entities(r#"Get the most recent trade price for a prediction market.

Weight(IP): 1"#, false),
    )]
    QueryLastTradePrice(QueryLastTradePriceArgs),
    #[command(
        about = decode_selected_entities(r#"Get the current order book (bids and asks) for a specific prediction market outcome token.

Weight(IP): 1"#, false),
    )]
    QueryOrderBook(QueryOrderBookArgs),
    #[command(
        about = decode_selected_entities(r#"Create an OTC blocktrade as the maker (BID to buy outcome shares with USDT, or ASK to sell outcome shares for USDT). The maker wallet is resolved server-side by `userId`; signing is done server-side via SAS `typedDataSign`. Returns `orderId` and a one-time `secretToken` to share out-of-band with the intended taker.

Weight(IP): 200

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access.
- Side & Amount Rules:

  | side   | makerAmount   | takerAmount   |
  | ------ | ------------- | ------------- |
  | `BUY`  | USDT (wei)    | shares (wei)  |
  | `SELL` | shares (wei)  | USDT (wei)    |
- "Note on `side` encoding: this request uses a string enum (`BUY`/`SELL`). Responses from Get Blocktrade Detail / Preview / List return `side` as an integer and also include a `quoteType` string — both encode the same concept."

  | Request `side` | Response `side` (Integer) | Response `quoteType` |
  | --------------- | -------------------------- | ---------------------- |
  | `BUY`           | `0`                         | `"Bid"`                |
  | `SELL`          | `1`                         | `"Ask"`                |"#, false),
    )]
    CreateOtcBlocktrade(CreateOtcBlocktradeArgs),
    #[command(
        about = decode_selected_entities(r#"Fulfil an open maker blocktrade as the taker, using the `secretToken` the maker shared out-of-band. All-or-nothing fill (no partial fill); the taker order is the server-derived symmetric inverse of the maker order.

Weight(IP): 200

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access."#, false),
    )]
    FulfilOtcBlocktrade(FulfilOtcBlocktradeArgs),
    #[command(
        about = decode_selected_entities(r#"Query the maker's own blocktrade by `orderId`. Returns full order data including status and `secretToken`.

Weight(IP): 200

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access."#, false),
    )]
    GetOtcBlocktradeDetail(GetOtcBlocktradeDetailArgs),
    #[command(
        about = decode_selected_entities(r#"Paginated feed of blocktrade lifecycle and settlement events (CREATE, FULFIL, MATCH_SUBMIT, MATCH_SUCCESS, EXPIRE, FAILED, etc.).

Weight(IP): 200

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access."#, false),
    )]
    GetOtcBlocktradeEvents(GetOtcBlocktradeEventsArgs),
    #[command(
        about = decode_selected_entities(r#"Query PredictFun reserved balances for the caller's bound wallet — these are funds locked by the caller's open OTC blocktrade orders (maker BID locks USDT, maker ASK locks shares). Not tied to a specific blocktrade id; the path nesting under `otc/blocktrade` reflects the cause of the lock, not a per-order query. Returns one entry per requested asset, aligned with the request order. Pass `{type:"USDT"}` for reserved USDT, or `{type:"SHARE", tokenId:"..."}` for a specific outcome token's reserved shares.

Weight(IP): 200

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access.
- AssetQuery:

  | Name    | Type   | Mandatory | Description |
  | ------- | ------ | --------- | ----------- |
  | type    | STRING | YES       | Asset type. Enum: `USDT`, `SHARE` |
  | tokenId | STRING | NO        | Outcome token id (present for `SHARE` entries only) |"#, false),
    )]
    GetOtcReservedBalances(GetOtcReservedBalancesArgs),
    #[command(
        about = decode_selected_entities(r#"List the maker's own blocktrades with optional status filter and cursor pagination.

Weight(IP): 200

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access."#, false),
    )]
    ListOtcBlocktrades(ListOtcBlocktradesArgs),
    #[command(
        about = decode_selected_entities(r#"Inspect an open blocktrade by `secretToken` (no `orderId` needed). Taker uses this to preview the maker order before fulfilling. Returns the same shape as `Get Blocktrade Detail`, but `orderId` is `null` and `secretToken` is never returned from this endpoint.

Weight(IP): 200

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access."#, false),
    )]
    PreviewOtcBlocktrade(PreviewOtcBlocktradeArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel open or fulfilled-but-unsettled blocktrades. Terminal orders (already `MATCHED`/`CANCELLED`/`EXPIRED`/`FAILED`) are returned in `noop` instead of `removed`.

Weight(IP): 200

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access."#, false),
    )]
    RemoveOtcBlocktrades(RemoveOtcBlocktradesArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's position detail for a specific prediction token.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    GetPositionByToken(GetPositionByTokenArgs),
    #[command(
        about = decode_selected_entities(r#"Query profit and loss records for the authenticated user's prediction positions. When `tokenId` is provided, returns a single record in `pnl`; otherwise returns a list in `pnlList`.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    QueryPnL(QueryPnLArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's prediction token positions with portfolio summary and tab-based filtering.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    QueryPositions(QueryPositionsArgs),
    #[command(
        about = decode_selected_entities(r#"Get prediction positions filtered by wallet address and/or market topic ID. Both parameters are optional.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    QueryPositionsByFilter(QueryPositionsByFilterArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's settled (resolved) prediction position history with optional filters.

Weight(IP): 1

Security Type: PREDICTION_TRADE

Response Notes:
- There is no separate cancelled, invalid or refunded status. A resolved market pays each outcome by its payout ratio, which can be fractional, for example `0.5` for each of two outcomes. This is not a refund of the purchase cost. Use `finalOutcome`, `isWinner`, `claimAmount` and `redeemStatus` together with your own claim records, and Get Market Detail for `outcomes[].winner`."#, false),
    )]
    QuerySettledPositionHistory(QuerySettledPositionHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem one or more settled prediction tokens on-chain to claim winnings. Requires SAS authorization.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    BatchRedeem(BatchRedeemArgs),
    #[command(
        about = decode_selected_entities(r#"Query the on-chain transaction status of a previously submitted redeem request.

Weight(IP): 1

Security Type: PREDICTION_TRADE

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

Weight(IP): 1

Security Type: PREDICTION_TRADE

Notes:
- Use dot notation for nested list fields: `cancelInfoList[0].orderId`, `cancelInfoList[1].orderId`, etc.
- `vendor` does not need to be supplied. The server automatically sets the correct vendor (`predict_fun`) for every item in the batch."#, false),
    )]
    BatchCancelOrders(BatchCancelOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get a price quote for a prediction order. The returned `quoteId` must be used in the subsequent Place Order request.

Weight(IP): 1

Security Type: PREDICTION_TRADE

Response Notes:
- `feeAmount` is a string because it is denominated in wei (18 decimals) and may exceed JavaScript's safe integer range. `feeDiscountBps` is also a string to allow fractional basis-point values in the future. `feeRateBps` and `slippageBps` are integers and will never exceed safe integer bounds.
- **Minimum order amount:** The minimum order amount is set by the server and the upstream market rules and may change, so no fixed value is documented. `BUY` orders are checked by amount (`amountIn`) and `SELL` orders by share quantity, for both `MARKET` and `LIMIT` orders. If the `amountIn` of a `BUY` order is below the minimum, the server returns `-9000` with the message `Your order amount is too small`. A successful quote does not guarantee that the order is accepted or filled, so check the error message and the final order status."#, false),
    )]
    GetQuote(GetQuoteArgs),
    #[command(
        about = decode_selected_entities(r#"Place a prediction order using a previously obtained quote. Requires SAS authorization.

Weight(IP): 1

Security Type: PREDICTION_TRADE

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

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    QueryActiveOrders(QueryActiveOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get historical prediction orders (all statuses) for the authenticated user, with optional filters.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    QueryOrderHistory(QueryOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Move funds from the user's bound CeDeFi MPC wallet to their CEX account (SPOT/FUNDING) via a contract escrow + credit flow. The maker wallet is resolved server-side by `userId`; the caller does not pass wallet or signature.

Weight(IP): 1

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access.
- "Note on `fromToken` / `toToken`: typically the same symbol (e.g. both `USDT`). When they differ, the backend may attempt a swap, but cross-symbol conversion is not guaranteed for all pairs — prefer using the same symbol.""#, false),
    )]
    ApplyMmDeposit(ApplyMmDepositArgs),
    #[command(
        about = decode_selected_entities(r#"Withdraw funds from the user's CEX account (SPOT/FUNDING) to their bound CeDeFi MPC wallet address. Unlike `v1/capital/withdraw/apply`, the caller does NOT pass `address`; the backend resolves the user's bound CeDeFi MPC wallet address by `userId` and reuses the existing capital withdraw flow with that address as the target.

Weight(IP): 1

Security Type: PREDICTION_TRADE

Notes:
- Restricted to authorized market makers. Requests from unauthorized accounts are rejected — contact BD to request access.
- walletType Validation:

  | Value           | Behavior                       |
  | --------------- | ------------------------------- |
  | `null`          | Allowed — defaults to SPOT      |
  | `0`             | Allowed — source = SPOT         |
  | `1`             | Allowed — source = FUNDING      |
  | Other (e.g. `99`) | Rejected — returns validation error |
- "Note on field naming: this endpoint uses `walletType` (INT `0`/`1`) for the source CEX account, while Apply MM Deposit uses `accountType` (STRING `SPOT`/`FUNDING`) for the target. The difference is intentional: withdraw reuses the existing `v1/capital/withdraw/apply` flow, which inherits that flow's integer `walletType` field.""#, false),
    )]
    ApplyMmWithdraw(ApplyMmWithdrawArgs),
    #[command(
        about = decode_selected_entities(r#"Transfer funds from the prediction wallet back to the user's CEX account (SPOT or FUNDING). Requires SAS authorization.

⚠️ **SAS Authorization Required:** This endpoint enforces SAS (Self-Authorization Service) authorization. If SAS is not enabled for the wallet, the request will be rejected with `-31003 SAS authorization required`. Enable SAS for your wallet before calling this endpoint.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    CreateInboundTransfer(CreateInboundTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Transfer funds from the user's CEX account (SPOT or FUNDING) into the prediction wallet. Requires SAS authorization.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    CreateOutboundTransfer(CreateOutboundTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's prediction wallet transfer history within a date range.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    QueryTransferList(QueryTransferListArgs),
    #[command(
        about = decode_selected_entities(r#"Query the current status of a prediction wallet transfer by transfer ID.

**`status` values:** Terminal states are `COMPLETED` and `FAILED`. Intermediate states are `PROCESSING` and `PENDING`. **Do not** poll for `SUCCESS` — it is not a valid terminal state.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    QueryTransferStatus(QueryTransferStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get the authenticated user's prediction portfolio overview including active positions count, aggregated PnL, and full position list.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    GetPortfolio(GetPortfolioArgs),
    #[command(
        about = decode_selected_entities(r#"Query the current user's daily trading quota limit and remaining allowance for prediction markets.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    GetQuotaStatus(GetQuotaStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get all prediction wallets registered for the authenticated user.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
    )]
    ListPredictionWallets(ListPredictionWalletsArgs),
    #[command(
        about = decode_selected_entities(r#"Get available balances for each payment option that can be used for prediction trading.

Weight(IP): 1

Security Type: PREDICTION_TRADE"#, false),
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

        W3WPredictionCommands::CreateOtcBlocktrade(args) => create_otc_blocktrade(args).await,

        W3WPredictionCommands::FulfilOtcBlocktrade(args) => fulfil_otc_blocktrade(args).await,

        W3WPredictionCommands::GetOtcBlocktradeDetail(args) => {
            get_otc_blocktrade_detail(args).await
        }

        W3WPredictionCommands::GetOtcBlocktradeEvents(args) => {
            get_otc_blocktrade_events(args).await
        }

        W3WPredictionCommands::GetOtcReservedBalances(args) => {
            get_otc_reserved_balances(args).await
        }

        W3WPredictionCommands::ListOtcBlocktrades(args) => list_otc_blocktrades(args).await,

        W3WPredictionCommands::PreviewOtcBlocktrade(args) => preview_otc_blocktrade(args).await,

        W3WPredictionCommands::RemoveOtcBlocktrades(args) => remove_otc_blocktrades(args).await,

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

        W3WPredictionCommands::ApplyMmDeposit(args) => apply_mm_deposit(args).await,

        W3WPredictionCommands::ApplyMmWithdraw(args) => apply_mm_withdraw(args).await,

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
                if args.interactive && args.market_topic_id.is_none() {
                    let market_topic_id: i64 = Input::new()
                        .with_prompt("Input market_topic_id:")
                        .interact_text()?;

                    args.market_topic_id = Some(market_topic_id);
                }
                GetMarketDetailParams::builder(
                    args.market_topic_id
                        .ok_or_else(|| anyhow::anyhow!("market-topic-id is required"))?,
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
                if args.interactive && args.query.is_none() {
                    let query: String = Input::new().with_prompt("Input query:").interact_text()?;

                    args.query = Some(query);
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
                if args.interactive && args.market_id.is_none() {
                    let market_id: i64 = Input::new()
                        .with_prompt("Input market_id:")
                        .interact_text()?;

                    args.market_id = Some(market_id);
                }
                QueryLastTradePriceParams::builder(
                    args.market_id
                        .ok_or_else(|| anyhow::anyhow!("market-id is required"))?,
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
                        .ok_or_else(|| anyhow::anyhow!("market-id is required"))?,
                    args.token_id
                        .ok_or_else(|| anyhow::anyhow!("token-id is required"))?,
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

async fn create_otc_blocktrade(mut args: CreateOtcBlocktradeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CreateOtcBlocktradeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CreateOtcBlocktradeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.market_id.is_none() {
                        let market_id: String = Input::new()
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
                    if args.side.is_none() {
                        let options = [
                            ("BUY", CreateOtcBlocktradeSideEnum::Buy),
                            ("SELL", CreateOtcBlocktradeSideEnum::Sell),
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
                    if args.maker_amount.is_none() {
                        let maker_amount: String = Input::new()
                            .with_prompt("Input maker_amount:")
                            .interact_text()?;

                        args.maker_amount = Some(maker_amount);
                    }
                    if args.taker_amount.is_none() {
                        let taker_amount: String = Input::new()
                            .with_prompt("Input taker_amount:")
                            .interact_text()?;

                        args.taker_amount = Some(taker_amount);
                    }
                    if args.price_per_share.is_none() {
                        let price_per_share: String = Input::new()
                            .with_prompt("Input price_per_share:")
                            .interact_text()?;

                        args.price_per_share = Some(price_per_share);
                    }
                    if args.expiration.is_none() {
                        let expiration: i64 = Input::new()
                            .with_prompt("Input expiration:")
                            .interact_text()?;

                        args.expiration = Some(expiration);
                    }
                }
                CreateOtcBlocktradeParams::builder(
                    args.market_id
                        .ok_or_else(|| anyhow::anyhow!("market-id is required"))?,
                    args.token_id
                        .ok_or_else(|| anyhow::anyhow!("token-id is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.maker_amount
                        .ok_or_else(|| anyhow::anyhow!("maker-amount is required"))?,
                    args.taker_amount
                        .ok_or_else(|| anyhow::anyhow!("taker-amount is required"))?,
                    args.price_per_share
                        .ok_or_else(|| anyhow::anyhow!("price-per-share is required"))?,
                    args.expiration
                        .ok_or_else(|| anyhow::anyhow!("expiration is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.create_otc_blocktrade(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fulfil_otc_blocktrade(mut args: FulfilOtcBlocktradeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FulfilOtcBlocktradeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FulfilOtcBlocktradeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.order_id.is_none() {
                        let order_id: String = Input::new()
                            .with_prompt("Input order_id:")
                            .interact_text()?;

                        args.order_id = Some(order_id);
                    }
                    if args.secret_token.is_none() {
                        let secret_token: String = Input::new()
                            .with_prompt("Input secret_token:")
                            .interact_text()?;

                        args.secret_token = Some(secret_token);
                    }
                }
                FulfilOtcBlocktradeParams::builder(
                    args.order_id
                        .ok_or_else(|| anyhow::anyhow!("order-id is required"))?,
                    args.secret_token
                        .ok_or_else(|| anyhow::anyhow!("secret-token is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.fulfil_otc_blocktrade(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_otc_blocktrade_detail(mut args: GetOtcBlocktradeDetailArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOtcBlocktradeDetailParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOtcBlocktradeDetailParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.order_id.is_none() {
                    let order_id: String = Input::new()
                        .with_prompt("Input order_id:")
                        .interact_text()?;

                    args.order_id = Some(order_id);
                }
                GetOtcBlocktradeDetailParams::builder(
                    args.order_id
                        .ok_or_else(|| anyhow::anyhow!("order-id is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_otc_blocktrade_detail(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_otc_blocktrade_events(args: GetOtcBlocktradeEventsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOtcBlocktradeEventsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOtcBlocktradeEventsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetOtcBlocktradeEventsParams::builder()
                .first(args.first)
                .after(args.after)
                .event_types(args.event_types)
                .market_id(args.market_id)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_otc_blocktrade_events(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_otc_reserved_balances(mut args: GetOtcReservedBalancesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOtcReservedBalancesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOtcReservedBalancesParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.assets.is_none() {
                    let assets: String =
                        Input::new().with_prompt("Input assets:").interact_text()?;

                    args.assets = Some(assets);
                }
                GetOtcReservedBalancesParams::builder(serde_json::from_str::<
                    Vec<models::GetOtcReservedBalancesAssetsParameterInner>,
                >(
                    &args
                        .assets
                        .ok_or_else(|| anyhow::anyhow!("assets is required"))?,
                )?)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_otc_reserved_balances(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn list_otc_blocktrades(args: ListOtcBlocktradesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ListOtcBlocktradesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ListOtcBlocktradesParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => ListOtcBlocktradesParams::builder()
                .first(args.first)
                .after(args.after)
                .status(args.status)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.list_otc_blocktrades(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn preview_otc_blocktrade(mut args: PreviewOtcBlocktradeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PreviewOtcBlocktradeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PreviewOtcBlocktradeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.secret_token.is_none() {
                    let secret_token: String = Input::new()
                        .with_prompt("Input secret_token:")
                        .interact_text()?;

                    args.secret_token = Some(secret_token);
                }
                PreviewOtcBlocktradeParams::builder(
                    args.secret_token
                        .ok_or_else(|| anyhow::anyhow!("secret-token is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.preview_otc_blocktrade(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn remove_otc_blocktrades(mut args: RemoveOtcBlocktradesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RemoveOtcBlocktradesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RemoveOtcBlocktradesParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.order_ids.is_none() {
                    let order_ids: String = Input::new()
                        .with_prompt("Input order_ids:")
                        .interact_text()?;

                    args.order_ids = Some(order_ids);
                }
                RemoveOtcBlocktradesParams::builder(serde_json::from_str::<Vec<String>>(
                    &args
                        .order_ids
                        .ok_or_else(|| anyhow::anyhow!("order-ids is required"))?,
                )?)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.remove_otc_blocktrades(params).await?;

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
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.token_id
                        .ok_or_else(|| anyhow::anyhow!("token-id is required"))?,
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
                if args.interactive && args.wallet_address.is_none() {
                    let wallet_address: String = Input::new()
                        .with_prompt("Input wallet_address:")
                        .interact_text()?;

                    args.wallet_address = Some(wallet_address);
                }
                QueryPnLParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
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
                if args.interactive && args.wallet_address.is_none() {
                    let wallet_address: String = Input::new()
                        .with_prompt("Input wallet_address:")
                        .interact_text()?;

                    args.wallet_address = Some(wallet_address);
                }
                QueryPositionsParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
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
                if args.interactive && args.wallet_address.is_none() {
                    let wallet_address: String = Input::new()
                        .with_prompt("Input wallet_address:")
                        .interact_text()?;

                    args.wallet_address = Some(wallet_address);
                }
                QuerySettledPositionHistoryParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
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
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.wallet_id
                        .ok_or_else(|| anyhow::anyhow!("wallet-id is required"))?,
                    serde_json::from_str::<Vec<String>>(
                        &args
                            .token_ids
                            .ok_or_else(|| anyhow::anyhow!("token-ids is required"))?,
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
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.tx_hash
                        .ok_or_else(|| anyhow::anyhow!("tx-hash is required"))?,
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
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.wallet_id
                        .ok_or_else(|| anyhow::anyhow!("wallet-id is required"))?,
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
                        let options = [
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
                        let options = [
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
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.token_id
                        .ok_or_else(|| anyhow::anyhow!("token-id is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.amount_in
                        .ok_or_else(|| anyhow::anyhow!("amount-in is required"))?,
                    args.order_type
                        .ok_or_else(|| anyhow::anyhow!("order-type is required"))?,
                    args.slippage_bps
                        .ok_or_else(|| anyhow::anyhow!("slippage-bps is required"))?,
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
                        let options = [
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
                        let options = [
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
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.wallet_id
                        .ok_or_else(|| anyhow::anyhow!("wallet-id is required"))?,
                    args.quote_id
                        .ok_or_else(|| anyhow::anyhow!("quote-id is required"))?,
                    args.time_in_force
                        .ok_or_else(|| anyhow::anyhow!("time-in-force is required"))?,
                    args.account_type
                        .ok_or_else(|| anyhow::anyhow!("account-type is required"))?,
                    args.order_type
                        .ok_or_else(|| anyhow::anyhow!("order-type is required"))?,
                    args.slippage_bps
                        .ok_or_else(|| anyhow::anyhow!("slippage-bps is required"))?,
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
                if args.interactive && args.wallet_address.is_none() {
                    let wallet_address: String = Input::new()
                        .with_prompt("Input wallet_address:")
                        .interact_text()?;

                    args.wallet_address = Some(wallet_address);
                }
                QueryActiveOrdersParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
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
                if args.interactive && args.wallet_address.is_none() {
                    let wallet_address: String = Input::new()
                        .with_prompt("Input wallet_address:")
                        .interact_text()?;

                    args.wallet_address = Some(wallet_address);
                }
                QueryOrderHistoryParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
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

async fn apply_mm_deposit(mut args: ApplyMmDepositArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ApplyMmDepositParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ApplyMmDepositParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.from_token.is_none() {
                        let from_token: String = Input::new()
                            .with_prompt("Input from_token:")
                            .interact_text()?;

                        args.from_token = Some(from_token);
                    }
                    if args.from_token_amount.is_none() {
                        let from_token_amount: String = Input::new()
                            .with_prompt("Input from_token_amount:")
                            .interact_text()?;

                        args.from_token_amount = Some(from_token_amount);
                    }
                    if args.to_token.is_none() {
                        let to_token: String = Input::new()
                            .with_prompt("Input to_token:")
                            .interact_text()?;

                        args.to_token = Some(to_token);
                    }
                    if args.account_type.is_none() {
                        let options = [
                            ("SPOT", ApplyMmDepositAccountTypeEnum::Spot),
                            ("FUNDING", ApplyMmDepositAccountTypeEnum::Funding),
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
                ApplyMmDepositParams::builder(
                    args.from_token
                        .ok_or_else(|| anyhow::anyhow!("from-token is required"))?,
                    args.from_token_amount
                        .ok_or_else(|| anyhow::anyhow!("from-token-amount is required"))?,
                    args.to_token
                        .ok_or_else(|| anyhow::anyhow!("to-token is required"))?,
                    args.account_type
                        .ok_or_else(|| anyhow::anyhow!("account-type is required"))?,
                )
                .chain_id(args.chain_id)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.apply_mm_deposit(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn apply_mm_withdraw(mut args: ApplyMmWithdrawArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ApplyMmWithdrawParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ApplyMmWithdrawParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.coin.is_none() {
                        let coin: String =
                            Input::new().with_prompt("Input coin:").interact_text()?;

                        args.coin = Some(coin);
                    }
                    if args.network.is_none() {
                        let network: String =
                            Input::new().with_prompt("Input network:").interact_text()?;

                        args.network = Some(network);
                    }
                    if args.amount.is_none() {
                        let amount: String =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                ApplyMmWithdrawParams::builder(
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                    args.network
                        .ok_or_else(|| anyhow::anyhow!("network is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .withdraw_order_id(args.withdraw_order_id)
                .wallet_type(args.wallet_type)
                .name(args.name)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.apply_mm_withdraw(params).await?;

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
                        let options = [
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
                        .ok_or_else(|| anyhow::anyhow!("wallet-id is required"))?,
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.from_token_amount
                        .ok_or_else(|| anyhow::anyhow!("from-token-amount is required"))?,
                    args.account_type
                        .ok_or_else(|| anyhow::anyhow!("account-type is required"))?,
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
                        let options = [
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
                        let options = [
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
                        .ok_or_else(|| anyhow::anyhow!("wallet-id is required"))?,
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.from_token_amount
                        .ok_or_else(|| anyhow::anyhow!("from-token-amount is required"))?,
                    args.account_type
                        .ok_or_else(|| anyhow::anyhow!("account-type is required"))?,
                    args.source_biz
                        .ok_or_else(|| anyhow::anyhow!("source-biz is required"))?,
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
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
                    args.start_date
                        .ok_or_else(|| anyhow::anyhow!("start-date is required"))?,
                    args.end_date
                        .ok_or_else(|| anyhow::anyhow!("end-date is required"))?,
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
                if args.interactive && args.transfer_id.is_none() {
                    let transfer_id: String = Input::new()
                        .with_prompt("Input transfer_id:")
                        .interact_text()?;

                    args.transfer_id = Some(transfer_id);
                }
                QueryTransferStatusParams::builder(
                    args.transfer_id
                        .ok_or_else(|| anyhow::anyhow!("transfer-id is required"))?,
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
                if args.interactive && args.wallet_address.is_none() {
                    let wallet_address: String = Input::new()
                        .with_prompt("Input wallet_address:")
                        .interact_text()?;

                    args.wallet_address = Some(wallet_address);
                }
                GetPortfolioParams::builder(
                    args.wallet_address
                        .ok_or_else(|| anyhow::anyhow!("wallet-address is required"))?,
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
