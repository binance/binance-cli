import {
    DerivativesTradingPortfolioMargin,
    DERIVATIVES_TRADING_PORTFOLIO_MARGIN_REST_API_PROD_URL,
    DERIVATIVES_TRADING_PORTFOLIO_MARGIN_REST_API_TESTNET_URL,
} from '@binance/derivatives-trading-portfolio-margin';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('derivatives-trading-portfolio-margin');

const stdinObj: any = readStdinObj();

let basePath = DERIVATIVES_TRADING_PORTFOLIO_MARGIN_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(
    parsedArgs?.profile,
    'derivatives-portfolio-margin'
);

if (process.env.BINANCE_DERIVATIVES_PORTFOLIO_MARGIN_BASE_PATH) {
    basePath = process.env.BINANCE_DERIVATIVES_PORTFOLIO_MARGIN_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
} else if (configurationRestAPI && configurationRestAPI['env']) {
    switch (configurationRestAPI['env']) {
        case 'testnet':
            basePath = DERIVATIVES_TRADING_PORTFOLIO_MARGIN_REST_API_TESTNET_URL;
            break;
    }
}

let client;
if (configurationRestAPI !== null) {
    client = new DerivativesTradingPortfolioMargin({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new DerivativesTradingPortfolioMargin({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const derivativesTradingPortfolioMarginCommands: any[] = [];

derivativesTradingPortfolioMarginCommands.push({
    command: 'account-balance',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query account balance

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
                'account-balance is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountBalance(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'account-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query account information

Weight: 20`),
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
                'account-information is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountInformation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'bnb-transfer',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Transfer BNB in and out of UM

* The endpoint can only be called 10 times per 10 minutes in a rolling manner

Weight: 750`),
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

derivativesTradingPortfolioMarginCommands.push({
    command: 'change-auto-repay-futures-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Change Auto-repay-futures Status

Weight: 750`),
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

derivativesTradingPortfolioMarginCommands.push({
    command: 'change-cm-initial-leverage',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Change user&#39;s initial leverage of specific symbol in CM.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                leverage: {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['leverage'] && !options?.interactive) {
                    requiredParams.push('leverage');
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
                'change-cm-initial-leverage is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['leverage']) {
            questions.push({
                type: 'input',
                name: 'leverage',
                message: 'Input leverage:',
                validate: (input: string) => (input ? true : 'leverage cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeCmInitialLeverage(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'change-cm-position-mode',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Change user&#39;s position mode (Hedge Mode or One-way Mode ) on EVERY symbol in CM

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'dual-side-position': {
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

                if (!options?.['dualSidePosition'] && !options?.interactive) {
                    requiredParams.push('dualSidePosition');
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
                'change-cm-position-mode is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['dualSidePosition']) {
            questions.push({
                type: 'input',
                name: 'dualSidePosition',
                message: 'Input dualSidePosition:',
                validate: (input: string) => (input ? true : 'dualSidePosition cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeCmPositionMode(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'change-um-initial-leverage',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Change user&#39;s initial leverage of specific symbol in UM.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                leverage: {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['leverage'] && !options?.interactive) {
                    requiredParams.push('leverage');
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
                'change-um-initial-leverage is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['leverage']) {
            questions.push({
                type: 'input',
                name: 'leverage',
                message: 'Input leverage:',
                validate: (input: string) => (input ? true : 'leverage cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeUmInitialLeverage(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'change-um-position-mode',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Change user&#39;s position mode (Hedge Mode or One-way Mode ) on EVERY symbol in UM

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'dual-side-position': {
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

                if (!options?.['dualSidePosition'] && !options?.interactive) {
                    requiredParams.push('dualSidePosition');
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
                'change-um-position-mode is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['dualSidePosition']) {
            questions.push({
                type: 'input',
                name: 'dualSidePosition',
                message: 'Input dualSidePosition:',
                validate: (input: string) => (input ? true : 'dualSidePosition cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeUmPositionMode(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cm-notional-and-leverage-brackets',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query CM notional and leverage brackets

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'cm-notional-and-leverage-brackets is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cmNotionalAndLeverageBrackets(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'fund-auto-collection',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Fund collection for Portfolio Margin

* The BNB would not be collected from UM-PM account to the Portfolio Margin account.
* You can only use this function 500 times per hour in a rolling manner.

Weight: 750`),
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

derivativesTradingPortfolioMarginCommands.push({
    command: 'fund-collection-by-asset',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Transfers specific asset from Futures Account to Margin account

* The BNB transfer is not be supported

Weight: 30`),
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

derivativesTradingPortfolioMarginCommands.push({
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

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-cm-account-detail',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current CM account asset and position information.

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
                'get-cm-account-detail is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCmAccountDetail(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-cm-current-position-mode',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get user&#39;s position mode (Hedge Mode or One-way Mode ) on EVERY symbol in CM

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
                'get-cm-current-position-mode is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCmCurrentPositionMode(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-cm-income-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get CM Income History


* If &#x60;incomeType&#x60; is not sent, all kinds of flow will be returned
* &quot;trandId&quot; is unique in the same &quot;incomeType&quot; for a user
* The interval between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not exceed 200 days:
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are not sent, the last 200 days will be returned

Weight: 30`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'income-type': {
                describe: decodeSelectedEntities(
                    'TRANSFER, WELCOME_BONUS, REALIZED_PNL, FUNDING_FEE, COMMISSION, INSURANCE_CLEAR, REFERRAL_KICKBACK, COMMISSION_REBATE, API_REBATE, CONTEST_REWARD, CROSS_COLLATERAL_TRANSFER, OPTIONS_PREMIUM_FEE, OPTIONS_SETTLE_PROFIT, INTERNAL_TRANSFER, AUTO_EXCHANGE, DELIVERED_SETTELMENT, COIN_SWAP_DEPOSIT, COIN_SWAP_WITHDRAW, POSITION_LIMIT_INCREASE_FEE'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            page: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
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
                'get-cm-income-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCmIncomeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-download-id-for-um-futures-order-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get download id for UM futures order history

* Request Limitation is 10 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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

                if (!options?.['startTime'] && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.['endTime'] && !options?.interactive) {
                    requiredParams.push('endTime');
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
                'get-download-id-for-um-futures-order-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.startTime) {
            questions.push({
                type: 'input',
                name: 'startTime',
                message: 'Input startTime:',
                validate: (input: string) => (input ? true : 'startTime cannot be empty'),
            });
        }
        if (options.interactive && !options.endTime) {
            questions.push({
                type: 'input',
                name: 'endTime',
                message: 'Input endTime:',
                validate: (input: string) => (input ? true : 'endTime cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getDownloadIdForUmFuturesOrderHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-download-id-for-um-futures-trade-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get download id for UM futures trade history

* Request Limitation is 5 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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

                if (!options?.['startTime'] && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.['endTime'] && !options?.interactive) {
                    requiredParams.push('endTime');
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
                'get-download-id-for-um-futures-trade-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.startTime) {
            questions.push({
                type: 'input',
                name: 'startTime',
                message: 'Input startTime:',
                validate: (input: string) => (input ? true : 'startTime cannot be empty'),
            });
        }
        if (options.interactive && !options.endTime) {
            questions.push({
                type: 'input',
                name: 'endTime',
                message: 'Input endTime:',
                validate: (input: string) => (input ? true : 'endTime cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getDownloadIdForUmFuturesTradeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-download-id-for-um-futures-transaction-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get download id for UM futures transaction history

* Request Limitation is 5 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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

                if (!options?.['startTime'] && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.['endTime'] && !options?.interactive) {
                    requiredParams.push('endTime');
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
                'get-download-id-for-um-futures-transaction-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.startTime) {
            questions.push({
                type: 'input',
                name: 'startTime',
                message: 'Input startTime:',
                validate: (input: string) => (input ? true : 'startTime cannot be empty'),
            });
        }
        if (options.interactive && !options.endTime) {
            questions.push({
                type: 'input',
                name: 'endTime',
                message: 'Input endTime:',
                validate: (input: string) => (input ? true : 'endTime cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.getDownloadIdForUmFuturesTransactionHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-margin-borrow-loan-interest-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Margin Borrow/Loan Interest History


* Response in descending order
* The max interval between startTime and endTime is 30 days. It is a MUST to ensure data correctness.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; not sent, return records of the last 7 days by default
* If &#x60;startTime&#x60; is sent and &#x60;endTime&#x60; is not sent, the records from &#x60;startTime&#x60; to the present will be returned; if &#x60;startTime&#x60; is more than 30 days ago, the records of the past 30 days will be returned.
* If &#x60;startTime&#x60; is not sent and &#x60;endTime&#x60; is sent, the records of the 7 days before &#x60;endTime&#x60; is returned.
* Type in response has 5 enums:
* &#x60;PERIODIC&#x60; interest charged per hour
* &#x60;ON_BORROW&#x60; first interest charged on borrow
* &#x60;PERIODIC_CONVERTED&#x60; interest charged per hour converted into BNB
* &#x60;ON_BORROW_CONVERTED&#x60; first interest charged on borrow converted into BNB
* &#x60;PORTFOLIO&#x60; Portfolio Margin negative balance daily interest

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
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
            archived: {
                describe: decodeSelectedEntities(
                    'Default: &#x60;false&#x60;. Set to &#x60;true&#x60; for archived data from 6 months ago'
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
                'get-margin-borrow-loan-interest-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getMarginBorrowLoanInterestHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-um-account-detail',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current UM account asset and position information.

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
                'get-um-account-detail is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUmAccountDetail(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-um-account-detail-v2',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current UM account asset and position information.

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
                'get-um-account-detail-v2 is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUmAccountDetailV2(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-um-current-position-mode',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get user&#39;s position mode (Hedge Mode or One-way Mode ) on EVERY symbol in UM

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
                'get-um-current-position-mode is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUmCurrentPositionMode(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-um-futures-order-download-link-by-id',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get UM futures order download link by Id

* Download link expiration: 24h

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'download-id': {
                    describe: decodeSelectedEntities('get by download id api'),
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

                if (!options?.['downloadId'] && !options?.interactive) {
                    requiredParams.push('downloadId');
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
                'get-um-futures-order-download-link-by-id is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.downloadId) {
            questions.push({
                type: 'input',
                name: 'downloadId',
                message: 'Input downloadId:',
                validate: (input: string) => (input ? true : 'downloadId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUmFuturesOrderDownloadLinkById(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-um-futures-trade-download-link-by-id',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get UM futures trade download link by Id

* Download link expiration: 24h

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'download-id': {
                    describe: decodeSelectedEntities('get by download id api'),
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

                if (!options?.['downloadId'] && !options?.interactive) {
                    requiredParams.push('downloadId');
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
                'get-um-futures-trade-download-link-by-id is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.downloadId) {
            questions.push({
                type: 'input',
                name: 'downloadId',
                message: 'Input downloadId:',
                validate: (input: string) => (input ? true : 'downloadId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUmFuturesTradeDownloadLinkById(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-um-futures-transaction-download-link-by-id',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get UM futures Transaction download link by Id

* Download link expiration: 24h

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'download-id': {
                    describe: decodeSelectedEntities('get by download id api'),
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

                if (!options?.['downloadId'] && !options?.interactive) {
                    requiredParams.push('downloadId');
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
                'get-um-futures-transaction-download-link-by-id is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.downloadId) {
            questions.push({
                type: 'input',
                name: 'downloadId',
                message: 'Input downloadId:',
                validate: (input: string) => (input ? true : 'downloadId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUmFuturesTransactionDownloadLinkById(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-um-income-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get UM Income History

* If neither &#x60;startTime&#x60; nor &#x60;endTime&#x60; is sent, the recent 7-day data will be returned.
* If &#x60;incomeType&#x60; is not sent, all kinds of flow will be returned
* &quot;trandId&quot; is unique in the same incomeType for a user
* Income history only contains data for the last three months

Weight: 30`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'income-type': {
                describe: decodeSelectedEntities(
                    'TRANSFER, WELCOME_BONUS, REALIZED_PNL, FUNDING_FEE, COMMISSION, INSURANCE_CLEAR, REFERRAL_KICKBACK, COMMISSION_REBATE, API_REBATE, CONTEST_REWARD, CROSS_COLLATERAL_TRANSFER, OPTIONS_PREMIUM_FEE, OPTIONS_SETTLE_PROFIT, INTERNAL_TRANSFER, AUTO_EXCHANGE, DELIVERED_SETTELMENT, COIN_SWAP_DEPOSIT, COIN_SWAP_WITHDRAW, POSITION_LIMIT_INCREASE_FEE'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            page: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
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
                'get-um-income-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUmIncomeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-user-commission-rate-for-cm',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get User Commission Rate for CM

Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'get-user-commission-rate-for-cm is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUserCommissionRateForCm(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-user-commission-rate-for-um',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get User Commission Rate for UM

Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'get-user-commission-rate-for-um is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUserCommissionRateForUm(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'margin-max-borrow',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query margin max borrow

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
                'margin-max-borrow is signed. Please create a profile using `binance-cli profile create`.'
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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.marginMaxBorrow(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'portfolio-margin-um-trading-quantitative-rules-indicators',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Portfolio Margin UM Trading Quantitative Rules Indicators

Weight: 1 for a single symbol
10 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'portfolio-margin-um-trading-quantitative-rules-indicators is signed. Please create a profile using `binance-cli profile create`.'
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
                await client.restAPI.portfolioMarginUmTradingQuantitativeRulesIndicators(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-cm-position-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current CM position information.

* If neither &#x60;marginAsset&#x60; nor &#x60;pair&#x60; is sent, positions of all symbols with &#x60;TRADING&#x60; status will be returned.
* for One-way Mode user, the response will only show the &quot;BOTH&quot; positions
* for Hedge Mode user, the response will show &quot;LONG&quot;, and &quot;SHORT&quot; positions.
* Please use with user data stream &#x60;ACCOUNT_UPDATE&#x60; to meet your timeliness and accuracy needs.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'margin-asset': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            pair: {
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
                'query-cm-position-information is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCmPositionInformation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-margin-loan-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query margin loan record

* txId or startTime must be sent. txId takes precedence.
* Response in descending order
* The max interval between &#x60;startTime&#x60; and &#x60;endTime&#x60; is 30 days.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; not sent, return records of the last 7 days by default
* Set &#x60;archived&#x60; to &#x60;true&#x60; to query data from 6 months ago

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'tx-id': {
                    describe: decodeSelectedEntities(
                        'the &#x60;tranId&#x60; in &#x60;POST/papi/v1/marginLoan&#x60;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
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
                archived: {
                    describe: decodeSelectedEntities(
                        'Default: &#x60;false&#x60;. Set to &#x60;true&#x60; for archived data from 6 months ago'
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
                'query-margin-loan-record is signed. Please create a profile using `binance-cli profile create`.'
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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryMarginLoanRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-margin-max-withdraw',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Max Withdraw

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
                'query-margin-max-withdraw is signed. Please create a profile using `binance-cli profile create`.'
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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryMarginMaxWithdraw(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-margin-repay-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query margin repay record.

* txId or startTime must be sent. txId takes precedence.
* Response in descending order
* The max interval between &#x60;startTime&#x60; and &#x60;endTime&#x60; is 30 days.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; not sent, return records of the last 7 days by default
* Set &#x60;archived&#x60; to &#x60;true&#x60; to query data from 6 months ago

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'tx-id': {
                    describe: decodeSelectedEntities(
                        'the &#x60;tranId&#x60; in &#x60;POST/papi/v1/marginLoan&#x60;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
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
                archived: {
                    describe: decodeSelectedEntities(
                        'Default: &#x60;false&#x60;. Set to &#x60;true&#x60; for archived data from 6 months ago'
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
                'query-margin-repay-record is signed. Please create a profile using `binance-cli profile create`.'
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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryMarginRepayRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-portfolio-margin-negative-balance-interest-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query interest history of negative balance for portfolio margin.

* Response in descending order
* The max interval between startTime and endTime is 30 days. It is a MUST to ensure data correctness.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; not sent, return records of the last 7 days by default
* If &#x60;startTime&#x60; is sent and &#x60;endTime&#x60; is not sent, the records from &#x60;startTime&#x60; to the present will be returned; if &#x60;startTime&#x60; is more than 30 days ago, the records of the past 30 days will be returned.
* If &#x60;startTime&#x60; is not sent and &#x60;endTime&#x60; is sent, the records of the 7 days before &#x60;endTime&#x60; is returned.

Weight: 50`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
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
                'query-portfolio-margin-negative-balance-interest-history is signed. Please create a profile using `binance-cli profile create`.'
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
                await client.restAPI.queryPortfolioMarginNegativeBalanceInterestHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-um-position-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current UM position information.

* Please use with user data stream &#x60;ACCOUNT_UPDATE&#x60; to meet your timeliness and accuracy needs.
* for One-way Mode user, the response will only show the &quot;BOTH&quot; positions
* for Hedge Mode user, the response will show &quot;LONG&quot;, and &quot;SHORT&quot; positions.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'query-um-position-information is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUmPositionInformation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-user-negative-balance-auto-exchange-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query user negative balance auto exchange record

* Response in descending order
* The max interval between &#x60;startTime&#x60; and &#x60;endTime&#x60; is 3 months.

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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

                if (!options?.['startTime'] && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.['endTime'] && !options?.interactive) {
                    requiredParams.push('endTime');
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
                'query-user-negative-balance-auto-exchange-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.startTime) {
            questions.push({
                type: 'input',
                name: 'startTime',
                message: 'Input startTime:',
                validate: (input: string) => (input ? true : 'startTime cannot be empty'),
            });
        }
        if (options.interactive && !options.endTime) {
            questions.push({
                type: 'input',
                name: 'endTime',
                message: 'Input endTime:',
                validate: (input: string) => (input ? true : 'endTime cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.queryUserNegativeBalanceAutoExchangeRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-user-rate-limit',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query User Rate Limit

Weight: 1`),
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
                'query-user-rate-limit is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUserRateLimit(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'repay-futures-negative-balance',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Repay futures Negative Balance

Weight: 750`),
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

derivativesTradingPortfolioMarginCommands.push({
    command: 'um-futures-account-configuration',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query UM Futures account configuration

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
                'um-futures-account-configuration is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.umFuturesAccountConfiguration(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'um-futures-symbol-configuration',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current UM account symbol configuration.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'um-futures-symbol-configuration is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.umFuturesSymbolConfiguration(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'um-notional-and-leverage-brackets',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query UM notional and leverage brackets

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'um-notional-and-leverage-brackets is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.umNotionalAndLeverageBrackets(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'test-connectivity',
    describe: decodeSelectedEntities(`Test connectivity to the Rest API.

Weight: 1`),
    handler: async () => {
        try {
            await client.restAPI.testConnectivity();
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-all-cm-open-conditional-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel All CM Open Conditional Orders

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-all-cm-open-conditional-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelAllCmOpenConditionalOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-all-cm-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel all active LIMIT orders on specific symbol

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-all-cm-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelAllCmOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-all-um-open-conditional-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel All UM Open Conditional Orders

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-all-um-open-conditional-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelAllUmOpenConditionalOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-all-um-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel all active LIMIT orders on specific symbol

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-all-um-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelAllUmOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-cm-conditional-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel CM Conditional Order

* Either &#x60;strategyId&#x60; or &#x60;newClientStrategyId&#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-strategy-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-cm-conditional-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelCmConditionalOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-cm-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an active LIMIT order

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-cm-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelCmOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-margin-account-all-open-orders-on-a-symbol',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel Margin Account All Open Orders on a Symbol

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-margin-account-all-open-orders-on-a-symbol is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.cancelMarginAccountAllOpenOrdersOnASymbol(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-margin-account-oco-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel Margin Account OCO Orders

* Additional notes: Canceling an individual leg will cancel the entire OCO

Weight: 2`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-list-id': {
                    describe: decodeSelectedEntities(
                        'Either &#x60;orderListId&#x60; or &#x60;listClientOrderId&#x60; must be provided'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'list-client-order-id': {
                    describe: decodeSelectedEntities(
                        'Either &#x60;orderListId&#x60; or &#x60;listClientOrderId&#x60; must be provided'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    describe: decodeSelectedEntities(
                        'Used to uniquely identify this cancel. Automatically generated by default'
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-margin-account-oco-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelMarginAccountOcoOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-margin-account-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel Margin Account Order

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.

Weight: 2`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    describe: decodeSelectedEntities(
                        'Used to uniquely identify this cancel. Automatically generated by default'
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-margin-account-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelMarginAccountOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-um-conditional-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel UM Conditional Order

* Either &#x60;strategyId&#x60; or &#x60;newClientStrategyId&#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-strategy-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-um-conditional-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelUmConditionalOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cancel-um-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an active UM LIMIT order

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'cancel-um-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelUmOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cm-account-trade-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get trades for a specific account and CM symbol.

* Either &#x60;symbol&#x60; or &#x60;pair&#x60; must be sent
* &#x60;symbol&#x60; and &#x60;pair&#x60; cannot be sent together
* &#x60;pair&#x60; and &#x60;fromId&#x60; cannot be sent together
* &#x60;OrderId&#x60; can only be sent together with symbol
* If a &#x60;pair&#x60; is sent, tickers for all symbols of the &#x60;pair&#x60; will be returned
* The parameter &#x60;fromId&#x60; cannot be sent with &#x60;startTime&#x60; or &#x60;endTime&#x60;
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last &#39;24 hours&#39; data will be returned.
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 24 hours.

Weight: 20 with symbol, 40 with pair`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            pair: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'from-id': {
                describe: decodeSelectedEntities(
                    'Trade id to fetch from. Default gets most recent trades.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
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
                'cm-account-trade-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cmAccountTradeList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'cm-position-adl-quantile-estimation',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query CM Position ADL Quantile Estimation
* Values update every 30s.
* Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
* For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode, &quot;LONG&quot;, &quot;SHORT&quot;, and &quot;BOTH&quot; will be returned to show the positions&#39; adl quantiles of different position sides.
* If the positions of the symbol are crossed margined in Hedge Mode:
* &quot;HEDGE&quot; as a sign will be returned instead of &quot;BOTH&quot;;
* A same value caculated on unrealized pnls on long and short sides&#39; positions will be shown for &quot;LONG&quot; and &quot;SHORT&quot; when there are positions in both of long and short sides.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'cm-position-adl-quantile-estimation is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cmPositionAdlQuantileEstimation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'get-um-futures-bnb-burn-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get user&#39;s BNB Fee Discount for UM Futures (Fee Discount On or Fee Discount Off )

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
                'get-um-futures-bnb-burn-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getUmFuturesBnbBurnStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'margin-account-borrow',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Apply for a margin loan.

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
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
                'margin-account-borrow is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.marginAccountBorrow(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'margin-account-new-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send in a new OCO for a margin account

* Price Restrictions:
* &#x60;SELL&#x60;: Limit Price &gt; Last Price &gt; Stop Price
* &#x60;BUY&#x60;: Limit Price &lt; Last Price &lt; Stop Price
* Quantity Restrictions:
* Both legs must have the same quantity
* &#x60;ICEBERG&#x60; quantities however do not have to be the same.
* Order Rate Limit
* &#x60;OCO&#x60; counts as 2 orders against the order rate limit.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'list-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'limit-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'limit-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-limit-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-limit-time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-order-resp-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'side-effect-type': {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['quantity'] && !options?.interactive) {
                    requiredParams.push('quantity');
                }

                if (!options?.['price'] && !options?.interactive) {
                    requiredParams.push('price');
                }

                if (!options?.['stopPrice'] && !options?.interactive) {
                    requiredParams.push('stopPrice');
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
                'margin-account-new-oco is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['side']) {
            questions.push({
                type: 'input',
                name: 'side',
                message: 'Input side:',
                validate: (input: string) => (input ? true : 'side cannot be empty'),
            });
        }

        if (options.interactive && !options?.['quantity']) {
            questions.push({
                type: 'input',
                name: 'quantity',
                message: 'Input quantity:',
                validate: (input: string) => (input ? true : 'quantity cannot be empty'),
            });
        }

        if (options.interactive && !options?.['price']) {
            questions.push({
                type: 'input',
                name: 'price',
                message: 'Input price:',
                validate: (input: string) => (input ? true : 'price cannot be empty'),
            });
        }

        if (options.interactive && !options?.['stopPrice']) {
            questions.push({
                type: 'input',
                name: 'stopPrice',
                message: 'Input stopPrice:',
                validate: (input: string) => (input ? true : 'stopPrice cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.marginAccountNewOco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'margin-account-repay',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Repay for a margin loan.

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
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
                'margin-account-repay is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.marginAccountRepay(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'margin-account-repay-debt',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Repay debt for a margin loan.

* The repay asset amount cannot exceed 50000 USD equivalent value for a single request.
* If &#x60;amount&#x60; is not sent, all the asset loan will be repaid if having enough specific repay assets.
* If &#x60;amount&#x60; is sent, only the certain amount of the asset loan will be repaid if having enough specific repay assets.
* The system will use the same asset to repay the loan first (if have) no matter whether put the asset in &#x60;specifyRepayAssets&#x60;

Weight: 3000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'specify-repay-assets': {
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
                'margin-account-repay-debt is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.marginAccountRepayDebt(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'margin-account-trade-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Margin Account Trade List

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-id': {
                    describe: decodeSelectedEntities(
                        'Trade id to fetch from. Default gets most recent trades.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'margin-account-trade-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.marginAccountTradeList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'modify-cm-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent, and the &#x60;orderId&#x60; will prevail if both are sent.
* Both &#x60;quantity&#x60; and &#x60;price&#x60; must be sent
* When the new &#x60;quantity&#x60; or &#x60;price&#x60; doesn&#39;t satisfy PRICE_FILTER / PERCENT_FILTER / LOT_SIZE, amendment will be rejected and the order will stay as it is.
* However the order will be cancelled by the amendment in the following situations:
* when the order is in partially filled status and the new &#x60;quantity&#x60; &lt;&#x3D; &#x60;executedQty&#x60;
* When the order is &#x60;GTX&#x60; and the new price will cause it to be executed immediately

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'price-match': {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['quantity'] && !options?.interactive) {
                    requiredParams.push('quantity');
                }

                if (!options?.['price'] && !options?.interactive) {
                    requiredParams.push('price');
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
                'modify-cm-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['side']) {
            questions.push({
                type: 'input',
                name: 'side',
                message: 'Input side:',
                validate: (input: string) => (input ? true : 'side cannot be empty'),
            });
        }

        if (options.interactive && !options?.['quantity']) {
            questions.push({
                type: 'input',
                name: 'quantity',
                message: 'Input quantity:',
                validate: (input: string) => (input ? true : 'quantity cannot be empty'),
            });
        }

        if (options.interactive && !options?.['price']) {
            questions.push({
                type: 'input',
                name: 'price',
                message: 'Input price:',
                validate: (input: string) => (input ? true : 'price cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.modifyCmOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'modify-um-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue

* Either orderId or origClientOrderId must be sent, and the orderId will prevail if both are sent.
* Both quantity and price must be sent
* When the new quantity or price doesn&#39;t satisfy PRICE_FILTER / PERCENT_FILTER / LOT_SIZE, amendment will be rejected and the order will stay as it is.
* However the order will be cancelled by the amendment in the following situations:
* when the order is in partially filled status and the new quantity &lt;&#x3D; executedQty
* When the order is GTX and the new price will cause it to be executed immediately

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'price-match': {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['quantity'] && !options?.interactive) {
                    requiredParams.push('quantity');
                }

                if (!options?.['price'] && !options?.interactive) {
                    requiredParams.push('price');
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
                'modify-um-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['side']) {
            questions.push({
                type: 'input',
                name: 'side',
                message: 'Input side:',
                validate: (input: string) => (input ? true : 'side cannot be empty'),
            });
        }

        if (options.interactive && !options?.['quantity']) {
            questions.push({
                type: 'input',
                name: 'quantity',
                message: 'Input quantity:',
                validate: (input: string) => (input ? true : 'quantity cannot be empty'),
            });
        }

        if (options.interactive && !options?.['price']) {
            questions.push({
                type: 'input',
                name: 'price',
                message: 'Input price:',
                validate: (input: string) => (input ? true : 'price cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.modifyUmOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'new-cm-conditional-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`New CM Conditional Order

* Order with type &#x60;STOP/TAKE_PROFIT&#x60;, parameter &#x60;timeInForce&#x60; can be sent ( default &#x60;GTC&#x60;).
* Condition orders will be triggered when:
* &#x60;STOP&#x60;, &#x60;STOP_MARKET&#x60;:
* BUY: &quot;MARK_PRICE&quot;  &gt;&#x3D; &#x60;stopPrice&#x60;
* SELL: &quot;MARK_PRICE&quot; &lt;&#x3D; &#x60;stopPrice&#x60;
* &#x60;TAKE_PROFIT&#x60;, &#x60;TAKE_PROFIT_MARKET&#x60;:
* BUY: &quot;MARK_PRICE&quot; &lt;&#x3D; &#x60;stopPrice&#x60;
* SELL: &quot;MARK_PRICE&quot; &gt;&#x3D; &#x60;stopPrice&#x60;
* &#x60;TRAILING_STOP_MARKET&#x60;:
* BUY: the lowest mark price after order placed &#x60;&lt;&#x3D; &#x60;activationPrice&#x60;, and the latest mark price &gt;&#x60;&#x3D; the lowest mark price * (1 + &#x60;callbackRate&#x60;)
* SELL: the highest mark price after order placed &gt;&#x3D; &#x60;activationPrice&#x60;, and the latest mark price &lt;&#x3D; the highest mark price * (1 - &#x60;callbackRate&#x60;)
* For &#x60;TRAILING_STOP_MARKET&#x60;, if you got such error code. &#x60;{&quot;code&quot;: -2021, &quot;msg&quot;: &quot;Order would immediately trigger.&quot;}&#x60; means that the parameters you send do not meet the following requirements:
* BUY: &#x60;activationPrice&#x60; should be smaller than latest mark price.
* SELL: &#x60;activationPrice&#x60; should be larger than latest mark price.
* Condition orders will be triggered when:
* If parameter&#x60;priceProtect&#x60;is sent as true:
* when price reaches the &#x60;stopPrice&#x60; ，the difference rate between &quot;MARK_PRICE&quot; and &quot;CONTRACT_PRICE&quot; cannot be larger than the &quot;triggerProtect&quot; of the symbol
* &quot;triggerProtect&quot; of a symbol can be got from &#x60;GET /fapi/v1/exchangeInfo&#x60;
* &#x60;STOP&#x60;, &#x60;STOP_MARKET&#x60;:
* BUY: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &gt;&#x3D; &#x60;stopPrice&#x60;
* SELL: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &lt;&#x3D; &#x60;stopPrice&#x60;
* &#x60;TAKE_PROFIT&#x60;, &#x60;TAKE_PROFIT_MARKET&#x60;:
* BUY: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &lt;&#x3D; &#x60;stopPrice&#x60;
* SELL: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &gt;&#x3D; &#x60;stopPrice&#x60;

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'position-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'reduce-only': {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'price-protect': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'activation-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'callback-rate': {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['strategyType'] && !options?.interactive) {
                    requiredParams.push('strategyType');
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
                'new-cm-conditional-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['side']) {
            questions.push({
                type: 'input',
                name: 'side',
                message: 'Input side:',
                validate: (input: string) => (input ? true : 'side cannot be empty'),
            });
        }

        if (options.interactive && !options?.['strategyType']) {
            questions.push({
                type: 'input',
                name: 'strategyType',
                message: 'Input strategyType:',
                validate: (input: string) => (input ? true : 'strategyType cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newCmConditionalOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'new-cm-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Place new CM order

* If &#x60;newOrderRespType&#x60; is sent as &#x60;RESULT&#x60; :
* &#x60;MARKET&#x60; order: the final FILLED result of the order will be return directly.
* &#x60;LIMIT&#x60; order with special &#x60;timeInForce&#x60;: the final status result of the order(FILLED or EXPIRED) will be returned directly.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'position-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                type: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'reduce-only': {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'price-match': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-order-resp-type': {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['type'] && !options?.interactive) {
                    requiredParams.push('type');
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
                'new-cm-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['side']) {
            questions.push({
                type: 'input',
                name: 'side',
                message: 'Input side:',
                validate: (input: string) => (input ? true : 'side cannot be empty'),
            });
        }

        if (options.interactive && !options?.['type']) {
            questions.push({
                type: 'input',
                name: 'type',
                message: 'Input type:',
                validate: (input: string) => (input ? true : 'type cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newCmOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'new-margin-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`New Margin Order

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                type: {
                    type: 'string',
                    group: 'Command Options:',
                },
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'quote-order-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-order-resp-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'side-effect-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'self-trade-prevention-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-repay-at-cancel': {
                    type: 'boolean',
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['type'] && !options?.interactive) {
                    requiredParams.push('type');
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
                'new-margin-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['side']) {
            questions.push({
                type: 'input',
                name: 'side',
                message: 'Input side:',
                validate: (input: string) => (input ? true : 'side cannot be empty'),
            });
        }

        if (options.interactive && !options?.['type']) {
            questions.push({
                type: 'input',
                name: 'type',
                message: 'Input type:',
                validate: (input: string) => (input ? true : 'type cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newMarginOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'new-um-conditional-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Place new UM conditional order

* Order with type &#x60;STOP/TAKE_PROFIT&#x60;, parameter &#x60;timeInForce&#x60; can be sent ( default &#x60;GTC&#x60;).
* Condition orders will be triggered when:
* &#x60;STOP&#x60;, &#x60;STOP_MARKET&#x60;:
* BUY: &quot;MARK_PRICE&quot;  &gt;&#x3D; &#x60;stopPrice&#x60;
* SELL: &quot;MARK_PRICE&quot; &lt;&#x3D; &#x60;stopPrice&#x60;
* &#x60;TAKE_PROFIT&#x60;, &#x60;TAKE_PROFIT_MARKET&#x60;:
* BUY: &quot;MARK_PRICE&quot; &lt;&#x3D; &#x60;stopPrice&#x60;
* SELL: &quot;MARK_PRICE&quot; &gt;&#x3D; &#x60;stopPrice&#x60;
* &#x60;TRAILING_STOP_MARKET&#x60;:
* BUY: the lowest mark price after order placed &#x60;&lt;&#x3D; &#x60;activationPrice&#x60;, and the latest mark price &gt;&#x60;&#x3D; the lowest mark price * (1 + &#x60;callbackRate&#x60;)
* SELL: the highest mark price after order placed &gt;&#x3D; &#x60;activationPrice&#x60;, and the latest mark price &lt;&#x3D; the highest mark price * (1 - &#x60;callbackRate&#x60;)
* For &#x60;TRAILING_STOP_MARKET&#x60;, if you got such error code. &#x60;{&quot;code&quot;: -2021, &quot;msg&quot;: &quot;Order would immediately trigger.&quot;}&#x60; means that the parameters you send do not meet the following requirements:
* BUY: &#x60;activationPrice&#x60; should be smaller than latest mark price.
* SELL: &#x60;activationPrice&#x60; should be larger than latest mark price.
* Condition orders will be triggered when:
* If parameter&#x60;priceProtect&#x60;is sent as true:
* when price reaches the &#x60;stopPrice&#x60; ，the difference rate between &quot;MARK_PRICE&quot; and &quot;CONTRACT_PRICE&quot; cannot be larger than the &quot;triggerProtect&quot; of the symbol
* &quot;triggerProtect&quot; of a symbol can be got from &#x60;GET /fapi/v1/exchangeInfo&#x60;
* &#x60;STOP&#x60;, &#x60;STOP_MARKET&#x60;:
* BUY: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &gt;&#x3D; &#x60;stopPrice&#x60;
* SELL: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &lt;&#x3D; &#x60;stopPrice&#x60;
* &#x60;TAKE_PROFIT&#x60;, &#x60;TAKE_PROFIT_MARKET&#x60;:
* BUY: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &lt;&#x3D; &#x60;stopPrice&#x60;
* SELL: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &gt;&#x3D; &#x60;stopPrice&#x60;
* &#x60;selfTradePreventionMode&#x60; is only effective when &#x60;timeInForce&#x60; set to &#x60;IOC&#x60; or &#x60;GTC&#x60; or &#x60;GTD&#x60;.
* In extreme market conditions, timeInForce &#x60;GTD&#x60; order auto cancel time might be delayed comparing to &#x60;goodTillDate&#x60;

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'position-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'reduce-only': {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'price-protect': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'activation-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'callback-rate': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'price-match': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'self-trade-prevention-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'good-till-date': {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['strategyType'] && !options?.interactive) {
                    requiredParams.push('strategyType');
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
                'new-um-conditional-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['side']) {
            questions.push({
                type: 'input',
                name: 'side',
                message: 'Input side:',
                validate: (input: string) => (input ? true : 'side cannot be empty'),
            });
        }

        if (options.interactive && !options?.['strategyType']) {
            questions.push({
                type: 'input',
                name: 'strategyType',
                message: 'Input strategyType:',
                validate: (input: string) => (input ? true : 'strategyType cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newUmConditionalOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'new-um-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Place new UM order

* If &#x60;newOrderRespType&#x60; is sent as &#x60;RESULT&#x60; :
* &#x60;MARKET&#x60; order: the final FILLED result of the order will be return directly.
* &#x60;LIMIT&#x60; order with special &#x60;timeInForce&#x60;: the final status result of the order(FILLED or EXPIRED) will be returned directly.
* &#x60;selfTradePreventionMode&#x60; is only effective when &#x60;timeInForce&#x60; set to &#x60;IOC&#x60; or &#x60;GTC&#x60; or &#x60;GTD&#x60;.
* In extreme market conditions, timeInForce &#x60;GTD&#x60; order auto cancel time might be delayed comparing to &#x60;goodTillDate&#x60;

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'position-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                type: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'reduce-only': {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-order-resp-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'price-match': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'self-trade-prevention-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'good-till-date': {
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['type'] && !options?.interactive) {
                    requiredParams.push('type');
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
                'new-um-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['side']) {
            questions.push({
                type: 'input',
                name: 'side',
                message: 'Input side:',
                validate: (input: string) => (input ? true : 'side cannot be empty'),
            });
        }

        if (options.interactive && !options?.['type']) {
            questions.push({
                type: 'input',
                name: 'type',
                message: 'Input type:',
                validate: (input: string) => (input ? true : 'type cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newUmOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-cm-conditional-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query All CM Conditional Orders

* These orders will not be found:
* order strategyStatus is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 7 days &lt; current time
* The query time period must be less than 7 days( default as the recent 7 days).

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'strategy-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
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
                'query-all-cm-conditional-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllCmConditionalOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-cm-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all account CM orders; active, canceled, or filled.

* Either &#x60;symbol&#x60; or &#x60;pair&#x60; must be sent.
* If &#x60;orderId&#x60; is set, it will get orders &gt;&#x3D; that orderId. Otherwise most recent orders are returned.
* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 3 days &lt; current time

Weight: 20 with symbol, 40 with pair`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                pair: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-all-cm-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllCmOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-current-cm-open-conditional-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all open conditional orders on a symbol. **Careful** when accessing this with no symbol.

* If the symbol is not sent, orders for all symbols will be returned in an array.

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'query-all-current-cm-open-conditional-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllCurrentCmOpenConditionalOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-current-cm-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all open orders on a symbol.

* If the symbol is not sent, orders for all symbols will be returned in an array.

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted
Careful when accessing this with no symbol.`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            pair: {
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
                'query-all-current-cm-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllCurrentCmOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-current-um-open-conditional-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all open conditional orders on a symbol.

* If the symbol is not sent, orders for all symbols will be returned in an array.

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted
Careful when accessing this with no symbol.`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'query-all-current-um-open-conditional-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllCurrentUmOpenConditionalOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-current-um-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all open orders on a symbol.


* If the symbol is not sent, orders for all symbols will be returned in an array.

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'query-all-current-um-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllCurrentUmOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-margin-account-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query All Margin Account Orders

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-all-margin-account-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllMarginAccountOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-um-conditional-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query All UM Conditional Orders

* These orders will not be found:
* order strategyStatus is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 7 days &lt; current time
* The query time period must be less than 7 days( default as the recent 7 days).

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'strategy-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
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
                'query-all-um-conditional-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllUmConditionalOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-all-um-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all account UM orders; active, canceled, or filled.
* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 3 days &lt; current time

* If &#x60;orderId&#x60; is set, it will get orders &gt;&#x3D; that orderId. Otherwise most recent orders are returned.
* The query time period must be less then 7 days( default as the recent 7 days).

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-all-um-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAllUmOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-cm-conditional-order-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query CM Conditional Order History


* Either &#x60;strategyId&#x60; or &#x60;newClientStrategyId&#x60; must be sent.
* &#x60;NEW&#x60; orders will not be found.
* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 7 days &lt; current time

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-strategy-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-cm-conditional-order-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCmConditionalOrderHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-cm-modify-order-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get order modification history

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent, and the &#x60;orderId&#x60; will prevail if both are sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-cm-modify-order-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCmModifyOrderHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-cm-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Check an CM order&#39;s status.

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.
* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 3 days &lt; current time

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-cm-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCmOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-current-cm-open-conditional-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Current CM Open Conditional Order

* Either &#x60;strategyId&#x60; or &#x60;newClientStrategyId&#x60; must be sent.
* If the queried order has been triggered, cancelled or expired, the error message &quot;Order does not exist&quot; will be returned.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-strategy-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-current-cm-open-conditional-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentCmOpenConditionalOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-current-cm-open-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query current CM open order

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.
* If the queried order has been filled or cancelled, the error message &quot;Order does not exist&quot; will be returned.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-current-cm-open-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentCmOpenOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-current-margin-open-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Current Margin Open Order

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-current-margin-open-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentMarginOpenOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-current-um-open-conditional-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Current UM Open Conditional Order

* Either &#x60;strategyId&#x60; or &#x60;newClientStrategyId&#x60; must be sent.
* If the queried order has been &#x60;CANCELED&#x60;, &#x60;TRIGGERED&#x60; or &#x60;EXPIRED&#x60;, the error message &quot;Order does not exist&quot; will be returned.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-strategy-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-current-um-open-conditional-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentUmOpenConditionalOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-current-um-open-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query current UM open order


* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.
* If the queried order has been filled or cancelled, the error message &quot;Order does not exist&quot; will be returned.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-current-um-open-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentUmOpenOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-margin-account-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Account Order

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-margin-account-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryMarginAccountOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-margin-accounts-all-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query all OCO for a specific margin account based on provided optional parameters

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'from-id': {
                describe: decodeSelectedEntities(
                    'Trade id to fetch from. Default gets most recent trades.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
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
                'query-margin-accounts-all-oco is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryMarginAccountsAllOco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-margin-accounts-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Retrieves a specific OCO based on provided optional parameters

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-list-id': {
                describe: decodeSelectedEntities(
                    'Either &#x60;orderListId&#x60; or &#x60;listClientOrderId&#x60; must be provided'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'orig-client-order-id': {
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
                'query-margin-accounts-oco is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryMarginAccountsOco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-margin-accounts-open-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Account&#39;s Open OCO

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
                'query-margin-accounts-open-oco is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryMarginAccountsOpenOco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-um-conditional-order-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query UM Conditional Order History

* Either &#x60;strategyId&#x60; or &#x60;newClientStrategyId&#x60; must be sent.
* &#x60;NEW&#x60; orders will not be found.
* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 7 days &lt; current time

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-strategy-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-um-conditional-order-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUmConditionalOrderHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-um-modify-order-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get order modification history

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent, and the &#x60;orderId&#x60; will prevail if both are sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-um-modify-order-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUmModifyOrderHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-um-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Check an UM order&#39;s status.

* These orders will not be found:
* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 3 days &lt; current time

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'query-um-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUmOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-users-cm-force-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query User&#39;s CM Force Orders

* If &quot;autoCloseType&quot; is not sent, orders with both of the types will be returned
* If &quot;startTime&quot; is not sent, data within 7 days before &quot;endTime&quot; can be queried

Weight: 20 with symbol, 50 without symbol`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'auto-close-type': {
                describe: decodeSelectedEntities(
                    '&#x60;LIQUIDATION&#x60; for liquidation orders, &#x60;ADL&#x60; for ADL orders.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
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
                'query-users-cm-force-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUsersCmForceOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-users-margin-force-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query user&#39;s margin force orders

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
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
                'query-users-margin-force-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUsersMarginForceOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'query-users-um-force-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query User&#39;s UM Force Orders

* If &#x60;autoCloseType&#x60; is not sent, orders with both of the types will be returned
* If &#x60;startTime&#x60; is not sent, data within 7 days before &#x60;endTime&#x60; can be queried

Weight: 20 with symbol, 50 without symbol`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'auto-close-type': {
                describe: decodeSelectedEntities(
                    '&#x60;LIQUIDATION&#x60; for liquidation orders, &#x60;ADL&#x60; for ADL orders.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('Timestamp in ms to get funding until INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
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
                'query-users-um-force-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUsersUmForceOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'toggle-bnb-burn-on-um-futures-trade',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Change user&#39;s BNB Fee Discount for UM Futures (Fee Discount On or Fee Discount Off ) on ***EVERY symbol***


* The BNB would not be collected from UM-PM account to the Portfolio Margin account.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'fee-burn': {
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

                if (!options?.['feeBurn'] && !options?.interactive) {
                    requiredParams.push('feeBurn');
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
                'toggle-bnb-burn-on-um-futures-trade is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['feeBurn']) {
            questions.push({
                type: 'input',
                name: 'feeBurn',
                message: 'Input feeBurn:',
                validate: (input: string) => (input ? true : 'feeBurn cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.toggleBnbBurnOnUmFuturesTrade(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'um-account-trade-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get trades for a specific account and UM symbol.


* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last &#39;7 days&#39; data will be returned.
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 7 days.
* The parameter &#x60;fromId&#x60; cannot be sent with &#x60;startTime&#x60; or &#x60;endTime&#x60;.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get funding until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-id': {
                    describe: decodeSelectedEntities(
                        'Trade id to fetch from. Default gets most recent trades.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
                'um-account-trade-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.umAccountTradeList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'um-position-adl-quantile-estimation',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query UM Position ADL Quantile Estimation

* Values update every 30s.
* Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
* For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode, &quot;LONG&quot;, &quot;SHORT&quot;, and &quot;BOTH&quot; will be returned to show the positions&#39; adl quantiles of different position sides.
* If the positions of the symbol are crossed margined in Hedge Mode:
* &quot;HEDGE&quot; as a sign will be returned instead of &quot;BOTH&quot;;
* A same value caculated on unrealized pnls on long and short sides&#39; positions will be shown for &quot;LONG&quot; and &quot;SHORT&quot; when there are positions in both of long and short sides.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'um-position-adl-quantile-estimation is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.umPositionAdlQuantileEstimation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'close-user-data-stream',
    describe: decodeSelectedEntities(`Close out a user data stream.

Weight: 1`),
    handler: async () => {
        try {
            await client.restAPI.closeUserDataStream();
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'keepalive-user-data-stream',
    describe:
        decodeSelectedEntities(`Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes. It&#39;s recommended to send a ping about every 60 minutes.

Weight: 1`),
    handler: async () => {
        try {
            await client.restAPI.keepaliveUserDataStream();
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingPortfolioMarginCommands.push({
    command: 'start-user-data-stream',
    describe:
        decodeSelectedEntities(`Start a new user data stream. The stream will close after 60 minutes unless a keepalive is sent. If the account has an active &#x60;listenKey&#x60;, that &#x60;listenKey&#x60; will be returned and its validity will be extended for 60 minutes.

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.startUserDataStream();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'derivatives-portfolio-margin',
    description: 'Binance Derivatives Trading Portfolio Margin REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli derivatives-portfolio-margin <command> [options]');
        derivativesTradingPortfolioMarginCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
