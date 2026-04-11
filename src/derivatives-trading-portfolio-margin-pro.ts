import {
    DerivativesTradingPortfolioMarginPro,
    DERIVATIVES_TRADING_PORTFOLIO_MARGIN_PRO_REST_API_PROD_URL,
} from '@binance/derivatives-trading-portfolio-margin-pro';
import inquirer from 'inquirer';
import {
    decodeSelectedEntities,
    getConfigurationRestAPI,
    getUserAgent,
    isEmpty,
    readStdinObj,
} from './utils';
import { Parser, hideBin } from 'yargs/helpers';

const parsedArgs = Parser(hideBin(process.argv));

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent(
    'derivatives-trading-portfolio-margin-pro'
);

const stdinObj: any = readStdinObj();

let basePath = DERIVATIVES_TRADING_PORTFOLIO_MARGIN_PRO_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(
    parsedArgs?.profile,
    'derivatives-portfolio-margin-pro'
);

if (process.env.BINANCE_DERIVATIVES_PORTFOLIO_MARGIN_PRO_BASE_PATH) {
    basePath = process.env.BINANCE_DERIVATIVES_PORTFOLIO_MARGIN_PRO_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new DerivativesTradingPortfolioMarginPro({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new DerivativesTradingPortfolioMarginPro({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const derivativesTradingPortfolioMarginProCommands: any[] = [];

derivativesTradingPortfolioMarginProCommands.push({
    command: 'bnb-transfer',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`BNB transfer can be between Margin Account and USDM Account


* You can only use this function 2 times per 10 minutes in a rolling manner

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'transfer-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    type: 'string',
                    group: 'Command Options:',
                },
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
                }

                if (!options?.['transferSide'] && !options?.interactive) {
                    requiredParams.push('transferSide');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'bnb-transfer is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['amount']) {
            questions.push({
                type: 'input',
                name: 'amount',
                message: 'Input amount:',
                validate: (input: string) => (input ? true : 'amount cannot be empty'),
            });
        }

        if (options.interactive && !options?.['transferSide']) {
            questions.push({
                type: 'input',
                name: 'transferSide',
                message: 'Input transferSide:',
                validate: (input: string) => (input ? true : 'transferSide cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.bnbTransfer(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'change-auto-repay-futures-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Change Auto-repay-futures Status

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'auto-repay': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    type: 'string',
                    group: 'Command Options:',
                },
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['autoRepay'] && !options?.interactive) {
                    requiredParams.push('autoRepay');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'change-auto-repay-futures-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['autoRepay']) {
            questions.push({
                type: 'input',
                name: 'autoRepay',
                message: 'Input autoRepay:',
                validate: (input: string) => (input ? true : 'autoRepay cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeAutoRepayFuturesStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'fund-auto-collection',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Transfers all assets from Futures Account to Margin account

* The BNB would not be collected from UM-PM account to the Portfolio Margin account.
* You can only use this function 500 times per hour in a rolling manner.

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'fund-auto-collection is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.fundAutoCollection(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'fund-collection-by-asset',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Transfers specific asset from Futures Account to Margin account

* The BNB transfer is not be supported

Weight: 60`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    type: 'string',
                    group: 'Command Options:',
                },
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'fund-collection-by-asset is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.fundCollectionByAsset(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'get-auto-repay-futures-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Auto-repay-futures Status

Weight: 30`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-auto-repay-futures-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getAutoRepayFuturesStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'get-delta-mode-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query the Delta mode status of current account.

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-delta-mode-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getDeltaModeStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'get-portfolio-margin-pro-account-balance',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Portfolio Margin Pro account balance

Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-portfolio-margin-pro-account-balance is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getPortfolioMarginProAccountBalance(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'get-portfolio-margin-pro-account-info',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Portfolio Margin Pro Account Info

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-portfolio-margin-pro-account-info is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getPortfolioMarginProAccountInfo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'get-portfolio-margin-pro-span-account-info',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Portfolio Margin Pro SPAN Account Info (For Portfolio Margin Pro SPAN users only)

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-portfolio-margin-pro-span-account-info is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getPortfolioMarginProSpanAccountInfo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'get-transferable-earn-asset-balance-for-portfolio-margin',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get transferable earn asset balance for all types of Portfolio Margin account

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    describe: decodeSelectedEntities('&#x60;LDUSDT&#x60; only'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'transfer-type': {
                    describe: decodeSelectedEntities(
                        '&#x60;EARN_TO_FUTURE&#x60; /&#x60;FUTURE_TO_EARN&#x60;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
                }

                if (!options?.['transferType'] && !options?.interactive) {
                    requiredParams.push('transferType');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-transferable-earn-asset-balance-for-portfolio-margin is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.asset) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
            });
        }
        if (options.interactive && !options.transferType) {
            questions.push({
                type: 'input',
                name: 'transferType',
                message: 'Input transferType:',
                validate: (input: string) => (input ? true : 'transferType cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.getTransferableEarnAssetBalanceForPortfolioMargin(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'portfolio-margin-pro-bankruptcy-loan-repay',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Repay Portfolio Margin Pro Bankruptcy Loan

* Please note that the API Key has enabled Spot &amp; Margin Trading permissions to access this endpoint.

Weight: 3000`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            from: {
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'portfolio-margin-pro-bankruptcy-loan-repay is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.portfolioMarginProBankruptcyLoanRepay(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'query-portfolio-margin-pro-bankruptcy-loan-amount',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Portfolio Margin Pro Bankruptcy Loan Amount

* If there’s no classic portfolio margin bankruptcy loan, the amount would be 0

Weight: 500`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'query-portfolio-margin-pro-bankruptcy-loan-amount is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.queryPortfolioMarginProBankruptcyLoanAmount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'query-portfolio-margin-pro-bankruptcy-loan-repay-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query repay history of pmloan for portfolio margin pro.

* &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 360 days
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; not sent, return records of the last 30 days by default.
* If &#x60;startTime&#x60;is sent and &#x60;endTime&#x60; is not sent, return records of [startTime, startTime+30d].
* If &#x60;startTime&#x60; is not sent and &#x60;endTime&#x60; is sent, return records of [endTime-30d, endTime].

Weight: 500`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'start-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Start from 1. Default:1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities('Default:10 Max:100'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'query-portfolio-margin-pro-bankruptcy-loan-repay-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.queryPortfolioMarginProBankruptcyLoanRepayHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'query-portfolio-margin-pro-negative-balance-interest-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query interest history of negative balance for portfolio margin.

Weight: 50`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities('Default:10 Max:100'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'query-portfolio-margin-pro-negative-balance-interest-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.queryPortfolioMarginProNegativeBalanceInterestHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'repay-futures-negative-balance',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Repay futures Negative Balance

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            from: {
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'repay-futures-negative-balance is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.repayFuturesNegativeBalance(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'switch-delta-mode',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Switch the Delta mode for existing PM PRO / PM RETAIL accounts.

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'delta-enabled': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    type: 'string',
                    group: 'Command Options:',
                },
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['deltaEnabled'] && !options?.interactive) {
                    requiredParams.push('deltaEnabled');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'switch-delta-mode is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['deltaEnabled']) {
            questions.push({
                type: 'input',
                name: 'deltaEnabled',
                message: 'Input deltaEnabled:',
                validate: (input: string) => (input ? true : 'deltaEnabled cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.switchDeltaMode(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'transfer-ldusdt-rwusd-for-portfolio-margin',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Transfer LDUSDT/RWUSD as collateral for all types of Portfolio Margin account

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'transfer-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    type: 'string',
                    group: 'Command Options:',
                },
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
                }

                if (!options?.['transferType'] && !options?.interactive) {
                    requiredParams.push('transferType');
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'transfer-ldusdt-rwusd-for-portfolio-margin is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
            });
        }

        if (options.interactive && !options?.['transferType']) {
            questions.push({
                type: 'input',
                name: 'transferType',
                message: 'Input transferType:',
                validate: (input: string) => (input ? true : 'transferType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['amount']) {
            questions.push({
                type: 'input',
                name: 'amount',
                message: 'Input amount:',
                validate: (input: string) => (input ? true : 'amount cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.transferLdusdtRwusdForPortfolioMargin(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'get-portfolio-margin-asset-leverage',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Portfolio Margin Asset Leverage

Weight: 50`),
    handler: async () => {
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-portfolio-margin-asset-leverage is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        try {
            const response = await client.restAPI.getPortfolioMarginAssetLeverage();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'portfolio-margin-collateral-rate',
    describe: decodeSelectedEntities(`Portfolio Margin Collateral Rate

Weight: 50`),
    handler: async () => {
        try {
            const response = await client.restAPI.portfolioMarginCollateralRate();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'portfolio-margin-pro-tiered-collateral-rate',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Portfolio Margin PRO Tiered Collateral Rate

Weight: 50`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'portfolio-margin-pro-tiered-collateral-rate is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.portfolioMarginProTieredCollateralRate(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginProCommands.push({
    command: 'query-portfolio-margin-asset-index-price',
    describe: decodeSelectedEntities(`Query Portfolio Margin Asset Index Price

Weight: 1 if send asset or 50 if not send asset`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryPortfolioMarginAssetIndexPrice(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'derivatives-portfolio-margin-pro',
    description: 'Binance Derivatives Trading Portfolio Margin Pro REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli derivatives-portfolio-margin-pro <command> [options]');
        derivativesTradingPortfolioMarginProCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
