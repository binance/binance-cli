use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::STOCKS_REST_API_PROD_URL;
use binance_sdk::stocks::StocksRestApi;
use binance_sdk::stocks::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("stocks");

    let client_config = get_client_configuration(profile, "stocks").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => STOCKS_REST_API_PROD_URL.to_string(),
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

    Ok(StocksRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct SignUsEquityDisclaimerArgs {
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
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
        help = r#"Filter to a single US-equity ticker, e.g. `AAPL`. When omitted, returns all active symbols. An unknown ticker returns an empty `symbols` array (HTTP 200), not an error."#,
        long
    )]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct LatestQuoteArgs {
    #[arg(
        help = r#"US-equity ticker, e.g. `AAPL`, `TSLA`. Case-insensitive; uppercased server-side."#,
        long
    )]
    symbol: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TokenizedAssetsArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TokenizedConvertHistoryArgs {
    #[arg(help = r#"Start time (ms epoch)."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time (ms epoch)."#, long)]
    end_time: Option<i64>,
    #[arg(
        help = r#"Last record id from the previous page. Omit (or leave unset) to fetch the first page."#,
        long
    )]
    last_id: Option<i64>,
    #[arg(help = r#"Page size. Default `20`, max `100`."#, long)]
    size: Option<i32>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TokenizedConvertStatusArgs {
    #[arg(
        help = r#"Convert request id returned by `/tokenized/mint` or `/redeem`."#,
        long
    )]
    issuer_request_id: Option<String>,
    #[arg(help = r#"`MINT` or `REDEEM`."#, long)]
    convert_type: Option<TokenizedConvertStatusConvertTypeEnum>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TokenizedMintArgs {
    #[arg(
        help = r#"Underlying US-equity ticker, e.g. `AAPL`, `TSLA`. Resolved against the active-symbol list; unknown tickers return `-26004`. The target tokenized asset is looked up from this field via `/market/tokenized-assets`."#,
        long
    )]
    underlying_asset: Option<String>,
    #[arg(
        help = r#"Quantity of the underlying asset to mint from. Must be > 0."#,
        long
    )]
    underlying_asset_amount: Option<String>,
    #[arg(
        help = r#"Client order id for idempotency. Format `^[a-zA-Z0-9-_]{32,36}$`. Auto-generated when omitted."#,
        long
    )]
    client_order_id: Option<String>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TokenizedRedeemArgs {
    #[arg(
        help = r#"Tokenized asset to redeem, e.g. `AAPLB`. Not a US-equity ticker — this is the on-chain tokenized asset identifier. Unknown asset returns `-1102` (the message currently says the parameter was empty/malformed, but it was in fact sent — it is simply unknown). The target underlying ticker is looked up from this field via `/market/tokenized-assets`."#,
        long
    )]
    tokenized_asset: Option<String>,
    #[arg(
        help = r#"Quantity of the tokenized asset to redeem. Must be > 0."#,
        long
    )]
    tokenized_asset_amount: Option<String>,
    #[arg(
        help = r#"Client order id for idempotency. Format `^[a-zA-Z0-9-_]{32,36}$`. Auto-generated when omitted."#,
        long
    )]
    client_order_id: Option<String>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelAllEquityOrdersArgs {
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelEquityOrderArgs {
    #[arg(
        help = r#"Equity order id returned by `/order/place` or a query endpoint."#,
        long
    )]
    order_id: Option<String>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CurrentOpenOrdersArgs {
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct EquityOrderDetailArgs {
    #[arg(
        help = r#"Equity order id. Either `orderId` or `clientOrderId` must be provided."#,
        long
    )]
    order_id: Option<String>,
    #[arg(
        help = r#"Client-supplied order id. Either `orderId` or `clientOrderId` must be provided."#,
        long
    )]
    client_order_id: Option<String>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct EquityOrderHistoryArgs {
    #[arg(help = r#"Start time (ms epoch)."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time (ms epoch)."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"US-equity ticker filter, e.g. `NVDA`."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Order type filter: `MARKET` / `LIMIT`."#, long)]
    order_type: Option<EquityOrderHistoryOrderTypeEnum>,
    #[arg(help = r#"Side filter: `BUY` / `SELL`."#, long)]
    side: Option<EquityOrderHistorySideEnum>,
    #[arg(
        help = r#"Comma-separated status filter. Allowed values: `FILLED`, `PARTIALLY_FILLED`, `CANCELED`, `EXPIRED`, `REJECTED`."#,
        long
    )]
    order_status: Option<String>,
    #[arg(help = r#"Page number, 1-based. Default `1`."#, long)]
    current: Option<i32>,
    #[arg(help = r#"Page size. Default `20`, max `100`."#, long)]
    size: Option<i32>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct EquityTradeHistoryArgs {
    #[arg(help = r#"Start time (ms epoch)."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time (ms epoch)."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"US-equity ticker filter, e.g. `NVDA`."#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Side filter: `BUY` / `SELL`."#, long)]
    side: Option<EquityTradeHistorySideEnum>,
    #[arg(help = r#"Narrow the result to executions of a single order."#, long)]
    order_id: Option<String>,
    #[arg(help = r#"Page number, 1-based. Default `1`."#, long)]
    current: Option<i32>,
    #[arg(help = r#"Page size. Default `20`, max `100`."#, long)]
    size: Option<i32>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct PlaceEquityOrderArgs {
    #[arg(
        help = r#"US stock ticker, e.g. `AAPL`, `TSLA`. Must be a tradable US-equity symbol — verify via `/sapi/v1/equity/market/exchangeInfo`. Tokenization enablement (verifiable via `/sapi/v1/equity/market/tokenized-assets`) is *not* required; non-tokenized symbols are accepted and settle as traditional underlying-equity trades. The `tokenize` parameter only takes effect on tokenization-enabled symbols and is silently ignored otherwise."#,
        long
    )]
    symbol: Option<String>,
    #[arg(help = r#"`BUY` / `SELL`."#, long)]
    side: Option<PlaceEquityOrderSideEnum>,
    #[arg(help = r#"`MARKET` / `LIMIT`."#, long)]
    order_type: Option<PlaceEquityOrderOrderTypeEnum>,
    #[arg(
        help = r#"Quote asset. Defaults to `USDC`; must be within the server's allowed set."#,
        long
    )]
    quote_asset: Option<String>,
    #[arg(
        help = r#"**Required** for `LIMIT`; **forbidden** for `MARKET`. Maximum 2 decimal places."#,
        long
    )]
    price: Option<String>,
    #[arg(
        help = r#"**Required** for `LIMIT` (both sides) and `SELL MARKET`; **forbidden** for `BUY MARKET`."#,
        long
    )]
    quantity: Option<String>,
    #[arg(
        help = r#"**Required** for `BUY MARKET`; **forbidden** for `LIMIT` and `SELL MARKET`."#,
        long
    )]
    notional: Option<String>,
    #[arg(
        help = r#"`DAY` (default) / `GTC`. `GTC` is only supported for `LIMIT` orders; a fractional-share `GTC` order must be paired with `tradingSession = EXTENDED` or `24H`."#,
        long
    )]
    time_in_force: Option<PlaceEquityOrderTimeInForceEnum>,
    #[arg(
        help = r#"`RTH` / `EXTENDED` / `24H`. **Required** for `LIMIT`; **forbidden** for `MARKET`."#,
        long
    )]
    trading_session: Option<PlaceEquityOrderTradingSessionEnum>,
    #[arg(
        help = r#"Payment wallet for `BUY` orders: `CARD` (default) / `MAIN`. `SELL` orders always settle to `CARD`."#,
        long
    )]
    wallet_type: Option<PlaceEquityOrderWalletTypeEnum>,
    #[arg(
        help = r#"Client-supplied order id. Format `^[a-zA-Z0-9-_]{32,36}$`. Auto-generated when omitted."#,
        long
    )]
    client_order_id: Option<String>,
    #[arg(help = r#"Whether to tokenize the purchased stock asset upon settlement. Default `true`. Only takes effect when the symbol is tokenization-enabled (check via `/market/tokenized-assets`); silently ignored for non-tokenized symbols, which always settle as traditional underlying-equity trades."#, long, num_args = 0..=1, default_missing_value = "true")]
    tokenize: Option<bool>,
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CreateRenewListenKeyArgs {
    #[arg(help = r#"The value cannot be greater than `60000`."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum StocksCommands {
    #[command(
        about = decode_selected_entities(r#"Records the user's acknowledgement and acceptance of the US equity disclaimer. This must be completed before the account can access certain US equity trading features. The acceptance is tied to the account associated with the API key.

Weight: 1

Security Type: TRADE"#, false),
    )]
    SignUsEquityDisclaimer(SignUsEquityDisclaimerArgs),
    #[command(
        about = decode_selected_entities(r#"Returns current exchange trading rules and the list of tradable US-equity symbols. MARKET_DATA endpoint — requires an API key (`X-MBX-APIKEY`) but no signature. Response is served from a server-side cache.

Weight: 1

Security Type: MARKET_DATA"#, false),
    )]
    ExchangeInfo(ExchangeInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Returns the latest best bid / best ask for a single US-equity ticker. The returned value is at most ~5 seconds stale. Returns an **empty response body** (not the literal `null`) when no quote is available for the supplied ticker (e.g. the ticker is unknown, halted, or de-listed).

Weight: 1

Security Type: MARKET_DATA"#, false),
    )]
    LatestQuote(LatestQuoteArgs),
    #[command(
        about = decode_selected_entities(r#"Returns the list of all tokenized assets currently available for mint / redeem, together with each asset's underlying equity symbol and conversion multiplier. MARKET_DATA endpoint — requires an API key (`X-MBX-APIKEY`) but no signature. Response is served from a server-side cache.

Weight: 1

Security Type: MARKET_DATA"#, false),
    )]
    TokenizedAssets(TokenizedAssetsArgs),
    #[command(
        about = decode_selected_entities(r#"Paged history of mint / redeem conversions for the caller. Cursor-style pagination — supply the `nextLastId` from the previous page's response to fetch the next page.

Weight: 1

Security Type: USER_DATA"#, false),
    )]
    TokenizedConvertHistory(TokenizedConvertHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query the current status of a single mint / redeem request by its `issuerRequestId`. Returns an empty object when no record matches (caller's ID scope only).

Weight: 1

Security Type: USER_DATA"#, false),
    )]
    TokenizedConvertStatus(TokenizedConvertStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Mint a tokenized asset from an underlying equity holding. The caller's underlying equity (e.g. `AAPL`) is burned (or locked), and the corresponding tokenized asset (e.g. `AAPLB`) is credited. The tokenized asset is resolved server-side from `underlyingAsset`; callers only provide the underlying ticker and quantity. Mint is asynchronous: the endpoint returns an `issuerRequestId` immediately with a transient status; poll `/tokenized/convert-status` to observe the terminal state, or inspect `/tokenized/history` for the full record. Rate limit: 50 requests / min (UID).

Weight: 1

Security Type: TRADE"#, false),
    )]
    TokenizedMint(TokenizedMintArgs),
    #[command(
        about = decode_selected_entities(r#"Redeem a tokenized asset back into the underlying equity. The caller's tokenized asset (e.g. `AAPLB`) is burned, and the corresponding underlying equity (e.g. `AAPL`) is released. The underlying asset is resolved server-side from `tokenizedAsset`; callers only provide the tokenized asset and quantity. Redeem is asynchronous: the endpoint returns an `issuerRequestId` immediately with a transient status; poll `/tokenized/convert-status` to observe the terminal state, or inspect `/tokenized/history` for the full record. Rate limit: 200 requests / min (UID).

Weight: 1

Security Type: TRADE"#, false),
    )]
    TokenizedRedeem(TokenizedRedeemArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel all open orders for the caller. No request body is required — the scope is all of the caller's open orders. The downstream service handles idempotency; to inspect per-order outcomes, query `/order/history` after the call.

Weight: 1

Security Type: TRADE"#, false),
    )]
    CancelAllEquityOrders(CancelAllEquityOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel a single open order by its `orderId`.

Weight: 1

Security Type: TRADE"#, false),
    )]
    CancelEquityOrder(CancelEquityOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query all unfinished (open) orders for the caller. No business parameters are needed — user identity is derived from the signature.

Weight: 1

Security Type: USER_DATA"#, false),
    )]
    CurrentOpenOrders(CurrentOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Returns a single order along with its trade execution list. Callers can only read their own orders — the user identity is derived from the signed request.

Weight: 1

Security Type: USER_DATA"#, false),
    )]
    EquityOrderDetail(EquityOrderDetailArgs),
    #[command(
        about = decode_selected_entities(r#"Paged order history for the caller. Filters by symbol, order type, side, status, and time range.

Weight: 1

Security Type: USER_DATA"#, false),
    )]
    EquityOrderHistory(EquityOrderHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Paged equity trade (per-fill) history for the caller. Each row is one execution, not one order — a partially filled order produces multiple rows. Filters by symbol, side, specific `orderId`, and time range.

Weight: 1

Security Type: USER_DATA"#, false),
    )]
    EquityTradeHistory(EquityTradeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Place a new equity order. Supports all combinations of `LIMIT` /
`MARKET` × `BUY` / `SELL`. For `LIMIT BUY` orders the commission fee is
automatically computed and reserved by the server at placement time —
callers submit `price` and `quantity` only, no `fee` field is required.


**Field combination matrix**


| Side | OrderType | Required | Forbidden |
| ---- | --------- | -------- | --------- |
| BUY | LIMIT | `price`, `quantity`, `tradingSession` | `notional` |
| BUY | MARKET | `notional` | `price`, `quantity`, `tradingSession` |
| SELL | LIMIT | `price`, `quantity`, `tradingSession` | `notional` |
| SELL | MARKET | `quantity` | `price`, `notional`, `tradingSession` |


**Fractional shares**: when `quantity` has a decimal component, or an
order is placed by `notional`, it is treated as a fractional-share
order. A fractional-share `GTC` order must be paired with
`tradingSession = EXTENDED` or `24H`.


Rate limit: 200 requests / min (UID).

Weight: 1

Security Type: TRADE"#, false),
    )]
    PlaceEquityOrder(PlaceEquityOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Create a new stock user-data-stream `listenKey`, or renew an existing one. Calling this endpoint for the same user with an active `listenKey` extends its validity; if there is no active key, a new one is issued. Use the returned `listenKey` to subscribe to the stock user data stream.

Weight: 1

Security Type: USER_STREAM"#, false),
    )]
    CreateRenewListenKey(CreateRenewListenKeyArgs),
}

pub async fn handle_stocks_command(command: StocksCommands) -> anyhow::Result<()> {
    match command {
        StocksCommands::SignUsEquityDisclaimer(args) => sign_us_equity_disclaimer(args).await,

        StocksCommands::ExchangeInfo(args) => exchange_info(args).await,

        StocksCommands::LatestQuote(args) => latest_quote(args).await,

        StocksCommands::TokenizedAssets(args) => tokenized_assets(args).await,

        StocksCommands::TokenizedConvertHistory(args) => tokenized_convert_history(args).await,

        StocksCommands::TokenizedConvertStatus(args) => tokenized_convert_status(args).await,

        StocksCommands::TokenizedMint(args) => tokenized_mint(args).await,

        StocksCommands::TokenizedRedeem(args) => tokenized_redeem(args).await,

        StocksCommands::CancelAllEquityOrders(args) => cancel_all_equity_orders(args).await,

        StocksCommands::CancelEquityOrder(args) => cancel_equity_order(args).await,

        StocksCommands::CurrentOpenOrders(args) => current_open_orders(args).await,

        StocksCommands::EquityOrderDetail(args) => equity_order_detail(args).await,

        StocksCommands::EquityOrderHistory(args) => equity_order_history(args).await,

        StocksCommands::EquityTradeHistory(args) => equity_trade_history(args).await,

        StocksCommands::PlaceEquityOrder(args) => place_equity_order(args).await,

        StocksCommands::CreateRenewListenKey(args) => create_renew_listen_key(args).await,
    }
}

async fn sign_us_equity_disclaimer(args: SignUsEquityDisclaimerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SignUsEquityDisclaimerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SignUsEquityDisclaimerParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => SignUsEquityDisclaimerParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.sign_us_equity_disclaimer(params).await?;

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
            None => ExchangeInfoParams::builder().symbol(args.symbol).build()?,
        },
    };

    // Make the API call
    let response = rest_client.exchange_info(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn latest_quote(mut args: LatestQuoteArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<LatestQuoteParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<LatestQuoteParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.symbol.is_none() {
                    let symbol: String =
                        Input::new().with_prompt("Input symbol:").interact_text()?;

                    args.symbol = Some(symbol);
                }
                LatestQuoteParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.latest_quote(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn tokenized_assets(args: TokenizedAssetsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.tokenized_assets().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn tokenized_convert_history(args: TokenizedConvertHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TokenizedConvertHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TokenizedConvertHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => TokenizedConvertHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .last_id(args.last_id)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.tokenized_convert_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn tokenized_convert_status(mut args: TokenizedConvertStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TokenizedConvertStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TokenizedConvertStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.issuer_request_id.is_none() {
                        let issuer_request_id: String = Input::new()
                            .with_prompt("Input issuer_request_id:")
                            .interact_text()?;

                        args.issuer_request_id = Some(issuer_request_id);
                    }
                    if args.convert_type.is_none() {
                        let options = [
                            ("MINT", TokenizedConvertStatusConvertTypeEnum::Mint),
                            ("REDEEM", TokenizedConvertStatusConvertTypeEnum::Redeem),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the convert_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.convert_type = Some(selected);
                    }
                }
                TokenizedConvertStatusParams::builder(
                    args.issuer_request_id
                        .ok_or_else(|| anyhow::anyhow!("issuer-request-id is required"))?,
                    args.convert_type
                        .ok_or_else(|| anyhow::anyhow!("convert-type is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.tokenized_convert_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn tokenized_mint(mut args: TokenizedMintArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TokenizedMintParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TokenizedMintParams>(json).ok_or_else(|| {
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
                    if args.underlying_asset_amount.is_none() {
                        let underlying_asset_amount: String = Input::new()
                            .with_prompt("Input underlying_asset_amount:")
                            .interact_text()?;

                        args.underlying_asset_amount = Some(underlying_asset_amount);
                    }
                }
                TokenizedMintParams::builder(
                    args.underlying_asset
                        .ok_or_else(|| anyhow::anyhow!("underlying-asset is required"))?,
                    args.underlying_asset_amount
                        .ok_or_else(|| anyhow::anyhow!("underlying-asset-amount is required"))?,
                )
                .client_order_id(args.client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.tokenized_mint(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn tokenized_redeem(mut args: TokenizedRedeemArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TokenizedRedeemParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TokenizedRedeemParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.tokenized_asset.is_none() {
                        let tokenized_asset: String = Input::new()
                            .with_prompt("Input tokenized_asset:")
                            .interact_text()?;

                        args.tokenized_asset = Some(tokenized_asset);
                    }
                    if args.tokenized_asset_amount.is_none() {
                        let tokenized_asset_amount: String = Input::new()
                            .with_prompt("Input tokenized_asset_amount:")
                            .interact_text()?;

                        args.tokenized_asset_amount = Some(tokenized_asset_amount);
                    }
                }
                TokenizedRedeemParams::builder(
                    args.tokenized_asset
                        .ok_or_else(|| anyhow::anyhow!("tokenized-asset is required"))?,
                    args.tokenized_asset_amount
                        .ok_or_else(|| anyhow::anyhow!("tokenized-asset-amount is required"))?,
                )
                .client_order_id(args.client_order_id)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.tokenized_redeem(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_all_equity_orders(args: CancelAllEquityOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAllEquityOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAllEquityOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => CancelAllEquityOrdersParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.cancel_all_equity_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_equity_order(mut args: CancelEquityOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelEquityOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelEquityOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive && args.order_id.is_none() {
                    let order_id: String = Input::new()
                        .with_prompt("Input order_id:")
                        .interact_text()?;

                    args.order_id = Some(order_id);
                }
                CancelEquityOrderParams::builder(
                    args.order_id
                        .ok_or_else(|| anyhow::anyhow!("order-id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_equity_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn current_open_orders(args: CurrentOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CurrentOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CurrentOpenOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => CurrentOpenOrdersParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.current_open_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn equity_order_detail(args: EquityOrderDetailArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EquityOrderDetailParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<EquityOrderDetailParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => EquityOrderDetailParams::builder()
                .order_id(args.order_id)
                .client_order_id(args.client_order_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.equity_order_detail(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn equity_order_history(mut args: EquityOrderHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EquityOrderHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<EquityOrderHistoryParams>(json).ok_or_else(|| {
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
                EquityOrderHistoryParams::builder(
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start-time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end-time is required"))?,
                )
                .symbol(args.symbol)
                .order_type(args.order_type)
                .side(args.side)
                .order_status(args.order_status)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.equity_order_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn equity_trade_history(mut args: EquityTradeHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EquityTradeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<EquityTradeHistoryParams>(json).ok_or_else(|| {
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
                EquityTradeHistoryParams::builder(
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start-time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end-time is required"))?,
                )
                .symbol(args.symbol)
                .side(args.side)
                .order_id(args.order_id)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.equity_trade_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn place_equity_order(mut args: PlaceEquityOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PlaceEquityOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PlaceEquityOrderParams>(json).ok_or_else(|| {
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
                        let options = [
                            ("BUY", PlaceEquityOrderSideEnum::Buy),
                            ("SELL", PlaceEquityOrderSideEnum::Sell),
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
                    if args.order_type.is_none() {
                        let options = [
                            ("MARKET", PlaceEquityOrderOrderTypeEnum::Market),
                            ("LIMIT", PlaceEquityOrderOrderTypeEnum::Limit),
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
                }
                PlaceEquityOrderParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.order_type
                        .ok_or_else(|| anyhow::anyhow!("order-type is required"))?,
                )
                .quote_asset(args.quote_asset)
                .price(args.price)
                .quantity(args.quantity)
                .notional(args.notional)
                .time_in_force(args.time_in_force)
                .trading_session(args.trading_session)
                .wallet_type(args.wallet_type)
                .client_order_id(args.client_order_id)
                .tokenize(args.tokenize)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.place_equity_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn create_renew_listen_key(args: CreateRenewListenKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<CreateRenewListenKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CreateRenewListenKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => CreateRenewListenKeyParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.create_renew_listen_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
