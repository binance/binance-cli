use crate::utils::{
    decode_selected_entities, get_client_configuration, init_user_agent, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::FIAT_REST_API_PROD_URL;
use binance_sdk::fiat::FiatRestApi;
use binance_sdk::fiat::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    init_user_agent("fiat");

    let client_config = get_client_configuration(profile, "fiat").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(client_config.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = client_config.base_path.unwrap_or(match api_env.as_str() {
        "prod" => FIAT_REST_API_PROD_URL.to_string(),
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

    Ok(FiatRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct DepositArgs {
    currency: Option<String>,
    #[arg(help = r#"payment method; current supported: pix"#, long)]
    api_payment_method: Option<DepositApiPaymentMethodEnum>,
    #[arg(help = r#"deposit amount"#, long)]
    amount: Option<String>,
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
struct FiatWithdrawArgs {
    #[arg(help = r#"Fiat currency, such as BRL, ARS, MXN"#, long)]
    currency: Option<String>,
    #[arg(help = r#"payment method; current supported: `bank_transfer`"#, long)]
    api_payment_method: Option<FiatWithdrawApiPaymentMethodEnum>,
    #[arg(help = r#"withdraw amount"#, long)]
    amount: Option<i64>,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    account_info: Option<String>,

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
struct GetFiatDepositWithdrawHistoryArgs {
    #[arg(help = r#"0: deposit, 1: withdraw"#, long)]
    transaction_type: Option<String>,
    #[arg(help = r#""#, long)]
    begin_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    rows: Option<i64>,
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
struct GetFiatPaymentsHistoryArgs {
    #[arg(help = r#"0: buy, 1: sell"#, long)]
    transaction_type: Option<String>,
    #[arg(help = r#""#, long)]
    begin_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    page: Option<i64>,
    #[arg(help = r#""#, long)]
    rows: Option<i64>,
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
struct GetOrderDetailArgs {
    #[arg(help = r#"Order ID retrieved from the withdrawal API"#, long)]
    order_no: Option<String>,
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
pub enum FiatCommands {
    #[command(
        about = decode_selected_entities(r#"Submit deposit request, in this version, we only support BRL deposit via pix.

For BRL deposit via pix, you need to place an order before making a transfer from your bank.

Before calling this api, please make sure you have already completed your KYC or KYB, and already activated your
fiat service on our website.

Weight(UID): 45000

Security Type: TRADE

Notes:
- `timestamp`, `signature` and `recvWindow` are sent as query-string parameters, while the business fields (`currency`, `apiPaymentMethod`, `amount`, `ext`) are sent in the JSON request body with `Content-Type: application/json`."#, false),
    )]
    Deposit(DepositArgs),
    #[command(
        about = decode_selected_entities(r#"Submit withdraw request, in this version, we support BRL,ARS,MXN withdrawal via bank_transfer.

You need to call this api first, and call query order detail api in a loop to get the status of the order until
this order is successful.

Before calling this api, please make sure you have already completed your KYC or KYB, and already activated your
fiat service on our website.

Weight(UID): 45000

Security Type: TRADE"#, false),
    )]
    FiatWithdraw(FiatWithdrawArgs),
    #[command(
        about = decode_selected_entities(r#"Get Fiat Deposit/Withdraw History

Weight(UID): 45000

Security Type: USER_DATA

Notes:
- If `beginTime` and `endTime` are not sent, recent 30-day data is returned."#, false),
    )]
    GetFiatDepositWithdrawHistory(GetFiatDepositWithdrawHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Fiat Payments History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- If `beginTime` and `endTime` are not sent, recent 30-day data is returned.
- `paymentMethod` is returned only when querying buy history (`transactionType=0`).
- Supported payment methods: `Cash Balance`, `Credit Card`, `Online Banking`, `Bank Transfer`."#, false),
    )]
    GetFiatPaymentsHistory(GetFiatPaymentsHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get Order Detail

Before calling this api, please make sure you have already completed your KYC or KYB, and already activated your
fiat service on our website.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetOrderDetail(GetOrderDetailArgs),
}

pub async fn handle_fiat_command(command: FiatCommands) -> anyhow::Result<()> {
    match command {
        FiatCommands::Deposit(args) => deposit(args).await,

        FiatCommands::FiatWithdraw(args) => fiat_withdraw(args).await,

        FiatCommands::GetFiatDepositWithdrawHistory(args) => {
            get_fiat_deposit_withdraw_history(args).await
        }

        FiatCommands::GetFiatPaymentsHistory(args) => get_fiat_payments_history(args).await,

        FiatCommands::GetOrderDetail(args) => get_order_detail(args).await,
    }
}

async fn deposit(mut args: DepositArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DepositParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DepositParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.currency.is_none() {
                        let currency: String = Input::new()
                            .with_prompt("Please enter the currency")
                            .interact_text()?;

                        args.currency = Some(currency);
                    }

                    if args.api_payment_method.is_none() {
                        let options = vec![("pix", DepositApiPaymentMethodEnum::Pix)];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the api_payment_method")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.api_payment_method = Some(selected);
                    }

                    if args.amount.is_none() {
                        let amount: String = Input::new()
                            .with_prompt("Please enter the amount")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                DepositParams::builder(
                    args.currency
                        .ok_or_else(|| anyhow::anyhow!("currency is required"))?,
                    args.api_payment_method
                        .ok_or_else(|| anyhow::anyhow!("api_payment_method is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.deposit(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fiat_withdraw(mut args: FiatWithdrawArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FiatWithdrawParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FiatWithdrawParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.currency.is_none() {
                        let currency: String = Input::new()
                            .with_prompt("Please enter the currency")
                            .interact_text()?;

                        args.currency = Some(currency);
                    }

                    if args.api_payment_method.is_none() {
                        let options = vec![(
                            "bank_transfer",
                            FiatWithdrawApiPaymentMethodEnum::BankTransfer,
                        )];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the api_payment_method")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.api_payment_method = Some(selected);
                    }

                    if args.amount.is_none() {
                        let amount: i64 = Input::new()
                            .with_prompt("Please enter the amount")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }

                    if args.account_info.is_none() {
                        let account_info: String = Input::new()
                            .with_prompt("Please enter the account_info")
                            .interact_text()?;

                        args.account_info = Some(account_info);
                    }
                }
                FiatWithdrawParams::builder(
                    args.currency
                        .ok_or_else(|| anyhow::anyhow!("currency is required"))?,
                    args.api_payment_method
                        .ok_or_else(|| anyhow::anyhow!("api_payment_method is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    serde_json::from_str::<FiatWithdrawRequestAccountInfo>(
                        &args
                            .account_info
                            .ok_or_else(|| anyhow::anyhow!("account_info is required"))?,
                    )?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.fiat_withdraw(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_fiat_deposit_withdraw_history(
    mut args: GetFiatDepositWithdrawHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFiatDepositWithdrawHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetFiatDepositWithdrawHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.transaction_type.is_none() {
                        let transaction_type: String = Input::new()
                            .with_prompt("Please enter the transaction_type name")
                            .interact_text()?;

                        args.transaction_type = Some(transaction_type);
                    }
                }
                GetFiatDepositWithdrawHistoryParams::builder(
                    args.transaction_type
                        .ok_or_else(|| anyhow::anyhow!("transaction_type is required"))?,
                )
                .begin_time(args.begin_time)
                .end_time(args.end_time)
                .page(args.page)
                .rows(args.rows)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_fiat_deposit_withdraw_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_fiat_payments_history(mut args: GetFiatPaymentsHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetFiatPaymentsHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetFiatPaymentsHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.transaction_type.is_none() {
                        let transaction_type: String = Input::new()
                            .with_prompt("Please enter the transaction_type name")
                            .interact_text()?;

                        args.transaction_type = Some(transaction_type);
                    }
                }
                GetFiatPaymentsHistoryParams::builder(
                    args.transaction_type
                        .ok_or_else(|| anyhow::anyhow!("transaction_type is required"))?,
                )
                .begin_time(args.begin_time)
                .end_time(args.end_time)
                .page(args.page)
                .rows(args.rows)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_fiat_payments_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_order_detail(mut args: GetOrderDetailArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetOrderDetailParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetOrderDetailParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.order_no.is_none() {
                        let order_no: String = Input::new()
                            .with_prompt("Please enter the order_no name")
                            .interact_text()?;

                        args.order_no = Some(order_no);
                    }
                }
                GetOrderDetailParams::builder(
                    args.order_no
                        .ok_or_else(|| anyhow::anyhow!("order_no is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_order_detail(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
