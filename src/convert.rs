use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::CONVERT_REST_API_PROD_URL;
use binance_sdk::convert::ConvertRestApi;
use binance_sdk::convert::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("convert");

    let client_config = get_client_configuration(profile, "convert").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => CONVERT_REST_API_PROD_URL.to_string(),
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

    Ok(ConvertRestApi::from_config(rest_conf))
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
struct QueryOrderQuantityPrecisionPerAssetArgs {
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
struct AcceptQuoteArgs {
    #[arg(help = r#""#, long)]
    quote_id: Option<String>,
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
struct CancelLimitOrderArgs {
    #[arg(help = r#"The orderId from `placeOrder` api"#, long)]
    order_id: Option<i64>,
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
struct GetConvertTradeHistoryArgs {
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of records to return"#, long)]
    limit: Option<i64>,
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
struct PlaceLimitOrderArgs {
    #[arg(
        help = r#"base asset (use the response `fromIsBase` from `GET /sapi/v1/convert/exchangeInfo` api to check
which one is baseAsset )"#,
        long
    )]
    base_asset: Option<String>,
    #[arg(help = r#"quote asset"#, long)]
    quote_asset: Option<String>,
    #[arg(help = r#"Symbol limit price (from baseAsset to quoteAsset)"#, long)]
    limit_price: Option<rust_decimal::Decimal>,
    #[arg(help = r#"`BUY` or `SELL`"#, long)]
    side: Option<PlaceLimitOrderSideEnum>,
    #[arg(
        help = r#"Order expiry duration. 1_D, 3_D, 7_D, 30_D (D means day)"#,
        long
    )]
    expired_type: Option<PlaceLimitOrderExpiredTypeEnum>,
    #[arg(
        help = r#"Base asset amount. (One of `baseAmount` or `quoteAmount` is required)"#,
        long
    )]
    base_amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Quote asset amount. (One of `baseAmount` or `quoteAmount` is required)"#,
        long
    )]
    quote_amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Wallet to use for payment. Supported values: `SPOT`, `FUNDING`, `EARN`.
Combined wallets also supported: `SPOT_FUNDING`, `FUNDING_EARN`, `SPOT_FUNDING_EARN`, `SPOT_EARN`. Default is `SPOT`."#,
        long
    )]
    wallet_type: Option<PlaceLimitOrderWalletTypeEnum>,
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
struct QueryLimitOpenOrdersArgs {
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
    #[arg(
        help = r#"Wallet to use for payment. Supported values: `SPOT`, `FUNDING`, `EARN`.
Combined wallets also supported: `SPOT_FUNDING`, `FUNDING_EARN`, `SPOT_FUNDING_EARN`, `SPOT_EARN`. Default is `SPOT`."#,
        long
    )]
    wallet_type: Option<SendQuoteRequestWalletTypeEnum>,
    #[arg(
        help = r#"Quote valid duration. Supported values: 10s, 30s, 1m. Default is 10s."#,
        long
    )]
    valid_time: Option<SendQuoteRequestValidTimeEnum>,
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
pub enum ConvertCommands {
    #[command(
        about = decode_selected_entities(r#"Query for all convertible token pairs and the tokens’ respective
upper/lower limits

Weight(IP): 3000

Notes:
- User needs to supply either or both input parameters.
- If only one of `fromAsset` and `toAsset` is provided, only partial token pairs are returned."#, false),
    )]
    ListAllConvertPairs(ListAllConvertPairsArgs),
    #[command(
        about = decode_selected_entities(r#"Query for supported asset’s precision information

Weight(IP): 100

Security Type: USER_DATA"#, false),
    )]
    QueryOrderQuantityPrecisionPerAsset(QueryOrderQuantityPrecisionPerAssetArgs),
    #[command(
        about = decode_selected_entities(r#"Accept the offered quote by quote ID.

Weight(UID): 500

Security Type: TRADE"#, false),
    )]
    AcceptQuote(AcceptQuoteArgs),
    #[command(
        about = decode_selected_entities(r#"Enable users to cancel a limit order

Weight(UID): 200

Security Type: TRADE"#, false),
    )]
    CancelLimitOrder(CancelLimitOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Get Convert Trade History

Weight(UID): 3000

Security Type: USER_DATA

Notes:
- The max interval between `startTime` and `endTime` is 30 days."#, false),
    )]
    GetConvertTradeHistory(GetConvertTradeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query order status by order ID.

Weight(UID): 100

Security Type: USER_DATA"#, false),
    )]
    OrderStatus(OrderStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Enable users to place a limit order

Weight(UID): 500

Security Type: TRADE

Notes:
- `baseAsset` and `quoteAsset` can be determined via the `exchangeInfo` endpoint.
- Limit price is defined from `baseAsset` to `quoteAsset`.
- Exactly one of `baseAmount` or `quoteAmount` should be sent."#, false),
    )]
    PlaceLimitOrder(PlaceLimitOrderArgs),
    #[command(
        about = decode_selected_entities(r#"Query current open limit orders

Weight(UID): 3000

Security Type: USER_DATA"#, false),
    )]
    QueryLimitOpenOrders(QueryLimitOpenOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Request a quote for the requested token pairs

Weight(UID): 200

Security Type: TRADE

Notes:
- Either `fromAmount` or `toAmount` should be sent.
- `quoteId` is returned only if you have enough funds to convert."#, false),
    )]
    SendQuoteRequest(SendQuoteRequestArgs),
}

pub async fn handle_convert_command(command: ConvertCommands) -> anyhow::Result<()> {
    match command {
        ConvertCommands::ListAllConvertPairs(args) => list_all_convert_pairs(args).await,

        ConvertCommands::QueryOrderQuantityPrecisionPerAsset(args) => {
            query_order_quantity_precision_per_asset(args).await
        }

        ConvertCommands::AcceptQuote(args) => accept_quote(args).await,

        ConvertCommands::CancelLimitOrder(args) => cancel_limit_order(args).await,

        ConvertCommands::GetConvertTradeHistory(args) => get_convert_trade_history(args).await,

        ConvertCommands::OrderStatus(args) => order_status(args).await,

        ConvertCommands::PlaceLimitOrder(args) => place_limit_order(args).await,

        ConvertCommands::QueryLimitOpenOrders(args) => query_limit_open_orders(args).await,

        ConvertCommands::SendQuoteRequest(args) => send_quote_request(args).await,
    }
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

async fn query_order_quantity_precision_per_asset(
    args: QueryOrderQuantityPrecisionPerAssetArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryOrderQuantityPrecisionPerAssetParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryOrderQuantityPrecisionPerAssetParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryOrderQuantityPrecisionPerAssetParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_order_quantity_precision_per_asset(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn accept_quote(mut args: AcceptQuoteArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AcceptQuoteParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AcceptQuoteParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.quote_id.is_none() {
                        let quote_id: String = Input::new()
                            .with_prompt("Input quote_id:")
                            .interact_text()?;

                        args.quote_id = Some(quote_id);
                    }
                }
                AcceptQuoteParams::builder(
                    args.quote_id
                        .ok_or_else(|| anyhow::anyhow!("quote_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.accept_quote(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_limit_order(mut args: CancelLimitOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelLimitOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelLimitOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.order_id.is_none() {
                        let order_id: i64 = Input::new()
                            .with_prompt("Input order_id:")
                            .interact_text()?;

                        args.order_id = Some(order_id);
                    }
                }
                CancelLimitOrderParams::builder(
                    args.order_id
                        .ok_or_else(|| anyhow::anyhow!("order_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_limit_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_convert_trade_history(mut args: GetConvertTradeHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetConvertTradeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetConvertTradeHistoryParams>(json).ok_or_else(|| {
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
                GetConvertTradeHistoryParams::builder(
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                )
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_convert_trade_history(params).await?;

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

async fn place_limit_order(mut args: PlaceLimitOrderArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PlaceLimitOrderParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PlaceLimitOrderParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.base_asset.is_none() {
                        let base_asset: String = Input::new()
                            .with_prompt("Input base_asset:")
                            .interact_text()?;

                        args.base_asset = Some(base_asset);
                    }
                    if args.quote_asset.is_none() {
                        let quote_asset: String = Input::new()
                            .with_prompt("Input quote_asset:")
                            .interact_text()?;

                        args.quote_asset = Some(quote_asset);
                    }
                    if args.limit_price.is_none() {
                        let limit_price: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input limit_price:")
                            .interact_text()?;

                        args.limit_price = Some(limit_price);
                    }
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", PlaceLimitOrderSideEnum::Buy),
                            ("SELL", PlaceLimitOrderSideEnum::Sell),
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
                    if args.expired_type.is_none() {
                        let options = vec![
                            ("1_D", PlaceLimitOrderExpiredTypeEnum::ExpiredType1D),
                            ("3_D", PlaceLimitOrderExpiredTypeEnum::ExpiredType3D),
                            ("7_D", PlaceLimitOrderExpiredTypeEnum::ExpiredType7D),
                            ("30_D", PlaceLimitOrderExpiredTypeEnum::ExpiredType30D),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the expired_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.expired_type = Some(selected);
                    }
                }
                PlaceLimitOrderParams::builder(
                    args.base_asset
                        .ok_or_else(|| anyhow::anyhow!("base_asset is required"))?,
                    args.quote_asset
                        .ok_or_else(|| anyhow::anyhow!("quote_asset is required"))?,
                    args.limit_price
                        .ok_or_else(|| anyhow::anyhow!("limit_price is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.expired_type
                        .ok_or_else(|| anyhow::anyhow!("expired_type is required"))?,
                )
                .base_amount(args.base_amount)
                .quote_amount(args.quote_amount)
                .wallet_type(args.wallet_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.place_limit_order(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_limit_open_orders(args: QueryLimitOpenOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryLimitOpenOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryLimitOpenOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryLimitOpenOrdersParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_limit_open_orders(params).await?;

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
                            .with_prompt("Input from_asset:")
                            .interact_text()?;

                        args.from_asset = Some(from_asset);
                    }
                    if args.to_asset.is_none() {
                        let to_asset: String = Input::new()
                            .with_prompt("Input to_asset:")
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
                .wallet_type(args.wallet_type)
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
