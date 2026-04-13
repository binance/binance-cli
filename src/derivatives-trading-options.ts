import {
    DerivativesTradingOptions,
    DERIVATIVES_TRADING_OPTIONS_REST_API_PROD_URL,
    DERIVATIVES_TRADING_OPTIONS_REST_API_TESTNET_URL,
} from '@binance/derivatives-trading-options';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('derivatives-trading-options');

const stdinObj: any = readStdinObj();

let basePath = DERIVATIVES_TRADING_OPTIONS_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'derivatives-options');

if (process.env.BINANCE_DERIVATIVES_OPTIONS_BASE_PATH) {
    basePath = process.env.BINANCE_DERIVATIVES_OPTIONS_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
} else if (configurationRestAPI && configurationRestAPI['env']) {
    switch (configurationRestAPI['env']) {
        case 'testnet':
            basePath = DERIVATIVES_TRADING_OPTIONS_REST_API_TESTNET_URL;
            break;
    }
}

let client;
if (configurationRestAPI !== null) {
    client = new DerivativesTradingOptions({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new DerivativesTradingOptions({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const derivativesTradingOptionsCommands: any[] = [];

derivativesTradingOptionsCommands.push({
    command: 'account-funding-flow',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query account funding flows.


* Only support querying data in the past 3 months

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                currency: {
                    describe: decodeSelectedEntities('Asset type, only support USDT  as of now'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'record-id': {
                    describe: decodeSelectedEntities(
                        'Return the recordId and subsequent data, the latest data is returned by default, e.g 100000'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'Number of result sets returned Default:100 Max:1000'
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

                if (!options?.['currency'] && !options?.interactive) {
                    requiredParams.push('currency');
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
            console.error(
                'account-funding-flow is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.currency) {
            questions.push({
                type: 'input',
                name: 'currency',
                message: 'Input currency:',
                validate: (input: string) => (input ? true : 'currency cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountFundingFlow(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'option-margin-account-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current account information.

Weight: 3`),
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
            console.error(
                'option-margin-account-information is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.optionMarginAccountInformation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'check-server-time',
    describe:
        decodeSelectedEntities(`Test connectivity to the Rest API and get the current server time.

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.checkServerTime();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'exchange-information',
    describe: decodeSelectedEntities(`Current exchange trading rules and symbol information

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.exchangeInformation();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'historical-exercise-records',
    describe: decodeSelectedEntities(`Get historical exercise records.
* REALISTIC_VALUE_STRICKEN -&gt; Exercised
* EXTRINSIC_VALUE_EXPIRED -&gt; Expired OTM

Weight: 3`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            underlying: {
                describe: decodeSelectedEntities('underlying, e.g BTCUSDT'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities(
                    'Number of result sets returned Default:100 Max:1000'
                ),
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
            const response = await client.restAPI.historicalExerciseRecords(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'index-price',
    describe: decodeSelectedEntities(`Get spot index price for option underlying.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                underlying: {
                    describe: decodeSelectedEntities('Option underlying, e.g BTCUSDT'),
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

                if (!options?.['underlying'] && !options?.interactive) {
                    requiredParams.push('underlying');
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

        if (options.interactive && !options.underlying) {
            questions.push({
                type: 'input',
                name: 'underlying',
                message: 'Input underlying:',
                validate: (input: string) => (input ? true : 'underlying cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.indexPrice(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'kline-candlestick-data',
    describe: decodeSelectedEntities(`Kline/candlestick bars for an option symbol.
Klines are uniquely identified by their open time.

* If startTime and endTime are not sent, the most recent klines are returned.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
                    describe: decodeSelectedEntities('Time interval'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'Number of result sets returned Default:100 Max:1000'
                    ),
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

                if (!options?.['interval'] && !options?.interactive) {
                    requiredParams.push('interval');
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

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (options.interactive && !options.interval) {
            questions.push({
                type: 'input',
                name: 'interval',
                message: 'Input interval:',
                validate: (input: string) => (input ? true : 'interval cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.klineCandlestickData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'open-interest',
    describe:
        decodeSelectedEntities(`Get open interest for specific underlying asset on specific expiration date.

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'underlying-asset': {
                    describe: decodeSelectedEntities('underlying asset, e.g ETH/BTC'),
                    type: 'string',
                    group: 'Command Options:',
                },
                expiration: {
                    describe: decodeSelectedEntities('expiration date, e.g 221225'),
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

                if (!options?.['underlyingAsset'] && !options?.interactive) {
                    requiredParams.push('underlyingAsset');
                }

                if (!options?.['expiration'] && !options?.interactive) {
                    requiredParams.push('expiration');
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

        if (options.interactive && !options.underlyingAsset) {
            questions.push({
                type: 'input',
                name: 'underlyingAsset',
                message: 'Input underlyingAsset:',
                validate: (input: string) => (input ? true : 'underlyingAsset cannot be empty'),
            });
        }
        if (options.interactive && !options.expiration) {
            questions.push({
                type: 'input',
                name: 'expiration',
                message: 'Input expiration:',
                validate: (input: string) => (input ? true : 'expiration cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.openInterest(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'option-mark-price',
    describe: decodeSelectedEntities(`Option mark price and greek info.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
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
            const response = await client.restAPI.optionMarkPrice(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'order-book',
    describe: decodeSelectedEntities(`Check orderbook depth on specific symbol

Weight: limit         | weight
------------  | ------------
5, 10, 20, 50 | 1
100           | 5
500           | 10
1000          | 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'Number of result sets returned Default:100 Max:1000'
                    ),
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
            const response = await client.restAPI.orderBook(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'recent-block-trades-list',
    describe: decodeSelectedEntities(`Get recent block trades

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities(
                    'Number of result sets returned Default:100 Max:1000'
                ),
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
            const response = await client.restAPI.recentBlockTradesList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'recent-trades-list',
    describe: decodeSelectedEntities(`Get recent market trades

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'Number of result sets returned Default:100 Max:1000'
                    ),
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
            const response = await client.restAPI.recentTradesList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
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

derivativesTradingOptionsCommands.push({
    command: 'ticker24hr-price-change-statistics',
    describe: decodeSelectedEntities(`24 hour rolling window price change statistics.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
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
            const response = await client.restAPI.ticker24hrPriceChangeStatistics(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'accept-block-trade-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Accept a block trade order

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'block-order-matching-key': {
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

                if (!options?.['blockOrderMatchingKey'] && !options?.interactive) {
                    requiredParams.push('blockOrderMatchingKey');
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
            console.error(
                'accept-block-trade-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['blockOrderMatchingKey']) {
            questions.push({
                type: 'input',
                name: 'blockOrderMatchingKey',
                message: 'Input blockOrderMatchingKey:',
                validate: (input: string) =>
                    input ? true : 'blockOrderMatchingKey cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.acceptBlockTradeOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'account-block-trade-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Gets block trades for a specific account.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'end-time': {
                describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                type: 'string',
                group: 'Command Options:',
            },
            underlying: {
                describe: decodeSelectedEntities('underlying, e.g BTCUSDT'),
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
            console.error(
                'account-block-trade-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountBlockTradeList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'cancel-block-trade-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel a block trade order.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'block-order-matching-key': {
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

                if (!options?.['blockOrderMatchingKey'] && !options?.interactive) {
                    requiredParams.push('blockOrderMatchingKey');
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
            console.error(
                'cancel-block-trade-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.blockOrderMatchingKey) {
            questions.push({
                type: 'input',
                name: 'blockOrderMatchingKey',
                message: 'Input blockOrderMatchingKey:',
                validate: (input: string) =>
                    input ? true : 'blockOrderMatchingKey cannot be empty',
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            await client.restAPI.cancelBlockTradeOrder(options);
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'extend-block-trade-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Extends a block trade expire time by 30 mins from the current time.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'block-order-matching-key': {
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

                if (!options?.['blockOrderMatchingKey'] && !options?.interactive) {
                    requiredParams.push('blockOrderMatchingKey');
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
            console.error(
                'extend-block-trade-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['blockOrderMatchingKey']) {
            questions.push({
                type: 'input',
                name: 'blockOrderMatchingKey',
                message: 'Input blockOrderMatchingKey:',
                validate: (input: string) =>
                    input ? true : 'blockOrderMatchingKey cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.extendBlockTradeOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'new-block-trade-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send in a new block trade order.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                liquidity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                legs: {
                    type: 'array',
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

                if (!options?.['liquidity'] && !options?.interactive) {
                    requiredParams.push('liquidity');
                }

                if (!options?.['legs'] && !options?.interactive) {
                    requiredParams.push('legs');
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
            console.error(
                'new-block-trade-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['liquidity']) {
            questions.push({
                type: 'input',
                name: 'liquidity',
                message: 'Input liquidity:',
                validate: (input: string) => (input ? true : 'liquidity cannot be empty'),
            });
        }

        if (options.interactive && !options?.['legs']) {
            questions.push({
                type: 'input',
                name: 'legs',
                message: 'Input legs:',
                validate: (input: string) => (input ? true : 'legs cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newBlockTradeOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'query-block-trade-details',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query block trade details; returns block trade details from counterparty&#39;s perspective.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'block-order-matching-key': {
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

                if (!options?.['blockOrderMatchingKey'] && !options?.interactive) {
                    requiredParams.push('blockOrderMatchingKey');
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
            console.error(
                'query-block-trade-details is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.blockOrderMatchingKey) {
            questions.push({
                type: 'input',
                name: 'blockOrderMatchingKey',
                message: 'Input blockOrderMatchingKey:',
                validate: (input: string) =>
                    input ? true : 'blockOrderMatchingKey cannot be empty',
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryBlockTradeDetails(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'query-block-trade-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Check block trade order status.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'block-order-matching-key': {
                describe: decodeSelectedEntities(
                    'If specified, returns the specific block trade associated with the blockOrderMatchingKey'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                type: 'string',
                group: 'Command Options:',
            },
            underlying: {
                describe: decodeSelectedEntities('underlying, e.g BTCUSDT'),
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
            console.error(
                'query-block-trade-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryBlockTradeOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'auto-cancel-all-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This endpoint resets the time from which the countdown will begin to the time this messaged is received.  It should be called repeatedly as heartbeats.  Multiple heartbeats can be updated at once by specifying the underlying symbols as a list (ex. BTCUSDT,ETHUSDT) in the underlyings parameter.

* The response will only include underlying symbols where the heartbeat has been successfully updated.

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                underlyings: {
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

                if (!options?.['underlyings'] && !options?.interactive) {
                    requiredParams.push('underlyings');
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
            console.error(
                'auto-cancel-all-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['underlyings']) {
            questions.push({
                type: 'input',
                name: 'underlyings',
                message: 'Input underlyings:',
                validate: (input: string) => (input ? true : 'underlyings cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.autoCancelAllOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'get-auto-cancel-all-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This endpoint returns the auto-cancel parameters for each underlying symbol. Note only active auto-cancel parameters will be returned, if countdownTime is set to 0 (ie. countdownTime has been turned off), the underlying symbol and corresponding countdownTime parameter will not be returned in the response.

* countdownTime &#x3D; 0 means the function is disabled.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            underlying: {
                describe: decodeSelectedEntities('underlying, e.g BTCUSDT'),
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
            console.error(
                'get-auto-cancel-all-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getAutoCancelAllOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'get-market-maker-protection-config',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get config for MMP.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            underlying: {
                describe: decodeSelectedEntities('underlying, e.g BTCUSDT'),
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
            console.error(
                'get-market-maker-protection-config is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getMarketMakerProtectionConfig(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'reset-market-maker-protection-config',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Reset MMP, start MMP order again.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            underlying: {
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
            console.error(
                'reset-market-maker-protection-config is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.resetMarketMakerProtectionConfig(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'set-auto-cancel-all-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This endpoint sets the parameters of the auto-cancel feature which cancels all open orders (both market maker protection and non market maker protection order types) of the underlying symbol at the end of the specified countdown time period if no heartbeat message is sent.  After the countdown time period, all open orders will be cancelled and new orders will be rejected with error code -2010 until either a heartbeat message is sent or the auto-cancel feature is turned off by setting countdownTime to 0.


* This rest endpoint sets up the parameters to cancel your open orders in case of an outage or disconnection.
* Example usage:
Call this endpoint with a countdownTime value of 10000 (10 seconds) to turn on the auto-cancel feature. If the corresponding countdownCancelAllHeartBeat endpoint is not called within 10 seconds with the specified underlying symbol, all open orders of the specified symbol will be automatically canceled. If this endpoint is called with an countdownTime of 0, the countdown timer will be stopped.
* The system will check all countdowns approximately every 100 milliseconds, **please note that sufficient redundancy should be considered when using this function**. We do not recommend setting the countdown time to be too precise or too small.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                underlying: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'countdown-time': {
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

                if (!options?.['underlying'] && !options?.interactive) {
                    requiredParams.push('underlying');
                }

                if (!options?.['countdownTime'] && !options?.interactive) {
                    requiredParams.push('countdownTime');
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
            console.error(
                'set-auto-cancel-all-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['underlying']) {
            questions.push({
                type: 'input',
                name: 'underlying',
                message: 'Input underlying:',
                validate: (input: string) => (input ? true : 'underlying cannot be empty'),
            });
        }

        if (options.interactive && !options?.['countdownTime']) {
            questions.push({
                type: 'input',
                name: 'countdownTime',
                message: 'Input countdownTime:',
                validate: (input: string) => (input ? true : 'countdownTime cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.setAutoCancelAllOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'set-market-maker-protection-config',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Set config for MMP.
Market Maker Protection(MMP) is a set of protection mechanism for option market maker, this mechanism is able to prevent mass trading in short period time. Once market maker&#39;s account branches the threshold, the Market Maker Protection will be triggered. When Market Maker Protection triggers, all the current MMP orders will be canceled, new MMP orders will be rejected. Market maker can use this time to reevaluate market and modify order price.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            underlying: {
                type: 'string',
                group: 'Command Options:',
            },
            'window-time-in-milliseconds': {
                type: 'string',
                group: 'Command Options:',
            },
            'frozen-time-in-milliseconds': {
                type: 'string',
                group: 'Command Options:',
            },
            'qty-limit': {
                type: 'string',
                group: 'Command Options:',
            },
            'delta-limit': {
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
            console.error(
                'set-market-maker-protection-config is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.setMarketMakerProtectionConfig(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'account-trade-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get trades for a specific account and symbol.

* Only support querying trades in the past 3 months

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                type: 'string',
                group: 'Command Options:',
            },
            'from-id': {
                describe: decodeSelectedEntities(
                    'Trade id to fetch from. Default gets most recent trades, e.g 4611875134427365376'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities(
                    'Number of result sets returned Default:100 Max:1000'
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
            console.error(
                'account-trade-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountTradeList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'cancel-all-option-orders-by-underlying',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel all active orders on specified underlying.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                underlying: {
                    describe: decodeSelectedEntities('Option underlying, e.g BTCUSDT'),
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

                if (!options?.['underlying'] && !options?.interactive) {
                    requiredParams.push('underlying');
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
            console.error(
                'cancel-all-option-orders-by-underlying is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.underlying) {
            questions.push({
                type: 'input',
                name: 'underlying',
                message: 'Input underlying:',
                validate: (input: string) => (input ? true : 'underlying cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelAllOptionOrdersByUnderlying(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'cancel-all-option-orders-on-specific-symbol',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel all active order on a symbol.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
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
            console.error(
                'cancel-all-option-orders-on-specific-symbol is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.cancelAllOptionOrdersOnSpecificSymbol(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'cancel-multiple-option-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel multiple orders.

* At least one instance of &#x60;orderId&#x60; and &#x60;clientOrderId&#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-ids': {
                    describe: decodeSelectedEntities(
                        'Order ID, e.g [4611875134427365377,4611875134427365378]'
                    ),
                    type: 'array',
                    group: 'Command Options:',
                },
                'client-order-ids': {
                    describe: decodeSelectedEntities(
                        'User-defined order ID, e.g [\&quot;my_id_1\&quot;,\&quot;my_id_2\&quot;]'
                    ),
                    type: 'array',
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
            console.error(
                'cancel-multiple-option-orders is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.cancelMultipleOptionOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'cancel-option-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an active order.

* At least one instance of &#x60;orderId&#x60; and &#x60;clientOrderId&#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities('Order ID, e.g 4611875134427365377'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-order-id': {
                    describe: decodeSelectedEntities('User-defined order ID, e.g 10000'),
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
            console.error(
                'cancel-option-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.cancelOptionOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'new-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send a new order.

Weight: 0`),
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
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'reduce-only': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'post-only': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'new-order-resp-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-mmp': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'self-trade-prevention-mode': {
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

                if (!options?.['quantity'] && !options?.interactive) {
                    requiredParams.push('quantity');
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
            console.error(
                'new-order is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['quantity']) {
            questions.push({
                type: 'input',
                name: 'quantity',
                message: 'Input quantity:',
                validate: (input: string) => (input ? true : 'quantity cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'option-position-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current position information.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
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
            console.error(
                'option-position-information is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.optionPositionInformation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'place-multiple-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send multiple option orders.

* Parameter rules are same with New Order
* Batch orders are processed concurrently, and the order of matching is not guaranteed.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                orders: {
                    type: 'array',
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

                if (!options?.['orders'] && !options?.interactive) {
                    requiredParams.push('orders');
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
            console.error(
                'place-multiple-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['orders']) {
            questions.push({
                type: 'input',
                name: 'orders',
                message: 'Input orders:',
                validate: (input: string) => (input ? true : 'orders cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.placeMultipleOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'query-current-open-option-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query current all open orders, status: ACCEPTED PARTIALLY_FILLED

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                type: 'string',
                group: 'Command Options:',
            },
            'order-id': {
                describe: decodeSelectedEntities('Order ID, e.g 4611875134427365377'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
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
            console.error(
                'query-current-open-option-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentOpenOptionOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'query-option-order-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query all finished orders within 5 days, finished status: CANCELLED FILLED REJECTED.

Weight: 3`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities('Order ID, e.g 4611875134427365377'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'Number of result sets returned Default:100 Max:1000'
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
            console.error(
                'query-option-order-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryOptionOrderHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'query-single-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Check an order status.

* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;REJECTED&#x60;, **AND**
* order has NO filled trade, **AND**
* created time + 3 days &lt; current time


* Either &#x60;orderId&#x60; or &#x60;clientOrderId &#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities('Order ID, e.g 4611875134427365377'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-order-id': {
                    describe: decodeSelectedEntities('User-defined order ID, e.g 10000'),
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
            console.error(
                'query-single-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.querySingleOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'user-commission',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get account commission.

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
            console.error(
                'user-commission is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.userCommission(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
    command: 'user-exercise-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get account exercise records.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Option trading pair, e.g BTC-200730-9000-C'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('Start Time, e.g 1593511200000'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('End Time, e.g 1593512200000'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities(
                    'Number of result sets returned Default:100 Max:1000'
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
            console.error(
                'user-exercise-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.userExerciseRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingOptionsCommands.push({
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

derivativesTradingOptionsCommands.push({
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

derivativesTradingOptionsCommands.push({
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
    command: 'derivatives-options',
    description: 'Binance Derivatives Trading Options REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli derivatives-options <command> [options]');
        derivativesTradingOptionsCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
