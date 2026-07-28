use crate::utils::{
    build_user_agent, decode_selected_entities, get_configuration_rest_api, read_json_as,
    read_stdin_as,
};
use binance_sdk::config::{ConfigurationRestApi, PrivateKey};
use binance_sdk::constants::WALLET_REST_API_PROD_URL;
use binance_sdk::wallet::WalletRestApi;
use binance_sdk::wallet::rest_api::*;
use clap::{Args, Subcommand};
use dialoguer::{Input, Select};
use std::env;
use std::io;
use std::io::{Error, ErrorKind};

fn get_client(profile: Option<&str>, is_signed: bool) -> Result<RestApi, Error> {
    unsafe {
        env::set_var(
            "BINANCE_CONNECTOR_RUST_USER_AGENT",
            build_user_agent("wallet"),
        );
    }

    let config_rest_api = get_configuration_rest_api(profile, "wallet").unwrap();
    let api_env = env::var("BINANCE_API_ENV")
        .ok()
        .or(config_rest_api.env)
        .unwrap_or_else(|| "prod".to_string());

    let base_path = match api_env.as_str() {
        "prod" => WALLET_REST_API_PROD_URL,
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

    Ok(WalletRestApi::from_config(rest_conf))
}

#[derive(Args, Debug)]
struct AccountApiTradingStatusArgs {
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
struct AccountInfoArgs {
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
struct AccountStatusArgs {
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
struct DailyAccountSnapshotArgs {
    #[arg(help = r#""#, long)]
    r#type: Option<DailyAccountSnapshotTypeEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
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
struct DisableFastWithdrawSwitchArgs {
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
struct EnableFastWithdrawSwitchArgs {
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
struct GetApiKeyPermissionArgs {
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
struct AssetDetailArgs {
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
struct AssetDividendRecordArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
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
struct DustConvertArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#"`SPOT` or `MARGIN`, default `SPOT`"#, long)]
    account_type: Option<String>,
    #[arg(help = r#"A unique id for the request"#, long)]
    client_id: Option<String>,
    #[arg(help = r#""#, long)]
    target_asset: Option<String>,
    #[arg(help = r#""#, long)]
    third_party_client_id: Option<String>,
    #[arg(help = r#""#, long)]
    dust_quota_asset_to_target_asset_price: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct DustConvertibleAssetsArgs {
    #[arg(help = r#""#, long)]
    target_asset: Option<String>,
    #[arg(help = r#"`SPOT` or `MARGIN`, default `SPOT`"#, long)]
    account_type: Option<String>,
    #[arg(help = r#""#, long)]
    dust_quota_asset_to_target_asset_price: Option<rust_decimal::Decimal>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct DustTransferArgs {
    #[arg(
        help = r#"The asset being converted. For example: asset=BTC,USDT"#,
        long
    )]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    account_type: Option<DustTransferAccountTypeEnum>,
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
struct DustlogArgs {
    #[arg(help = r#""#, long)]
    account_type: Option<DustlogAccountTypeEnum>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
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
struct FundingWalletArgs {
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long, num_args = 0..=1, default_missing_value = "true")]
    need_btc_valuation: Option<bool>,
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
struct GetAssetsThatCanBeConvertedIntoBnbArgs {
    #[arg(help = r#""#, long)]
    account_type: Option<GetAssetsThatCanBeConvertedIntoBnbAccountTypeEnum>,
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
struct GetCloudMiningPaymentAndRefundHistoryArgs {
    #[arg(help = r#"inclusive, unit: ms"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"exclusive, unit: ms"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"The transaction id"#, long)]
    tran_id: Option<i64>,
    #[arg(help = r#"The unique flag"#, long)]
    client_tran_id: Option<String>,
    #[arg(help = r#"If it is blank, we will query all assets"#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct GetOpenSymbolListArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct QueryUserDelegationHistoryArgs {
    #[arg(help = r#""#, long)]
    email: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    r#type: Option<QueryUserDelegationHistoryTypeEnum>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
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
struct QueryUserUniversalTransferHistoryArgs {
    #[arg(help = r#""#, long)]
    r#type: Option<String>,
    #[arg(help = r#""#, long)]
    start_time: Option<i64>,
    #[arg(help = r#""#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    current: Option<i64>,
    #[arg(help = r#""#, long)]
    size: Option<i64>,
    #[arg(help = r#""#, long)]
    from_symbol: Option<QueryUserUniversalTransferHistoryFromSymbolEnum>,
    #[arg(help = r#""#, long)]
    to_symbol: Option<QueryUserUniversalTransferHistoryToSymbolEnum>,
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
struct QueryUserWalletBalanceArgs {
    #[arg(help = r#""#, long)]
    quote_asset: Option<String>,
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
struct ToggleBnbBurnOnSpotTradeAndMarginInterestArgs {
    #[arg(
        help = r#"Determines whether to use BNB to pay for trading fees on SPOT"#,
        long
    )]
    spot_bnb_burn: Option<String>,
    #[arg(
        help = r#"Determines whether to use BNB to pay for margin loan's interest"#,
        long
    )]
    interest_bnb_burn: Option<String>,
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
struct TradeFeeArgs {
    #[arg(help = r#""#, long)]
    symbol: Option<String>,
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
struct UserAssetArgs {
    #[arg(
        help = r#"If asset is blank, then query all positive assets user have."#,
        long
    )]
    asset: Option<String>,
    #[arg(help = r#"Whether need btc valuation or not."#, long, num_args = 0..=1, default_missing_value = "true")]
    need_btc_valuation: Option<bool>,
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
struct UserUniversalTransferArgs {
    #[arg(help = r#""#, long)]
    r#type: Option<UserUniversalTransferTypeEnum>,
    #[arg(help = r#""#, long)]
    asset: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    from_symbol: Option<UserUniversalTransferFromSymbolEnum>,
    #[arg(help = r#""#, long)]
    to_symbol: Option<UserUniversalTransferToSymbolEnum>,
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
struct AllCoinsInformationArgs {
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
struct DepositAddressArgs {
    #[arg(
        help = r#"`coin` refers to the parent network address format that the address is using"#,
        long
    )]
    coin: Option<String>,
    #[arg(help = r#""#, long)]
    network: Option<String>,
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
struct DepositHistoryArgs {
    #[arg(help = r#"return `sourceAddress` field when set to `true`"#, long, num_args = 0..=1, default_missing_value = "true")]
    include_source: Option<bool>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(
        help = r#"0: pending, 6: credited but cannot withdraw, 7: Wrong Deposit, 8: Waiting User confirm, 1: success"#,
        long
    )]
    status: Option<DepositHistoryStatusEnum>,
    #[arg(help = r#"Default: 90 days from current timestamp"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Default: present timestamp"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    offset: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#""#, long)]
    recv_window: Option<i64>,
    #[arg(help = r#""#, long)]
    tx_id: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct FetchDepositAddressListWithNetworkArgs {
    #[arg(help = r#"Coin name"#, long)]
    coin: Option<String>,
    #[arg(
        help = r#"If network is not send, return with default network of the coin. You can get network and isDefault in networkList in the response of `Get /sapi/v1/capital/config/getall`"#,
        long
    )]
    network: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct FetchWithdrawAddressListArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct FetchWithdrawQuotaArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct OneClickArrivalDepositApplyArgs {
    #[arg(help = r#"Deposit record Id, priority use"#, long)]
    deposit_id: Option<i64>,
    #[arg(help = r#"Deposit txId, used when depositId is not specified"#, long)]
    tx_id: Option<String>,
    #[arg(help = r#"Sub-accountId of Cloud user"#, long)]
    sub_account_id: Option<String>,
    #[arg(help = r#"Sub-userId of parent user"#, long)]
    sub_user_id: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct WithdrawArgs {
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#"Withdrawal address"#, long)]
    address: Option<String>,
    #[arg(help = r#"Amount"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"client side id for withdrawal, if provide here, can be used in GET
`/sapi/v1/capital/withdraw/history` for query."#,
        long
    )]
    withdraw_order_id: Option<String>,
    #[arg(help = r#"Withdrawal network"#, long)]
    network: Option<String>,
    #[arg(
        help = r#"Secondary address identifier for coins like XRP,XMR etc."#,
        long
    )]
    address_tag: Option<String>,
    #[arg(help = r#"When making internal transfer, `true` for returning the fee to the destination account; `false` for
returning the fee back to the departure account. Default `false`."#, long, num_args = 0..=1, default_missing_value = "true")]
    transaction_fee_flag: Option<bool>,
    #[arg(help = r#""#, long)]
    name: Option<String>,
    #[arg(
        help = r#"The wallet type for withdraw，0-spot wallet ，1-funding wallet. Default walletType is the current
"selected wallet" under wallet->Fiat and Spot/Funding->Deposit"#,
        long
    )]
    wallet_type: Option<i64>,
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
struct WithdrawHistoryArgs {
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(
        help = r#"client side id for withdrawal, if provided in POST `/sapi/v1/capital/withdraw/apply`, can be used here for
query."#,
        long
    )]
    withdraw_order_id: Option<String>,
    #[arg(
        help = r#"0(0:Email Sent, 2:Awaiting Approval 3:Rejected 4:Processing 6:Completed)"#,
        long
    )]
    status: Option<i64>,
    #[arg(help = r#"Default: 0"#, long)]
    offset: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(
        help = r#"id list returned in the response of POST `/sapi/v1/capital/withdraw/apply`, separated by `,`"#,
        long
    )]
    id_list: Option<String>,
    #[arg(help = r#"Default: 90 days from current timestamp"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Default: present timestamp"#, long)]
    end_time: Option<i64>,
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
struct GetSymbolsDelistScheduleForSpotArgs {
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
struct SystemStatusArgs {
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct BrokerWithdrawArgs {
    #[arg(help = r#""#, long)]
    address: Option<String>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(
        help = r#"withdrawID defined by the client (i.e. client's internal withdrawID)"#,
        long
    )]
    withdraw_order_id: Option<String>,
    #[arg(help = r#"JSON format questionnaire answers."#, long)]
    questionnaire: Option<String>,
    #[arg(
        help = r#"JSON format originator Pii, see StandardPii section below"#,
        long
    )]
    originator_pii: Option<String>,
    #[arg(
        help = r#"Secondary address identifier for coins like XRP,XMR etc."#,
        long
    )]
    address_tag: Option<String>,
    #[arg(help = r#""#, long)]
    network: Option<String>,
    #[arg(
        help = r#"Description of the address. Address book cap is 200, space in name should be encoded into `%20`"#,
        long
    )]
    address_name: Option<String>,
    #[arg(help = r#"When making internal transfer, `true` for returning the fee to the destination account; `false` for
returning the fee back to the departure account. Default `false`."#, long, num_args = 0..=1, default_missing_value = "true")]
    transaction_fee_flag: Option<bool>,
    #[arg(
        help = r#"The wallet type for withdraw，0-spot wallet ，1-funding wallet. Default walletType is the current
"selected wallet" under wallet->Fiat and Spot/Funding->Deposit"#,
        long
    )]
    wallet_type: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct CheckQuestionnaireRequirementsArgs {
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
struct DepositHistoryTravelRuleArgs {
    #[arg(help = r#"Comma(,) separated list of travel rule record Ids."#, long)]
    tr_id: Option<String>,
    #[arg(help = r#"Comma(,) separated list of transaction Ids."#, long)]
    tx_id: Option<String>,
    #[arg(help = r#"Comma(,) separated list of wallet tran Ids."#, long)]
    tran_id: Option<String>,
    #[arg(help = r#""#, long)]
    network: Option<String>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#"0:Completed,1:Pending,2:Failed"#, long)]
    travel_rule_status: Option<i64>,
    #[arg(help = r#"true: Only return records that pending deposit questionnaire. false/not provided: return all records."#, long, num_args = 0..=1, default_missing_value = "true")]
    pending_questionnaire: Option<bool>,
    #[arg(help = r#"Default: 90 days from current timestamp"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Default: present timestamp"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#"Default: 0"#, long)]
    offset: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct DepositHistoryV2Args {
    #[arg(help = r#"Comma(,) separated list of wallet tran Ids."#, long)]
    deposit_id: Option<i64>,
    #[arg(help = r#"Comma(,) separated list of transaction Ids."#, long)]
    tx_id: Option<String>,
    #[arg(help = r#""#, long)]
    network: Option<String>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#"true: return `questionnaire` within response."#, long, num_args = 0..=1, default_missing_value = "true")]
    retrieve_questionnaire: Option<bool>,
    #[arg(help = r#"Default: 90 days from current timestamp"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Default: present timestamp"#, long)]
    end_time: Option<i64>,
    #[arg(help = r#""#, long)]
    offset: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct FetchAddressVerificationListArgs {
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
struct GetCountryListArgs {
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
struct GetRegionListArgs {
    #[arg(help = r#"ISO 2-digit country code (from Country List API)."#, long)]
    country_code: Option<String>,
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
struct SubmitDepositQuestionnaireArgs {
    #[arg(help = r#"External user ID."#, long)]
    sub_account_id: Option<String>,
    #[arg(help = r#"Wallet deposit ID."#, long)]
    deposit_id: Option<i64>,
    #[arg(help = r#"JSON format questionnaire answers."#, long)]
    questionnaire: Option<String>,
    #[arg(help = r#"JSON format beneficiary Pii."#, long)]
    beneficiary_pii: Option<String>,
    #[arg(help = r#""#, long)]
    network: Option<String>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#""#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#""#, long)]
    address: Option<String>,
    #[arg(help = r#""#, long)]
    address_tag: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SubmitDepositQuestionnaireTravelRuleArgs {
    #[arg(help = r#"Wallet tran ID"#, long)]
    tran_id: Option<i64>,
    #[arg(help = r#"JSON format questionnaire answers."#, long)]
    questionnaire: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct SubmitDepositQuestionnaireV2Args {
    #[arg(help = r#"Wallet deposit ID"#, long)]
    deposit_id: Option<i64>,
    #[arg(help = r#"JSON format questionnaire answers."#, long)]
    questionnaire: Option<String>,
    #[arg(short = 'i', long)]
    interactive: bool,
    #[arg(help = r#"Send all fields as JSON"#, long)]
    json: Option<String>,
    #[arg(help = r#"Select a profile"#, long)]
    profile: Option<String>,
}
#[derive(Args, Debug)]
struct VaspListArgs {
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
struct WithdrawHistoryV1Args {
    #[arg(help = r#"Comma(,) separated list of travel rule record Ids."#, long)]
    tr_id: Option<String>,
    #[arg(help = r#"Comma(,) separated list of transaction Ids."#, long)]
    tx_id: Option<String>,
    #[arg(
        help = r#"client side id for withdrawal, if provided in POST `/sapi/v1/capital/withdraw/apply`, can be used here for
query."#,
        long
    )]
    withdraw_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    network: Option<String>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#"0:Completed,1:Pending,2:Failed"#, long)]
    travel_rule_status: Option<i64>,
    #[arg(help = r#""#, long)]
    offset: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Default: 90 days from current timestamp"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Default: present timestamp"#, long)]
    end_time: Option<i64>,
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
struct WithdrawHistoryV2Args {
    #[arg(help = r#"Comma(,) separated list of travel rule record Ids."#, long)]
    tr_id: Option<String>,
    #[arg(help = r#"Comma(,) separated list of transaction Ids."#, long)]
    tx_id: Option<String>,
    #[arg(
        help = r#"client side id for withdrawal, if provided in POST `/sapi/v1/capital/withdraw/apply`, can be used here for
query."#,
        long
    )]
    withdraw_order_id: Option<String>,
    #[arg(help = r#""#, long)]
    network: Option<String>,
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#"0:Completed,1:Pending,2:Failed"#, long)]
    travel_rule_status: Option<i64>,
    #[arg(help = r#""#, long)]
    offset: Option<i64>,
    #[arg(help = r#""#, long)]
    limit: Option<i64>,
    #[arg(help = r#"Default: 90 days from current timestamp"#, long)]
    start_time: Option<i64>,
    #[arg(help = r#"Default: present timestamp"#, long)]
    end_time: Option<i64>,
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
struct WithdrawTravelRuleArgs {
    #[arg(help = r#""#, long)]
    coin: Option<String>,
    #[arg(help = r#"Withdrawal address"#, long)]
    address: Option<String>,
    #[arg(help = r#"Amount"#, long)]
    amount: Option<rust_decimal::Decimal>,
    #[arg(help = r#"JSON format questionnaire answers."#, long)]
    questionnaire: Option<String>,
    #[arg(
        help = r#"withdrawID defined by the client (i.e. client's internal withdrawID)"#,
        long
    )]
    withdraw_order_id: Option<String>,
    #[arg(help = r#"Withdrawal network"#, long)]
    network: Option<String>,
    #[arg(
        help = r#"Secondary address identifier for coins like XRP,XMR etc."#,
        long
    )]
    address_tag: Option<String>,
    #[arg(help = r#"When making internal transfer, `true` for returning the fee to the destination account; `false` for
returning the fee back to the departure account. Default `false`."#, long, num_args = 0..=1, default_missing_value = "true")]
    transaction_fee_flag: Option<bool>,
    #[arg(help = r#""#, long)]
    name: Option<String>,
    #[arg(
        help = r#"The wallet type for withdraw，0-spot wallet ，1-funding wallet. Default walletType is the current
"selected wallet" under wallet->Fiat and Spot/Funding->Deposit"#,
        long
    )]
    wallet_type: Option<i64>,
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
pub enum WalletCommands {
    #[command(
        about = decode_selected_entities(r#"Fetch account api trading status detail.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    AccountApiTradingStatus(AccountApiTradingStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch account info detail.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    AccountInfo(AccountInfoArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch account status detail.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    AccountStatus(AccountStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Daily account snapshot

Weight(IP): 2400

Security Type: USER_DATA

Notes:
- The query time period must be less then 30 days
- Support query within the last one month only
- If startTimeand endTime not sent, return records of the last 7 days by default"#, false),
    )]
    DailyAccountSnapshot(DailyAccountSnapshotArgs),
    #[command(
        about = decode_selected_entities(r#"Disable Fast Withdraw Switch

Weight(IP): 1

Security Type: USER_DATA

Notes:
- This request will disable fastwithdraw switch under your account. You need to enable "trade" option for the api key which requests this endpoint."#, false),
    )]
    DisableFastWithdrawSwitch(DisableFastWithdrawSwitchArgs),
    #[command(
        about = decode_selected_entities(r#"Enable Fast Withdraw Switch (USER_DATA)

Weight(IP): 1

Security Type: USER_DATA

Notes:
- This request will enable fastwithdraw switch under your account. You need to enable "trade" option for the api key which requests this endpoint.
- When Fast Withdraw Switch is on, transferring funds to a Binance account will be done instantly. There is no on-chain transaction, no transaction ID and no withdrawal fee."#, false),
    )]
    EnableFastWithdrawSwitch(EnableFastWithdrawSwitchArgs),
    #[command(
        about = decode_selected_entities(r#"Get API Key Permission

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetApiKeyPermission(GetApiKeyPermissionArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch details of assets supported on Binance.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Please get network and other deposit or withdraw details from `GET /sapi/v1/capital/config/getall`."#, false),
    )]
    AssetDetail(AssetDetailArgs),
    #[command(
        about = decode_selected_entities(r#"Query asset dividend record.

Weight(IP): 10

Security Type: USER_DATA

Notes:
- There cannot be more than 180 days between parameter `startTime` and `endTime`."#, false),
    )]
    AssetDividendRecord(AssetDividendRecordArgs),
    #[command(
        about = decode_selected_entities(r#"Convert dust assets

Weight(UID): 10

Security Type: USER_DATA"#, false),
    )]
    DustConvert(DustConvertArgs),
    #[command(
        about = decode_selected_entities(r#"Query dust convertible assets

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    DustConvertibleAssets(DustConvertibleAssetsArgs),
    #[command(
        about = decode_selected_entities(r#"Convert dust assets to BNB.

Weight(UID): 10

Security Type: USER_DATA

Notes:
- You need to open`Enable Spot & Margin Trading` permission for the API Key which requests this endpoint."#, false),
    )]
    DustTransfer(DustTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Dustlog

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Only return last 100 records
- Only return records after 2020/12/01"#, false),
    )]
    Dustlog(DustlogArgs),
    #[command(
        about = decode_selected_entities(r#"Query Funding Wallet

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Currently supports querying the following business assets：Binance Pay, Binance Card, Binance Gift Card, Stock Token"#, false),
    )]
    FundingWallet(FundingWalletArgs),
    #[command(
        about = decode_selected_entities(r#"Get Assets That Can Be Converted Into BNB

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetAssetsThatCanBeConvertedIntoBnb(GetAssetsThatCanBeConvertedIntoBnbArgs),
    #[command(
        about = decode_selected_entities(r#"The query of Cloud-Mining payment and refund history

Weight(UID): 600

Security Type: USER_DATA

Notes:
- Just return the SUCCESS records of payment and refund.
- For response, type = 248 means payment, type = 249 means refund, status =S means SUCCESS."#, false),
    )]
    GetCloudMiningPaymentAndRefundHistory(GetCloudMiningPaymentAndRefundHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get the list of symbols that are scheduled to be opened for trading in
the market.

Weight(IP): 100

Security Type: MARKET_DATA"#, false),
    )]
    GetOpenSymbolList(GetOpenSymbolListArgs),
    #[command(
        about = decode_selected_entities(r#"Query User Delegation History

Weight(IP): 60

Security Type: USER_DATA"#, false),
    )]
    QueryUserDelegationHistory(QueryUserDelegationHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query User Universal Transfer History

Weight(IP): 1

Security Type: USER_DATA

Notes:
- `fromSymbol` must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
- `toSymbol` must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
- Support query within the last 6 months only
- If `startTime`and `endTime` not sent, return records of the last 7 days by default"#, false),
    )]
    QueryUserUniversalTransferHistory(QueryUserUniversalTransferHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Query User Wallet Balance

Weight(IP): 60

Security Type: USER_DATA"#, false),
    )]
    QueryUserWalletBalance(QueryUserWalletBalanceArgs),
    #[command(
        about = decode_selected_entities(r#"Toggle BNB Burn On Spot Trade And Margin Interest

Weight(IP): 1

Security Type: USER_DATA

Notes:
- "spotBNBBurn" and "interestBNBBurn" should be sent at least one."#, false),
    )]
    ToggleBnbBurnOnSpotTradeAndMarginInterest(ToggleBnbBurnOnSpotTradeAndMarginInterestArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch trade fee

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    TradeFee(TradeFeeArgs),
    #[command(
        about = decode_selected_entities(r#"Get user assets, just for positive data.

Weight(IP): 5

Security Type: USER_DATA

Notes:
- If asset is set, then return this asset, otherwise return all assets positive.
- If needBtcValuation is set, then return btcValudation."#, false),
    )]
    UserAsset(UserAssetArgs),
    #[command(
        about = decode_selected_entities(r#"User universal transfer

Weight(UID): 900

Security Type: USER_DATA

Notes:
- You need to enable Permits Universal Transfer option for the API Key that requests this endpoint.
- `fromSymbol` must be sent when type is `ISOLATEDMARGIN_MARGIN` or `ISOLATEDMARGIN_ISOLATEDMARGIN`.
- `toSymbol` must be sent when type is `MARGIN_ISOLATEDMARGIN` or `ISOLATEDMARGIN_ISOLATEDMARGIN`.
- ENUM of transfer types:
- `MAIN_UMFUTURE`: Spot → USDⓈ-M Futures
- `MAIN_CMFUTURE`: Spot → COIN-M Futures
- `MAIN_MARGIN`: Spot → Margin (cross)
- `UMFUTURE_MAIN`: USDⓈ-M Futures → Spot
- `UMFUTURE_MARGIN`: USDⓈ-M Futures → Margin (cross)
- `CMFUTURE_MAIN`: COIN-M Futures → Spot
- `CMFUTURE_MARGIN`: COIN-M Futures → Margin (cross)
- `MARGIN_MAIN`: Margin (cross) → Spot
- `MARGIN_UMFUTURE`: Margin (cross) → USDⓈ-M Futures
- `MARGIN_CMFUTURE`: Margin (cross) → COIN-M Futures
- `ISOLATEDMARGIN_MARGIN`: Isolated margin → Margin (cross)
- `MARGIN_ISOLATEDMARGIN`: Margin (cross) → Isolated margin
- `ISOLATEDMARGIN_ISOLATEDMARGIN`: Isolated margin → Isolated margin
- `MAIN_FUNDING`: Spot → Funding
- `FUNDING_MAIN`: Funding → Spot
- `FUNDING_UMFUTURE`: Funding → USDⓈ-M Futures
- `UMFUTURE_FUNDING`: USDⓈ-M Futures → Funding
- `MARGIN_FUNDING`: Margin (cross) → Funding
- `FUNDING_MARGIN`: Funding → Margin (cross)
- `FUNDING_CMFUTURE`: Funding → COIN-M Futures
- `CMFUTURE_FUNDING`: COIN-M Futures → Funding
- `MAIN_OPTION`: Spot → Options
- `OPTION_MAIN`: Options → Spot
- `UMFUTURE_OPTION`: USDⓈ-M Futures → Options
- `OPTION_UMFUTURE`: Options → USDⓈ-M Futures
- `MARGIN_OPTION`: Margin (cross) → Options
- `OPTION_MARGIN`: Options → Margin (cross)
- `FUNDING_OPTION`: Funding → Options
- `OPTION_FUNDING`: Options → Funding
- `MAIN_PORTFOLIO_MARGIN`: Spot → Portfolio Margin
- `PORTFOLIO_MARGIN_MAIN`: Portfolio Margin → Spot"#, false),
    )]
    UserUniversalTransfer(UserUniversalTransferArgs),
    #[command(
        about = decode_selected_entities(r#"Get information of coins (available for deposit and withdraw) for user.

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    AllCoinsInformation(AllCoinsInformationArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch deposit address with network.

Weight(IP): 10

Security Type: USER_DATA

Notes:
- If `network` is not send, return with default network of the coin.
- You can get `network` and `isDefault` in `networkList` in the response of `Get /sapi/v1/capital/config/getall (HMAC SHA256)`.
- `amount` needs to be sent if using LIGHTNING network"#, false),
    )]
    DepositAddress(DepositAddressArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch deposit history.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days.
- If both `startTime` and `endTime` are sent, time between `startTime` and `endTime` must be less than 90 days."#, false),
    )]
    DepositHistory(DepositHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch deposit address list with network.

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    FetchDepositAddressListWithNetwork(FetchDepositAddressListWithNetworkArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch withdraw address list

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    FetchWithdrawAddressList(FetchWithdrawAddressListArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch withdraw quota

Weight(IP): 10

Security Type: USER_DATA"#, false),
    )]
    FetchWithdrawQuota(FetchWithdrawQuotaArgs),
    #[command(
        about = decode_selected_entities(r#"Apply deposit credit for expired address (One click arrival)

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    OneClickArrivalDepositApply(OneClickArrivalDepositApplyArgs),
    #[command(
        about = decode_selected_entities(r#"Submit a withdraw request

Weight(UID): 900

Security Type: USER_DATA

Notes:
- If `network` not send, return with default network of the coin.
- You can get `network` and `isDefault` in `networkList` of a coin in the response of `Get /sapi/v1/capital/config/getall (HMAC SHA256)`.
- To check if travel rule is required, by using `GET /sapi/v1/localentity/questionnaire-requirements` and if it returns anything other than `NIL` you will need update SAPI to `POST /sapi/v1/localentity/withdraw/apply` else you can continue `POST /sapi/v1/capital/withdraw/apply`. Please note that if you are required to comply to travel rule please refer to the Travel Rule SAPI.
- "For networks that do not support memo/tag, submitting a withdrawal request with a non-empty `addressTag` will return error `-4106 TAG_NOT_SUPPORTED_FOR_NETWORK`. Please omit the `addressTag` field for such networks. You can check whether a network requires a tag via `GET /sapi/v1/capital/config/getall`: If `withdrawTag` = `true` → memo/tag is required. If `withdrawTag` = `false` → memo/tag is not supported; omit `addressTag`.""#, false),
    )]
    Withdraw(WithdrawArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch withdraw history

Weight(UID): 18000 (10 requests per second)

Security Type: USER_DATA

Notes:
- `network` may not be in the response for old withdraw.
- Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days.
- If both `startTime` and `endTime`are sent, time between `startTime`and `endTime`must be less than 90 days.
- If `withdrawOrderId` is sent, time between `startTime` and `endTime` must be less than 7 days.
- If `withdrawOrderId` is sent, `startTime` and `endTime` are not sent, will return last 7 days records by default.
- Maximum support `idList` number is 45."#, false),
    )]
    WithdrawHistory(WithdrawHistoryArgs),
    #[command(
        about = decode_selected_entities(r#"Get symbols delist schedule for spot

Weight(IP): 100

Security Type: MARKET_DATA"#, false),
    )]
    GetSymbolsDelistScheduleForSpot(GetSymbolsDelistScheduleForSpotArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch system status.

Weight(IP): 1

Security Type: System"#, false),
    )]
    SystemStatus(SystemStatusArgs),
    #[command(
        about = decode_selected_entities(r#"Submit a withdrawal request for brokers of local entities that required travel rule.

Weight(UID): 600

Security Type: USER_DATA

Notes:
- If `network` not send, return with default network of the coin, but if the address could not match default network, the withdraw will be rejected.
- You can get `network` in `networkList` of a coin in the response of `Get /sapi/v1/capital/config/getall (HMAC SHA256)`.
- Questionnaire is different for each local entity, please refer to the `Withdraw Questionnaire Contents` page.
- If getting error like `Questionnaire format not valid.` or `Questionnaire must not be blank`, please try to verify the format of the questionnaire and use URL-encoded format."#, false),
    )]
    BrokerWithdraw(BrokerWithdrawArgs),
    #[command(
        about = decode_selected_entities(r#"This API will return user-specific Travel Rule questionnaire requirement information in reference to the current
API key.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    CheckQuestionnaireRequirements(CheckQuestionnaireRequirementsArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch deposit history for local entities that required travel rule.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days.
- If both `startTime` and `endTime` are sent, time between `startTime` and `endTime` must be less than 90 days.
- Please, note that due to network-specific characteristics, the returned source address may be inaccurate. If multiple source addresses are found, only the first one will be returned."#, false),
    )]
    DepositHistoryTravelRule(DepositHistoryTravelRuleArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch deposit history for local entities that with required travel rule information.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days.
- If both `startTime` and `endTime` are sent, time between `startTime` and `endTime` must be less than 90 days.
- Please, note that due to network-specific characteristics, the returned source address may be inaccurate. If multiple source addresses are found, only the first one will be returned."#, false),
    )]
    DepositHistoryV2(DepositHistoryV2Args),
    #[command(
        about = decode_selected_entities(r#"Fetch address verification list for user to check on status and other details for the addresses stored in
Address Book.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    FetchAddressVerificationList(FetchAddressVerificationListArgs),
    #[command(
        about = decode_selected_entities(r#"Query the active country list for travel rule questionnaires. Currently, only supports AU entity.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetCountryList(GetCountryListArgs),
    #[command(
        about = decode_selected_entities(r#"Query the active region/city list for a given country. Currently, only supports AU entity.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    GetRegionList(GetRegionListArgs),
    #[command(
        about = decode_selected_entities(r#"Submit questionnaire for brokers of local entities that require travel rule.
The questionnaire is only applies to transactions from un-hosted wallets or VASPs that are not
yet onboarded with GTR.

Weight(UID): 600

Security Type: USER_DATA

Notes:
- Questionnaire is different for each local entity, please refer to `Deposit Questionnaire Content` page.
- If getting error like `Questionnaire format not valid.` or `Questionnaire must not be blank`, please try to verify the format of the questionnaire and use URL-encoded format."#, false),
    )]
    SubmitDepositQuestionnaire(SubmitDepositQuestionnaireArgs),
    #[command(
        about = decode_selected_entities(r#"Submit questionnaire for local entities that require travel rule.
The questionnaire is only applies to transactions from unhosted wallets or VASPs that are not
yet onboarded with GTR.

Weight(UID): 600

Security Type: USER_DATA

Notes:
- Questionnaire is different for each local entity, please refer to `Deposit Questionnaire Content` page.
- If getting error like `Questionnaire format not valid.` or `Questionnaire must not be blank`, please try to verify the format of the questionnaire and use URL-encoded format."#, false),
    )]
    SubmitDepositQuestionnaireTravelRule(SubmitDepositQuestionnaireTravelRuleArgs),
    #[command(
        about = decode_selected_entities(r#"Submit questionnaire for local entities that require travel rule.
The questionnaire is only applies to transactions from unhosted wallets or VASPs that are not
yet onboarded with GTR.

Weight(UID): 600

Security Type: USER_DATA

Notes:
- Questionnaire is different for each local entity, please refer to `Deposit Questionnaire Content` page.
- If getting error like `Questionnaire format not valid.` or `Questionnaire must not be blank`, please try to verify the format of the questionnaire and use URL-encoded format."#, false),
    )]
    SubmitDepositQuestionnaireV2(SubmitDepositQuestionnaireV2Args),
    #[command(
        about = decode_selected_entities(r#"Fetch the VASP list for local entities.

Weight(IP): 1

Security Type: USER_DATA"#, false),
    )]
    VaspList(VaspListArgs),
    #[command(
        about = decode_selected_entities(r#"Fetch withdraw history for local entities that required travel rule.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- `network` may not be in the response for old withdraw.
- Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days.
- If both `startTime` and `endTime`are sent, time between `startTime`and `endTime`must be less than 90 days."#, false),
    )]
    WithdrawHistoryV1(WithdrawHistoryV1Args),
    #[command(
        about = decode_selected_entities(r#"Fetch withdraw history for local entities that required travel rule.

Weight(IP): 1

Security Type: USER_DATA

Notes:
- `network` may not be in the response for old withdraw.
- Withdrawal made through /sapi/v1/capital/withdraw/apply may not be in the response.
- Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days.
- If both `startTime` and `endTime`are sent, time between `startTime`and `endTime`must be less than 90 days.
- If withdrawOrderId is sent, time between startTime and endTime must be less than 7 days.
- If withdrawOrderId is sent, startTime and endTime are not sent, will return last 7 days records by default.
- Maximum support trId,txId number is 45.
- WithdrawOrderId only support 1.
- If responsible does not include withdrawalStatus, please input trId or txId retrieve the data."#, false),
    )]
    WithdrawHistoryV2(WithdrawHistoryV2Args),
    #[command(
        about = decode_selected_entities(r#"Submit a withdrawal request for local entities that required travel rule.

Weight(UID): 600

Security Type: USER_DATA

Notes:
- If `network` not send, return with default network of the coin, but if the address could not match default network, the withdraw will be rejected.
- You can get `network` and `isDefault` in `networkList` of a coin in the response of `Get /sapi/v1/capital/config/getall (HMAC SHA256)`.
- Questionnaire is different for each local entity, please refer to the `Withdraw Questionnaire Contents` page.
- If getting error like `Questionnaire format not valid.` or `Questionnaire must not be blank`, please try to verify the format of the questionnaire and use URL-encoded format."#, false),
    )]
    WithdrawTravelRule(WithdrawTravelRuleArgs),
}

pub async fn handle_wallet_command(command: WalletCommands) -> anyhow::Result<()> {
    match command {
        WalletCommands::AccountApiTradingStatus(args) => account_api_trading_status(args).await,

        WalletCommands::AccountInfo(args) => account_info(args).await,

        WalletCommands::AccountStatus(args) => account_status(args).await,

        WalletCommands::DailyAccountSnapshot(args) => daily_account_snapshot(args).await,

        WalletCommands::DisableFastWithdrawSwitch(args) => disable_fast_withdraw_switch(args).await,

        WalletCommands::EnableFastWithdrawSwitch(args) => enable_fast_withdraw_switch(args).await,

        WalletCommands::GetApiKeyPermission(args) => get_api_key_permission(args).await,

        WalletCommands::AssetDetail(args) => asset_detail(args).await,

        WalletCommands::AssetDividendRecord(args) => asset_dividend_record(args).await,

        WalletCommands::DustConvert(args) => dust_convert(args).await,

        WalletCommands::DustConvertibleAssets(args) => dust_convertible_assets(args).await,

        WalletCommands::DustTransfer(args) => dust_transfer(args).await,

        WalletCommands::Dustlog(args) => dustlog(args).await,

        WalletCommands::FundingWallet(args) => funding_wallet(args).await,

        WalletCommands::GetAssetsThatCanBeConvertedIntoBnb(args) => {
            get_assets_that_can_be_converted_into_bnb(args).await
        }

        WalletCommands::GetCloudMiningPaymentAndRefundHistory(args) => {
            get_cloud_mining_payment_and_refund_history(args).await
        }

        WalletCommands::GetOpenSymbolList(args) => get_open_symbol_list(args).await,

        WalletCommands::QueryUserDelegationHistory(args) => {
            query_user_delegation_history(args).await
        }

        WalletCommands::QueryUserUniversalTransferHistory(args) => {
            query_user_universal_transfer_history(args).await
        }

        WalletCommands::QueryUserWalletBalance(args) => query_user_wallet_balance(args).await,

        WalletCommands::ToggleBnbBurnOnSpotTradeAndMarginInterest(args) => {
            toggle_bnb_burn_on_spot_trade_and_margin_interest(args).await
        }

        WalletCommands::TradeFee(args) => trade_fee(args).await,

        WalletCommands::UserAsset(args) => user_asset(args).await,

        WalletCommands::UserUniversalTransfer(args) => user_universal_transfer(args).await,

        WalletCommands::AllCoinsInformation(args) => all_coins_information(args).await,

        WalletCommands::DepositAddress(args) => deposit_address(args).await,

        WalletCommands::DepositHistory(args) => deposit_history(args).await,

        WalletCommands::FetchDepositAddressListWithNetwork(args) => {
            fetch_deposit_address_list_with_network(args).await
        }

        WalletCommands::FetchWithdrawAddressList(args) => fetch_withdraw_address_list(args).await,

        WalletCommands::FetchWithdrawQuota(args) => fetch_withdraw_quota(args).await,

        WalletCommands::OneClickArrivalDepositApply(args) => {
            one_click_arrival_deposit_apply(args).await
        }

        WalletCommands::Withdraw(args) => withdraw(args).await,

        WalletCommands::WithdrawHistory(args) => withdraw_history(args).await,

        WalletCommands::GetSymbolsDelistScheduleForSpot(args) => {
            get_symbols_delist_schedule_for_spot(args).await
        }

        WalletCommands::SystemStatus(args) => system_status(args).await,

        WalletCommands::BrokerWithdraw(args) => broker_withdraw(args).await,

        WalletCommands::CheckQuestionnaireRequirements(args) => {
            check_questionnaire_requirements(args).await
        }

        WalletCommands::DepositHistoryTravelRule(args) => deposit_history_travel_rule(args).await,

        WalletCommands::DepositHistoryV2(args) => deposit_history_v2(args).await,

        WalletCommands::FetchAddressVerificationList(args) => {
            fetch_address_verification_list(args).await
        }

        WalletCommands::GetCountryList(args) => get_country_list(args).await,

        WalletCommands::GetRegionList(args) => get_region_list(args).await,

        WalletCommands::SubmitDepositQuestionnaire(args) => {
            submit_deposit_questionnaire(args).await
        }

        WalletCommands::SubmitDepositQuestionnaireTravelRule(args) => {
            submit_deposit_questionnaire_travel_rule(args).await
        }

        WalletCommands::SubmitDepositQuestionnaireV2(args) => {
            submit_deposit_questionnaire_v2(args).await
        }

        WalletCommands::VaspList(args) => vasp_list(args).await,

        WalletCommands::WithdrawHistoryV1(args) => withdraw_history_v1(args).await,

        WalletCommands::WithdrawHistoryV2(args) => withdraw_history_v2(args).await,

        WalletCommands::WithdrawTravelRule(args) => withdraw_travel_rule(args).await,
    }
}

async fn account_api_trading_status(args: AccountApiTradingStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountApiTradingStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountApiTradingStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountApiTradingStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_api_trading_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn account_info(args: AccountInfoArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountInfoParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountInfoParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountInfoParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_info(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn account_status(args: AccountStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AccountStatusParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AccountStatusParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AccountStatusParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.account_status(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn daily_account_snapshot(mut args: DailyAccountSnapshotArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DailyAccountSnapshotParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DailyAccountSnapshotParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.r#type.is_none() {
                        let options = vec![
                            ("SPOT", DailyAccountSnapshotTypeEnum::Spot),
                            ("MARGIN", DailyAccountSnapshotTypeEnum::Margin),
                            ("FUTURES", DailyAccountSnapshotTypeEnum::Futures),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the r#type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.r#type = Some(selected);
                    }
                }
                DailyAccountSnapshotParams::builder(
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.daily_account_snapshot(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn disable_fast_withdraw_switch(
    args: DisableFastWithdrawSwitchArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DisableFastWithdrawSwitchParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<DisableFastWithdrawSwitchParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => DisableFastWithdrawSwitchParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.disable_fast_withdraw_switch(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn enable_fast_withdraw_switch(args: EnableFastWithdrawSwitchArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<EnableFastWithdrawSwitchParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<EnableFastWithdrawSwitchParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => EnableFastWithdrawSwitchParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.enable_fast_withdraw_switch(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_api_key_permission(args: GetApiKeyPermissionArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetApiKeyPermissionParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetApiKeyPermissionParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetApiKeyPermissionParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_api_key_permission(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn asset_detail(args: AssetDetailArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AssetDetailParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AssetDetailParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AssetDetailParams::builder()
                .asset(args.asset)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.asset_detail(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn asset_dividend_record(args: AssetDividendRecordArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AssetDividendRecordParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AssetDividendRecordParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AssetDividendRecordParams::builder()
                .asset(args.asset)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.asset_dividend_record(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn dust_convert(mut args: DustConvertArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DustConvertParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DustConvertParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                }
                DustConvertParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .account_type(args.account_type)
                .client_id(args.client_id)
                .target_asset(args.target_asset)
                .third_party_client_id(args.third_party_client_id)
                .dust_quota_asset_to_target_asset_price(args.dust_quota_asset_to_target_asset_price)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.dust_convert(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn dust_convertible_assets(mut args: DustConvertibleAssetsArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DustConvertibleAssetsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DustConvertibleAssetsParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.target_asset.is_none() {
                        let target_asset: String = Input::new()
                            .with_prompt("Please enter the target_asset name")
                            .interact_text()?;

                        args.target_asset = Some(target_asset);
                    }
                }
                DustConvertibleAssetsParams::builder(
                    args.target_asset
                        .ok_or_else(|| anyhow::anyhow!("target_asset is required"))?,
                )
                .account_type(args.account_type)
                .dust_quota_asset_to_target_asset_price(args.dust_quota_asset_to_target_asset_price)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.dust_convertible_assets(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn dust_transfer(mut args: DustTransferArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DustTransferParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DustTransferParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                }
                DustTransferParams::builder(
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                )
                .account_type(args.account_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.dust_transfer(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn dustlog(args: DustlogArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DustlogParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DustlogParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => DustlogParams::builder()
                .account_type(args.account_type)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.dustlog(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn funding_wallet(args: FundingWalletArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FundingWalletParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FundingWalletParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => FundingWalletParams::builder()
                .asset(args.asset)
                .need_btc_valuation(args.need_btc_valuation)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.funding_wallet(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_assets_that_can_be_converted_into_bnb(
    args: GetAssetsThatCanBeConvertedIntoBnbArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetAssetsThatCanBeConvertedIntoBnbParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetAssetsThatCanBeConvertedIntoBnbParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => GetAssetsThatCanBeConvertedIntoBnbParams::builder()
                .account_type(args.account_type)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_assets_that_can_be_converted_into_bnb(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_cloud_mining_payment_and_refund_history(
    mut args: GetCloudMiningPaymentAndRefundHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCloudMiningPaymentAndRefundHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetCloudMiningPaymentAndRefundHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.start_time.is_none() {
                        let start_time: i64 = Input::new()
                            .with_prompt("Please enter the start_time name")
                            .interact_text()?;

                        args.start_time = Some(start_time);
                    }
                    if args.end_time.is_none() {
                        let end_time: i64 = Input::new()
                            .with_prompt("Please enter the end_time name")
                            .interact_text()?;

                        args.end_time = Some(end_time);
                    }
                }
                GetCloudMiningPaymentAndRefundHistoryParams::builder(
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                )
                .tran_id(args.tran_id)
                .client_tran_id(args.client_tran_id)
                .asset(args.asset)
                .current(args.current)
                .size(args.size)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .get_cloud_mining_payment_and_refund_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_open_symbol_list(args: GetOpenSymbolListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.get_open_symbol_list().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_user_delegation_history(
    mut args: QueryUserDelegationHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUserDelegationHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<QueryUserDelegationHistoryParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.email.is_none() {
                        let email: String = Input::new()
                            .with_prompt("Please enter the email name")
                            .interact_text()?;

                        args.email = Some(email);
                    }
                    if args.start_time.is_none() {
                        let start_time: i64 = Input::new()
                            .with_prompt("Please enter the start_time name")
                            .interact_text()?;

                        args.start_time = Some(start_time);
                    }
                    if args.end_time.is_none() {
                        let end_time: i64 = Input::new()
                            .with_prompt("Please enter the end_time name")
                            .interact_text()?;

                        args.end_time = Some(end_time);
                    }
                }
                QueryUserDelegationHistoryParams::builder(
                    args.email
                        .ok_or_else(|| anyhow::anyhow!("email is required"))?,
                    args.start_time
                        .ok_or_else(|| anyhow::anyhow!("start_time is required"))?,
                    args.end_time
                        .ok_or_else(|| anyhow::anyhow!("end_time is required"))?,
                )
                .r#type(args.r#type)
                .asset(args.asset)
                .current(args.current)
                .size(args.size)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.query_user_delegation_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_user_universal_transfer_history(
    mut args: QueryUserUniversalTransferHistoryArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUserUniversalTransferHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryUserUniversalTransferHistoryParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.r#type.is_none() {
                        let r#type: String = Input::new()
                            .with_prompt("Please enter the r#type name")
                            .interact_text()?;

                        args.r#type = Some(r#type);
                    }
                }
                QueryUserUniversalTransferHistoryParams::builder(
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                )
                .start_time(args.start_time)
                .end_time(args.end_time)
                .current(args.current)
                .size(args.size)
                .from_symbol(args.from_symbol)
                .to_symbol(args.to_symbol)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .query_user_universal_transfer_history(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn query_user_wallet_balance(args: QueryUserWalletBalanceArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<QueryUserWalletBalanceParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<QueryUserWalletBalanceParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => QueryUserWalletBalanceParams::builder()
                .quote_asset(args.quote_asset)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.query_user_wallet_balance(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn toggle_bnb_burn_on_spot_trade_and_margin_interest(
    args: ToggleBnbBurnOnSpotTradeAndMarginInterestArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<ToggleBnbBurnOnSpotTradeAndMarginInterestParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<ToggleBnbBurnOnSpotTradeAndMarginInterestParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => ToggleBnbBurnOnSpotTradeAndMarginInterestParams::builder()
                .spot_bnb_burn(args.spot_bnb_burn)
                .interest_bnb_burn(args.interest_bnb_burn)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .toggle_bnb_burn_on_spot_trade_and_margin_interest(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn trade_fee(args: TradeFeeArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<TradeFeeParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<TradeFeeParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => TradeFeeParams::builder()
                .symbol(args.symbol)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.trade_fee(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn user_asset(args: UserAssetArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UserAssetParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UserAssetParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => UserAssetParams::builder()
                .asset(args.asset)
                .need_btc_valuation(args.need_btc_valuation)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.user_asset(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn user_universal_transfer(mut args: UserUniversalTransferArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<UserUniversalTransferParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<UserUniversalTransferParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.r#type.is_none() {
                        let options = vec![
                            ("MAIN_UMFUTURE", UserUniversalTransferTypeEnum::MainUmfuture),
                            ("MAIN_CMFUTURE", UserUniversalTransferTypeEnum::MainCmfuture),
                            ("MAIN_MARGIN", UserUniversalTransferTypeEnum::MainMargin),
                            ("UMFUTURE_MAIN", UserUniversalTransferTypeEnum::UmfutureMain),
                            (
                                "UMFUTURE_MARGIN",
                                UserUniversalTransferTypeEnum::UmfutureMargin,
                            ),
                            ("CMFUTURE_MAIN", UserUniversalTransferTypeEnum::CmfutureMain),
                            (
                                "CMFUTURE_MARGIN",
                                UserUniversalTransferTypeEnum::CmfutureMargin,
                            ),
                            ("MARGIN_MAIN", UserUniversalTransferTypeEnum::MarginMain),
                            (
                                "MARGIN_UMFUTURE",
                                UserUniversalTransferTypeEnum::MarginUmfuture,
                            ),
                            (
                                "MARGIN_CMFUTURE",
                                UserUniversalTransferTypeEnum::MarginCmfuture,
                            ),
                            (
                                "ISOLATEDMARGIN_MARGIN",
                                UserUniversalTransferTypeEnum::IsolatedmarginMargin,
                            ),
                            (
                                "MARGIN_ISOLATEDMARGIN",
                                UserUniversalTransferTypeEnum::MarginIsolatedmargin,
                            ),
                            (
                                "ISOLATEDMARGIN_ISOLATEDMARGIN",
                                UserUniversalTransferTypeEnum::IsolatedmarginIsolatedmargin,
                            ),
                            ("MAIN_FUNDING", UserUniversalTransferTypeEnum::MainFunding),
                            ("FUNDING_MAIN", UserUniversalTransferTypeEnum::FundingMain),
                            (
                                "FUNDING_UMFUTURE",
                                UserUniversalTransferTypeEnum::FundingUmfuture,
                            ),
                            (
                                "UMFUTURE_FUNDING",
                                UserUniversalTransferTypeEnum::UmfutureFunding,
                            ),
                            (
                                "MARGIN_FUNDING",
                                UserUniversalTransferTypeEnum::MarginFunding,
                            ),
                            (
                                "FUNDING_MARGIN",
                                UserUniversalTransferTypeEnum::FundingMargin,
                            ),
                            (
                                "FUNDING_CMFUTURE",
                                UserUniversalTransferTypeEnum::FundingCmfuture,
                            ),
                            (
                                "CMFUTURE_FUNDING",
                                UserUniversalTransferTypeEnum::CmfutureFunding,
                            ),
                            ("MAIN_OPTION", UserUniversalTransferTypeEnum::MainOption),
                            ("OPTION_MAIN", UserUniversalTransferTypeEnum::OptionMain),
                            (
                                "UMFUTURE_OPTION",
                                UserUniversalTransferTypeEnum::UmfutureOption,
                            ),
                            (
                                "OPTION_UMFUTURE",
                                UserUniversalTransferTypeEnum::OptionUmfuture,
                            ),
                            ("MARGIN_OPTION", UserUniversalTransferTypeEnum::MarginOption),
                            ("OPTION_MARGIN", UserUniversalTransferTypeEnum::OptionMargin),
                            (
                                "FUNDING_OPTION",
                                UserUniversalTransferTypeEnum::FundingOption,
                            ),
                            (
                                "OPTION_FUNDING",
                                UserUniversalTransferTypeEnum::OptionFunding,
                            ),
                            (
                                "MAIN_PORTFOLIO_MARGIN",
                                UserUniversalTransferTypeEnum::MainPortfolioMargin,
                            ),
                            (
                                "PORTFOLIO_MARGIN_MAIN",
                                UserUniversalTransferTypeEnum::PortfolioMarginMain,
                            ),
                        ];

                        let labels: Vec<&str> = options.iter().map(|item| item.0).collect();

                        let selected = Select::new()
                            .with_prompt("Please select the r#type")
                            .items(&labels)
                            .default(0)
                            .interact()?;

                        let selected = options[selected].1.clone();

                        println!("Selected option: {:?}", selected);

                        args.r#type = Some(selected);
                    }
                    if args.asset.is_none() {
                        let asset: String = Input::new()
                            .with_prompt("Please enter the asset name")
                            .interact_text()?;

                        args.asset = Some(asset);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                UserUniversalTransferParams::builder(
                    args.r#type
                        .ok_or_else(|| anyhow::anyhow!("r#type is required"))?,
                    args.asset
                        .ok_or_else(|| anyhow::anyhow!("asset is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .from_symbol(args.from_symbol)
                .to_symbol(args.to_symbol)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.user_universal_transfer(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn all_coins_information(args: AllCoinsInformationArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<AllCoinsInformationParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<AllCoinsInformationParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => AllCoinsInformationParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.all_coins_information(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn deposit_address(mut args: DepositAddressArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DepositAddressParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DepositAddressParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.coin.is_none() {
                        let coin: String = Input::new()
                            .with_prompt("Please enter the coin name")
                            .interact_text()?;

                        args.coin = Some(coin);
                    }
                }
                DepositAddressParams::builder(
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                )
                .network(args.network)
                .amount(args.amount)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.deposit_address(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn deposit_history(args: DepositHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DepositHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DepositHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => DepositHistoryParams::builder()
                .include_source(args.include_source)
                .coin(args.coin)
                .status(args.status)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .offset(args.offset)
                .limit(args.limit)
                .recv_window(args.recv_window)
                .tx_id(args.tx_id)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.deposit_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fetch_deposit_address_list_with_network(
    mut args: FetchDepositAddressListWithNetworkArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FetchDepositAddressListWithNetworkParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<FetchDepositAddressListWithNetworkParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.coin.is_none() {
                        let coin: String = Input::new()
                            .with_prompt("Please enter the coin name")
                            .interact_text()?;

                        args.coin = Some(coin);
                    }
                }
                FetchDepositAddressListWithNetworkParams::builder(
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                )
                .network(args.network)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .fetch_deposit_address_list_with_network(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fetch_withdraw_address_list(args: FetchWithdrawAddressListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    // Make the API call
    let response = rest_client.fetch_withdraw_address_list().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fetch_withdraw_quota(args: FetchWithdrawQuotaArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    // Make the API call
    let response = rest_client.fetch_withdraw_quota().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn one_click_arrival_deposit_apply(
    args: OneClickArrivalDepositApplyArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<OneClickArrivalDepositApplyParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<OneClickArrivalDepositApplyParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => OneClickArrivalDepositApplyParams::builder()
                .deposit_id(args.deposit_id)
                .tx_id(args.tx_id)
                .sub_account_id(args.sub_account_id)
                .sub_user_id(args.sub_user_id)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.one_click_arrival_deposit_apply(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn withdraw(mut args: WithdrawArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<WithdrawParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<WithdrawParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.coin.is_none() {
                        let coin: String = Input::new()
                            .with_prompt("Please enter the coin name")
                            .interact_text()?;

                        args.coin = Some(coin);
                    }
                    if args.address.is_none() {
                        let address: String = Input::new()
                            .with_prompt("Please enter the address name")
                            .interact_text()?;

                        args.address = Some(address);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                }
                WithdrawParams::builder(
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                    args.address
                        .ok_or_else(|| anyhow::anyhow!("address is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                )
                .withdraw_order_id(args.withdraw_order_id)
                .network(args.network)
                .address_tag(args.address_tag)
                .transaction_fee_flag(args.transaction_fee_flag)
                .name(args.name)
                .wallet_type(args.wallet_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.withdraw(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn withdraw_history(args: WithdrawHistoryArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<WithdrawHistoryParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<WithdrawHistoryParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => WithdrawHistoryParams::builder()
                .coin(args.coin)
                .withdraw_order_id(args.withdraw_order_id)
                .status(args.status)
                .offset(args.offset)
                .limit(args.limit)
                .id_list(args.id_list)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.withdraw_history(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_symbols_delist_schedule_for_spot(
    args: GetSymbolsDelistScheduleForSpotArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    let params = match read_stdin_as::<GetSymbolsDelistScheduleForSpotParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<GetSymbolsDelistScheduleForSpotParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => GetSymbolsDelistScheduleForSpotParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client
        .get_symbols_delist_schedule_for_spot(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn system_status(args: SystemStatusArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), false)?;

    // Make the API call
    let response = rest_client.system_status().await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn broker_withdraw(mut args: BrokerWithdrawArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<BrokerWithdrawParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<BrokerWithdrawParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.address.is_none() {
                        let address: String = Input::new()
                            .with_prompt("Please enter the address name")
                            .interact_text()?;

                        args.address = Some(address);
                    }
                    if args.coin.is_none() {
                        let coin: String = Input::new()
                            .with_prompt("Please enter the coin name")
                            .interact_text()?;

                        args.coin = Some(coin);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.withdraw_order_id.is_none() {
                        let withdraw_order_id: String = Input::new()
                            .with_prompt("Please enter the withdraw_order_id name")
                            .interact_text()?;

                        args.withdraw_order_id = Some(withdraw_order_id);
                    }
                    if args.questionnaire.is_none() {
                        let questionnaire: String = Input::new()
                            .with_prompt("Please enter the questionnaire name")
                            .interact_text()?;

                        args.questionnaire = Some(questionnaire);
                    }
                    if args.originator_pii.is_none() {
                        let originator_pii: String = Input::new()
                            .with_prompt("Please enter the originator_pii name")
                            .interact_text()?;

                        args.originator_pii = Some(originator_pii);
                    }
                }
                BrokerWithdrawParams::builder(
                    args.address
                        .ok_or_else(|| anyhow::anyhow!("address is required"))?,
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    args.withdraw_order_id
                        .ok_or_else(|| anyhow::anyhow!("withdraw_order_id is required"))?,
                    args.questionnaire
                        .ok_or_else(|| anyhow::anyhow!("questionnaire is required"))?,
                    args.originator_pii
                        .ok_or_else(|| anyhow::anyhow!("originator_pii is required"))?,
                )
                .address_tag(args.address_tag)
                .network(args.network)
                .address_name(args.address_name)
                .transaction_fee_flag(args.transaction_fee_flag)
                .wallet_type(args.wallet_type)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.broker_withdraw(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn check_questionnaire_requirements(
    args: CheckQuestionnaireRequirementsArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<CheckQuestionnaireRequirementsParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<CheckQuestionnaireRequirementsParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => CheckQuestionnaireRequirementsParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.check_questionnaire_requirements(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn deposit_history_travel_rule(args: DepositHistoryTravelRuleArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DepositHistoryTravelRuleParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<DepositHistoryTravelRuleParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => DepositHistoryTravelRuleParams::builder()
                .tr_id(args.tr_id)
                .tx_id(args.tx_id)
                .tran_id(args.tran_id)
                .network(args.network)
                .coin(args.coin)
                .travel_rule_status(args.travel_rule_status)
                .pending_questionnaire(args.pending_questionnaire)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .offset(args.offset)
                .limit(args.limit)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.deposit_history_travel_rule(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn deposit_history_v2(args: DepositHistoryV2Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<DepositHistoryV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<DepositHistoryV2Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => DepositHistoryV2Params::builder()
                .deposit_id(args.deposit_id)
                .tx_id(args.tx_id)
                .network(args.network)
                .coin(args.coin)
                .retrieve_questionnaire(args.retrieve_questionnaire)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .offset(args.offset)
                .limit(args.limit)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.deposit_history_v2(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn fetch_address_verification_list(
    args: FetchAddressVerificationListArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<FetchAddressVerificationListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<FetchAddressVerificationListParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => FetchAddressVerificationListParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.fetch_address_verification_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_country_list(args: GetCountryListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetCountryListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetCountryListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => GetCountryListParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.get_country_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn get_region_list(mut args: GetRegionListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<GetRegionListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<GetRegionListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.country_code.is_none() {
                        let country_code: String = Input::new()
                            .with_prompt("Please enter the country_code name")
                            .interact_text()?;

                        args.country_code = Some(country_code);
                    }
                }
                GetRegionListParams::builder(
                    args.country_code
                        .ok_or_else(|| anyhow::anyhow!("country_code is required"))?,
                )
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.get_region_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn submit_deposit_questionnaire(
    mut args: SubmitDepositQuestionnaireArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubmitDepositQuestionnaireParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SubmitDepositQuestionnaireParams>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.sub_account_id.is_none() {
                        let sub_account_id: String = Input::new()
                            .with_prompt("Please enter the sub_account_id name")
                            .interact_text()?;

                        args.sub_account_id = Some(sub_account_id);
                    }
                    if args.deposit_id.is_none() {
                        let deposit_id: i64 = Input::new()
                            .with_prompt("Please enter the deposit_id name")
                            .interact_text()?;

                        args.deposit_id = Some(deposit_id);
                    }
                    if args.questionnaire.is_none() {
                        let questionnaire: String = Input::new()
                            .with_prompt("Please enter the questionnaire name")
                            .interact_text()?;

                        args.questionnaire = Some(questionnaire);
                    }
                    if args.beneficiary_pii.is_none() {
                        let beneficiary_pii: String = Input::new()
                            .with_prompt("Please enter the beneficiary_pii name")
                            .interact_text()?;

                        args.beneficiary_pii = Some(beneficiary_pii);
                    }
                }
                SubmitDepositQuestionnaireParams::builder(
                    args.sub_account_id
                        .ok_or_else(|| anyhow::anyhow!("sub_account_id is required"))?,
                    args.deposit_id
                        .ok_or_else(|| anyhow::anyhow!("deposit_id is required"))?,
                    args.questionnaire
                        .ok_or_else(|| anyhow::anyhow!("questionnaire is required"))?,
                    args.beneficiary_pii
                        .ok_or_else(|| anyhow::anyhow!("beneficiary_pii is required"))?,
                )
                .network(args.network)
                .coin(args.coin)
                .amount(args.amount)
                .address(args.address)
                .address_tag(args.address_tag)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.submit_deposit_questionnaire(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn submit_deposit_questionnaire_travel_rule(
    mut args: SubmitDepositQuestionnaireTravelRuleArgs,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubmitDepositQuestionnaireTravelRuleParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<SubmitDepositQuestionnaireTravelRuleParams>(json)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?,
            None => {
                if args.interactive {
                    if args.tran_id.is_none() {
                        let tran_id: i64 = Input::new()
                            .with_prompt("Please enter the tran_id name")
                            .interact_text()?;

                        args.tran_id = Some(tran_id);
                    }
                    if args.questionnaire.is_none() {
                        let questionnaire: String = Input::new()
                            .with_prompt("Please enter the questionnaire name")
                            .interact_text()?;

                        args.questionnaire = Some(questionnaire);
                    }
                }
                SubmitDepositQuestionnaireTravelRuleParams::builder(
                    args.tran_id
                        .ok_or_else(|| anyhow::anyhow!("tran_id is required"))?,
                    args.questionnaire
                        .ok_or_else(|| anyhow::anyhow!("questionnaire is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client
        .submit_deposit_questionnaire_travel_rule(params)
        .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn submit_deposit_questionnaire_v2(
    mut args: SubmitDepositQuestionnaireV2Args,
) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<SubmitDepositQuestionnaireV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => {
                read_json_as::<SubmitDepositQuestionnaireV2Params>(json).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
                })?
            }
            None => {
                if args.interactive {
                    if args.deposit_id.is_none() {
                        let deposit_id: i64 = Input::new()
                            .with_prompt("Please enter the deposit_id name")
                            .interact_text()?;

                        args.deposit_id = Some(deposit_id);
                    }
                    if args.questionnaire.is_none() {
                        let questionnaire: String = Input::new()
                            .with_prompt("Please enter the questionnaire name")
                            .interact_text()?;

                        args.questionnaire = Some(questionnaire);
                    }
                }
                SubmitDepositQuestionnaireV2Params::builder(
                    args.deposit_id
                        .ok_or_else(|| anyhow::anyhow!("deposit_id is required"))?,
                    args.questionnaire
                        .ok_or_else(|| anyhow::anyhow!("questionnaire is required"))?,
                )
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.submit_deposit_questionnaire_v2(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn vasp_list(args: VaspListArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<VaspListParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<VaspListParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => VaspListParams::builder()
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.vasp_list(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn withdraw_history_v1(args: WithdrawHistoryV1Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<WithdrawHistoryV1Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<WithdrawHistoryV1Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => WithdrawHistoryV1Params::builder()
                .tr_id(args.tr_id)
                .tx_id(args.tx_id)
                .withdraw_order_id(args.withdraw_order_id)
                .network(args.network)
                .coin(args.coin)
                .travel_rule_status(args.travel_rule_status)
                .offset(args.offset)
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.withdraw_history_v1(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn withdraw_history_v2(args: WithdrawHistoryV2Args) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<WithdrawHistoryV2Params>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<WithdrawHistoryV2Params>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => WithdrawHistoryV2Params::builder()
                .tr_id(args.tr_id)
                .tx_id(args.tx_id)
                .withdraw_order_id(args.withdraw_order_id)
                .network(args.network)
                .coin(args.coin)
                .travel_rule_status(args.travel_rule_status)
                .offset(args.offset)
                .limit(args.limit)
                .start_time(args.start_time)
                .end_time(args.end_time)
                .recv_window(args.recv_window)
                .build()?,
        },
    };

    // Make the API call
    let response = rest_client.withdraw_history_v2(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

async fn withdraw_travel_rule(mut args: WithdrawTravelRuleArgs) -> anyhow::Result<()> {
    let rest_client = get_client(args.profile.as_deref(), true)?;

    let params = match read_stdin_as::<WithdrawTravelRuleParams>() {
        Some(params) => params,
        None => match args.json {
            Some(json) => read_json_as::<WithdrawTravelRuleParams>(json).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse json params")
            })?,
            None => {
                if args.interactive {
                    if args.coin.is_none() {
                        let coin: String = Input::new()
                            .with_prompt("Please enter the coin name")
                            .interact_text()?;

                        args.coin = Some(coin);
                    }
                    if args.address.is_none() {
                        let address: String = Input::new()
                            .with_prompt("Please enter the address name")
                            .interact_text()?;

                        args.address = Some(address);
                    }
                    if args.amount.is_none() {
                        let amount: rust_decimal::Decimal = Input::new()
                            .with_prompt("Please enter the amount name")
                            .interact_text()?;

                        args.amount = Some(amount);
                    }
                    if args.questionnaire.is_none() {
                        let questionnaire: String = Input::new()
                            .with_prompt("Please enter the questionnaire name")
                            .interact_text()?;

                        args.questionnaire = Some(questionnaire);
                    }
                }
                WithdrawTravelRuleParams::builder(
                    args.coin
                        .ok_or_else(|| anyhow::anyhow!("coin is required"))?,
                    args.address
                        .ok_or_else(|| anyhow::anyhow!("address is required"))?,
                    args.amount
                        .ok_or_else(|| anyhow::anyhow!("amount is required"))?,
                    args.questionnaire
                        .ok_or_else(|| anyhow::anyhow!("questionnaire is required"))?,
                )
                .withdraw_order_id(args.withdraw_order_id)
                .network(args.network)
                .address_tag(args.address_tag)
                .transaction_fee_flag(args.transaction_fee_flag)
                .name(args.name)
                .wallet_type(args.wallet_type)
                .recv_window(args.recv_window)
                .build()?
            }
        },
    };

    // Make the API call
    let response = rest_client.withdraw_travel_rule(params).await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}
