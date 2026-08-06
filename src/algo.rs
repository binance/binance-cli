use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::algo::AlgoRestApi;
use binance_sdk::algo::rest_api::*;
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::ALGO_REST_API_PROD_URL;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("algo");

    let client_config = get_client_configuration(profile, "algo").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => ALGO_REST_API_PROD_URL.to_string(),
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

    Ok(AlgoRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct CancelAlgoOrderFutureAlgoArgs {
    #[arg(help = r#"eg. 14511"#, long)]
    algo_id: Option<i64>,
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
struct QueryCurrentAlgoOpenOrdersFutureAlgoArgs {
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
struct QueryHistoricalAlgoOrdersFutureAlgoArgs {
    #[arg(help = r#"Trading symbol eg. BTCUSDT"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"BUY or SELL"#, long)]
    side: Option<QueryHistoricalAlgoOrdersFutureAlgoSideEnum>,
    #[arg(help = r#"in milliseconds  eg.1641522717552"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"in milliseconds  eg.1641522526562"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Page number"#, long)]
    page: Option<i64>,
    #[arg(help = r#"Records per page"#, long)]
    page_size: Option<i64>,
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
struct QuerySubOrdersFutureAlgoArgs {
    #[arg(help = r#"eg. 14511"#, long)]
    algo_id: Option<i64>,
    #[arg(help = r#"Page number"#, long)]
    page: Option<i64>,
    #[arg(help = r#"Records per page"#, long)]
    page_size: Option<i64>,
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
struct TimeWeightedAveragePriceFutureAlgoArgs {
    #[arg(help = r#"Trading symbol eg. BTCUSDT"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Trading side ( BUY or SELL )"#, long)]
    side: Option<TimeWeightedAveragePriceFutureAlgoSideEnum>,
    #[arg(
        help = r#"Quantity of base asset; The notional (`quantity` * `mark price(base asset)`) must be more than the
equivalent of 1,000 USDT and less than the equivalent of 1,000,000 USDT"#,
        long
    )]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Duration for TWAP orders in seconds"#, long)]
    duration: Option<i64>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<TimeWeightedAveragePriceFutureAlgoPositionSideEnum>,
    #[arg(
        help = r#"A unique id among Algo orders (length should be 32 characters)， If it is not sent, we will give
default value"#,
        long
    )]
    client_algo_id: Option<String>,
    #[arg(help = r#""true" or "false". Default "false"; Cannot be sent in Hedge Mode; Cannot be sent when you open a
position"#, long, num_args = 0..=1, default_missing_value = "true")]
    reduce_only: Option<bool>,
    #[arg(
        help = r#"Limit price of the order; If it is not sent, will place order by market price by default"#,
        long
    )]
    limit_price: Option<rust_decimal::Decimal>,
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
struct VolumeParticipationFutureAlgoArgs {
    #[arg(help = r#"Trading symbol eg. BTCUSDT"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Trading side ( BUY or SELL )"#, long)]
    side: Option<VolumeParticipationFutureAlgoSideEnum>,
    #[arg(
        help = r#"Quantity of base asset; The notional (`quantity` * `mark price(base asset)`) must be more than the
equivalent of 10,000 USDT and less than the equivalent of 1,000,000 USDT"#,
        long
    )]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"Represent the relative speed of the current execution; ENUM: LOW, MEDIUM, HIGH"#,
        long
    )]
    urgency: Option<VolumeParticipationFutureAlgoUrgencyEnum>,
    #[arg(
        help = r#"Default `BOTH` for One-way Mode ; `LONG` or `SHORT` for Hedge Mode. It must be sent in Hedge Mode."#,
        long
    )]
    position_side: Option<VolumeParticipationFutureAlgoPositionSideEnum>,
    #[arg(
        help = r#"A unique id among Algo orders (length should be 32 characters)， If it is not sent, we will give
default value"#,
        long
    )]
    client_algo_id: Option<String>,
    #[arg(help = r#""true" or "false". Default "false"; Cannot be sent in Hedge Mode; Cannot be sent when you open a
position"#, long, num_args = 0..=1, default_missing_value = "true")]
    reduce_only: Option<bool>,
    #[arg(
        help = r#"Limit price of the order; If it is not sent, will place order by market price by default"#,
        long
    )]
    limit_price: Option<rust_decimal::Decimal>,
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
struct CancelAlgoOrderSpotAlgoArgs {
    #[arg(help = r#""#, long)]
    algo_id: Option<i64>,
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
struct QueryCurrentAlgoOpenOrdersSpotAlgoArgs {
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
struct QueryHistoricalAlgoOrdersSpotAlgoArgs {
    #[arg(help = r#"Trading symbol"#, long)]
    symbol: Option<String>,
    #[arg(help = r#""#, long)]
    side: Option<QueryHistoricalAlgoOrdersSpotAlgoSideEnum>,
    #[arg(help = r#"in milliseconds  eg.1641522717552"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"in milliseconds  eg.1641522526562"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Page number"#, long)]
    page: Option<i64>,
    #[arg(help = r#"Records per page"#, long)]
    page_size: Option<i64>,
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
struct QuerySubOrdersSpotAlgoArgs {
    #[arg(help = r#"eg. 14511"#, long)]
    algo_id: Option<i64>,
    #[arg(help = r#"Page number"#, long)]
    page: Option<i64>,
    #[arg(help = r#"Records per page"#, long)]
    page_size: Option<i64>,
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
struct TimeWeightedAveragePriceSpotAlgoArgs {
    #[arg(help = r#"Trading symbol eg. BTCUSDT"#, long)]
    symbol: Option<String>,
    #[arg(help = r#"Trading side ( BUY or SELL )"#, long)]
    side: Option<TimeWeightedAveragePriceSpotAlgoSideEnum>,
    #[arg(
        help = r#"Quantity of base asset; Maximum notional per order is 200k, 2mm or 10mm, depending on symbol. Please
reduce your size if you order is above the maximum notional per order."#,
        long
    )]
    quantity: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Duration for TWAP orders in seconds"#, long)]
    duration: Option<i64>,
    #[arg(
        help = r#"A unique id among Algo orders (length should be 32 characters)， If it is not sent, we will give
default value"#,
        long
    )]
    client_algo_id: Option<String>,
    #[arg(
        help = r#"Limit price of the order; If it is not sent, will place order by market price by default"#,
        long
    )]
    limit_price: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum AlgoCommands {
    #[command(
        about = decode_selected_entities(r#"Cancel an active order.

Weight(IP): 1

Security Type: TRADE

Notes:
- You need to enable `Futures Trading Permission` for the API key that requests this endpoint.
- Base URL: `https://api.binance.com`"#, false),
    )]
    CancelAlgoOrderFutureAlgo(CancelAlgoOrderFutureAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Query Current Algo Open Orders

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to enable `Futures Trading Permission` for the API key that requests this endpoint.
- Base URL: `https://api.binance.com`"#, false),
    )]
    QueryCurrentAlgoOpenOrdersFutureAlgo(QueryCurrentAlgoOpenOrdersFutureAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Query Historical Algo Order

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to enable `Futures Trading Permission` for the API key that requests this endpoint.
- Base URL: `https://api.binance.com`"#, false),
    )]
    QueryHistoricalAlgoOrdersFutureAlgo(QueryHistoricalAlgoOrdersFutureAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Get respective sub orders for a specified algoId

Weight(IP): 1

Security Type: USER_DATA

Notes:
- You need to enable `Futures Trading Permission` for the API key that requests this endpoint.
- Base URL: `https://api.binance.com`"#, false),
    )]
    QuerySubOrdersFutureAlgo(QuerySubOrdersFutureAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a Twap new order. Only support on USDⓈ-M Contracts.

Weight(UID): 3000

Security Type: TRADE

Notes:
- Other info:
  - Total Algo open orders max allowed: `30` orders.
  - Leverage and position mode follow your futures account settings.
  - Receiving `"success": true` does not guarantee execution; query order endpoints for final status.
  - If balance/position constraints fail, response may still return success but order status becomes `expired`.
  - `quantity * 60 / duration` must be greater than `minQty`.
  - `duration` cannot be less than 5 minutes or greater than 24 hours.
  - For delivery contracts, TWAP end time should be one hour earlier than symbol delivery time.
  - You need to enable the corresponding permission for the API key requesting this endpoint:
    - `Futures Trading Permission` — for Classic Trading Account mode
    - `Portfolio Margin Trading Permission` — for Portfolio Margin Account mode
  - Base URL: `https://api.binance.com`"#, false),
    )]
    TimeWeightedAveragePriceFutureAlgo(TimeWeightedAveragePriceFutureAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Send in a VP new order. Only support on USDⓈ-M Contracts.

Weight(UID): 300

Security Type: TRADE

Notes:
- Other info:
  - Total Algo open orders max allowed: `10` orders.
  - Leverage and position mode follow your futures account settings.
  - Receiving `"success": true` does not guarantee execution; query order endpoints for final status.
  - If balance/position constraints fail, response may still return success but order status becomes `expired`.
  - You need to enable the corresponding permission for the API key requesting this endpoint:
    - `Futures Trading Permission` — for Classic Trading Account mode
    - `Portfolio Margin Trading Permission` — for Portfolio Margin Account mode
  - Base URL: `https://api.binance.com`"#, false),
    )]
    VolumeParticipationFutureAlgo(VolumeParticipationFutureAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel an open TWAP order

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CancelAlgoOrderSpotAlgo(CancelAlgoOrderSpotAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Get all open SPOT TWAP orders

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryCurrentAlgoOpenOrdersSpotAlgo(QueryCurrentAlgoOpenOrdersSpotAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Get all historical SPOT TWAP orders

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QueryHistoricalAlgoOrdersSpotAlgo(QueryHistoricalAlgoOrdersSpotAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Get respective sub orders for a specified algoId

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    QuerySubOrdersSpotAlgo(QuerySubOrdersSpotAlgoArgs),
    #[command(
        about = decode_selected_entities(r#"Place a new spot TWAP order with Algo service.

Weight(UID): 3000

Security Type: TRADE

Notes:
- Total Algo open orders max allowed: `20` orders."#, false),
    )]
    TimeWeightedAveragePriceSpotAlgo(TimeWeightedAveragePriceSpotAlgoArgs),
}

pub async fn handle_algo_command(command: AlgoCommands) -> anyhow::Result<()> {
    match command {
        AlgoCommands::CancelAlgoOrderFutureAlgo(args) => cancel_algo_order_future_algo(args).await,

        AlgoCommands::QueryCurrentAlgoOpenOrdersFutureAlgo(args) => {
            query_current_algo_open_orders_future_algo(args).await
        }

        AlgoCommands::QueryHistoricalAlgoOrdersFutureAlgo(args) => {
            query_historical_algo_orders_future_algo(args).await
        }

        AlgoCommands::QuerySubOrdersFutureAlgo(args) => query_sub_orders_future_algo(args).await,

        AlgoCommands::TimeWeightedAveragePriceFutureAlgo(args) => {
            time_weighted_average_price_future_algo(args).await
        }

        AlgoCommands::VolumeParticipationFutureAlgo(args) => {
            volume_participation_future_algo(args).await
        }

        AlgoCommands::CancelAlgoOrderSpotAlgo(args) => cancel_algo_order_spot_algo(args).await,

        AlgoCommands::QueryCurrentAlgoOpenOrdersSpotAlgo(args) => {
            query_current_algo_open_orders_spot_algo(args).await
        }

        AlgoCommands::QueryHistoricalAlgoOrdersSpotAlgo(args) => {
            query_historical_algo_orders_spot_algo(args).await
        }

        AlgoCommands::QuerySubOrdersSpotAlgo(args) => query_sub_orders_spot_algo(args).await,

        AlgoCommands::TimeWeightedAveragePriceSpotAlgo(args) => {
            time_weighted_average_price_spot_algo(args).await
        }
    }
}

async fn cancel_algo_order_future_algo(
    mut args: CancelAlgoOrderFutureAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAlgoOrderFutureAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CancelAlgoOrderFutureAlgoParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.algo_id.is_none() {
                        let algo_id: i64 =
                            Input::new().with_prompt("Input algo_id:").interact_text()?;

                        args.algo_id = Some(algo_id);
                    }
                }
                CancelAlgoOrderFutureAlgoParams::builder(
                    args.algo_id
                        .ok_or_else(|| anyhow::anyhow!("algo_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_algo_order_future_algo(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_algo_open_orders_future_algo(
    args: QueryCurrentAlgoOpenOrdersFutureAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentAlgoOpenOrdersFutureAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCurrentAlgoOpenOrdersFutureAlgoParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryCurrentAlgoOpenOrdersFutureAlgoParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_current_algo_open_orders_future_algo(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_historical_algo_orders_future_algo(
    args: QueryHistoricalAlgoOrdersFutureAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryHistoricalAlgoOrdersFutureAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryHistoricalAlgoOrdersFutureAlgoParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryHistoricalAlgoOrdersFutureAlgoParams::builder()
                .symbol(args.symbol)
                .side(args.side)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .page(args.page)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_historical_algo_orders_future_algo(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_orders_future_algo(
    mut args: QuerySubOrdersFutureAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubOrdersFutureAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QuerySubOrdersFutureAlgoParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.algo_id.is_none() {
                        let algo_id: i64 =
                            Input::new().with_prompt("Input algo_id:").interact_text()?;

                        args.algo_id = Some(algo_id);
                    }
                }
                QuerySubOrdersFutureAlgoParams::builder(
                    args.algo_id
                        .ok_or_else(|| anyhow::anyhow!("algo_id is required"))?,
                )
                .page(args.page)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_sub_orders_future_algo(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn time_weighted_average_price_future_algo(
    mut args: TimeWeightedAveragePriceFutureAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TimeWeightedAveragePriceFutureAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TimeWeightedAveragePriceFutureAlgoParams>(json)
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
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", TimeWeightedAveragePriceFutureAlgoSideEnum::Buy),
                            ("SELL", TimeWeightedAveragePriceFutureAlgoSideEnum::Sell),
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
                    if args.duration.is_none() {
                        let duration: i64 = Input::new()
                            .with_prompt("Input duration:")
                            .interact_text()?;

                        args.duration = Some(duration);
                    }
                }
                TimeWeightedAveragePriceFutureAlgoParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                    args.duration
                        .ok_or_else(|| anyhow::anyhow!("duration is required"))?,
                )
                .position_side(args.position_side)
                .client_algo_id(args.client_algo_id)
                .reduce_only(args.reduce_only)
                .limit_price(args.limit_price)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .time_weighted_average_price_future_algo(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn volume_participation_future_algo(
    mut args: VolumeParticipationFutureAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<VolumeParticipationFutureAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<VolumeParticipationFutureAlgoParams>(json).ok_or_else(|| {
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
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", VolumeParticipationFutureAlgoSideEnum::Buy),
                            ("SELL", VolumeParticipationFutureAlgoSideEnum::Sell),
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
                    if args.urgency.is_none() {
                        let options = vec![
                            ("LOW", VolumeParticipationFutureAlgoUrgencyEnum::Low),
                            ("MEDIUM", VolumeParticipationFutureAlgoUrgencyEnum::Medium),
                            ("HIGH", VolumeParticipationFutureAlgoUrgencyEnum::High),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the urgency")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.urgency = Some(selected);
                    }
                }
                VolumeParticipationFutureAlgoParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                    args.urgency
                        .ok_or_else(|| anyhow::anyhow!("urgency is required"))?,
                )
                .position_side(args.position_side)
                .client_algo_id(args.client_algo_id)
                .reduce_only(args.reduce_only)
                .limit_price(args.limit_price)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.volume_participation_future_algo(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_algo_order_spot_algo(mut args: CancelAlgoOrderSpotAlgoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelAlgoOrderSpotAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelAlgoOrderSpotAlgoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo_id.is_none() {
                        let algo_id: i64 =
                            Input::new().with_prompt("Input algo_id:").interact_text()?;

                        args.algo_id = Some(algo_id);
                    }
                }
                CancelAlgoOrderSpotAlgoParams::builder(
                    args.algo_id
                        .ok_or_else(|| anyhow::anyhow!("algo_id is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.cancel_algo_order_spot_algo(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_current_algo_open_orders_spot_algo(
    args: QueryCurrentAlgoOpenOrdersSpotAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryCurrentAlgoOpenOrdersSpotAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryCurrentAlgoOpenOrdersSpotAlgoParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryCurrentAlgoOpenOrdersSpotAlgoParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_current_algo_open_orders_spot_algo(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_historical_algo_orders_spot_algo(
    args: QueryHistoricalAlgoOrdersSpotAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryHistoricalAlgoOrdersSpotAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryHistoricalAlgoOrdersSpotAlgoParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryHistoricalAlgoOrdersSpotAlgoParams::builder()
                .symbol(args.symbol)
                .side(args.side)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .page(args.page)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_historical_algo_orders_spot_algo(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_sub_orders_spot_algo(mut args: QuerySubOrdersSpotAlgoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QuerySubOrdersSpotAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QuerySubOrdersSpotAlgoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo_id.is_none() {
                        let algo_id: i64 =
                            Input::new().with_prompt("Input algo_id:").interact_text()?;

                        args.algo_id = Some(algo_id);
                    }
                }
                QuerySubOrdersSpotAlgoParams::builder(
                    args.algo_id
                        .ok_or_else(|| anyhow::anyhow!("algo_id is required"))?,
                )
                .page(args.page)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_sub_orders_spot_algo(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn time_weighted_average_price_spot_algo(
    mut args: TimeWeightedAveragePriceSpotAlgoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TimeWeightedAveragePriceSpotAlgoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<TimeWeightedAveragePriceSpotAlgoParams>(json).ok_or_else(|| {
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
                    if args.side.is_none() {
                        let options = vec![
                            ("BUY", TimeWeightedAveragePriceSpotAlgoSideEnum::Buy),
                            ("SELL", TimeWeightedAveragePriceSpotAlgoSideEnum::Sell),
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
                    if args.duration.is_none() {
                        let duration: i64 = Input::new()
                            .with_prompt("Input duration:")
                            .interact_text()?;

                        args.duration = Some(duration);
                    }
                }
                TimeWeightedAveragePriceSpotAlgoParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                    args.side
                        .ok_or_else(|| anyhow::anyhow!("side is required"))?,
                    args.quantity
                        .ok_or_else(|| anyhow::anyhow!("quantity is required"))?,
                    args.duration
                        .ok_or_else(|| anyhow::anyhow!("duration is required"))?,
                )
                .client_algo_id(args.client_algo_id)
                .limit_price(args.limit_price)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .time_weighted_average_price_spot_algo(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
