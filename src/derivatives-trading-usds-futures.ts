import {
    DerivativesTradingUsdsFutures,
    DERIVATIVES_TRADING_USDS_FUTURES_REST_API_PROD_URL,
    DERIVATIVES_TRADING_USDS_FUTURES_REST_API_DEMO_URL,
    DERIVATIVES_TRADING_USDS_FUTURES_REST_API_TESTNET_URL,
} from '@binance/derivatives-trading-usds-futures';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('derivatives-trading-usds-futures');

const stdinObj: any = readStdinObj();

let basePath = DERIVATIVES_TRADING_USDS_FUTURES_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'futures-usds');

if (process.env.BINANCE_FUTURES_USDS_BASE_PATH) {
    basePath = process.env.BINANCE_FUTURES_USDS_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
} else if (configurationRestAPI && configurationRestAPI['env']) {
    switch (configurationRestAPI['env']) {
        case 'testnet':
            basePath = DERIVATIVES_TRADING_USDS_FUTURES_REST_API_TESTNET_URL;
            break;
        case 'demo':
            basePath = DERIVATIVES_TRADING_USDS_FUTURES_REST_API_DEMO_URL;
            break;
    }
}

let client;
if (configurationRestAPI !== null) {
    client = new DerivativesTradingUsdsFutures({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new DerivativesTradingUsdsFutures({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const derivativesTradingUsdsFuturesCommands: any[] = [];

derivativesTradingUsdsFuturesCommands.push({
    command: 'account-information-v2',
    describe:
        decodeSelectedEntities(`Get current account information. User in single-asset/ multi-assets mode will see different value, see comments in response section for detail.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('account-information-v2 is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountInformationV2({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'account-information-v3',
    describe:
        decodeSelectedEntities(`Get current account information. User in single-asset/ multi-assets mode will see different value, see comments in response section for detail.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('account-information-v3 is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountInformationV3({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'futures-account-balance-v2',
    describe: decodeSelectedEntities(`Query account balance info

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'futures-account-balance-v2 is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.futuresAccountBalanceV2({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'futures-account-balance-v3',
    describe: decodeSelectedEntities(`Query account balance info

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'futures-account-balance-v3 is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.futuresAccountBalanceV3({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'futures-account-configuration',
    describe: decodeSelectedEntities(`Query account configuration

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'futures-account-configuration is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.futuresAccountConfiguration({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'futures-trading-quantitative-rules-indicators',
    describe:
        decodeSelectedEntities(`Futures trading quantitative rules indicators, for more information on this, please refer to the [Futures Trading Quantitative Rules](https://www.binance.com/en/support/faq/4f462ebe6ff445d4a170be7d9e897272)

Weight: - 1 for a single symbol
- 10 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'futures-trading-quantitative-rules-indicators is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.futuresTradingQuantitativeRulesIndicators({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-bnb-burn-status',
    describe:
        decodeSelectedEntities(`Get user&#39;s BNB Fee Discount (Fee Discount On or Fee Discount Off )

Weight: 30`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('get-bnb-burn-status is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBnbBurnStatus({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-current-multi-assets-mode',
    describe:
        decodeSelectedEntities(`Get user&#39;s Multi-Assets mode (Multi-Assets Mode or Single-Asset Mode) on ***Every symbol***

Weight: 30`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-current-multi-assets-mode is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCurrentMultiAssetsMode({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-current-position-mode',
    describe:
        decodeSelectedEntities(`Get user&#39;s position mode (Hedge Mode or One-way Mode ) on ***EVERY symbol***

Weight: 30`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-current-position-mode is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCurrentPositionMode({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-download-id-for-futures-order-history',
    describe: decodeSelectedEntities(`Get Download Id For Futures Order History

* Request Limitation is 10 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 1000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'start-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.startTime && !stdinObj?.startTime && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.endTime && !stdinObj?.endTime && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-download-id-for-futures-order-history is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.getDownloadIdForFuturesOrderHistory({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-download-id-for-futures-trade-history',
    describe: decodeSelectedEntities(`Get download id for futures trade history

* Request Limitation is 5 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 1000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'start-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.startTime && !stdinObj?.startTime && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.endTime && !stdinObj?.endTime && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-download-id-for-futures-trade-history is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.getDownloadIdForFuturesTradeHistory({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-download-id-for-futures-transaction-history',
    describe: decodeSelectedEntities(`Get download id for futures transaction history

* Request Limitation is 5 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 1000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'start-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.startTime && !stdinObj?.startTime && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.endTime && !stdinObj?.endTime && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-download-id-for-futures-transaction-history is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.getDownloadIdForFuturesTransactionHistory({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-futures-order-history-download-link-by-id',
    describe: decodeSelectedEntities(`Get futures order history download link by Id

* Download link expiration: 24h

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'download-id': {
                    describe: decodeSelectedEntities('get by download id api'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.downloadId && !stdinObj?.downloadId && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-futures-order-history-download-link-by-id is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.getFuturesOrderHistoryDownloadLinkById({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-futures-trade-download-link-by-id',
    describe: decodeSelectedEntities(`Get futures trade download link by Id

* Download link expiration: 24h

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'download-id': {
                    describe: decodeSelectedEntities('get by download id api'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.downloadId && !stdinObj?.downloadId && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-futures-trade-download-link-by-id is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.getFuturesTradeDownloadLinkById({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-futures-transaction-history-download-link-by-id',
    describe: decodeSelectedEntities(`Get futures transaction history download link by Id

* Download link expiration: 24h

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'download-id': {
                    describe: decodeSelectedEntities('get by download id api'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.downloadId && !stdinObj?.downloadId && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-futures-transaction-history-download-link-by-id is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.getFuturesTransactionHistoryDownloadLinkById({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-income-history',
    describe: decodeSelectedEntities(`Query income history

* If neither &#x60;startTime&#x60; nor &#x60;endTime&#x60; is sent, the recent 7-day data will be returned.
* If &#x60;incomeType &#x60; is not sent, all kinds of flow will be returned
* &quot;trandId&quot; is unique in the same incomeType for a user
* Income history only contains data for the last three months

Weight: 30`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'income-type': {
                describe: decodeSelectedEntities(
                    'TRANSFER, WELCOME_BONUS, REALIZED_PNL, FUNDING_FEE, COMMISSION, INSURANCE_CLEAR, REFERRAL_KICKBACK, COMMISSION_REBATE, API_REBATE, CONTEST_REWARD, CROSS_COLLATERAL_TRANSFER, OPTIONS_PREMIUM_FEE, OPTIONS_SETTLE_PROFIT, INTERNAL_TRANSFER, AUTO_EXCHANGE, DELIVERED_SETTELMENT, COIN_SWAP_DEPOSIT, COIN_SWAP_WITHDRAW, POSITION_LIMIT_INCREASE_FEE, STRATEGY_UMFUTURES_TRANSFER，FEE_RETURN，BFUSD_REWARD'
                ),
                type: 'string',
            },
            'start-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'end-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            page: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('get-income-history is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getIncomeHistory({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'notional-and-leverage-brackets',
    describe: decodeSelectedEntities(`Query user notional and leverage bracket on speicfic symbol

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'notional-and-leverage-brackets is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.notionalAndLeverageBrackets({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'query-user-rate-limit',
    describe: decodeSelectedEntities(`Query User Rate Limit

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('query-user-rate-limit is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUserRateLimit({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'symbol-configuration',
    describe: decodeSelectedEntities(`Get current account symbol configuration.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('symbol-configuration is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.symbolConfiguration({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'toggle-bnb-burn-on-futures-trade',
    describe:
        decodeSelectedEntities(`Change user&#39;s BNB Fee Discount (Fee Discount On or Fee Discount Off ) on ***EVERY symbol***

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'toggleBnbBurnOnFuturesTradeRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'toggle-bnb-burn-on-futures-trade is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input toggleBnbBurnOnFuturesTradeRequest:',
                validate: (input: string) =>
                    input ? true : 'toggleBnbBurnOnFuturesTradeRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.toggleBnbBurnOnFuturesTrade(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'user-commission-rate',
    describe: decodeSelectedEntities(`Get User Commission Rate

Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('user-commission-rate is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.userCommissionRate({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'accept-the-offered-quote',
    describe: decodeSelectedEntities(`Accept the offered quote by quote ID.

Weight: 200(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'acceptTheOfferedQuoteRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'accept-the-offered-quote is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input acceptTheOfferedQuoteRequest:',
                validate: (input: string) =>
                    input ? true : 'acceptTheOfferedQuoteRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.acceptTheOfferedQuote(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'list-all-convert-pairs',
    describe:
        decodeSelectedEntities(`Query for all convertible token pairs and the tokens’ respective upper/lower limits

* User needs to supply either or both of the input parameter
* If not defined for both fromAsset and toAsset, only partial token pairs will be returned
* Asset BNFCR is only available to convert for MICA region users.

Weight: 20(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'from-asset': {
                describe: decodeSelectedEntities('User spends coin'),
                type: 'string',
            },
            'to-asset': {
                describe: decodeSelectedEntities('User receives coin'),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.listAllConvertPairs({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'order-status',
    describe: decodeSelectedEntities(`Query order status by order ID.

Weight: 50(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-id': {
                describe: decodeSelectedEntities('Either orderId or quoteId is required'),
                type: 'string',
            },
            'quote-id': {
                describe: decodeSelectedEntities('Either orderId or quoteId is required'),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('order-status is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderStatus({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'send-quote-request',
    describe: decodeSelectedEntities(`Request a quote for the requested token pairs

* Either fromAmount or toAmount should be sent
* &#x60;quoteId&#x60; will be returned only if you have enough funds to convert

Weight: 50(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'sendQuoteRequestRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('send-quote-request is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input sendQuoteRequestRequest:',
                validate: (input: string) =>
                    input ? true : 'sendQuoteRequestRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.sendQuoteRequest(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'adl-risk',
    describe: decodeSelectedEntities(`Query the symbol-level ADL risk rating.
The ADL risk rating measures the likelihood of ADL during liquidation, and the rating takes into account the insurance fund balance, position concentration on the symbol, order book depth, price volatility, average leverage, unrealized PnL, and margin utilization at the symbol level.
The rating can be high, medium and low, and is updated every 30 minutes.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.adlRisk({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'basis',
    describe: decodeSelectedEntities(`Query future basis

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'contract-type': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 30,Max 500'),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.pair && !stdinObj?.pair && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.contractType && !stdinObj?.contractType && !options?.interactive) {
                    requiredParams.push('contractType');
                }

                if (!options?.period && !stdinObj?.period && !options?.interactive) {
                    requiredParams.push('period');
                }

                if (!options?.limit && !stdinObj?.limit && !options?.interactive) {
                    requiredParams.push('limit');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (options.interactive && !options.pair) {
            questions.push({
                type: 'input',
                name: 'pair',
                message: 'Input pair:',
                validate: (input: string) => (input ? true : 'pair cannot be empty'),
            });
        }
        if (options.interactive && !options.contractType) {
            questions.push({
                type: 'input',
                name: 'contractType',
                message: 'Input contractType:',
                validate: (input: string) => (input ? true : 'contractType cannot be empty'),
            });
        }
        if (options.interactive && !options.period) {
            questions.push({
                type: 'input',
                name: 'period',
                message: 'Input period:',
                validate: (input: string) => (input ? true : 'period cannot be empty'),
            });
        }
        if (options.interactive && !options.limit) {
            questions.push({
                type: 'input',
                name: 'limit',
                message: 'Input limit:',
                validate: (input: string) => (input ? true : 'limit cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.basis({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
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

derivativesTradingUsdsFuturesCommands.push({
    command: 'composite-index-symbol-information',
    describe: decodeSelectedEntities(`Query composite index symbol information

* Only for composite index symbols

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.compositeIndexSymbolInformation({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'compressed-aggregate-trades-list',
    describe:
        decodeSelectedEntities(`Get compressed, aggregate market trades. Market trades that fill in 100ms with the same price and the same taking side will have the quantity aggregated.


Retail Price Improvement(RPI) orders are aggregated and without special tags to be distinguished.
* support querying futures trade histories that are not older than one year
* If both &#x60;startTime&#x60; and &#x60;endTime&#x60; are sent, time between &#x60;startTime&#x60; and &#x60;endTime&#x60; must be less than 1 hour.
* If &#x60;fromId&#x60;, &#x60;startTime&#x60;, and &#x60;endTime&#x60; are not sent, the most recent aggregate trades will be returned.
* Only market trades will be aggregated and returned, which means the insurance fund trades and ADL trades won&#39;t be aggregated.
* Sending both &#x60;startTime&#x60;/&#x60;endTime&#x60; and &#x60;fromId&#x60; might cause response timeout, please send either &#x60;fromId&#x60; or &#x60;startTime&#x60;/&#x60;endTime&#x60;

Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'from-id': {
                    describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
            const response = await client.restAPI.compressedAggregateTradesList({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'continuous-contract-kline-candlestick-data',
    describe: decodeSelectedEntities(`Kline/candlestick bars for a specific contract type.
Klines are uniquely identified by their open time.

* If startTime and endTime are not sent, the most recent klines are returned.
* Contract type:
* PERPETUAL
* CURRENT_QUARTER
* NEXT_QUARTER
* TRADIFI_PERPETUAL

Weight: based on parameter LIMIT
| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| &gt; 1000      | 10     |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'contract-type': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                interval: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.pair && !stdinObj?.pair && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.contractType && !stdinObj?.contractType && !options?.interactive) {
                    requiredParams.push('contractType');
                }

                if (!options?.interval && !stdinObj?.interval && !options?.interactive) {
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

        if (options.interactive && !options.pair) {
            questions.push({
                type: 'input',
                name: 'pair',
                message: 'Input pair:',
                validate: (input: string) => (input ? true : 'pair cannot be empty'),
            });
        }
        if (options.interactive && !options.contractType) {
            questions.push({
                type: 'input',
                name: 'contractType',
                message: 'Input contractType:',
                validate: (input: string) => (input ? true : 'contractType cannot be empty'),
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
            const response = await client.restAPI.continuousContractKlineCandlestickData({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
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

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-funding-rate-history',
    describe: decodeSelectedEntities(`Get Funding Rate History


* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are not sent, the most recent 200 records are returned.
* If the number of data between &#x60;startTime&#x60; and &#x60;endTime&#x60; is larger than &#x60;limit&#x60;, return as &#x60;startTime&#x60; + &#x60;limit&#x60;.
* In ascending order.

Weight: share 500/5min/IP rate limit with GET /fapi/v1/fundingInfo`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'start-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'end-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFundingRateHistory({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-funding-rate-info',
    describe:
        decodeSelectedEntities(`Query funding rate info for symbols that had FundingRateCap/ FundingRateFloor / fundingIntervalHours adjustment

Weight: 0
share 500/5min/IP rate limit with GET /fapi/v1/fundingRate`),
    handler: async () => {
        try {
            const response = await client.restAPI.getFundingRateInfo();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'index-price-kline-candlestick-data',
    describe: decodeSelectedEntities(`Kline/candlestick bars for the index price of a pair.
Klines are uniquely identified by their open time.


* If startTime and endTime are not sent, the most recent klines are returned.

Weight: based on parameter LIMIT
| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| &gt; 1000      | 10     |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                interval: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.pair && !stdinObj?.pair && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.interval && !stdinObj?.interval && !options?.interactive) {
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

        if (options.interactive && !options.pair) {
            questions.push({
                type: 'input',
                name: 'pair',
                message: 'Input pair:',
                validate: (input: string) => (input ? true : 'pair cannot be empty'),
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
            const response = await client.restAPI.indexPriceKlineCandlestickData({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'kline-candlestick-data',
    describe: decodeSelectedEntities(`Kline/candlestick bars for a symbol.
Klines are uniquely identified by their open time.

* If startTime and endTime are not sent, the most recent klines are returned.

Weight: based on parameter LIMIT
| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| &gt; 1000      | 10     |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                interval: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.interval && !stdinObj?.interval && !options?.interactive) {
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
            const response = await client.restAPI.klineCandlestickData({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'long-short-ratio',
    describe: decodeSelectedEntities(`Query symbol Long/Short Ratio

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.
* IP rate limit 1000 requests/5min

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.period && !stdinObj?.period && !options?.interactive) {
                    requiredParams.push('period');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (options.interactive && !options.period) {
            questions.push({
                type: 'input',
                name: 'period',
                message: 'Input period:',
                validate: (input: string) => (input ? true : 'period cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.longShortRatio({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'mark-price',
    describe: decodeSelectedEntities(`Mark Price and Funding Rate

Weight: 1 with symbol, 10 without symbol`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.markPrice({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'mark-price-kline-candlestick-data',
    describe: decodeSelectedEntities(`Kline/candlestick bars for the mark price of a symbol.
Klines are uniquely identified by their open time.

* If startTime and endTime are not sent, the most recent klines are returned.

Weight: based on parameter LIMIT
| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| &gt; 1000      | 10     |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                interval: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.interval && !stdinObj?.interval && !options?.interactive) {
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
            const response = await client.restAPI.markPriceKlineCandlestickData({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'multi-assets-mode-asset-index',
    describe: decodeSelectedEntities(`asset index for Multi-Assets mode

Weight: 1 for a single symbol; 10 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.multiAssetsModeAssetIndex({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'old-trades-lookup',
    describe: decodeSelectedEntities(`Get older market historical trades.

* Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won&#39;t be returned.
* Only supports data from within the last three months

Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'from-id': {
                    describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
            const response = await client.restAPI.oldTradesLookup({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'open-interest',
    describe: decodeSelectedEntities(`Get present open interest of a specific symbol.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
            const response = await client.restAPI.openInterest({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'open-interest-statistics',
    describe: decodeSelectedEntities(`Open Interest Statistics

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 1 month is available.
* IP rate limit 1000 requests/5min

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.period && !stdinObj?.period && !options?.interactive) {
                    requiredParams.push('period');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (options.interactive && !options.period) {
            questions.push({
                type: 'input',
                name: 'period',
                message: 'Input period:',
                validate: (input: string) => (input ? true : 'period cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.openInterestStatistics({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'order-book',
    describe: decodeSelectedEntities(`Query symbol orderbook

Retail Price Improvement(RPI) orders are not visible and excluded in the response message.

Weight: Adjusted based on the limit:
| Limit         | Weight |
| ------------- | ------ |
| 5, 10, 20, 50 | 2      |
| 100           | 5      |
| 500           | 10     |
| 1000          | 20     |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
            const response = await client.restAPI.orderBook({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'premium-index-kline-data',
    describe:
        decodeSelectedEntities(`Premium index kline bars of a symbol. Klines are uniquely identified by their open time.


* If startTime and endTime are not sent, the most recent klines are returned.

Weight: based on parameter LIMIT
| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| &gt; 1000      | 10     |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                interval: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.interval && !stdinObj?.interval && !options?.interactive) {
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
            const response = await client.restAPI.premiumIndexKlineData({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'quarterly-contract-settlement-price',
    describe: decodeSelectedEntities(`Latest price for a symbol or symbols.

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.pair && !stdinObj?.pair && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (options.interactive && !options.pair) {
            questions.push({
                type: 'input',
                name: 'pair',
                message: 'Input pair:',
                validate: (input: string) => (input ? true : 'pair cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.quarterlyContractSettlementPrice({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'query-index-price-constituents',
    describe: decodeSelectedEntities(`Query index price constituents


**Note**:

Prices from constituents of TradFi perps will be hiden and displayed as -1.

Weight: 2`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
            const response = await client.restAPI.queryIndexPriceConstituents({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'query-insurance-fund-balance-snapshot',
    describe: decodeSelectedEntities(`Query Insurance Fund Balance Snapshot

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryInsuranceFundBalanceSnapshot({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'recent-trades-list',
    describe: decodeSelectedEntities(`Get recent market trades

* Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won&#39;t be returned.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
            const response = await client.restAPI.recentTradesList({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'rpi-order-book',
    describe: decodeSelectedEntities(`Query symbol orderbook with RPI orders

RPI(Retail Price Improvement) orders are included and aggreated in the response message. Crossed price levels are hidden and invisible.

Weight: Adjusted based on the limit:
| Limit         | Weight |
| ------------- | ------ |
| 1000          | 20     |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
            const response = await client.restAPI.rpiOrderBook({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'symbol-order-book-ticker',
    describe: decodeSelectedEntities(`Best price/qty on the order book for a symbol or symbols.

Retail Price Improvement(RPI) orders are not visible and excluded in the response message.
* If the symbol is not sent, bookTickers for all symbols will be returned in an array.
* The field &#x60;X-MBX-USED-WEIGHT-1M&#x60; in response header is not accurate from this endpoint, please ignore.

Weight: 2 for a single symbol;
5 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.symbolOrderBookTicker({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'symbol-price-ticker',
    describe: decodeSelectedEntities(`Latest price for a symbol or symbols.

* If the symbol is not sent, prices for all symbols will be returned in an array.

Weight: 1 for a single symbol;
2 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.symbolPriceTicker({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'symbol-price-ticker-v2',
    describe: decodeSelectedEntities(`Latest price for a symbol or symbols.

* If the symbol is not sent, prices for all symbols will be returned in an array.
* The field &#x60;X-MBX-USED-WEIGHT-1M&#x60; in response header is not accurate from this endpoint, please ignore.

Weight: 1 for a single symbol;
2 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.symbolPriceTickerV2({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'taker-buy-sell-volume',
    describe: decodeSelectedEntities(`Taker Buy/Sell Volume

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.
* IP rate limit 1000 requests/5min

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.period && !stdinObj?.period && !options?.interactive) {
                    requiredParams.push('period');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (options.interactive && !options.period) {
            questions.push({
                type: 'input',
                name: 'period',
                message: 'Input period:',
                validate: (input: string) => (input ? true : 'period cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.takerBuySellVolume({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
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

derivativesTradingUsdsFuturesCommands.push({
    command: 'ticker24hr-price-change-statistics',
    describe: decodeSelectedEntities(`24 hour rolling window price change statistics.
**Careful** when accessing this with no symbol.

* If the symbol is not sent, tickers for all symbols will be returned in an array.

Weight: 1 for a single symbol;
40 when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.ticker24hrPriceChangeStatistics({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'top-trader-long-short-ratio-accounts',
    describe:
        decodeSelectedEntities(`The proportion of net long and net short accounts to total accounts of the top 20% users with the highest margin balance. Each account is counted once only.
Long Account % &#x3D; Accounts of top traders with net long positions / Total accounts of top traders with open positions
Short Account % &#x3D; Accounts of top traders with net short positions / Total accounts of top traders with open positions
Long/Short Ratio (Accounts) &#x3D; Long Account % / Short Account %

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.
* IP rate limit 1000 requests/5min

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.period && !stdinObj?.period && !options?.interactive) {
                    requiredParams.push('period');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (options.interactive && !options.period) {
            questions.push({
                type: 'input',
                name: 'period',
                message: 'Input period:',
                validate: (input: string) => (input ? true : 'period cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.topTraderLongShortRatioAccounts({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'top-trader-long-short-ratio-positions',
    describe:
        decodeSelectedEntities(`The proportion of net long and net short positions to total open positions of the top 20% users with the highest margin balance.
Long Position % &#x3D; Long positions of top traders / Total open positions of top traders
Short Position % &#x3D; Short positions of top traders / Total open positions of top traders
Long/Short Ratio (Positions) &#x3D; Long Position % / Short Position %

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.
* IP rate limit 1000 requests/5min

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.period && !stdinObj?.period && !options?.interactive) {
                    requiredParams.push('period');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (options.interactive && !options.period) {
            questions.push({
                type: 'input',
                name: 'period',
                message: 'Input period:',
                validate: (input: string) => (input ? true : 'period cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.topTraderLongShortRatioPositions({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'trading-schedule',
    describe:
        decodeSelectedEntities(`Trading session schedules for the underlying assets of TradFi Perps are provided for a one-week period starting from the day prior to the query time, covering both the U.S. equity and commodity markets. Equity market session types include &quot;PRE_MARKET&quot;, &quot;REGULAR&quot;, &quot;AFTER_MARKET&quot;, &quot;OVERNIGHT&quot;, and &quot;NO_TRADING&quot;, while commodity market session types include &quot;REGULAR&quot; and &quot;NO_TRADING&quot;.

Weight: 5`),
    handler: async () => {
        try {
            const response = await client.restAPI.tradingSchedule();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'classic-portfolio-margin-account-information',
    describe: decodeSelectedEntities(`Get Classic Portfolio Margin current account information.


* maxWithdrawAmount is for asset transfer out to the spot wallet.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.asset && !stdinObj?.asset && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'classic-portfolio-margin-account-information is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.classicPortfolioMarginAccountInformation({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'account-trade-list',
    describe: decodeSelectedEntities(`Get trades for a specific account and symbol.

* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 7 days&#39; data will be returned.
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 7 days.
* The parameter &#x60;fromId&#x60; cannot be sent with &#x60;startTime&#x60; or &#x60;endTime&#x60;.
* Only support querying trade in the past 6 months

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'from-id': {
                    describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('account-trade-list is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.accountTradeList({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'all-orders',
    describe: decodeSelectedEntities(`Get all account orders; active, canceled, or filled.

* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60; **AND** order has NO filled trade **AND** created time + 3 days &lt; current time
* order create time + 90 days &lt; current time

* If &#x60;orderId&#x60; is set, it will get orders &gt;&#x3D; that &#x60;orderId&#x60;. Otherwise most recent orders are returned.
* The query time period must be less then 7 days( default as the recent 7 days).

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('all-orders is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.allOrders({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'auto-cancel-all-open-orders',
    describe:
        decodeSelectedEntities(`Cancel all open orders of the specified symbol at the end of the specified countdown.
The endpoint should be called repeatedly as heartbeats so that the existing countdown time can be canceled and replaced by a new one.

* Example usage:
Call this endpoint at 30s intervals with an countdownTime of 120000 (120s).
If this endpoint is not called within 120 seconds, all your orders of the specified symbol will be automatically canceled.
If this endpoint is called with an countdownTime of 0, the countdown timer will be stopped.

The system will check all countdowns **approximately every 10 milliseconds**, so please note that sufficient redundancy should be considered when using this function. We do not recommend setting the countdown time to be too precise or too small.

Weight: 10`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'autoCancelAllOpenOrdersRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'auto-cancel-all-open-orders is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input autoCancelAllOpenOrdersRequest:',
                validate: (input: string) =>
                    input ? true : 'autoCancelAllOpenOrdersRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.autoCancelAllOpenOrders(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'cancel-algo-order',
    describe: decodeSelectedEntities(`Cancel an active algo order.

* Either &#x60;algoId&#x60; or &#x60;clientAlgoId&#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'algo-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'client-algo-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('cancel-algo-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelAlgoOrder({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'cancel-all-algo-open-orders',
    describe: decodeSelectedEntities(`Cancel All Algo Open Orders

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'cancel-all-algo-open-orders is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.cancelAllAlgoOpenOrders({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'cancel-all-open-orders',
    describe: decodeSelectedEntities(`Cancel All Open Orders

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('cancel-all-open-orders is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.cancelAllOpenOrders({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'cancel-multiple-orders',
    describe: decodeSelectedEntities(`Cancel Multiple Orders

* Either &#x60;orderIdList&#x60; or &#x60;origClientOrderIdList &#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-id-list': {
                    describe: decodeSelectedEntities(
                        'max length 10 &lt;br /&gt; e.g. [1234567,2345678]'
                    ),
                    type: 'array',
                },
                'orig-client-order-id-list': {
                    describe: decodeSelectedEntities(
                        'max length 10&lt;br /&gt; e.g. [\&quot;my_id_1\&quot;,\&quot;my_id_2\&quot;], encode the double quotes. No space after comma.'
                    ),
                    type: 'array',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('cancel-multiple-orders is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.cancelMultipleOrders({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'cancel-order',
    describe: decodeSelectedEntities(`Cancel an active order.

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'orig-client-order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('cancel-order is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.cancelOrder({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'change-initial-leverage',
    describe: decodeSelectedEntities(`Change user&#39;s initial leverage of specific symbol market.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'changeInitialLeverageRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'change-initial-leverage is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input changeInitialLeverageRequest:',
                validate: (input: string) =>
                    input ? true : 'changeInitialLeverageRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeInitialLeverage(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'change-margin-type',
    describe: decodeSelectedEntities(`Change symbol level margin type

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'changeMarginTypeRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('change-margin-type is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input changeMarginTypeRequest:',
                validate: (input: string) =>
                    input ? true : 'changeMarginTypeRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeMarginType(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'change-multi-assets-mode',
    describe:
        decodeSelectedEntities(`Change user&#39;s Multi-Assets mode (Multi-Assets Mode or Single-Asset Mode) on ***Every symbol***

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'changeMultiAssetsModeRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'change-multi-assets-mode is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input changeMultiAssetsModeRequest:',
                validate: (input: string) =>
                    input ? true : 'changeMultiAssetsModeRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeMultiAssetsMode(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'change-position-mode',
    describe:
        decodeSelectedEntities(`Change user&#39;s position mode (Hedge Mode or One-way Mode ) on ***EVERY symbol***

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'changePositionModeRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('change-position-mode is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input changePositionModeRequest:',
                validate: (input: string) =>
                    input ? true : 'changePositionModeRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changePositionMode(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'current-all-algo-open-orders',
    describe: decodeSelectedEntities(`Get all algo open orders on a symbol.

* If the symbol is not sent, orders for all symbols will be returned in an array.

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted
Careful when accessing this with no symbol.`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'algo-type': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'algo-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'current-all-algo-open-orders is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.currentAllAlgoOpenOrders({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'current-all-open-orders',
    describe: decodeSelectedEntities(`Get all open orders on a symbol.

* If the symbol is not sent, orders for all symbols will be returned in an array.

Weight: 1 for a single symbol; 40 when the symbol parameter is omitted
Careful when accessing this with no symbol.`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'current-all-open-orders is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.currentAllOpenOrders({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'futures-tradfi-perps-contract',
    describe: decodeSelectedEntities(`Sign TradFi-Perps agreement contract

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'futuresTradfiPerpsContractRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'futures-tradfi-perps-contract is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input futuresTradfiPerpsContractRequest:',
                validate: (input: string) =>
                    input ? true : 'futuresTradfiPerpsContractRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.futuresTradfiPerpsContract(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-order-modify-history',
    describe: decodeSelectedEntities(`Get order modification history

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent, and the &#x60;orderId&#x60; will prevail if both are sent.
* Order modify history longer than 3 month is not avaliable

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'orig-client-order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-order-modify-history is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.getOrderModifyHistory({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'get-position-margin-change-history',
    describe: decodeSelectedEntities(`Get Position Margin Change History

* Support querying future histories that are not older than 30 days
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60;can&#39;t be more than 30 days

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                type: {
                    describe: decodeSelectedEntities(
                        '1: Add position margin，2: Reduce position margin'
                    ),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-position-margin-change-history is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.getPositionMarginChangeHistory({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'modify-isolated-position-margin',
    describe: decodeSelectedEntities(`Modify Isolated Position Margin


* Only for isolated symbol

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'modifyIsolatedPositionMarginRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'modify-isolated-position-margin is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input modifyIsolatedPositionMarginRequest:',
                validate: (input: string) =>
                    input ? true : 'modifyIsolatedPositionMarginRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.modifyIsolatedPositionMargin(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'modify-multiple-orders',
    describe: decodeSelectedEntities(`Modify Multiple Orders (TRADE)

* Parameter rules are same with &#x60;Modify Order&#x60;
* Batch modify orders are processed concurrently, and the order of matching is not guaranteed.
* The order of returned contents for batch modify orders is the same as the order of the order list.
* One order can only be modfied for less than 10000 times

Weight: 5 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
5 on IP rate limit(x-mbx-used-weight-1m);`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'modifyMultipleOrdersRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('modify-multiple-orders is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input modifyMultipleOrdersRequest:',
                validate: (input: string) =>
                    input ? true : 'modifyMultipleOrdersRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.modifyMultipleOrders(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'modify-order',
    describe:
        decodeSelectedEntities(`Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue


* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent, and the &#x60;orderId&#x60; will prevail if both are sent.
* Both &#x60;quantity&#x60; and &#x60;price&#x60; must be sent, which is different from dapi modify order endpoint.
* When the new &#x60;quantity&#x60; or &#x60;price&#x60; doesn&#39;t satisfy PRICE_FILTER / PERCENT_FILTER / LOT_SIZE, amendment will be rejected and the order will stay as it is.
* However the order will be cancelled by the amendment in the following situations:
* when the order is in partially filled status and the new &#x60;quantity&#x60; &lt;&#x3D; &#x60;executedQty&#x60;
* When the order is &#x60;GTX&#x60; and the new price will cause it to be executed immediately
* One order can only be modfied for less than 10000 times

Weight: 1 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
0 on IP rate limit(x-mbx-used-weight-1m)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'modifyOrderRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('modify-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input modifyOrderRequest:',
                validate: (input: string) => (input ? true : 'modifyOrderRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.modifyOrder(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'new-algo-order',
    describe: decodeSelectedEntities(`Send in a new Algo order.

* Algo order with type &#x60;STOP&#x60;,  parameter &#x60;timeInForce&#x60; can be sent ( default &#x60;GTC&#x60;).
* Algo order with type &#x60;TAKE_PROFIT&#x60;,  parameter &#x60;timeInForce&#x60; can be sent ( default &#x60;GTC&#x60;).
* Condition orders will be triggered when:

* If parameter&#x60;priceProtect&#x60;is sent as true:
* when price reaches the &#x60;triggerPrice&#x60; ，the difference rate between &quot;MARK_PRICE&quot; and &quot;CONTRACT_PRICE&quot; cannot be larger than the &quot;triggerProtect&quot; of the symbol
* &quot;triggerProtect&quot; of a symbol can be got from &#x60;GET /fapi/v1/exchangeInfo&#x60;

* &#x60;STOP&#x60;, &#x60;STOP_MARKET&#x60;:
* BUY: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &gt;&#x3D; &#x60;triggerPrice&#x60;
* SELL: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &lt;&#x3D; &#x60;triggerPrice&#x60;
* &#x60;TAKE_PROFIT&#x60;, &#x60;TAKE_PROFIT_MARKET&#x60;:
* BUY: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &lt;&#x3D; &#x60;triggerPrice&#x60;
* SELL: latest price (&quot;MARK_PRICE&quot; or &quot;CONTRACT_PRICE&quot;) &gt;&#x3D; &#x60;triggerPrice&#x60;
* &#x60;TRAILING_STOP_MARKET&#x60;:
* BUY: the lowest price after order placed &lt;&#x3D; &#x60;activatePrice&#x60;, and the latest price &gt;&#x3D; the lowest price * (1 + &#x60;callbackRate&#x60;)
* SELL: the highest price after order placed &gt;&#x3D; &#x60;activatePrice&#x60;, and the latest price &lt;&#x3D; the highest price * (1 - &#x60;callbackRate&#x60;)

* For &#x60;TRAILING_STOP_MARKET&#x60;, if you got such error code.
&#x60;&#x60;{&quot;code&quot;: -2021, &quot;msg&quot;: &quot;Order would immediately trigger.&quot;}&#x60;&#x60;
means that the parameters you send do not meet the following requirements:
* BUY: &#x60;activatePrice&#x60; should be smaller than latest price.
* SELL: &#x60;activatePrice&#x60; should be larger than latest price.

* &#x60;STOP_MARKET&#x60;, &#x60;TAKE_PROFIT_MARKET&#x60; with &#x60;closePosition&#x60;&#x3D;&#x60;true&#x60;:
* Follow the same rules for condition orders.
* If triggered，**close all** current long position( if &#x60;SELL&#x60;) or current short position( if &#x60;BUY&#x60;).
* Cannot be used with &#x60;quantity&#x60; paremeter
* Cannot be used with &#x60;reduceOnly&#x60; parameter
* In Hedge Mode,cannot be used with &#x60;BUY&#x60; orders in &#x60;LONG&#x60; position side. and cannot be used with &#x60;SELL&#x60; orders in &#x60;SHORT&#x60; position side
* &#x60;selfTradePreventionMode&#x60; is only effective when &#x60;timeInForce&#x60; set to &#x60;IOC&#x60; or &#x60;GTC&#x60; or &#x60;GTD&#x60;.

Weight: 0 on IP rate limit(x-mbx-used-weight-1m)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'newAlgoOrderRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('new-algo-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input newAlgoOrderRequest:',
                validate: (input: string) => (input ? true : 'newAlgoOrderRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newAlgoOrder(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'new-order',
    describe: decodeSelectedEntities(`Send in a new order.

* If &#x60;newOrderRespType &#x60; is sent as &#x60;RESULT&#x60; :
* &#x60;MARKET&#x60; order: the final FILLED result of the order will be return directly.
* &#x60;LIMIT&#x60; order with special &#x60;timeInForce&#x60;: the final status result of the order(FILLED or EXPIRED) will be returned directly.

* &#x60;selfTradePreventionMode&#x60; is only effective when &#x60;timeInForce&#x60; set to &#x60;IOC&#x60; or &#x60;GTC&#x60; or &#x60;GTD&#x60;.
* In extreme market conditions, timeInForce &#x60;GTD&#x60; order auto cancel time might be delayed comparing to &#x60;goodTillDate&#x60;

Weight: 1 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
0 on IP rate limit(x-mbx-used-weight-1m)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'newOrderRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('new-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input newOrderRequest:',
                validate: (input: string) => (input ? true : 'newOrderRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.newOrder(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'place-multiple-orders',
    describe: decodeSelectedEntities(`Place Multiple Orders

* Paremeter rules are same with &#x60;New Order&#x60;
* Batch orders are processed concurrently, and the order of matching is not guaranteed.
* The order of returned contents for batch orders is the same as the order of the order list.

Weight: 5 on 10s order rate limit(X-MBX-ORDER-COUNT-10S);
1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M);
5 on IP rate limit(x-mbx-used-weight-1m);`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'placeMultipleOrdersRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('place-multiple-orders is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input placeMultipleOrdersRequest:',
                validate: (input: string) =>
                    input ? true : 'placeMultipleOrdersRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.placeMultipleOrders(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'position-adl-quantile-estimation',
    describe: decodeSelectedEntities(`Position ADL Quantile Estimation

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
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'position-adl-quantile-estimation is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.positionAdlQuantileEstimation({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'position-information-v2',
    describe: decodeSelectedEntities(`Get current position information.

Please use with user data stream &#x60;ACCOUNT_UPDATE&#x60; to meet your timeliness and accuracy needs.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'position-information-v2 is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.positionInformationV2({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'position-information-v3',
    describe:
        decodeSelectedEntities(`Get current position information(only symbol that has position or open orders will be returned).

Please use with user data stream &#x60;ACCOUNT_UPDATE&#x60; to meet your timeliness and accuracy needs.

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'position-information-v3 is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.positionInformationV3({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'query-algo-order',
    describe: decodeSelectedEntities(`Check an algo order&#39;s status.

* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60; **AND** order has NO filled trade **AND** created time + 3 days &lt; current time
* order create time + 90 days &lt; current time

* Either &#x60;algoId&#x60; or &#x60;clientAlgoId&#x60; must be sent.
* &#x60;algoId&#x60; is self-increment for each specific &#x60;symbol&#x60;

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'algo-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'client-algo-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('query-algo-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryAlgoOrder({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'query-all-algo-orders',
    describe: decodeSelectedEntities(`Get all algo orders; active, CANCELED, TRIGGERED or FINISHED .

* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60; **AND** order has NO filled trade **AND** created time + 3 days &lt; current time
* order create time + 90 days &lt; current time

* If &#x60;algoId&#x60; is set, it will get orders &gt;&#x3D; that &#x60;algoId&#x60;. Otherwise most recent orders are returned.
* The query time period must be less then 7 days( default as the recent 7 days).

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'algo-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                page: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('query-all-algo-orders is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.queryAllAlgoOrders({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'query-current-open-order',
    describe: decodeSelectedEntities(`Query open order


* Either&#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent
* If the queried order has been filled or cancelled, the error message &quot;Order does not exist&quot; will be returned.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'orig-client-order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'query-current-open-order is signed. Please login using `binance-cli login`'
            );
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
            const response = await client.restAPI.queryCurrentOpenOrder({
                ...stdinObj,
                ...options,
            });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'query-order',
    describe: decodeSelectedEntities(`Check an order&#39;s status.

* These orders will not be found:
* order status is &#x60;CANCELED&#x60; or &#x60;EXPIRED&#x60; **AND** order has NO filled trade **AND** created time + 3 days &lt; current time
* order create time + 90 days &lt; current time

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.
* &#x60;orderId&#x60; is self-increment for each specific &#x60;symbol&#x60;

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'orig-client-order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('query-order is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.queryOrder({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'test-order',
    describe:
        decodeSelectedEntities(`Testing order request, this order will not be submitted to matching engine

* Order with type &#x60;STOP&#x60;,  parameter &#x60;timeInForce&#x60; can be sent ( default &#x60;GTC&#x60;).
* Order with type &#x60;TAKE_PROFIT&#x60;,  parameter &#x60;timeInForce&#x60; can be sent ( default &#x60;GTC&#x60;).
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
* &#x60;TRAILING_STOP_MARKET&#x60;:
* BUY: the lowest price after order placed &#x60;&lt;&#x3D; &#x60;activationPrice&#x60;, and the latest price &gt;&#x60;&#x3D; the lowest price * (1 + &#x60;callbackRate&#x60;)
* SELL: the highest price after order placed &gt;&#x3D; &#x60;activationPrice&#x60;, and the latest price &lt;&#x3D; the highest price * (1 - &#x60;callbackRate&#x60;)

* For &#x60;TRAILING_STOP_MARKET&#x60;, if you got such error code.
&#x60;&#x60;{&quot;code&quot;: -2021, &quot;msg&quot;: &quot;Order would immediately trigger.&quot;}&#x60;&#x60;
means that the parameters you send do not meet the following requirements:
* BUY: &#x60;activationPrice&#x60; should be smaller than latest price.
* SELL: &#x60;activationPrice&#x60; should be larger than latest price.

* If &#x60;newOrderRespType &#x60; is sent as &#x60;RESULT&#x60; :
* &#x60;MARKET&#x60; order: the final FILLED result of the order will be return directly.
* &#x60;LIMIT&#x60; order with special &#x60;timeInForce&#x60;: the final status result of the order(FILLED or EXPIRED) will be returned directly.

* &#x60;STOP_MARKET&#x60;, &#x60;TAKE_PROFIT_MARKET&#x60; with &#x60;closePosition&#x60;&#x3D;&#x60;true&#x60;:
* Follow the same rules for condition orders.
* If triggered，**close all** current long position( if &#x60;SELL&#x60;) or current short position( if &#x60;BUY&#x60;).
* Cannot be used with &#x60;quantity&#x60; paremeter
* Cannot be used with &#x60;reduceOnly&#x60; parameter
* In Hedge Mode,cannot be used with &#x60;BUY&#x60; orders in &#x60;LONG&#x60; position side. and cannot be used with &#x60;SELL&#x60; orders in &#x60;SHORT&#x60; position side
* &#x60;selfTradePreventionMode&#x60; is only effective when &#x60;timeInForce&#x60; set to &#x60;IOC&#x60; or &#x60;GTC&#x60; or &#x60;GTD&#x60;.
* In extreme market conditions, timeInForce &#x60;GTD&#x60; order auto cancel time might be delayed comparing to &#x60;goodTillDate&#x60;

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'testOrderRequest: ',
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options.json && !stdinObj) {
                    requiredParams.push('json');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('test-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input testOrderRequest:',
                validate: (input: string) => (input ? true : 'testOrderRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.testOrder(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : {}
            );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
    command: 'users-force-orders',
    describe: decodeSelectedEntities(`Query user&#39;s Force Orders

* If &quot;autoCloseType&quot; is not sent, orders with both of the types will be returned
* If &quot;startTime&quot; is not sent, data within 7 days before &quot;endTime&quot; can be queried

Weight: 20 with symbol, 50 without symbol`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'auto-close-type': {
                describe: decodeSelectedEntities(
                    '\&quot;LIQUIDATION\&quot; for liquidation orders, \&quot;ADL\&quot; for ADL orders.'
                ),
                type: 'string',
            },
            'start-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'end-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            limit: {
                describe: decodeSelectedEntities('Default 100; max 1000'),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('users-force-orders is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.usersForceOrders({ ...stdinObj, ...options });
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
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

derivativesTradingUsdsFuturesCommands.push({
    command: 'keepalive-user-data-stream',
    describe:
        decodeSelectedEntities(`Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes. It&#39;s recommended to send a ping about every 60 minutes.

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.keepaliveUserDataStream();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingUsdsFuturesCommands.push({
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
    command: 'futures-usds',
    description: 'Binance Derivatives Trading USDS Futures REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli futures-usds <command> [options]');
        derivativesTradingUsdsFuturesCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
