use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::alpha::AlphaRestApi;
use binance_sdk::alpha::rest_api::*;
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::ALPHA_REST_API_PROD_URL;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("alpha");

    let client_config = get_client_configuration(profile, "alpha").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => ALPHA_REST_API_PROD_URL.to_string(),
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

    Ok(AlphaRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct AggregatedTradesArgs {
    #[arg(
        help = r#"Trading pair symbol, e.g. ALPHA_118USDC (use token ID from Token List)."#,
        long
    )]
    symbol: Option<String>,
    #[arg(help = r#"Starting aggregate trade ID to fetch from."#, long)]
    from_id: Option<i64>,
    #[arg(help = r#"Start timestamp in milliseconds."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End timestamp in milliseconds."#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results to return."#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct FullDepthArgs {
    #[arg(
        help = r#"Trading pair symbol, e.g. ALPHA_175USDT (use token ID from Token List)."#,
        long
    )]
    symbol: Option<String>,
    #[arg(
        help = r#"Number of price levels to return. Valid values: 5, 10, 20, 50, 100, 500, 1000."#,
        long
    )]
    limit: Option<FullDepthLimitEnum>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetExchangeInfoArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct KlinesArgs {
    #[arg(
        help = r#"Trading pair symbol, e.g. ALPHA_175USDT (use token ID from Token List)."#,
        long
    )]
    symbol: Option<String>,
    #[arg(help = r#"Kline interval."#, long)]
    interval: Option<KlinesIntervalEnum>,
    #[arg(help = r#"Number of klines to return."#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Start timestamp in milliseconds."#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End timestamp in milliseconds."#, long)]
    end_time: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct TickerArgs {
    #[arg(
        help = r#"Trading pair symbol, e.g. ALPHA_175USDT (use token ID from Token List)."#,
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
struct TokenListArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum AlphaCommands {
    #[command(
        about = decode_selected_entities(r#"Retrieves compressed, aggregated historical trades for a specific symbol. Useful for recent trade history."#, false),
    )]
    AggregatedTrades(AggregatedTradesArgs),
    #[command(
        about = decode_selected_entities(r#"Fetches the full order book depth (UI & API orders) for a symbol, including bid and ask orders with their prices and quantities."#, false),
    )]
    FullDepth(FullDepthArgs),
    #[command(
        about = decode_selected_entities(r#"Fetches general exchange information, such as supported symbols, rate limits, and server time."#, false),
    )]
    GetExchangeInfo(GetExchangeInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Fetches Kline/candlestick bars for a symbol, which include open/high/low/close prices and volume over intervals. Useful for charting and analysis."#, false),
    )]
    Klines(KlinesArgs),
    #[command(
        about = decode_selected_entities(r#"Gets the 24-hour rolling window price change statistics for a symbol, including volume and price changes."#, false),
    )]
    Ticker(TickerArgs),
    #[command(
        about = decode_selected_entities(r#"Retrieves a list of all available ALPHA tokens, including their IDs and symbols. Use this to find the token ID for constructing symbols in other endpoints."#, false),
    )]
    TokenList(TokenListArgs),
}

pub async fn handle_alpha_command(command: AlphaCommands) -> anyhow::Result<()> {
    match command {
        AlphaCommands::AggregatedTrades(args) => aggregated_trades(args).await,

        AlphaCommands::FullDepth(args) => full_depth(args).await,

        AlphaCommands::GetExchangeInfo(args) => get_exchange_info(args).await,

        AlphaCommands::Klines(args) => klines(args).await,

        AlphaCommands::Ticker(args) => ticker(args).await,

        AlphaCommands::TokenList(args) => token_list(args).await,
    }
}

async fn aggregated_trades(mut args: AggregatedTradesArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<AggregatedTradesParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AggregatedTradesParams>(json).ok_or_else(|| {
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
                AggregatedTradesParams::builder(
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
    let response = rest_client.aggregated_trades(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn full_depth(mut args: FullDepthArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<FullDepthParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FullDepthParams>(json).ok_or_else(|| {
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
                FullDepthParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.full_depth(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_exchange_info(args: GetExchangeInfoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.get_exchange_info().await?;

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
                            ("15s", KlinesIntervalEnum::Interval15s),
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
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
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

async fn ticker(mut args: TickerArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<TickerParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TickerParams>(json).ok_or_else(|| {
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
                TickerParams::builder(
                    args.symbol
                        .ok_or_else(|| anyhow::anyhow!("symbol is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.ticker(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn token_list(args: TokenListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.token_list().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
