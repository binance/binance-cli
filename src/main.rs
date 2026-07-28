mod algo;
mod alpha;
mod c2c;
mod convert;
mod copy_trading;
mod crypto_loan;
mod custom_requests;
mod derivatives_trading_coin_futures;
mod derivatives_trading_options;
mod derivatives_trading_portfolio_margin;
mod derivatives_trading_portfolio_margin_pro;
mod derivatives_trading_usds_futures;
mod dual_investment;
mod gift_card;
mod margin_trading;
mod mining;
mod pay;
mod profile;
mod rebate;
mod simple_earn;
mod spot;
mod staking;
mod sub_account;
mod utils;
mod vip_loan;
mod w3w_prediction;
mod wallet;

use crate::algo::{AlgoCommands, handle_algo_command};
use crate::alpha::{AlphaCommands, handle_alpha_command};
use crate::c2c::{C2CCommands, handle_c2c_command};
use crate::convert::{ConvertCommands, handle_convert_command};
use crate::copy_trading::{CopyTradingCommands, handle_copy_trading_command};
use crate::crypto_loan::{CryptoLoanCommands, handle_crypto_loan_command};
use crate::custom_requests::{CustomRequestCommand, handle_custom_request};
use crate::derivatives_trading_coin_futures::{
    DerivativesTradingCoinFuturesCommands, handle_derivatives_trading_coin_futures_command,
};
use crate::derivatives_trading_options::{
    DerivativesTradingOptionsCommands, handle_derivatives_trading_options_command,
};
use crate::derivatives_trading_portfolio_margin::{
    DerivativesTradingPortfolioMarginCommands, handle_derivatives_trading_portfolio_margin_command,
};
use crate::derivatives_trading_portfolio_margin_pro::{
    DerivativesTradingPortfolioMarginProCommands,
    handle_derivatives_trading_portfolio_margin_pro_command,
};
use crate::derivatives_trading_usds_futures::{
    DerivativesTradingUsdsFuturesCommands, handle_derivatives_trading_usds_futures_command,
};
use crate::dual_investment::{DualInvestmentCommands, handle_dual_investment_command};
use crate::gift_card::{GiftCardCommands, handle_gift_card_command};
use crate::margin_trading::{MarginTradingCommands, handle_margin_trading_command};
use crate::mining::{MiningCommands, handle_mining_command};
use crate::pay::{PayCommands, handle_pay_command};
use crate::rebate::{RebateCommands, handle_rebate_command};
use crate::simple_earn::{SimpleEarnCommands, handle_simple_earn_command};
use crate::spot::{SpotCommands, handle_spot_command};
use crate::staking::{StakingCommands, handle_staking_command};
use crate::sub_account::{SubAccountCommands, handle_sub_account_command};
use crate::vip_loan::{VIPLoanCommands, handle_vip_loan_command};
use crate::w3w_prediction::{W3WPredictionCommands, handle_w3w_prediction_command};
use crate::wallet::{WalletCommands, handle_wallet_command};

use crate::profile::{ProfileCommand, handle_profile_command};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};

#[derive(Parser)]
#[command(
    name = "binance-cli",
    after_help = "Environment Variables:\n  BINANCE_API_KEY\n  BINANCE_SECRET_KEY\n  BINANCE_API_ENV                   API Environment (prod | testnet | demo)\n  BINANCE_<PRODUCT>_BASE_PATH       Base path of the product (e.g. \"https://api.binance.com\" for Spot)"
)]
#[command(version)]
#[command(about = "A simple CLI that interacts with the Binance API")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Binance Algo REST API")]
    Algo {
        #[command(subcommand)]
        command: AlgoCommands,
    },
    #[command(about = "Binance Alpha REST API")]
    Alpha {
        #[command(subcommand)]
        command: AlphaCommands,
    },
    #[command(about = "Binance C2C REST API")]
    C2C {
        #[command(subcommand)]
        command: C2CCommands,
    },
    #[command(about = "Binance Convert REST API")]
    Convert {
        #[command(subcommand)]
        command: ConvertCommands,
    },
    #[command(about = "Binance Copy Trading REST API")]
    CopyTrading {
        #[command(subcommand)]
        command: CopyTradingCommands,
    },
    #[command(about = "Binance Crypto Loan REST API")]
    CryptoLoan {
        #[command(subcommand)]
        command: CryptoLoanCommands,
    },
    #[command(about = "Binance Derivatives Trading Options REST API")]
    DerivativesTradingOptions {
        #[command(subcommand)]
        command: DerivativesTradingOptionsCommands,
    },
    #[command(about = "Binance Derivatives Trading Portfolio Margin REST API")]
    DerivativesTradingPortfolioMargin {
        #[command(subcommand)]
        command: DerivativesTradingPortfolioMarginCommands,
    },
    #[command(about = "Binance Derivatives Trading Portfolio Margin Pro REST API")]
    DerivativesTradingPortfolioMarginPro {
        #[command(subcommand)]
        command: DerivativesTradingPortfolioMarginProCommands,
    },
    #[command(about = "Binance Dual Investment REST API")]
    DualInvestment {
        #[command(subcommand)]
        command: DualInvestmentCommands,
    },
    // Fiat {
    //     #[command(subcommand)]
    //     command: FiatCommands,
    // },
    #[command(about = "Binance Derivatives Trading COIN Futures REST API")]
    FuturesCoin {
        #[command(subcommand)]
        command: DerivativesTradingCoinFuturesCommands,
    },
    #[command(about = "Binance Derivatives Trading USDS Futures REST API")]
    FuturesUsds {
        #[command(subcommand)]
        command: DerivativesTradingUsdsFuturesCommands,
    },
    #[command(about = "Binance Gift Card REST API")]
    GiftCard {
        #[command(subcommand)]
        command: GiftCardCommands,
    },
    #[command(about = "Binance Margin Trading REST API")]
    MarginTrading {
        #[command(subcommand)]
        command: MarginTradingCommands,
    },
    #[command(about = "Binance Mining REST API")]
    Mining {
        #[command(subcommand)]
        command: MiningCommands,
    },
    #[command(about = "Binance Pay REST API")]
    Pay {
        #[command(subcommand)]
        command: PayCommands,
    },
    #[command(about = "Binance Rebate REST API")]
    Rebate {
        #[command(subcommand)]
        command: RebateCommands,
    },
    #[command(about = "Binance Simple Earn REST API")]
    SimpleEarn {
        #[command(subcommand)]
        command: SimpleEarnCommands,
    },
    #[command(about = "Binance Spot REST API")]
    Spot {
        #[command(subcommand)]
        command: SpotCommands,
    },
    #[command(about = "Binance Staking REST API")]
    Staking {
        #[command(subcommand)]
        command: StakingCommands,
    },
    #[command(about = "Binance Sub Account REST API")]
    SubAccount {
        #[command(subcommand)]
        command: SubAccountCommands,
    },
    #[command(about = "Binance VIP Loan REST API")]
    VIPLoan {
        #[command(subcommand)]
        command: VIPLoanCommands,
    },
    #[command(about = "Binance W3W Prediction REST API")]
    W3WPrediction {
        #[command(subcommand)]
        command: W3WPredictionCommands,
    },
    #[command(about = "Binance Wallet REST API")]
    Wallet {
        #[command(subcommand)]
        command: WalletCommands,
    },
    #[command(about = "Completion commands (generate autocompletion script)")]
    Completion { shell: Shell },
    #[command(about = "Custom request")]
    Request(CustomRequestCommand),
    #[command(about = "Profile commands (create, select, view, list, delete)")]
    Profile {
        #[command(subcommand)]
        command: ProfileCommand,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Algo { command } => {
            handle_algo_command(command).await?;
        }
        Commands::Alpha { command } => {
            handle_alpha_command(command).await?;
        }
        Commands::C2C { command } => {
            handle_c2c_command(command).await?;
        }
        Commands::Convert { command } => {
            handle_convert_command(command).await?;
        }
        Commands::CopyTrading { command } => {
            handle_copy_trading_command(command).await?;
        }
        Commands::CryptoLoan { command } => {
            handle_crypto_loan_command(command).await?;
        }
        Commands::DerivativesTradingOptions { command } => {
            handle_derivatives_trading_options_command(command).await?;
        }
        Commands::DerivativesTradingPortfolioMargin { command } => {
            handle_derivatives_trading_portfolio_margin_command(command).await?;
        }
        Commands::DerivativesTradingPortfolioMarginPro { command } => {
            handle_derivatives_trading_portfolio_margin_pro_command(command).await?;
        }
        Commands::DualInvestment { command } => {
            handle_dual_investment_command(command).await?;
        }
        // Commands::Fiat { command } => {
        //     handle_fiat_command(command).await?;
        // }
        Commands::FuturesCoin { command } => {
            handle_derivatives_trading_coin_futures_command(command).await?;
        }
        Commands::FuturesUsds { command } => {
            handle_derivatives_trading_usds_futures_command(command).await?;
        }
        Commands::GiftCard { command } => {
            handle_gift_card_command(command).await?;
        }
        Commands::MarginTrading { command } => {
            handle_margin_trading_command(command).await?;
        }
        Commands::Mining { command } => {
            handle_mining_command(command).await?;
        }
        Commands::Pay { command } => {
            handle_pay_command(command).await?;
        }
        Commands::Rebate { command } => {
            handle_rebate_command(command).await?;
        }
        Commands::SimpleEarn { command } => {
            handle_simple_earn_command(command).await?;
        }
        Commands::Spot { command } => {
            handle_spot_command(command).await?;
        }
        Commands::Staking { command } => {
            handle_staking_command(command).await?;
        }
        Commands::SubAccount { command } => {
            handle_sub_account_command(command).await?;
        }
        Commands::VIPLoan { command } => {
            handle_vip_loan_command(command).await?;
        }
        Commands::W3WPrediction { command } => {
            handle_w3w_prediction_command(command).await?;
        }
        Commands::Wallet { command } => {
            handle_wallet_command(command).await?;
        }
        Commands::Request(command) => {
            handle_custom_request(command).await?;
        }
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
        }
        Commands::Profile { command } => {
            handle_profile_command(command)?;
        }
    }

    Ok(())
}
