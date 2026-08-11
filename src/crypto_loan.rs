use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::CRYPTO_LOAN_REST_API_PROD_URL;
use binance_sdk::crypto_loan::CryptoLoanRestApi;
use binance_sdk::crypto_loan::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("crypto-loan");

    let client_config = get_client_configuration(profile, "crypto-loan").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => CRYPTO_LOAN_REST_API_PROD_URL.to_string(),
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

    Ok(CryptoLoanRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct CheckCollateralRepayRateArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
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
struct FlexibleLoanAdjustLtvArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    adjustment_amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    direction: Option<FlexibleLoanAdjustLtvDirectionEnum>,
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
struct FlexibleLoanBorrowArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#"Mandatory when collateralAmount is empty"#, long)]
    loan_amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"Mandatory when loanAmount is empty"#, long)]
    collateral_amount: Option<rust_decimal::Decimal>,
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
struct FlexibleLoanRepayArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    repay_amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"TRUE: Return extra collateral to spot account; FALSE: Keep extra collateral in the order and lower
LTV."#, long, num_args = 0..=1, default_missing_value = "true")]
    collateral_return: Option<bool>,
    #[arg(help = r#"TRUE: Full repayment; FALSE: Partial repayment based on loan amount"#, long, num_args = 0..=1, default_missing_value = "true")]
    full_repayment: Option<bool>,
    #[arg(
        help = r#"1: Repayment with loan asset; 2: Repayment with collateral"#,
        long
    )]
    repayment_type: Option<FlexibleLoanRepayRepaymentTypeEnum>,
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
struct GetFlexibleLoanAssetsDataArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
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
struct GetFlexibleLoanBorrowHistoryArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
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
struct GetFlexibleLoanCollateralAssetsDataArgs {
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
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
struct GetFlexibleLoanInterestRateHistoryArgs {
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#"Request validity window in milliseconds"#, long)]
    recv_window: Option<i64>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
    #[arg(help = r#"Number of records to return"#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetFlexibleLoanLiquidationHistoryArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
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
struct GetFlexibleLoanLtvAdjustmentHistoryArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
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
struct GetFlexibleLoanOngoingOrdersArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
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
struct GetFlexibleLoanRepaymentHistoryArgs {
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
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
struct GetCryptoLoansIncomeHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"All types will be returned by default."#, long)]
    r#type: Option<GetCryptoLoansIncomeHistoryTypeEnum>,
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
struct GetLoanBorrowHistoryArgs {
    #[arg(help = r#"orderId in `POST /sapi/v1/loan/borrow`"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
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
struct GetLoanLtvAdjustmentHistoryArgs {
    #[arg(help = r#"orderId in `POST /sapi/v1/loan/borrow`"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
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
struct GetLoanRepaymentHistoryArgs {
    #[arg(help = r#"orderId in `POST /sapi/v1/loan/borrow`"#, long)]
    order_id: Option<i64>,
    #[arg(help = r#""#, long)]
    loan_coin: Option<String>,
    #[arg(help = r#""#, long)]
    collateral_coin: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Current querying page"#, long)]
    current: Option<i64>,
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

#[derive(Subcommand)]
pub enum CryptoLoanCommands {
    #[command(
        about = decode_selected_entities(r#"Get the latest rate of collateral coin/loan coin when using collateral repay.

Weight(IP): 6000

Security Type: USER_DATA"#, false),
    )]
    CheckCollateralRepayRate(CheckCollateralRepayRateArgs),
    #[command(
        about = decode_selected_entities(r#"Flexible Loan Adjust LTV

Weight(UID): 6000

Security Type: TRADE

Notes:
- API key needs Spot & Margin Trading permission for this endpoint."#, false),
    )]
    FlexibleLoanAdjustLtv(FlexibleLoanAdjustLtvArgs),
    #[command(
        about = decode_selected_entities(r#"Borrow Flexible Loan

Weight(IP): 6000

Security Type: TRADE

Notes:
- This endpoint is available for both master and sub-accounts.
- You can customize LTV by entering `loanAmount` and `collateralAmount`."#, false),
    )]
    FlexibleLoanBorrow(FlexibleLoanBorrowArgs),
    #[command(
        about = decode_selected_entities(r#"Flexible Loan Repay

Weight(IP): 6000

Security Type: TRADE

Notes:
- `repayAmount` is mandatory even when `fullRepayment = FALSE`."#, false),
    )]
    FlexibleLoanRepay(FlexibleLoanRepayArgs),
    #[command(
        about = decode_selected_entities(r#"Get interest rate and borrow limit of flexible loanable assets. The borrow limit is shown in USD value.

Weight(IP): 400

Security Type: USER_DATA"#, false),
    )]
    GetFlexibleLoanAssetsData(GetFlexibleLoanAssetsDataArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Loan Borrow History. It can be used to check history before 2024-02-27 08:00.

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, the recent 90-day data is returned.
- The max interval between `startTime` and `endTime` is 180 days."#, false),
    )]
    GetFlexibleLoanBorrowHistory(GetFlexibleLoanBorrowHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get LTV information and collateral limit of flexible loan's collateral assets. The collateral limit is shown in
USD value.

Weight(IP): 400

Security Type: USER_DATA"#, false),
    )]
    GetFlexibleLoanCollateralAssetsData(GetFlexibleLoanCollateralAssetsDataArgs),
    #[command(
        about = decode_selected_entities(r#"Check Flexible Loan interest rate history

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, the recent 90-day data is returned.
- The max interval between `startTime` and `endTime` is 90 days.
- Time is based on UTC+0."#, false),
    )]
    GetFlexibleLoanInterestRateHistory(GetFlexibleLoanInterestRateHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Loan Liquidation History

Weight(IP): 400

Security Type: USER_DATA"#, false),
    )]
    GetFlexibleLoanLiquidationHistory(GetFlexibleLoanLiquidationHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Loan LTV Adjustment History. It can be used to check history before 2024-02-27 08:00.

Weight(UID): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, the recent 90-day data is returned.
- The max interval between `startTime` and `endTime` is 180 days."#, false),
    )]
    GetFlexibleLoanLtvAdjustmentHistory(GetFlexibleLoanLtvAdjustmentHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Loan Ongoing Orders

Weight(IP): 300

Security Type: USER_DATA"#, false),
    )]
    GetFlexibleLoanOngoingOrders(GetFlexibleLoanOngoingOrdersArgs),
    #[command(
        about = decode_selected_entities(r#"Get Flexible Loan Repayment History. It can be used to check history before 2024-02-27 08:00.

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, the recent 90-day data is returned.
- The max interval between `startTime` and `endTime` is 180 days."#, false),
    )]
    GetFlexibleLoanRepaymentHistory(GetFlexibleLoanRepaymentHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Crypto Loans Income History

Weight(UID): 6000

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are both omitted, the most recent 7 days of data are returned.
- The maximum interval between `startTime` and `endTime` is 30 days."#, false),
    )]
    GetCryptoLoansIncomeHistory(GetCryptoLoansIncomeHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Loan Borrow History

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, the recent 90-day data is returned.
- The max interval between `startTime` and `endTime` is 180 days."#, false),
    )]
    GetLoanBorrowHistory(GetLoanBorrowHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Loan LTV Adjustment History

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, the recent 90-day data is returned.
- The max interval between `startTime` and `endTime` is 180 days."#, false),
    )]
    GetLoanLtvAdjustmentHistory(GetLoanLtvAdjustmentHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Loan Repayment History

Weight(IP): 400

Security Type: USER_DATA

Notes:
- If `startTime` and `endTime` are not sent, the recent 90-day data is returned.
- The max interval between `startTime` and `endTime` is 180 days."#, false),
    )]
    GetLoanRepaymentHistory(GetLoanRepaymentHistoryArgs),
}

pub async fn handle_crypto_loan_command(command: CryptoLoanCommands) -> anyhow::Result<()> {
    match command {
        CryptoLoanCommands::CheckCollateralRepayRate(args) => {
            check_collateral_repay_rate(args).await
        }

        CryptoLoanCommands::FlexibleLoanAdjustLtv(args) => flexible_loan_adjust_ltv(args).await,

        CryptoLoanCommands::FlexibleLoanBorrow(args) => flexible_loan_borrow(args).await,

        CryptoLoanCommands::FlexibleLoanRepay(args) => flexible_loan_repay(args).await,

        CryptoLoanCommands::GetFlexibleLoanAssetsData(args) => {
            get_flexible_loan_assets_data(args).await
        }

        CryptoLoanCommands::GetFlexibleLoanBorrowHistory(args) => {
            get_flexible_loan_borrow_history(args).await
        }

        CryptoLoanCommands::GetFlexibleLoanCollateralAssetsData(args) => {
            get_flexible_loan_collateral_assets_data(args).await
        }

        CryptoLoanCommands::GetFlexibleLoanInterestRateHistory(args) => {
            get_flexible_loan_interest_rate_history(args).await
        }

        CryptoLoanCommands::GetFlexibleLoanLiquidationHistory(args) => {
            get_flexible_loan_liquidation_history(args).await
        }

        CryptoLoanCommands::GetFlexibleLoanLtvAdjustmentHistory(args) => {
            get_flexible_loan_ltv_adjustment_history(args).await
        }

        CryptoLoanCommands::GetFlexibleLoanOngoingOrders(args) => {
            get_flexible_loan_ongoing_orders(args).await
        }

        CryptoLoanCommands::GetFlexibleLoanRepaymentHistory(args) => {
            get_flexible_loan_repayment_history(args).await
        }

        CryptoLoanCommands::GetCryptoLoansIncomeHistory(args) => {
            get_crypto_loans_income_history(args).await
        }

        CryptoLoanCommands::GetLoanBorrowHistory(args) => get_loan_borrow_history(args).await,

        CryptoLoanCommands::GetLoanLtvAdjustmentHistory(args) => {
            get_loan_ltv_adjustment_history(args).await
        }

        CryptoLoanCommands::GetLoanRepaymentHistory(args) => get_loan_repayment_history(args).await,
    }
}

async fn check_collateral_repay_rate(mut args: CheckCollateralRepayRateArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CheckCollateralRepayRateParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CheckCollateralRepayRateParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.loan_coin.is_none() {
                        let loan_coin: String = Input::new()
                            .with_prompt("Input loan_coin:")
                            .interact_text()?;

                        args.loan_coin = Some(loan_coin);
                    }
                    if args.collateral_coin.is_none() {
                        let collateral_coin: String = Input::new()
                            .with_prompt("Input collateral_coin:")
                            .interact_text()?;

                        args.collateral_coin = Some(collateral_coin);
                    }
                }
                CheckCollateralRepayRateParams::builder(
                    args.loan_coin
                        .ok_or_else(|| anyhow::anyhow!("loan_coin is required"))?,
                    args.collateral_coin
                        .ok_or_else(|| anyhow::anyhow!("collateral_coin is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.check_collateral_repay_rate(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn flexible_loan_adjust_ltv(mut args: FlexibleLoanAdjustLtvArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FlexibleLoanAdjustLtvParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FlexibleLoanAdjustLtvParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.loan_coin.is_none() {
                        let loan_coin: String = Input::new()
                            .with_prompt("Input loan_coin:")
                            .interact_text()?;

                        args.loan_coin = Some(loan_coin);
                    }
                    if args.collateral_coin.is_none() {
                        let collateral_coin: String = Input::new()
                            .with_prompt("Input collateral_coin:")
                            .interact_text()?;

                        args.collateral_coin = Some(collateral_coin);
                    }
                    if args.adjustment_amount.is_none() {
                        let adjustment_amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input adjustment_amount:")
                            .interact_text()?;

                        args.adjustment_amount = Some(adjustment_amount);
                    }
                    if args.direction.is_none() {
                        let options = vec![
                            ("ADDITIONAL", FlexibleLoanAdjustLtvDirectionEnum::Additional),
                            ("REDUCED", FlexibleLoanAdjustLtvDirectionEnum::Reduced),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the direction")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.direction = Some(selected);
                    }
                }
                FlexibleLoanAdjustLtvParams::builder(
                    args.loan_coin
                        .ok_or_else(|| anyhow::anyhow!("loan_coin is required"))?,
                    args.collateral_coin
                        .ok_or_else(|| anyhow::anyhow!("collateral_coin is required"))?,
                    args.adjustment_amount
                        .ok_or_else(|| anyhow::anyhow!("adjustment_amount is required"))?,
                    args.direction
                        .ok_or_else(|| anyhow::anyhow!("direction is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.flexible_loan_adjust_ltv(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn flexible_loan_borrow(mut args: FlexibleLoanBorrowArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FlexibleLoanBorrowParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FlexibleLoanBorrowParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.loan_coin.is_none() {
                        let loan_coin: String = Input::new()
                            .with_prompt("Input loan_coin:")
                            .interact_text()?;

                        args.loan_coin = Some(loan_coin);
                    }
                    if args.collateral_coin.is_none() {
                        let collateral_coin: String = Input::new()
                            .with_prompt("Input collateral_coin:")
                            .interact_text()?;

                        args.collateral_coin = Some(collateral_coin);
                    }
                }
                FlexibleLoanBorrowParams::builder(
                    args.loan_coin
                        .ok_or_else(|| anyhow::anyhow!("loan_coin is required"))?,
                    args.collateral_coin
                        .ok_or_else(|| anyhow::anyhow!("collateral_coin is required"))?,
                )
                .loan_amount(args.loan_amount)
                .collateral_amount(args.collateral_amount)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.flexible_loan_borrow(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn flexible_loan_repay(mut args: FlexibleLoanRepayArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FlexibleLoanRepayParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FlexibleLoanRepayParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.loan_coin.is_none() {
                        let loan_coin: String = Input::new()
                            .with_prompt("Input loan_coin:")
                            .interact_text()?;

                        args.loan_coin = Some(loan_coin);
                    }
                    if args.collateral_coin.is_none() {
                        let collateral_coin: String = Input::new()
                            .with_prompt("Input collateral_coin:")
                            .interact_text()?;

                        args.collateral_coin = Some(collateral_coin);
                    }
                    if args.repay_amount.is_none() {
                        let repay_amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input repay_amount:")
                            .interact_text()?;

                        args.repay_amount = Some(repay_amount);
                    }
                }
                FlexibleLoanRepayParams::builder(
                    args.loan_coin
                        .ok_or_else(|| anyhow::anyhow!("loan_coin is required"))?,
                    args.collateral_coin
                        .ok_or_else(|| anyhow::anyhow!("collateral_coin is required"))?,
                    args.repay_amount
                        .ok_or_else(|| anyhow::anyhow!("repay_amount is required"))?,
                )
                .collateral_return(args.collateral_return)
                .full_repayment(args.full_repayment)
                .repayment_type(args.repayment_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.flexible_loan_repay(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_loan_assets_data(args: GetFlexibleLoanAssetsDataArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleLoanAssetsDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleLoanAssetsDataParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFlexibleLoanAssetsDataParams::builder()
                .loan_coin(args.loan_coin)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_flexible_loan_assets_data(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_loan_borrow_history(
    args: GetFlexibleLoanBorrowHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleLoanBorrowHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleLoanBorrowHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFlexibleLoanBorrowHistoryParams::builder()
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_flexible_loan_borrow_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_loan_collateral_assets_data(
    args: GetFlexibleLoanCollateralAssetsDataArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleLoanCollateralAssetsDataParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFlexibleLoanCollateralAssetsDataParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetFlexibleLoanCollateralAssetsDataParams::builder()
                .collateral_coin(args.collateral_coin)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_flexible_loan_collateral_assets_data(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_loan_interest_rate_history(
    mut args: GetFlexibleLoanInterestRateHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleLoanInterestRateHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFlexibleLoanInterestRateHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.coin.is_none() {
                        let coin: String =
                            Input::new().with_prompt("Input coin:").interact_text()?;

                        args.coin = Some(coin);
                    }
                    if args.recv_window.is_none() {
                        let recv_window: i64 = Input::new()
                            .with_prompt("Input recv_window:")
                            .interact_text()?;

                        args.recv_window = Some(recv_window);
                    }
                }
                GetFlexibleLoanInterestRateHistoryParams::builder(
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
        .get_flexible_loan_interest_rate_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_loan_liquidation_history(
    args: GetFlexibleLoanLiquidationHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleLoanLiquidationHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFlexibleLoanLiquidationHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetFlexibleLoanLiquidationHistoryParams::builder()
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_flexible_loan_liquidation_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_loan_ltv_adjustment_history(
    args: GetFlexibleLoanLtvAdjustmentHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleLoanLtvAdjustmentHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFlexibleLoanLtvAdjustmentHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetFlexibleLoanLtvAdjustmentHistoryParams::builder()
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_flexible_loan_ltv_adjustment_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_loan_ongoing_orders(
    args: GetFlexibleLoanOngoingOrdersArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleLoanOngoingOrdersParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleLoanOngoingOrdersParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFlexibleLoanOngoingOrdersParams::builder()
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_flexible_loan_ongoing_orders(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_flexible_loan_repayment_history(
    args: GetFlexibleLoanRepaymentHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFlexibleLoanRepaymentHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFlexibleLoanRepaymentHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetFlexibleLoanRepaymentHistoryParams::builder()
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_flexible_loan_repayment_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_crypto_loans_income_history(
    args: GetCryptoLoansIncomeHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCryptoLoansIncomeHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetCryptoLoansIncomeHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetCryptoLoansIncomeHistoryParams::builder()
                .asset(args.asset)
                .r#type(args.r#type)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_crypto_loans_income_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_loan_borrow_history(args: GetLoanBorrowHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLoanBorrowHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetLoanBorrowHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetLoanBorrowHistoryParams::builder()
                .order_id(args.order_id)
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_loan_borrow_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_loan_ltv_adjustment_history(
    args: GetLoanLtvAdjustmentHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLoanLtvAdjustmentHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetLoanLtvAdjustmentHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetLoanLtvAdjustmentHistoryParams::builder()
                .order_id(args.order_id)
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_loan_ltv_adjustment_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_loan_repayment_history(args: GetLoanRepaymentHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetLoanRepaymentHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetLoanRepaymentHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetLoanRepaymentHistoryParams::builder()
                .order_id(args.order_id)
                .loan_coin(args.loan_coin)
                .collateral_coin(args.collateral_coin)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_loan_repayment_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
