use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::MINING_REST_API_PROD_URL;
use binance_sdk::mining::MiningRestApi;
use binance_sdk::mining::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::Input;
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("mining");

    let client_config = get_client_configuration(profile, "mining").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => MINING_REST_API_PROD_URL.to_string(),
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

    Ok(MiningRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct AccountListArgs {
    #[arg(help = r#"Algorithm name."#, long)]
    algo: Option<String>,
    #[arg(help = r#"Mining account"#, long)]
    user_name: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct AcquiringAlgorithmArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct AcquiringCoinnameArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CancelHashrateResaleConfigurationArgs {
    #[arg(help = r#"Mining ID"#, long)]
    config_id: Option<i64>,
    #[arg(help = r#"Mining Account"#, long)]
    user_name: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct EarningsListArgs {
    #[arg(help = r#"Algorithm name."#, long)]
    algo: Option<String>,
    #[arg(help = r#"Mining account."#, long)]
    user_name: Option<String>,
    #[arg(help = r#"Coin name"#, long)]
    coin: Option<String>,
    #[arg(help = r#"Search start time in milliseconds."#, long)]
    start_date: Option<i64>,
    #[arg(help = r#"Search end time in milliseconds."#, long)]
    end_date: Option<i64>,
    #[arg(help = r#"Page number, starting from 1."#, long)]
    page_index: Option<i64>,
    #[arg(help = r#"Number of rows per page."#, long)]
    page_size: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct ExtraBonusListArgs {
    #[arg(help = r#"Transfer algorithm"#, long)]
    algo: Option<String>,
    #[arg(help = r#"Mining account"#, long)]
    user_name: Option<String>,
    #[arg(help = r#"Coin name"#, long)]
    coin: Option<String>,
    #[arg(help = r#"Search start time in milliseconds."#, long)]
    start_date: Option<i64>,
    #[arg(help = r#"Search end time in milliseconds."#, long)]
    end_date: Option<i64>,
    #[arg(help = r#"Page number, starting from 1."#, long)]
    page_index: Option<i64>,
    #[arg(help = r#"Number of rows per page."#, long)]
    page_size: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct HashrateResaleDetailArgs {
    #[arg(help = r#"Configuration ID."#, long)]
    config_id: Option<i64>,
    #[arg(help = r#"Page number, starting from 1."#, long)]
    page_index: Option<i64>,
    #[arg(help = r#"Number of rows per page."#, long)]
    page_size: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct HashrateResaleListArgs {
    #[arg(help = r#"Page number, starting from 1."#, long)]
    page_index: Option<i64>,
    #[arg(help = r#"Number of rows per page."#, long)]
    page_size: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct HashrateResaleRequestArgs {
    #[arg(help = r#"Mining Account"#, long)]
    user_name: Option<String>,
    #[arg(help = r#"Transfer algorithm"#, long)]
    algo: Option<String>,
    #[arg(help = r#"Resale End Time (Millisecond timestamp)"#, long)]
    end_date: Option<i64>,
    #[arg(help = r#"Resale Start Time(Millisecond timestamp)"#, long)]
    start_date: Option<i64>,
    #[arg(help = r#"Mining Account"#, long)]
    to_pool_user: Option<String>,
    #[arg(
        help = r#"Resale hashrate h/s must be transferred (BTC is greater than 500000000000 ETH is greater than
500000)"#,
        long
    )]
    hash_rate: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct MiningAccountEarningArgs {
    #[arg(help = r#"Algorithm"#, long)]
    algo: Option<String>,
    #[arg(help = r#"Millisecond timestamp"#, long)]
    start_date: Option<i64>,
    #[arg(help = r#"Millisecond timestamp"#, long)]
    end_date: Option<i64>,
    #[arg(help = r#"Page number, starting from 1."#, long)]
    page_index: Option<i64>,
    #[arg(help = r#"Number of rows per page."#, long)]
    page_size: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RequestForDetailMinerListArgs {
    #[arg(help = r#"Algorithm"#, long)]
    algo: Option<String>,
    #[arg(help = r#"Mining account"#, long)]
    user_name: Option<String>,
    #[arg(help = r#"Miner name."#, long)]
    worker_name: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct RequestForMinerListArgs {
    #[arg(help = r#"Algorithm"#, long)]
    algo: Option<String>,
    #[arg(help = r#"Mining account"#, long)]
    user_name: Option<String>,
    #[arg(help = r#"Page number, starting from 1."#, long)]
    page_index: Option<i64>,
    #[arg(help = r#"Sort order. 0 for ascending, 1 for descending."#, long)]
    sort: Option<i64>,
    #[arg(
        help = r#"Sort by: 1 miner name, 2 real-time hashrate, 3 daily average hashrate, 4 real-time rejection
rate, 5 last submission time"#,
        long
    )]
    sort_column: Option<i64>,
    #[arg(help = r#"Miner status. 0 all, 1 valid, 2 invalid, 3 failure."#, long)]
    worker_status: Option<i64>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct StatisticListArgs {
    #[arg(help = r#"Algorithm"#, long)]
    algo: Option<String>,
    #[arg(help = r#"Mining account"#, long)]
    user_name: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds."#, long)]
    recv_window: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum MiningCommands {
    #[command(
        about = decode_selected_entities(r#"Query Account List

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    AccountList(AccountListArgs),
    #[command(
        about = decode_selected_entities(r#"Acquiring Algorithm

Weight(IP): 1

Security Type: MARKET_DATA"#, false),
    )]
    AcquiringAlgorithm(AcquiringAlgorithmArgs),
    #[command(
        about = decode_selected_entities(r#"Acquiring CoinName

Weight(IP): 1

Security Type: MARKET_DATA"#, false),
    )]
    AcquiringCoinname(AcquiringCoinnameArgs),
    #[command(
        about = decode_selected_entities(r#"Cancel hashrate resale configuration

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    CancelHashrateResaleConfiguration(CancelHashrateResaleConfigurationArgs),
    #[command(
        about = decode_selected_entities(r#"Query Earnings List

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    EarningsList(EarningsListArgs),
    #[command(
        about = decode_selected_entities(r#"Extra Bonus List

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    ExtraBonusList(ExtraBonusListArgs),
    #[command(
        about = decode_selected_entities(r#"Hashrate Resale Detail(USER_DATA)

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    HashrateResaleDetail(HashrateResaleDetailArgs),
    #[command(
        about = decode_selected_entities(r#"Hashrate Resale List

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    HashrateResaleList(HashrateResaleListArgs),
    #[command(
        about = decode_selected_entities(r#"Hashrate Resale Request

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    HashrateResaleRequest(HashrateResaleRequestArgs),
    #[command(
        about = decode_selected_entities(r#"Mining Account Earning

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    MiningAccountEarning(MiningAccountEarningArgs),
    #[command(
        about = decode_selected_entities(r#"Request for Detail Miner List

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    RequestForDetailMinerList(RequestForDetailMinerListArgs),
    #[command(
        about = decode_selected_entities(r#"Request for Miner List

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    RequestForMinerList(RequestForMinerListArgs),
    #[command(
        about = decode_selected_entities(r#"Statistic List

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    StatisticList(StatisticListArgs),
}

pub async fn handle_mining_command(command: MiningCommands) -> anyhow::Result<()> {
    match command {
        MiningCommands::AccountList(args) => account_list(args).await,

        MiningCommands::AcquiringAlgorithm(args) => acquiring_algorithm(args).await,

        MiningCommands::AcquiringCoinname(args) => acquiring_coinname(args).await,

        MiningCommands::CancelHashrateResaleConfiguration(args) => {
            cancel_hashrate_resale_configuration(args).await
        }

        MiningCommands::EarningsList(args) => earnings_list(args).await,

        MiningCommands::ExtraBonusList(args) => extra_bonus_list(args).await,

        MiningCommands::HashrateResaleDetail(args) => hashrate_resale_detail(args).await,

        MiningCommands::HashrateResaleList(args) => hashrate_resale_list(args).await,

        MiningCommands::HashrateResaleRequest(args) => hashrate_resale_request(args).await,

        MiningCommands::MiningAccountEarning(args) => mining_account_earning(args).await,

        MiningCommands::RequestForDetailMinerList(args) => {
            request_for_detail_miner_list(args).await
        }

        MiningCommands::RequestForMinerList(args) => request_for_miner_list(args).await,

        MiningCommands::StatisticList(args) => statistic_list(args).await,
    }
}

async fn account_list(mut args: AccountListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo.is_none() {
                        let algo: String =
                            Input::new().with_prompt("Input algo:").interact_text()?;

                        args.algo = Some(algo);
                    }
                    if args.user_name.is_none() {
                        let user_name: String = Input::new()
                            .with_prompt("Input user_name:")
                            .interact_text()?;

                        args.user_name = Some(user_name);
                    }
                }
                AccountListParams::builder(
                    args.algo
                        .ok_or_else(|| anyhow::anyhow!("algo is required"))?,
                    args.user_name
                        .ok_or_else(|| anyhow::anyhow!("user_name is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.account_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn acquiring_algorithm(args: AcquiringAlgorithmArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.acquiring_algorithm().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn acquiring_coinname(args: AcquiringCoinnameArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.acquiring_coinname().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn cancel_hashrate_resale_configuration(
    mut args: CancelHashrateResaleConfigurationArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CancelHashrateResaleConfigurationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<CancelHashrateResaleConfigurationParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.config_id.is_none() {
                        let config_id: i64 = Input::new()
                            .with_prompt("Input config_id:")
                            .interact_text()?;

                        args.config_id = Some(config_id);
                    }
                    if args.user_name.is_none() {
                        let user_name: String = Input::new()
                            .with_prompt("Input user_name:")
                            .interact_text()?;

                        args.user_name = Some(user_name);
                    }
                }
                CancelHashrateResaleConfigurationParams::builder(
                    args.config_id
                        .ok_or_else(|| anyhow::anyhow!("config_id is required"))?,
                    args.user_name
                        .ok_or_else(|| anyhow::anyhow!("user_name is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .cancel_hashrate_resale_configuration(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn earnings_list(mut args: EarningsListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EarningsListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<EarningsListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo.is_none() {
                        let algo: String =
                            Input::new().with_prompt("Input algo:").interact_text()?;

                        args.algo = Some(algo);
                    }
                    if args.user_name.is_none() {
                        let user_name: String = Input::new()
                            .with_prompt("Input user_name:")
                            .interact_text()?;

                        args.user_name = Some(user_name);
                    }
                }
                EarningsListParams::builder(
                    args.algo
                        .ok_or_else(|| anyhow::anyhow!("algo is required"))?,
                    args.user_name
                        .ok_or_else(|| anyhow::anyhow!("user_name is required"))?,
                )
                .coin(args.coin)
                .start_date(args.start_date)
                .end_date(args.end_date)
                .page_index(args.page_index)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.earnings_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn extra_bonus_list(mut args: ExtraBonusListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ExtraBonusListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ExtraBonusListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo.is_none() {
                        let algo: String =
                            Input::new().with_prompt("Input algo:").interact_text()?;

                        args.algo = Some(algo);
                    }
                    if args.user_name.is_none() {
                        let user_name: String = Input::new()
                            .with_prompt("Input user_name:")
                            .interact_text()?;

                        args.user_name = Some(user_name);
                    }
                }
                ExtraBonusListParams::builder(
                    args.algo
                        .ok_or_else(|| anyhow::anyhow!("algo is required"))?,
                    args.user_name
                        .ok_or_else(|| anyhow::anyhow!("user_name is required"))?,
                )
                .coin(args.coin)
                .start_date(args.start_date)
                .end_date(args.end_date)
                .page_index(args.page_index)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.extra_bonus_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn hashrate_resale_detail(mut args: HashrateResaleDetailArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<HashrateResaleDetailParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<HashrateResaleDetailParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.config_id.is_none() {
                        let config_id: i64 = Input::new()
                            .with_prompt("Input config_id:")
                            .interact_text()?;

                        args.config_id = Some(config_id);
                    }
                }
                HashrateResaleDetailParams::builder(
                    args.config_id
                        .ok_or_else(|| anyhow::anyhow!("config_id is required"))?,
                )
                .page_index(args.page_index)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.hashrate_resale_detail(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn hashrate_resale_list(args: HashrateResaleListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<HashrateResaleListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<HashrateResaleListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => HashrateResaleListParams::builder()
                .page_index(args.page_index)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.hashrate_resale_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn hashrate_resale_request(mut args: HashrateResaleRequestArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<HashrateResaleRequestParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<HashrateResaleRequestParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.user_name.is_none() {
                        let user_name: String = Input::new()
                            .with_prompt("Input user_name:")
                            .interact_text()?;

                        args.user_name = Some(user_name);
                    }
                    if args.algo.is_none() {
                        let algo: String =
                            Input::new().with_prompt("Input algo:").interact_text()?;

                        args.algo = Some(algo);
                    }
                    if args.end_date.is_none() {
                        let end_date: i64 = Input::new()
                            .with_prompt("Input end_date:")
                            .interact_text()?;

                        args.end_date = Some(end_date);
                    }
                    if args.start_date.is_none() {
                        let start_date: i64 = Input::new()
                            .with_prompt("Input start_date:")
                            .interact_text()?;

                        args.start_date = Some(start_date);
                    }
                    if args.to_pool_user.is_none() {
                        let to_pool_user: String = Input::new()
                            .with_prompt("Input to_pool_user:")
                            .interact_text()?;

                        args.to_pool_user = Some(to_pool_user);
                    }
                    if args.hash_rate.is_none() {
                        let hash_rate: i64 = Input::new()
                            .with_prompt("Input hash_rate:")
                            .interact_text()?;

                        args.hash_rate = Some(hash_rate);
                    }
                }
                HashrateResaleRequestParams::builder(
                    args.user_name
                        .ok_or_else(|| anyhow::anyhow!("user_name is required"))?,
                    args.algo
                        .ok_or_else(|| anyhow::anyhow!("algo is required"))?,
                    args.end_date
                        .ok_or_else(|| anyhow::anyhow!("end_date is required"))?,
                    args.start_date
                        .ok_or_else(|| anyhow::anyhow!("start_date is required"))?,
                    args.to_pool_user
                        .ok_or_else(|| anyhow::anyhow!("to_pool_user is required"))?,
                    args.hash_rate
                        .ok_or_else(|| anyhow::anyhow!("hash_rate is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.hashrate_resale_request(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn mining_account_earning(mut args: MiningAccountEarningArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<MiningAccountEarningParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<MiningAccountEarningParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo.is_none() {
                        let algo: String =
                            Input::new().with_prompt("Input algo:").interact_text()?;

                        args.algo = Some(algo);
                    }
                }
                MiningAccountEarningParams::builder(
                    args.algo
                        .ok_or_else(|| anyhow::anyhow!("algo is required"))?,
                )
                .start_date(args.start_date)
                .end_date(args.end_date)
                .page_index(args.page_index)
                .page_size(args.page_size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.mining_account_earning(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn request_for_detail_miner_list(
    mut args: RequestForDetailMinerListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RequestForDetailMinerListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<RequestForDetailMinerListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.algo.is_none() {
                        let algo: String =
                            Input::new().with_prompt("Input algo:").interact_text()?;

                        args.algo = Some(algo);
                    }
                    if args.user_name.is_none() {
                        let user_name: String = Input::new()
                            .with_prompt("Input user_name:")
                            .interact_text()?;

                        args.user_name = Some(user_name);
                    }
                    if args.worker_name.is_none() {
                        let worker_name: String = Input::new()
                            .with_prompt("Input worker_name:")
                            .interact_text()?;

                        args.worker_name = Some(worker_name);
                    }
                }
                RequestForDetailMinerListParams::builder(
                    args.algo
                        .ok_or_else(|| anyhow::anyhow!("algo is required"))?,
                    args.user_name
                        .ok_or_else(|| anyhow::anyhow!("user_name is required"))?,
                    args.worker_name
                        .ok_or_else(|| anyhow::anyhow!("worker_name is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.request_for_detail_miner_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn request_for_miner_list(mut args: RequestForMinerListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RequestForMinerListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RequestForMinerListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo.is_none() {
                        let algo: String =
                            Input::new().with_prompt("Input algo:").interact_text()?;

                        args.algo = Some(algo);
                    }
                    if args.user_name.is_none() {
                        let user_name: String = Input::new()
                            .with_prompt("Input user_name:")
                            .interact_text()?;

                        args.user_name = Some(user_name);
                    }
                }
                RequestForMinerListParams::builder(
                    args.algo
                        .ok_or_else(|| anyhow::anyhow!("algo is required"))?,
                    args.user_name
                        .ok_or_else(|| anyhow::anyhow!("user_name is required"))?,
                )
                .page_index(args.page_index)
                .sort(args.sort)
                .sort_column(args.sort_column)
                .worker_status(args.worker_status)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.request_for_miner_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn statistic_list(mut args: StatisticListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<StatisticListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<StatisticListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.algo.is_none() {
                        let algo: String =
                            Input::new().with_prompt("Input algo:").interact_text()?;

                        args.algo = Some(algo);
                    }
                    if args.user_name.is_none() {
                        let user_name: String = Input::new()
                            .with_prompt("Input user_name:")
                            .interact_text()?;

                        args.user_name = Some(user_name);
                    }
                }
                StatisticListParams::builder(
                    args.algo
                        .ok_or_else(|| anyhow::anyhow!("algo is required"))?,
                    args.user_name
                        .ok_or_else(|| anyhow::anyhow!("user_name is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.statistic_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
