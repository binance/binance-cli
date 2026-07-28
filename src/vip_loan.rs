use crate::utils::{
    build_user_agent, decode_selected_entities, get_configuration_rest_api, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::VIP_LOAN_REST_API_PROD_URL;
use binance_sdk::vip_loan::VIPLoanRestApi;
use binance_sdk::vip_loan::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::Input;
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    unsafe {
        env::set_var(
            "BINANCE_CONNECTOR_RUST_USER_AGENT",
            build_user_agent("vip-loan"),
        );
    }

    let config_rest_api = get_configuration_rest_api(profile, "vip-loan").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(config_rest_api.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = match api_env.as_str() {
        "prod" => VIP_LOAN_REST_API_PROD_URL,
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

    Ok(VIPLoanRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct GetBorrowInterestRateArgs {
    #[arg(help = r#"Max 10 assets, Multiple split by ",""#, long)]
    loan_coin: Option<String>,
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
struct GetCollateralAssetDataArgs {
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
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
struct GetLoanableAssetsDataArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#"Defaults to the user's VIP level."#, long)]
    vip_level: Option<i64>,
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
struct GetVipLoanInterestRateHistoryArgs {
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(
        help = r#"If both startTime and endTime are omitted, the most recent 90 days are returned."#,
        long
    )]
    start_time: Option<i64>,
    #[arg(
        help = r#"Maximum interval between startTime and endTime is 180 days. Time is based on UTC+0."#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#"Current page number, starting from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of records per page."#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryVipLoanFixedRateMarketArgs {
    #[arg(help = r#"Loan coin"#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#"Duration in days, minimum 1"#, long)]
    duration: Option<i64>,
    #[arg(help = r#"Page number, default 1, minimum 1"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Page size, default 10, range [1, 100]"#, long)]
    size: Option<i64>,
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
struct VipLoanBorrowArgs {
    #[arg(help = r#""#, long)]
    loan_account_id: Option<i64>,
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    loan_amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Collateral account ID(s). Multiple split by `,`"#, long)]
    collateral_account_id: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#"TRUE: flexible rate; FALSE: fixed rate"#, long, num_args = 0..=1, default_missing_value = "true")]
    is_flexible_rate: Option<bool>,
    #[arg(
        help = r#"Mandatory for fixed rate. Optional for flexible rate. e.g. 30/60 days"#,
        long
    )]
    loan_term: Option<i64>,
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
struct VipLoanFixedRateBorrowArgs {
    #[arg(
        help = r#"Supply request string, positional encoding (no key). Multiple entries separated by `;`, fields separated by `:`, order: `<requestId>:<interestRate>:<amount>`. Example: `1212:0.12:100;3434:0.13:50`"#,
        long
    )]
    supply_request: Option<String>,
    #[arg(help = r#"Borrow coin"#, long)]
    borrow_coin: Option<String>,
    #[arg(help = r#"Loan term in days"#, long)]
    loan_term: Option<i64>,
    #[arg(help = r#"Borrow receiving account UID"#, long)]
    borrow_uid: Option<i64>,
    #[arg(
        help = r#"Collateral coin(s), multiple separated by `,`. Only coin names, no amount (VIP loan collateral amount = entire spot account balance)"#,
        long
    )]
    collateral_coin: Option<String>,
    #[arg(help = r#"Collateral account ID(s), multiple separated by `,`"#, long)]
    collateral_account_id: Option<String>,
    #[arg(help = r#"Default: `true`. `true`: auto repay at expiration; `false`: auto-convert to flexible (floating rate) at expiration"#, long, num_args = 0..=1, default_missing_value = "true")]
    auto_repay: Option<bool>,
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
struct VipLoanRenewArgs {
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#"30/60 days"#, long)]
    loan_term: Option<i64>,
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
struct VipLoanRepayArgs {
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
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
struct CheckVipLoanCollateralAccountArgs {
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    collateral_account_id: Option<i64>,
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
struct GetVipLoanAccruedInterestArgs {
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(
        help = r#"If both startTime and endTime are omitted, the most recent 90 days are returned."#,
        long
    )]
    start_time: Option<i64>,
    #[arg(
        help = r#"Maximum interval between startTime and endTime is 90 days."#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#"Current page number, starting from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of records per page."#, long)]
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
struct GetVipLoanOngoingOrdersArgs {
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    collateral_account_id: Option<i64>,
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
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
struct GetVipLoanRepaymentHistoryArgs {
    #[arg(help = r#""#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(
        help = r#"If both startTime and endTime are omitted, the most recent 90 days are returned."#,
        long
    )]
    start_time: Option<i64>,
    #[arg(
        help = r#"Maximum interval between startTime and endTime is 180 days."#,
        long
    )]
    end_time: Option<i64>,
    #[arg(help = r#"Current page number, starting from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of records per page."#, long)]
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
struct QueryApplicationStatusArgs {
    #[arg(help = r#"Current page number, starting from 1."#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
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

#[derive(Subcommand)]
pub enum VIPLoanCommands {
    #[command(
        about = decode_selected_entities(r#"Get Borrow Interest Rate

Weight(IP): 400

Security Type: USER_DATA"#, false),
    )]
    GetBorrowInterestRate(GetBorrowInterestRateArgs),
    #[command(
        about = decode_selected_entities(r#"Get Collateral Asset Data

Weight(IP): 400

Security Type: USER_DATA"#, false),
    )]
    GetCollateralAssetData(GetCollateralAssetDataArgs),
    #[command(
        about = decode_selected_entities(r#"Get interest rate and borrow limit of loanable assets. The borrow limit is shown in USD value.

Weight(IP): 400

Security Type: USER_DATA"#, false),
    )]
    GetLoanableAssetsData(GetLoanableAssetsDataArgs),
    #[command(
        about = decode_selected_entities(r#"Check VIP Loan flexible interest rate history

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, recent 90-day data is returned.
- The maximum interval between `startTime` and `endTime` is 180 days.
- Time is based on UTC+0."#, false),
    )]
    GetVipLoanInterestRateHistory(GetVipLoanInterestRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query the VIP Loan fixed rate market. Returns a paginated list of fixed-rate supply orders.

Weight(IP): 6000

Security Type: USER_DATA"#, false),
    )]
    QueryVipLoanFixedRateMarket(QueryVipLoanFixedRateMarketArgs),
    #[command(
        about = decode_selected_entities(r#"VIP loan is available for VIP users only.

Weight(UID): 6000

Security Type: TRADE

Notes:
- `loanAccountId` refers to the loan receiving account.
- Only master account applications are supported.
- `loanAccountId` and `collateralAccountId` must be under the same master account.
- `loanTerm` is mandatory if the user chooses a fixed rate (`isFlexibleRate = FALSE`)."#, false),
    )]
    VipLoanBorrow(VipLoanBorrowArgs),
    #[command(
        about = decode_selected_entities(r#"Submit a fixed rate borrow request by matching market supply orders.

Weight(UID): 6000

Security Type: TRADE

Notes:
- **Rate limit:** 2 requests per second per account.
- When multiple `supplyRequest` entries are provided, all `requestId` values must correspond to the same `borrowCoin` and `loanTerm` (validated by collateral facade)."#, false),
    )]
    VipLoanFixedRateBorrow(VipLoanFixedRateBorrowArgs),
    #[command(
        about = decode_selected_entities(r#"VIP loan is available for VIP users only.

Weight(UID): 6000

Security Type: TRADE"#, false),
    )]
    VipLoanRenew(VipLoanRenewArgs),
    #[command(
        about = decode_selected_entities(r#"VIP loan is available for VIP users only.

Weight(UID): 6000

Security Type: TRADE"#, false),
    )]
    VipLoanRepay(VipLoanRepayArgs),
    #[command(
        about = decode_selected_entities(r#"VIP loan is available for VIP users only

Weight(IP): 6000

Security Type: USER_DATA

Notes:
- If the logged-in account is a borrowing account, all collateral accounts bound to that borrowing account can be queried.
- If the logged-in account is a collateral account, only collateral assets under that account can be queried."#, false),
    )]
    CheckVipLoanCollateralAccount(CheckVipLoanCollateralAccountArgs),
    #[command(
        about = decode_selected_entities(r#"Check VIP Loan interest record

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, recent 90-day data is returned.
- The maximum interval between `startTime` and `endTime` is 90 days."#, false),
    )]
    GetVipLoanAccruedInterest(GetVipLoanAccruedInterestArgs),
    #[command(
        about = decode_selected_entities(r#"VIP loan is available for VIP users only.

Weight(IP): 400

Security Type: USER_DATA"#, false),
    )]
    GetVipLoanOngoingOrders(GetVipLoanOngoingOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"VIP Loans are available only to VIP users.

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, recent 90-day data is returned.
- The maximum interval between `startTime` and `endTime` is 180 days."#, false),
    )]
    GetVipLoanRepaymentHistory(GetVipLoanRepaymentHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query Application Status

Weight(UID): 400

Security Type: USER_DATA"#, false),
    )]
    QueryApplicationStatus(QueryApplicationStatusArgs),
}

pub async fn handle_vip_loan_command(command: VIPLoanCommands) -> anyhow::Result<()> {
    match command {
        VIPLoanCommands::GetBorrowInterestRate(args) => get_borrow_interest_rate(args).await,

        VIPLoanCommands::GetCollateralAssetData(args) => get_collateral_asset_data(args).await,

        VIPLoanCommands::GetLoanableAssetsData(args) => get_loanable_assets_data(args).await,

        VIPLoanCommands::GetVipLoanInterestRateHistory(args) => {
            get_vip_loan_interest_rate_history(args).await
        }

        VIPLoanCommands::QueryVipLoanFixedRateMarket(args) => {
            query_vip_loan_fixed_rate_market(args).await
        }

        VIPLoanCommands::VipLoanBorrow(args) => vip_loan_borrow(args).await,

        VIPLoanCommands::VipLoanFixedRateBorrow(args) => vip_loan_fixed_rate_borrow(args).await,

        VIPLoanCommands::VipLoanRenew(args) => vip_loan_renew(args).await,

        VIPLoanCommands::VipLoanRepay(args) => vip_loan_repay(args).await,

        VIPLoanCommands::CheckVipLoanCollateralAccount(args) => {
            check_vip_loan_collateral_account(args).await
        }

        VIPLoanCommands::GetVipLoanAccruedInterest(args) => {
            get_vip_loan_accrued_interest(args).await
        }

        VIPLoanCommands::GetVipLoanOngoingOrders(args) => get_vip_loan_ongoing_orders(args).await,

        VIPLoanCommands::GetVipLoanRepaymentHistory(args) => {
            get_vip_loan_repayment_history(args).await
        }

        VIPLoanCommands::QueryApplicationStatus(args) => query_application_status(args).await,
    }
}

async fn get_borrow_interest_rate(mut args: GetBorrowInterestRateArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetBorrowInterestRateParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetBorrowInterestRateParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.loan_coin.is_none() {
                        let loan_coin: String = Input::new()
                            .with_prompt("Please enter the loan_coin name")
                            .interact_text()?;

                        args.loan_coin = Some(loan_coin);
                    }
                }
                GetBorrowInterestRateParams::builder(
                    args.loan_coin
                        .ok_or_else(|| anyhow::anyhow!("loan_coin is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_borrow_interest_rate(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_collateral_asset_data(args: GetCollateralAssetDataArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCollateralAssetDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetCollateralAssetDataParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetCollateralAssetDataParams::builder()
                .collateral_coin(args.collateral_coin)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_collateral_asset_data(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_loanable_assets_data(args: GetLoanableAssetsDataArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLoanableAssetsDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetLoanableAssetsDataParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetLoanableAssetsDataParams::builder()
                .loan_coin(args.loan_coin)
                .vip_level(args.vip_level)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_loanable_assets_data(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_vip_loan_interest_rate_history(
    mut args: GetVipLoanInterestRateHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetVipLoanInterestRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetVipLoanInterestRateHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.coin.is_none() {
                        let coin: String = Input::new()
                            .with_prompt("Please enter the coin name")
                            .interact_text()?;

                        args.coin = Some(coin);
                    }
                    if args.recv_window.is_none() {
                        let recv_window: i64 = Input::new()
                            .with_prompt("Please enter the recv_window name")
                            .interact_text()?;

                        args.recv_window = Some(recv_window);
                    }
                }
                GetVipLoanInterestRateHistoryParams::builder(
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                    args.recv_window
                        .ok_or_else(|| anyhow::anyhow!("recv_window is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_vip_loan_interest_rate_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_vip_loan_fixed_rate_market(
    mut args: QueryVipLoanFixedRateMarketArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryVipLoanFixedRateMarketParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryVipLoanFixedRateMarketParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.loan_coin.is_none() {
                        let loan_coin: String = Input::new()
                            .with_prompt("Please enter the loan_coin name")
                            .interact_text()?;

                        args.loan_coin = Some(loan_coin);
                    }
                }
                QueryVipLoanFixedRateMarketParams::builder(
                    args.loan_coin
                        .ok_or_else(|| anyhow::anyhow!("loan_coin is required"))?,
                )
                .duration(args.duration)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_vip_loan_fixed_rate_market(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn vip_loan_borrow(mut args: VipLoanBorrowArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<VipLoanBorrowParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<VipLoanBorrowParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.loan_account_id.is_none() {
                        let loan_account_id: i64 = Input::new()
                            .with_prompt("Please enter the loan_account_id name")
                            .interact_text()?;

                        args.loan_account_id = Some(loan_account_id);
                    }
                    if args.loan_coin.is_none() {
                        let loan_coin: String = Input::new()
                            .with_prompt("Please enter the loan_coin name")
                            .interact_text()?;

                        args.loan_coin = Some(loan_coin);
                    }
                    if args.loan_amount.is_none() {
                        let loan_amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the loan_amount name")
                            .interact_text()?;

                        args.loan_amount = Some(loan_amount);
                    }
                    if args.collateral_account_id.is_none() {
                        let collateral_account_id: String = Input::new()
                            .with_prompt("Please enter the collateral_account_id name")
                            .interact_text()?;

                        args.collateral_account_id = Some(collateral_account_id);
                    }
                    if args.collateral_coin.is_none() {
                        let collateral_coin: String = Input::new()
                            .with_prompt("Please enter the collateral_coin name")
                            .interact_text()?;

                        args.collateral_coin = Some(collateral_coin);
                    }
                    if args.is_flexible_rate.is_none() {
                        let is_flexible_rate: bool = Input::new()
                            .with_prompt("Please enter the is_flexible_rate name")
                            .interact_text()?;

                        args.is_flexible_rate = Some(is_flexible_rate);
                    }
                }
                VipLoanBorrowParams::builder(
                    args.loan_account_id
                        .ok_or_else(|| anyhow::anyhow!("loan_account_id is required"))?,
                    args.loan_coin
                        .ok_or_else(|| anyhow::anyhow!("loan_coin is required"))?,
                    args.loan_amount
                        .ok_or_else(|| anyhow::anyhow!("loan_amount is required"))?,
                    args.collateral_account_id
                        .ok_or_else(|| anyhow::anyhow!("collateral_account_id is required"))?,
                    args.collateral_coin
                        .ok_or_else(|| anyhow::anyhow!("collateral_coin is required"))?,
                    args.is_flexible_rate
                        .ok_or_else(|| anyhow::anyhow!("is_flexible_rate is required"))?,
                )
                .loan_term(args.loan_term)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.vip_loan_borrow(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn vip_loan_fixed_rate_borrow(mut args: VipLoanFixedRateBorrowArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<VipLoanFixedRateBorrowParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<VipLoanFixedRateBorrowParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.supply_request.is_none() {
                        let supply_request: String = Input::new()
                            .with_prompt("Please enter the supply_request name")
                            .interact_text()?;

                        args.supply_request = Some(supply_request);
                    }
                    if args.borrow_coin.is_none() {
                        let borrow_coin: String = Input::new()
                            .with_prompt("Please enter the borrow_coin name")
                            .interact_text()?;

                        args.borrow_coin = Some(borrow_coin);
                    }
                    if args.loan_term.is_none() {
                        let loan_term: i64 = Input::new()
                            .with_prompt("Please enter the loan_term name")
                            .interact_text()?;

                        args.loan_term = Some(loan_term);
                    }
                    if args.borrow_uid.is_none() {
                        let borrow_uid: i64 = Input::new()
                            .with_prompt("Please enter the borrow_uid name")
                            .interact_text()?;

                        args.borrow_uid = Some(borrow_uid);
                    }
                    if args.collateral_coin.is_none() {
                        let collateral_coin: String = Input::new()
                            .with_prompt("Please enter the collateral_coin name")
                            .interact_text()?;

                        args.collateral_coin = Some(collateral_coin);
                    }
                    if args.collateral_account_id.is_none() {
                        let collateral_account_id: String = Input::new()
                            .with_prompt("Please enter the collateral_account_id name")
                            .interact_text()?;

                        args.collateral_account_id = Some(collateral_account_id);
                    }
                }
                VipLoanFixedRateBorrowParams::builder(
                    args.supply_request
                        .ok_or_else(|| anyhow::anyhow!("supply_request is required"))?,
                    args.borrow_coin
                        .ok_or_else(|| anyhow::anyhow!("borrow_coin is required"))?,
                    args.loan_term
                        .ok_or_else(|| anyhow::anyhow!("loan_term is required"))?,
                    args.borrow_uid
                        .ok_or_else(|| anyhow::anyhow!("borrow_uid is required"))?,
                    args.collateral_coin
                        .ok_or_else(|| anyhow::anyhow!("collateral_coin is required"))?,
                    args.collateral_account_id
                        .ok_or_else(|| anyhow::anyhow!("collateral_account_id is required"))?,
                )
                .auto_repay(args.auto_repay)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.vip_loan_fixed_rate_borrow(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn vip_loan_renew(mut args: VipLoanRenewArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<VipLoanRenewParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<VipLoanRenewParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.order_id.is_none() {
                        let order_id: i64 = Input::new()
                            .with_prompt("Please enter the order_id name")
                            .interact_text()?;

                        args.order_id = Some(order_id);
                    }
                    if args.loan_term.is_none() {
                        let loan_term: i64 = Input::new()
                            .with_prompt("Please enter the loan_term name")
                            .interact_text()?;

                        args.loan_term = Some(loan_term);
                    }
                }
                VipLoanRenewParams::builder(
                    args.order_id
                        .ok_or_else(|| anyhow::anyhow!("order_id is required"))?,
                    args.loan_term
                        .ok_or_else(|| anyhow::anyhow!("loan_term is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.vip_loan_renew(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn vip_loan_repay(mut args: VipLoanRepayArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<VipLoanRepayParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<VipLoanRepayParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.order_id.is_none() {
                        let order_id: i64 = Input::new()
                            .with_prompt("Please enter the order_id name")
                            .interact_text()?;

                        args.order_id = Some(order_id);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                VipLoanRepayParams::builder(
                    args.order_id
                        .ok_or_else(|| anyhow::anyhow!("order_id is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.vip_loan_repay(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn check_vip_loan_collateral_account(
    args: CheckVipLoanCollateralAccountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CheckVipLoanCollateralAccountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CheckVipLoanCollateralAccountParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => CheckVipLoanCollateralAccountParams::builder()
                .order_id(args.order_id)
                .collateral_account_id(args.collateral_account_id)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .check_vip_loan_collateral_account(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_vip_loan_accrued_interest(
    args: GetVipLoanAccruedInterestArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetVipLoanAccruedInterestParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetVipLoanAccruedInterestParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetVipLoanAccruedInterestParams::builder()
                .order_id(args.order_id)
                .loan_coin(args.loan_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_vip_loan_accrued_interest(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_vip_loan_ongoing_orders(args: GetVipLoanOngoingOrdersArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetVipLoanOngoingOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetVipLoanOngoingOrdersParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetVipLoanOngoingOrdersParams::builder()
                .order_id(args.order_id)
                .collateral_account_id(args.collateral_account_id)
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_vip_loan_ongoing_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_vip_loan_repayment_history(
    args: GetVipLoanRepaymentHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetVipLoanRepaymentHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetVipLoanRepaymentHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetVipLoanRepaymentHistoryParams::builder()
                .order_id(args.order_id)
                .loan_coin(args.loan_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_vip_loan_repayment_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_application_status(args: QueryApplicationStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryApplicationStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryApplicationStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryApplicationStatusParams::builder()
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_application_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
