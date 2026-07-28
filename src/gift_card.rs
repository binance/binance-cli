use crate::utils::{
    build_user_agent, decode_selected_entities, get_configuration_rest_api, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::GIFT_CARD_REST_API_PROD_URL;
use binance_sdk::gift_card::GiftCardRestApi;
use binance_sdk::gift_card::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::Input;
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    unsafe {
        env::set_var(
            "BINANCE_CONNECTOR_RUST_USER_AGENT",
            build_user_agent("gift-card"),
        );
    }

    let config_rest_api = get_configuration_rest_api(profile, "gift-card").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(config_rest_api.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = match api_env.as_str() {
        "prod" => GIFT_CARD_REST_API_PROD_URL,
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

    Ok(GiftCardRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct CreateADualTokenGiftCardArgs {
    #[arg(help = r#"The token you want to pay, example: BUSD"#, long)]
    base_token: Option<String>,
    #[arg(
        help = r#"The token you want to buy, example: BNB. If faceToken = baseToken, it's the same as createCode
endpoint."#,
        long
    )]
    face_token: Option<String>,
    #[arg(help = r#"The base token asset quantity, example : 1.002"#, long)]
    base_token_amount: Option<rust_decimal::Decimal>,
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
struct CreateASingleTokenGiftCardArgs {
    #[arg(help = r#"The token type contained in the Binance Gift Card"#, long)]
    token: Option<String>,
    #[arg(
        help = r#"The amount of the token contained in the Binance Gift Card"#,
        long
    )]
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
struct FetchRsaPublicKeyArgs {
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
struct FetchTokenLimitArgs {
    #[arg(help = r#"The token you want to pay, example: BUSD"#, long)]
    base_token: Option<String>,
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
struct RedeemABinanceGiftCardArgs {
    #[arg(
        help = r#"Redemption code of Binance Gift Card to be redeemed, supports both Plaintext & Encrypted code."#,
        long
    )]
    code: Option<String>,
    #[arg(
        help = r#"Each external unique ID represents a unique user on the partner platform. The function helps you to
identify the redemption behavior of different users, such as redemption frequency and amount. It
also helps risk and limit control of a single account, such as daily limit on redemption volume,
frequency, and incorrect number of entries. This will also prevent a single user account reach the
partner's daily redemption limits. We strongly recommend you to use this feature and transfer us the
User ID of your users if you have different users redeeming Binance Gift Cards on your platform. To
protect user data privacy, you may choose to transfer the user id in any desired format (max. 400
characters)."#,
        long
    )]
    external_uid: Option<String>,
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
struct VerifyBinanceGiftCardByGiftCardNumberArgs {
    #[arg(help = r#"Enter the Gift Card Number"#, long)]
    reference_no: Option<String>,
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
pub enum GiftCardCommands {
    #[command(
        about = decode_selected_entities(r#"* This API is for creating a dual-token ( stablecoin-denominated) Binance Gift Card. You may create a gift card
using USDT as baseToken, that is redeemable to another designated token (faceToken). For example, you can create
a fixed-value BTC gift card and pay with 100 USDT plus minting fee. This gift card can keep the value fixed at
100 USDT before redemption, and will be redeemable to BTC equivalent to 100 USDT upon redemption.

* Once successfully created, the amount of baseToken (e.g. USDT) in the fixed-value gift card along with the fee
would be deducted from your funding wallet.
  * To get started with, please make sure:
  * You have a Binance account
  * You have passed KYB
  * You have a sufﬁcient balance(Gift Card amount and fee amount) in your Binance funding wallet
  * You need Enable Withdrawals for the API Key which requests this endpoint.

Weight(IP): 1

Security Type: TRADE"#, false),
    )]
    CreateADualTokenGiftCard(CreateADualTokenGiftCardArgs),
    #[command(
        about = decode_selected_entities(r#"This API is for creating a Binance Gift Card.

To get started with, please make sure:
  * You have a Binance account
  * You have passed KYB
  * You have a sufﬁcient balance(Gift Card amount and fee amount) in your Binance funding wallet
  * You need `Enable Withdrawals` for the API Key which requests this endpoint.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    CreateASingleTokenGiftCard(CreateASingleTokenGiftCardArgs),
    #[command(
        about = decode_selected_entities(r#"This API is for fetching the RSA Public Key.
This RSA Public key will be used to encrypt the card code.

**Please note that the RSA Public key fetched is valid only for the current day.**

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    FetchRsaPublicKey(FetchRsaPublicKeyArgs),
    #[command(
        about = decode_selected_entities(r#"This API is to help you verify which tokens are available for you to create Stablecoin-Denominated gift cards as
mentioned in section 2 and its’ limitation.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    FetchTokenLimit(FetchTokenLimitArgs),
    #[command(
        about = decode_selected_entities(r#"This API is for redeeming a Binance Gift Card. Once redeemed, the coins will be deposited in your funding wallet.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Parameter `code` can be sent in two formats: `Plaintext` and `Encrypted`.
- Sending `code` in encrypted format is more secure than plaintext.
- To send encrypted `code`:
  - Fetch RSA public key from `GET /sapi/v1/giftcard/cryptography/rsa-public-key`.
  - Encrypt card code using `RSA/ECB/OAEPWithSHA-256AndMGF1Padding`.
- If you enter the wrong redemption code 5 times within 24 hours, you will no longer be able to redeem any Binance Gift Cards that day."#, false),
    )]
    RedeemABinanceGiftCard(RedeemABinanceGiftCardArgs),
    #[command(
        about = decode_selected_entities(r#"This API is for verifying whether the Binance Gift Card is valid or not by entering Gift Card Number.

**Please note that if you enter the wrong Gift Card Number 5 times within an hour, you will no longer be able to
verify any Gift Card Number for that hour.**

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    VerifyBinanceGiftCardByGiftCardNumber(VerifyBinanceGiftCardByGiftCardNumberArgs),
}

pub async fn handle_gift_card_command(command: GiftCardCommands) -> anyhow::Result<()> {
    match command {
        GiftCardCommands::CreateADualTokenGiftCard(args) => {
            create_a_dual_token_gift_card(args).await
        }

        GiftCardCommands::CreateASingleTokenGiftCard(args) => {
            create_a_single_token_gift_card(args).await
        }

        GiftCardCommands::FetchRsaPublicKey(args) => fetch_rsa_public_key(args).await,

        GiftCardCommands::FetchTokenLimit(args) => fetch_token_limit(args).await,

        GiftCardCommands::RedeemABinanceGiftCard(args) => redeem_a_binance_gift_card(args).await,

        GiftCardCommands::VerifyBinanceGiftCardByGiftCardNumber(args) => {
            verify_binance_gift_card_by_gift_card_number(args).await
        }
    }
}

async fn create_a_dual_token_gift_card(
    mut args: CreateADualTokenGiftCardArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CreateADualTokenGiftCardParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CreateADualTokenGiftCardParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.base_token.is_none() {
                        let base_token: String = Input::new()
                            .with_prompt("Please enter the base_token name")
                            .interact_text()?;

                        args.base_token = Some(base_token);
                    }
                    if args.face_token.is_none() {
                        let face_token: String = Input::new()
                            .with_prompt("Please enter the face_token name")
                            .interact_text()?;

                        args.face_token = Some(face_token);
                    }
                    if args.base_token_amount.is_none() {
                        let base_token_amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the base_token_amount name")
                            .interact_text()?;

                        args.base_token_amount = Some(base_token_amount);
                    }
                }
                CreateADualTokenGiftCardParams::builder(
                    args.base_token
                        .ok_or_else(|| anyhow::anyhow!("base_token is required"))?,
                    args.face_token
                        .ok_or_else(|| anyhow::anyhow!("face_token is required"))?,
                    args.base_token_amount
                        .ok_or_else(|| anyhow::anyhow!("base_token_amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.create_a_dual_token_gift_card(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn create_a_single_token_gift_card(
    mut args: CreateASingleTokenGiftCardArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CreateASingleTokenGiftCardParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CreateASingleTokenGiftCardParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.token.is_none() {
                        let token: String = Input::new()
                            .with_prompt("Please enter the token name")
                            .interact_text()?;

                        args.token = Some(token);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                CreateASingleTokenGiftCardParams::builder(
                    args.token
                        .ok_or_else(|| anyhow::anyhow!("token is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.create_a_single_token_gift_card(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fetch_rsa_public_key(args: FetchRsaPublicKeyArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FetchRsaPublicKeyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FetchRsaPublicKeyParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => FetchRsaPublicKeyParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.fetch_rsa_public_key(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fetch_token_limit(mut args: FetchTokenLimitArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FetchTokenLimitParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FetchTokenLimitParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.base_token.is_none() {
                        let base_token: String = Input::new()
                            .with_prompt("Please enter the base_token name")
                            .interact_text()?;

                        args.base_token = Some(base_token);
                    }
                }
                FetchTokenLimitParams::builder(
                    args.base_token
                        .ok_or_else(|| anyhow::anyhow!("base_token is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.fetch_token_limit(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn redeem_a_binance_gift_card(mut args: RedeemABinanceGiftCardArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<RedeemABinanceGiftCardParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<RedeemABinanceGiftCardParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.code.is_none() {
                        let code: String = Input::new()
                            .with_prompt("Please enter the code name")
                            .interact_text()?;

                        args.code = Some(code);
                    }
                }
                RedeemABinanceGiftCardParams::builder(
                    args.code
                        .ok_or_else(|| anyhow::anyhow!("code is required"))?,
                )
                .external_uid(args.external_uid)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.redeem_a_binance_gift_card(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn verify_binance_gift_card_by_gift_card_number(
    mut args: VerifyBinanceGiftCardByGiftCardNumberArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<VerifyBinanceGiftCardByGiftCardNumberParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<VerifyBinanceGiftCardByGiftCardNumberParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.reference_no.is_none() {
                        let reference_no: String = Input::new()
                            .with_prompt("Please enter the reference_no name")
                            .interact_text()?;

                        args.reference_no = Some(reference_no);
                    }
                }
                VerifyBinanceGiftCardByGiftCardNumberParams::builder(
                    args.reference_no
                        .ok_or_else(|| anyhow::anyhow!("reference_no is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .verify_binance_gift_card_by_gift_card_number(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
