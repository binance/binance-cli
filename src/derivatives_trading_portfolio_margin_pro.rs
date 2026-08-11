use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::{
    DERIVATIVES_TRADING_PORTFOLIO_MARGIN_PRO_REST_API_PROD_URL,
    DERIVATIVES_TRADING_PORTFOLIO_MARGIN_PRO_REST_API_TESTNET_URL,
};
use binance_sdk::derivatives_trading_portfolio_margin_pro::DerivativesTradingPortfolioMarginProRestApi;
use binance_sdk::derivatives_trading_portfolio_margin_pro::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("derivatives-trading-portfolio-margin-pro");

    let client_config =
        get_client_configuration(profile, "derivatives-trading-portfolio-margin-pro").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "testnet" | "demo" => {
            DERIVATIVES_TRADING_PORTFOLIO_MARGIN_PRO_REST_API_TESTNET_URL.to_string()
        }
        "prod" => DERIVATIVES_TRADING_PORTFOLIO_MARGIN_PRO_REST_API_PROD_URL.to_string(),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid api env, valid values: testnet, demo, prod",
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

    Ok(DerivativesTradingPortfolioMarginProRestApi::from_config(
        rest_conf,
    ))
}

#[derive(Args, Debug)]
struct BnbTransferArgs {
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    transfer_side: Option<BnbTransferTransferSideEnum>,
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
struct ChangeAutoRepayFuturesStatusArgs {
    #[arg(
        help = r#"`false` for turn off the auto-repay futures negative balance function"#,
        long
    )]
    auto_repay: Option<ChangeAutoRepayFuturesStatusAutoRepayEnum>,
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
struct DeleteMarginCallLevelArgs {
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
struct FundAutoCollectionArgs {
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
struct FundCollectionByAssetArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
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
struct GetAutoRepayFuturesStatusArgs {
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
struct GetDeltaModeStatusArgs {
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
struct GetMarginCallLevelArgs {
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
struct GetPortfolioMarginProAccountBalanceArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
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
struct GetPortfolioMarginProAccountInfoArgs {
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
struct GetPortfolioMarginProSpanAccountInfoArgs {
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
struct GetTransferableEarnAssetBalanceForPortfolioMarginArgs {
    #[arg(help = r#"`LDUSDT` only"#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    transfer_type: Option<GetTransferableEarnAssetBalanceForPortfolioMarginTransferTypeEnum>,
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
struct PortfolioMarginProBankruptcyLoanRepayArgs {
    #[arg(help = r#""#, long)]
    from: Option<PortfolioMarginProBankruptcyLoanRepayFromEnum>,
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
struct QueryPortfolioMarginProBankruptcyLoanAmountArgs {
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
struct QueryPortfolioMarginProBankruptcyLoanRepayHistoryArgs {
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    size: Option<i64>,
    #[arg(help = r#"Currently querying page. Start from 1."#, long)]
    current: Option<i64>,
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
struct QueryPortfolioMarginProNegativeBalanceInterestHistoryArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"Start time"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"End time"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Number of results returned."#, long)]
    size: Option<i64>,
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
struct RepayFuturesNegativeBalanceArgs {
    #[arg(help = r#""#, long)]
    from: Option<RepayFuturesNegativeBalanceFromEnum>,
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
struct SetMarginCallLevelArgs {
    #[arg(help = r#"The value must be within the range [1.1, 2.0]."#, long)]
    margin_call_level: Option<rust_decimal::Decimal>,
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
struct SwitchDeltaModeArgs {
    #[arg(
        help = r#"`true` to enable Delta mode; `false` to disable Delta mode"#,
        long
    )]
    delta_enabled: Option<SwitchDeltaModeDeltaEnabledEnum>,
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
struct TransferLdusdtRwusdForPortfolioMarginArgs {
    #[arg(help = r#""#, long)]
    asset: Option<TransferLdusdtRwusdForPortfolioMarginAssetEnum>,
    #[arg(help = r#""#, long)]
    transfer_type: Option<TransferLdusdtRwusdForPortfolioMarginTransferTypeEnum>,
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
struct GetPortfolioMarginAssetLeverageArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct PortfolioMarginCollateralRateArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct PortfolioMarginProTieredCollateralRateArgs {
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
struct QueryPortfolioMarginAssetIndexPriceArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}

#[derive(Subcommand)]
pub enum DerivativesTradingPortfolioMarginProCommands {
    #[command(
        about = decode_selected_entities(r#"BNB transfer can be between Margin Account and USDM Account

Weight(IP): 1500

Security Type: USER_DATA

Notes:
- You can only use this function 2 times per 10 minutes in a rolling manner"#, false),
    )]
    BnbTransfer(BnbTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Change Auto-repay-futures Status

Weight(IP): 1500

Security Type: TRADE"#, false),
    )]
    ChangeAutoRepayFuturesStatus(ChangeAutoRepayFuturesStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Delete the margin call level for a Portfolio Margin account.

Weight(IP): 1500

Security Type: USER_DATA"#, false),
    )]
    DeleteMarginCallLevel(DeleteMarginCallLevelArgs),
    #[command(
        about = decode_selected_entities(r#"Transfers all assets from Futures Account to Margin account

Weight(IP): 1500

Security Type: USER_DATA

Notes:
- The BNB would not be collected from UM-PM account to the Portfolio Margin account.
- You can only use this function 500 times per hour in a rolling manner."#, false),
    )]
    FundAutoCollection(FundAutoCollectionArgs),
    #[command(
        about = decode_selected_entities(r#"Transfers specific asset from Futures Account to Margin account

Weight(IP): 60

Security Type: USER_DATA

Notes:
- The BNB transfer is not be supported"#, false),
    )]
    FundCollectionByAsset(FundCollectionByAssetArgs),
    #[command(
        about = decode_selected_entities(r#"Query Auto-repay-futures Status

Weight(IP): 30

Security Type: USER_DATA"#, false),
    )]
    GetAutoRepayFuturesStatus(GetAutoRepayFuturesStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Query the Delta mode status of current account.

Weight(IP): 1500

Security Type: USER_DATA"#, false),
    )]
    GetDeltaModeStatus(GetDeltaModeStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Get the margin call level for a Portfolio Margin account.

Weight(IP): 1500

Security Type: USER_DATA"#, false),
    )]
    GetMarginCallLevel(GetMarginCallLevelArgs),
    #[command(
        about = decode_selected_entities(r#"Query Portfolio Margin Pro account balance

Weight(IP): 20

Security Type: USER_DATA"#, false),
    )]
    GetPortfolioMarginProAccountBalance(GetPortfolioMarginProAccountBalanceArgs),
    #[command(
        about = decode_selected_entities(r#"Get Portfolio Margin Pro Account Info

Weight(UID): 5

Security Type: USER_DATA"#, false),
    )]
    GetPortfolioMarginProAccountInfo(GetPortfolioMarginProAccountInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Get Portfolio Margin Pro SPAN Account Info (For Portfolio Margin Pro SPAN users only)

Weight(IP): 5

Security Type: USER_DATA"#, false),
    )]
    GetPortfolioMarginProSpanAccountInfo(GetPortfolioMarginProSpanAccountInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Get transferable earn asset balance for all types of Portfolio Margin account

Weight(IP): 1500

Security Type: USER_DATA"#, false),
    )]
    GetTransferableEarnAssetBalanceForPortfolioMargin(
        GetTransferableEarnAssetBalanceForPortfolioMarginArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Repay Portfolio Margin Pro Bankruptcy Loan

Weight(UID): 3000

Security Type: TRADE

Notes:
- Please note that the API Key has enabled Spot & Margin Trading permissions to access this endpoint."#, false),
    )]
    PortfolioMarginProBankruptcyLoanRepay(PortfolioMarginProBankruptcyLoanRepayArgs),
    #[command(
        about = decode_selected_entities(r#"Query Portfolio Margin Pro Bankruptcy Loan Amount

Weight(UID): 500

Security Type: USER_DATA

Notes:
- If there’s no classic portfolio margin bankruptcy loan, the amount would be 0"#, false),
    )]
    QueryPortfolioMarginProBankruptcyLoanAmount(QueryPortfolioMarginProBankruptcyLoanAmountArgs),
    #[command(
        about = decode_selected_entities(r#"Query repay history of pmloan for portfolio margin pro.

Weight(IP): 500

Security Type: USER_DATA

Notes:
- `startTime` and `endTime` cannot be longer than 360 days
- If `startTime` and `endTime` not sent, return records of the last 30 days by default.
- If `startTime`is sent and `endTime` is not sent, return records of [startTime, startTime+30d].
- If `startTime` is not sent and `endTime` is sent, return records of [endTime-30d, endTime]."#, false),
    )]
    QueryPortfolioMarginProBankruptcyLoanRepayHistory(
        QueryPortfolioMarginProBankruptcyLoanRepayHistoryArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Query interest history of negative balance for portfolio margin.

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    QueryPortfolioMarginProNegativeBalanceInterestHistory(
        QueryPortfolioMarginProNegativeBalanceInterestHistoryArgs,
    ),
    #[command(
        about = decode_selected_entities(r#"Repay futures Negative Balance

Weight(IP): 1500

Security Type: USER_DATA"#, false),
    )]
    RepayFuturesNegativeBalance(RepayFuturesNegativeBalanceArgs),
    #[command(
        about = decode_selected_entities(r#"Set the margin call level for a Portfolio Margin account. When the account's uniMMR drops to the specified level, a notification will be sent via email and SMS.

Weight(IP): 1500

Security Type: USER_DATA"#, false),
    )]
    SetMarginCallLevel(SetMarginCallLevelArgs),
    #[command(
        about = decode_selected_entities(r#"Switch the Delta mode for existing PM PRO / PM RETAIL accounts.

Weight(IP): 1500

Security Type: TRADE"#, false),
    )]
    SwitchDeltaMode(SwitchDeltaModeArgs),
    #[command(
        about = decode_selected_entities(r#"Transfer LDUSDT/RWUSD as collateral for all types of Portfolio Margin account

Weight(UID): 1500

Security Type: TRADE"#, false),
    )]
    TransferLdusdtRwusdForPortfolioMargin(TransferLdusdtRwusdForPortfolioMarginArgs),
    #[command(
        about = decode_selected_entities(r#"Get Portfolio Margin Asset Leverage

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    GetPortfolioMarginAssetLeverage(GetPortfolioMarginAssetLeverageArgs),
    #[command(
        about = decode_selected_entities(r#"Portfolio Margin Collateral Rate

Weight(IP): 50

Security Type: MARKET_DATA"#, false),
    )]
    PortfolioMarginCollateralRate(PortfolioMarginCollateralRateArgs),
    #[command(
        about = decode_selected_entities(r#"Portfolio Margin PRO Tiered Collateral Rate

Weight(IP): 50

Security Type: USER_DATA"#, false),
    )]
    PortfolioMarginProTieredCollateralRate(PortfolioMarginProTieredCollateralRateArgs),
    #[command(
        about = decode_selected_entities(r#"Query Portfolio Margin Asset Index Price

Weight: - 1 if `asset` is sent
- 50 if `asset` is not sent

Security Type: MARKET_DATA"#, false),
    )]
    QueryPortfolioMarginAssetIndexPrice(QueryPortfolioMarginAssetIndexPriceArgs),
}

pub async fn handle_derivatives_trading_portfolio_margin_pro_command(
    command: DerivativesTradingPortfolioMarginProCommands,
) -> anyhow::Result<()> {
    match command {

          DerivativesTradingPortfolioMarginProCommands::BnbTransfer (args) => bnb_transfer(args).await,

          DerivativesTradingPortfolioMarginProCommands::ChangeAutoRepayFuturesStatus (args) => change_auto_repay_futures_status(args).await,

          DerivativesTradingPortfolioMarginProCommands::DeleteMarginCallLevel (args) => delete_margin_call_level(args).await,

          DerivativesTradingPortfolioMarginProCommands::FundAutoCollection (args) => fund_auto_collection(args).await,

          DerivativesTradingPortfolioMarginProCommands::FundCollectionByAsset (args) => fund_collection_by_asset(args).await,

          DerivativesTradingPortfolioMarginProCommands::GetAutoRepayFuturesStatus (args) => get_auto_repay_futures_status(args).await,

          DerivativesTradingPortfolioMarginProCommands::GetDeltaModeStatus (args) => get_delta_mode_status(args).await,

          DerivativesTradingPortfolioMarginProCommands::GetMarginCallLevel (args) => get_margin_call_level(args).await,

          DerivativesTradingPortfolioMarginProCommands::GetPortfolioMarginProAccountBalance (args) => get_portfolio_margin_pro_account_balance(args).await,

          DerivativesTradingPortfolioMarginProCommands::GetPortfolioMarginProAccountInfo (args) => get_portfolio_margin_pro_account_info(args).await,

          DerivativesTradingPortfolioMarginProCommands::GetPortfolioMarginProSpanAccountInfo (args) => get_portfolio_margin_pro_span_account_info(args).await,

          DerivativesTradingPortfolioMarginProCommands::GetTransferableEarnAssetBalanceForPortfolioMargin (args) => get_transferable_earn_asset_balance_for_portfolio_margin(args).await,

          DerivativesTradingPortfolioMarginProCommands::PortfolioMarginProBankruptcyLoanRepay (args) => portfolio_margin_pro_bankruptcy_loan_repay(args).await,

          DerivativesTradingPortfolioMarginProCommands::QueryPortfolioMarginProBankruptcyLoanAmount (args) => query_portfolio_margin_pro_bankruptcy_loan_amount(args).await,

          DerivativesTradingPortfolioMarginProCommands::QueryPortfolioMarginProBankruptcyLoanRepayHistory (args) => query_portfolio_margin_pro_bankruptcy_loan_repay_history(args).await,

          DerivativesTradingPortfolioMarginProCommands::QueryPortfolioMarginProNegativeBalanceInterestHistory (args) => query_portfolio_margin_pro_negative_balance_interest_history(args).await,

          DerivativesTradingPortfolioMarginProCommands::RepayFuturesNegativeBalance (args) => repay_futures_negative_balance(args).await,

          DerivativesTradingPortfolioMarginProCommands::SetMarginCallLevel (args) => set_margin_call_level(args).await,

          DerivativesTradingPortfolioMarginProCommands::SwitchDeltaMode (args) => switch_delta_mode(args).await,

          DerivativesTradingPortfolioMarginProCommands::TransferLdusdtRwusdForPortfolioMargin (args) => transfer_ldusdt_rwusd_for_portfolio_margin(args).await,

          DerivativesTradingPortfolioMarginProCommands::GetPortfolioMarginAssetLeverage (args) => get_portfolio_margin_asset_leverage(args).await,

          DerivativesTradingPortfolioMarginProCommands::PortfolioMarginCollateralRate (args) => portfolio_margin_collateral_rate(args).await,

          DerivativesTradingPortfolioMarginProCommands::PortfolioMarginProTieredCollateralRate (args) => portfolio_margin_pro_tiered_collateral_rate(args).await,

          DerivativesTradingPortfolioMarginProCommands::QueryPortfolioMarginAssetIndexPrice (args) => query_portfolio_margin_asset_index_price(args).await,

    }
}

async fn bnb_transfer(mut args: BnbTransferArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<BnbTransferParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BnbTransferParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal =
                            Input::new().with_prompt("Input amount:").interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.transfer_side.is_none() {
                        let options = vec![
                            ("TO_UM", BnbTransferTransferSideEnum::ToUm),
                            ("FROM_UM", BnbTransferTransferSideEnum::FromUm),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the transfer_side")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.transfer_side = Some(selected);
                    }
                }
                BnbTransferParams::builder(
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    args.transfer_side
                        .ok_or_else(|| anyhow::anyhow!("transfer_side is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.bnb_transfer(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn change_auto_repay_futures_status(
    mut args: ChangeAutoRepayFuturesStatusArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ChangeAutoRepayFuturesStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<ChangeAutoRepayFuturesStatusParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.auto_repay.is_none() {
                        let options = vec![
                            ("true", ChangeAutoRepayFuturesStatusAutoRepayEnum::True),
                            ("false", ChangeAutoRepayFuturesStatusAutoRepayEnum::False),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the auto_repay")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.auto_repay = Some(selected);
                    }
                }
                ChangeAutoRepayFuturesStatusParams::builder(
                    args.auto_repay
                        .ok_or_else(|| anyhow::anyhow!("auto_repay is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.change_auto_repay_futures_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn delete_margin_call_level(args: DeleteMarginCallLevelArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DeleteMarginCallLevelParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DeleteMarginCallLevelParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => DeleteMarginCallLevelParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.delete_margin_call_level(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fund_auto_collection(args: FundAutoCollectionArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FundAutoCollectionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FundAutoCollectionParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => FundAutoCollectionParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.fund_auto_collection(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fund_collection_by_asset(mut args: FundCollectionByAssetArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FundCollectionByAssetParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FundCollectionByAssetParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String =
                            Input::new().with_prompt("Input asset:").interact_text()?;

                        args.asset = Some(asset);
                    }
                }
                FundCollectionByAssetParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.fund_collection_by_asset(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_auto_repay_futures_status(args: GetAutoRepayFuturesStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetAutoRepayFuturesStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetAutoRepayFuturesStatusParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetAutoRepayFuturesStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_auto_repay_futures_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_delta_mode_status(args: GetDeltaModeStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetDeltaModeStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetDeltaModeStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetDeltaModeStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_delta_mode_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_margin_call_level(args: GetMarginCallLevelArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetMarginCallLevelParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetMarginCallLevelParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetMarginCallLevelParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_margin_call_level(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_portfolio_margin_pro_account_balance(
    args: GetPortfolioMarginProAccountBalanceArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetPortfolioMarginProAccountBalanceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetPortfolioMarginProAccountBalanceParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetPortfolioMarginProAccountBalanceParams::builder()
                .asset(args.asset)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_portfolio_margin_pro_account_balance(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_portfolio_margin_pro_account_info(
    args: GetPortfolioMarginProAccountInfoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetPortfolioMarginProAccountInfoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetPortfolioMarginProAccountInfoParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetPortfolioMarginProAccountInfoParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_portfolio_margin_pro_account_info(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_portfolio_margin_pro_span_account_info(
    args: GetPortfolioMarginProSpanAccountInfoArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetPortfolioMarginProSpanAccountInfoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetPortfolioMarginProSpanAccountInfoParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetPortfolioMarginProSpanAccountInfoParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_portfolio_margin_pro_span_account_info(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_transferable_earn_asset_balance_for_portfolio_margin(
    mut args: GetTransferableEarnAssetBalanceForPortfolioMarginArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetTransferableEarnAssetBalanceForPortfolioMarginParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetTransferableEarnAssetBalanceForPortfolioMarginParams>(json)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                    })?
            }
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String =
                            Input::new().with_prompt("Input asset:").interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.transfer_type.is_none() {
                        let options = vec![
                        ("EARN_TO_FUTURE", GetTransferableEarnAssetBalanceForPortfolioMarginTransferTypeEnum::EarnToFuture),
                        ("FUTURE_TO_EARN", GetTransferableEarnAssetBalanceForPortfolioMarginTransferTypeEnum::FutureToEarn),
                    ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the transfer_type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.transfer_type = Some(selected);
                    }
                }
                GetTransferableEarnAssetBalanceForPortfolioMarginParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                    args.transfer_type
                        .ok_or_else(|| anyhow::anyhow!("transfer_type is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_transferable_earn_asset_balance_for_portfolio_margin(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn portfolio_margin_pro_bankruptcy_loan_repay(
    args: PortfolioMarginProBankruptcyLoanRepayArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PortfolioMarginProBankruptcyLoanRepayParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PortfolioMarginProBankruptcyLoanRepayParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => PortfolioMarginProBankruptcyLoanRepayParams::builder()
                .from(args.from)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .portfolio_margin_pro_bankruptcy_loan_repay(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_portfolio_margin_pro_bankruptcy_loan_amount(
    args: QueryPortfolioMarginProBankruptcyLoanAmountArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryPortfolioMarginProBankruptcyLoanAmountParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryPortfolioMarginProBankruptcyLoanAmountParams>(json)
                .ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryPortfolioMarginProBankruptcyLoanAmountParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_portfolio_margin_pro_bankruptcy_loan_amount(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_portfolio_margin_pro_bankruptcy_loan_repay_history(
    args: QueryPortfolioMarginProBankruptcyLoanRepayHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryPortfolioMarginProBankruptcyLoanRepayHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryPortfolioMarginProBankruptcyLoanRepayHistoryParams>(json)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                    })?
            }
            None => QueryPortfolioMarginProBankruptcyLoanRepayHistoryParams::builder()
                .start_time(args.start_time)
                .end_time(args.end_time)
                .size(args.size)
                .current(args.current)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_portfolio_margin_pro_bankruptcy_loan_repay_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_portfolio_margin_pro_negative_balance_interest_history(
    args: QueryPortfolioMarginProNegativeBalanceInterestHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params =
        match read_stdin_as::<QueryPortfolioMarginProNegativeBalanceInterestHistoryParams>() {
            Some(params) => params,
            None => match args.json {
                Some(json) => read_json_as::<
                    QueryPortfolioMarginProNegativeBalanceInterestHistoryParams,
                >(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
                None => QueryPortfolioMarginProNegativeBalanceInterestHistoryParams::builder()
                    .asset(args.asset)
                    .start_time(args.start_time)
                    .end_time(args.end_time)
                    .size(args.size)
                    .recv_window(args.recv_window)
                    .build()?,
            },
        };

    // Make the API call
    let response = rest_client
        .query_portfolio_margin_pro_negative_balance_interest_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn repay_futures_negative_balance(
    args: RepayFuturesNegativeBalanceArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RepayFuturesNegativeBalanceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<RepayFuturesNegativeBalanceParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => RepayFuturesNegativeBalanceParams::builder()
                .from(args.from)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.repay_futures_negative_balance(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn set_margin_call_level(mut args: SetMarginCallLevelArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SetMarginCallLevelParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SetMarginCallLevelParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.margin_call_level.is_none() {
                        let margin_call_level: rust_decimal::Decimal = Input::new()
                            .with_prompt("Input margin_call_level:")
                            .interact_text()?;

                        args.margin_call_level = Some(margin_call_level);
                    }
                }
                SetMarginCallLevelParams::builder(
                    args.margin_call_level
                        .ok_or_else(|| anyhow::anyhow!("margin_call_level is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.set_margin_call_level(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn switch_delta_mode(mut args: SwitchDeltaModeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SwitchDeltaModeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SwitchDeltaModeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.delta_enabled.is_none() {
                        let options = vec![
                            ("true", SwitchDeltaModeDeltaEnabledEnum::True),
                            ("false", SwitchDeltaModeDeltaEnabledEnum::False),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the delta_enabled")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.delta_enabled = Some(selected);
                    }
                }
                SwitchDeltaModeParams::builder(
                    args.delta_enabled
                        .ok_or_else(|| anyhow::anyhow!("delta_enabled is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.switch_delta_mode(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn transfer_ldusdt_rwusd_for_portfolio_margin(
    mut args: TransferLdusdtRwusdForPortfolioMarginArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TransferLdusdtRwusdForPortfolioMarginParams>() {
        Some(params) => params,
        None => {
            match args.json {
                Some(json) => read_json_as::<TransferLdusdtRwusdForPortfolioMarginParams>(json)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                    })?,
                None => {
                    if args.interactive {
                        if args.asset.is_none() {
                            let options = vec![
                                (
                                    "LDUSDT",
                                    TransferLdusdtRwusdForPortfolioMarginAssetEnum::Ldusdt,
                                ),
                                (
                                    "RWUSD",
                                    TransferLdusdtRwusdForPortfolioMarginAssetEnum::Rwusd,
                                ),
                            ];

                            let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                            let selected = Select::new()
                                .with_prompt("Please select the asset")
                                .items(&labels)
                                .default(0)
                                .interact()?;

                            let selected = options[selected].1.clone();

                            println!("Selected option: {:?}", selected);

                            args.asset = Some(selected);
                        }
                        if args.transfer_type.is_none() {
                            let options = vec![
                        ("EARN_TO_FUTURE", TransferLdusdtRwusdForPortfolioMarginTransferTypeEnum::EarnToFuture),
                        ("FUTURE_TO_EARN", TransferLdusdtRwusdForPortfolioMarginTransferTypeEnum::FutureToEarn),
                    ];

                            let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                            let selected = Select::new()
                                .with_prompt("Please select the transfer_type")
                                .items(&labels)
                                .default(0)
                                .interact()?;

                            let selected = options[selected].1.clone();

                            println!("Selected option: {:?}", selected);

                            args.transfer_type = Some(selected);
                        }
                        if args.amount.is_none() {
                            let amount: rust_decimal::Decimal =
                                Input::new().with_prompt("Input amount:").interact_text()?;

                            args.amount = Some(amount);
                        }
                    }
                    TransferLdusdtRwusdForPortfolioMarginParams::builder(
                        args.asset
                            .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                        args.transfer_type
                            .ok_or_else(|| anyhow::anyhow!("transfer_type is required"))?,
                        args.amount
                            .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    )
                    .recv_window(args.recv_window)
                    .build()?
                }
            }
        }
    };

    // Make the API call
    let response = rest_client
        .transfer_ldusdt_rwusd_for_portfolio_margin(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_portfolio_margin_asset_leverage(
    args: GetPortfolioMarginAssetLeverageArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    // Make the API call
    let response = rest_client.get_portfolio_margin_asset_leverage().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn portfolio_margin_collateral_rate(
    args: PortfolioMarginCollateralRateArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.portfolio_margin_collateral_rate().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn portfolio_margin_pro_tiered_collateral_rate(
    args: PortfolioMarginProTieredCollateralRateArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<PortfolioMarginProTieredCollateralRateParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<PortfolioMarginProTieredCollateralRateParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => PortfolioMarginProTieredCollateralRateParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .portfolio_margin_pro_tiered_collateral_rate(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_portfolio_margin_asset_index_price(
    args: QueryPortfolioMarginAssetIndexPriceArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<QueryPortfolioMarginAssetIndexPriceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryPortfolioMarginAssetIndexPriceParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => QueryPortfolioMarginAssetIndexPriceParams::builder()
                .asset(args.asset)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .query_portfolio_margin_asset_index_price(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
