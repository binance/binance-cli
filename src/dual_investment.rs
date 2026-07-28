use crate::utils::{
    build_user_agent, decode_selected_entities, get_configuration_rest_api, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::DUAL_INVESTMENT_REST_API_PROD_URL;
use binance_sdk::dual_investment::DualInvestmentRestApi;
use binance_sdk::dual_investment::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    unsafe {
        env::set_var(
            "BINANCE_CONNECTOR_RUST_USER_AGENT",
            build_user_agent("dual-investment"),
        );
    }

    let config_rest_api = get_configuration_rest_api(profile, "dual-investment").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(config_rest_api.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = match api_env.as_str() {
        "prod" => DUAL_INVESTMENT_REST_API_PROD_URL,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid BINANCE_API_ENV",
            ));
        }
    };

    let mut builder = ConfigurationRestApi::builder().base_path(base_path);

    if is_signed {
        builder = builder
            .api_key(config_rest_api.api_key)
            .api_secret(config_rest_api.api_secret);

        if config_rest_api.private_key.is_some()  {
            builder = builder.private_key(PrivateKey::File(config_rest_api.private_key.unwrap()));
        }
    }

    let rest_conf = builder
        .build()
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    Ok(DualInvestmentRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct GetDualInvestmentProductListArgs {
    #[arg(help = r#"Input CALL or PUT"#, long)]
    option_type: Option<GetDualInvestmentProductListOptionTypeEnum>,
    #[arg(
        help = r#"Target exercised asset, e.g.: if you subscribe to a high sell product (call option), you should input:
`optionType: CALL`, `exercisedCoin: USDT`, `investCoin: BNB`; if you subscribe to a low buy product (put
option), you should input: `optionType: PUT`, `exercisedCoin: BNB`, `investCoin: USDT`"#,
        long
    )]
    exercised_coin: Option<String>,
    #[arg(
        help = r#"Asset used for subscribing, e.g.: if you subscribe to a high sell product (call option), you should input:
`optionType: CALL`, `exercisedCoin: USDT`, `investCoin: BNB`; if you subscribe to a low buy product (put
option), you should input: `optionType: PUT`, `exercisedCoin: BNB`, `investCoin: USDT`"#,
        long
    )]
    invest_coin: Option<String>,
    #[arg(help = r#"Number of records per page"#, long)]
    page_size: Option<i64>,
    #[arg(help = r#"Page index"#, long)]
    page_index: Option<i64>,
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
struct ChangeAutoCompoundStatusArgs {
    #[arg(help = r#"Get positionId from `/sapi/v1/dci/product/positions`"#, long)]
    position_id: Option<String>,
    #[arg(
        help = r#"`NONE`: switch off the plan, `STANDARD`: standard plan, `ADVANCED`: advanced plan"#,
        long
    )]
    auto_compound_plan: Option<ChangeAutoCompoundStatusAutoCompoundPlanEnum>,
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
struct CheckDualInvestmentAccountsArgs {
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
struct GetDualInvestmentPositionsArgs {
    #[arg(
        help = r#"`PENDING`: Products are purchasing, will give results later; `PURCHASE_SUCCESS`: purchase successfully;
`SETTLED`: Products are finish settling; `PURCHASE_FAIL`: fail to purchase; `REFUNDING`: refund ongoing;
`REFUND_SUCCESS`: refund to spot account successfully; `SETTLING`: Products are settling. If don't fill this
field, will response all the position status."#,
        long
    )]
    status: Option<GetDualInvestmentPositionsStatusEnum>,
    #[arg(help = r#"Number of records per page"#, long)]
    page_size: Option<i64>,
    #[arg(help = r#"Page index"#, long)]
    page_index: Option<i64>,
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
struct SubscribeDualInvestmentProductsArgs {
    #[arg(help = r#"get id from `/sapi/v1/dci/product/list`"#, long)]
    id: Option<String>,
    #[arg(help = r#"get orderId from `/sapi/v1/dci/product/list`"#, long)]
    order_id: Option<String>,
    #[arg(help = r#"the amount for subscribing"#, long)]
    deposit_amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"`NONE`: switch off the plan, `STANDARD`: standard plan, `ADVANCED`: advanced plan"#,
        long
    )]
    auto_compound_plan: Option<SubscribeDualInvestmentProductsAutoCompoundPlanEnum>,
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
pub enum DualInvestmentCommands {
    #[command(
        about = decode_selected_entities(r#"Get Dual Investment product list

Weight(IP): 1"#, false),
    )]
    GetDualInvestmentProductList(GetDualInvestmentProductListArgs),
    #[command(
        about = decode_selected_entities(r#"Change Auto-Compound status

Weight(IP): 1

Security Type: USER_DATA

Notes:
- 15:31 ~ 16:00 UTC+8: This function is disabled."#, false),
    )]
    ChangeAutoCompoundStatus(ChangeAutoCompoundStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Check Dual Investment accounts

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    CheckDualInvestmentAccounts(CheckDualInvestmentAccountsArgs),
    #[command(
        about = decode_selected_entities(r#"Get Dual Investment positions (batch)

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetDualInvestmentPositions(GetDualInvestmentPositionsArgs),
    #[command(
        about = decode_selected_entities(r#"Subscribe Dual Investment products

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Failed messages:
  - Products are not available. This means APR changed to a lower value, or the order is unavailable.
  - Failed. This means system or network errors."#, false),
    )]
    SubscribeDualInvestmentProducts(SubscribeDualInvestmentProductsArgs),
}

pub async fn handle_dual_investment_command(command: DualInvestmentCommands) -> anyhow::Result<()> {
    match command {
        DualInvestmentCommands::GetDualInvestmentProductList(args) => {
            get_dual_investment_product_list(args).await
        }

        DualInvestmentCommands::ChangeAutoCompoundStatus(args) => {
            change_auto_compound_status(args).await
        }

        DualInvestmentCommands::CheckDualInvestmentAccounts(args) => {
            check_dual_investment_accounts(args).await
        }

        DualInvestmentCommands::GetDualInvestmentPositions(args) => {
            get_dual_investment_positions(args).await
        }

        DualInvestmentCommands::SubscribeDualInvestmentProducts(args) => {
            subscribe_dual_investment_products(args).await
        }
    }
}

async fn get_dual_investment_product_list(
    mut args: GetDualInvestmentProductListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetDualInvestmentProductListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetDualInvestmentProductListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.option_type.is_none() {
                        let options = vec![
                            ("CALL", GetDualInvestmentProductListOptionTypeEnum::Call),
                            ("PUT", GetDualInvestmentProductListOptionTypeEnum::Put),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the option_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.option_type = Some(selected);
                    }
                    if args.exercised_coin.is_none() {
                        let exercised_coin: String = Input::new()
                            .with_prompt("Please enter the exercised_coin name")
                            .interact_text()?;

                        args.exercised_coin = Some(exercised_coin);
                    }
                    if args.invest_coin.is_none() {
                        let invest_coin: String = Input::new()
                            .with_prompt("Please enter the invest_coin name")
                            .interact_text()?;

                        args.invest_coin = Some(invest_coin);
                    }
                }
                GetDualInvestmentProductListParams::builder(
                    args.option_type
                        .ok_or_else(|| anyhow::anyhow!("option_type is required"))?,
                    args.exercised_coin
                        .ok_or_else(|| anyhow::anyhow!("exercised_coin is required"))?,
                    args.invest_coin
                        .ok_or_else(|| anyhow::anyhow!("invest_coin is required"))?,
                )
                .page_size(args.page_size)
                .page_index(args.page_index)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_dual_investment_product_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_auto_compound_status(mut args: ChangeAutoCompoundStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeAutoCompoundStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ChangeAutoCompoundStatusParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.position_id.is_none() {
                        let position_id: String = Input::new()
                            .with_prompt("Please enter the position_id name")
                            .interact_text()?;

                        args.position_id = Some(position_id);
                    }
                    if args.auto_compound_plan.is_none() {
                        let options = vec![
                            ("NONE", ChangeAutoCompoundStatusAutoCompoundPlanEnum::None),
                            (
                                "STANDARD",
                                ChangeAutoCompoundStatusAutoCompoundPlanEnum::Standard,
                            ),
                            (
                                "ADVANCED",
                                ChangeAutoCompoundStatusAutoCompoundPlanEnum::Advanced,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the auto_compound_plan")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.auto_compound_plan = Some(selected);
                    }
                }
                ChangeAutoCompoundStatusParams::builder(
                    args.position_id
                        .ok_or_else(|| anyhow::anyhow!("position_id is required"))?,
                    args.auto_compound_plan
                        .ok_or_else(|| anyhow::anyhow!("auto_compound_plan is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_auto_compound_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn check_dual_investment_accounts(
    args: CheckDualInvestmentAccountsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CheckDualInvestmentAccountsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CheckDualInvestmentAccountsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => CheckDualInvestmentAccountsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.check_dual_investment_accounts(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_dual_investment_positions(
    args: GetDualInvestmentPositionsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDualInvestmentPositionsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetDualInvestmentPositionsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetDualInvestmentPositionsParams::builder()
                .status(args.status)
                .page_size(args.page_size)
                .page_index(args.page_index)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_dual_investment_positions(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn subscribe_dual_investment_products(
    mut args: SubscribeDualInvestmentProductsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params =
        match read_stdin_as::<SubscribeDualInvestmentProductsParams>() {
            Some(params) => params,
            None => match args.json {
                Some(json) => read_json_as::<SubscribeDualInvestmentProductsParams>(json)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                    })?,
                None => {
                    if args.interactive {
                        if args.id.is_none() {
                            let id: String = Input::new()
                                .with_prompt("Please enter the id name")
                                .interact_text()?;

                            args.id = Some(id);
                        }
                        if args.order_id.is_none() {
                            let order_id: String = Input::new()
                                .with_prompt("Please enter the order_id name")
                                .interact_text()?;

                            args.order_id = Some(order_id);
                        }
                        if args.deposit_amount.is_none() {
                            let deposit_amount: rust_decimal::Decimal = Input::new()
                                .with_prompt("Please enter the deposit_amount name")
                                .interact_text()?;

                            args.deposit_amount = Some(deposit_amount);
                        }
                        if args.auto_compound_plan.is_none() {
                            let options =
                                vec![
                        ("NONE", SubscribeDualInvestmentProductsAutoCompoundPlanEnum::None),
                        ("STANDARD", SubscribeDualInvestmentProductsAutoCompoundPlanEnum::Standard),
                        ("ADVANCED", SubscribeDualInvestmentProductsAutoCompoundPlanEnum::Advanced),
                    ];

                            let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                            let selected = Select::new()
                                .with_prompt("Please select the auto_compound_plan")
                                .items(&labels)
                                .default(0)
                                .interact()?;

                            let selected = options[selected].1.clone();

                            println!("Selected option: {:?}", selected);

                            args.auto_compound_plan = Some(selected);
                        }
                    }
                    SubscribeDualInvestmentProductsParams::builder(
                        args.id.ok_or_else(|| anyhow::anyhow!("id is required"))?,
                        args.order_id
                            .ok_or_else(|| anyhow::anyhow!("order_id is required"))?,
                        args.deposit_amount
                            .ok_or_else(|| anyhow::anyhow!("deposit_amount is required"))?,
                        args.auto_compound_plan
                            .ok_or_else(|| anyhow::anyhow!("auto_compound_plan is required"))?,
                    )
                    .recv_window(args.recv_window)
                    .build()?
                }
            },
        };

    // Make the API call
    let response = rest_client
        .subscribe_dual_investment_products(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
