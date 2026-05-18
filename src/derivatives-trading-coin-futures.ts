import {
    DerivativesTradingCoinFutures,
    DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL,
    DERIVATIVES_TRADING_COIN_FUTURES_REST_API_TESTNET_URL,
} from '@binance/derivatives-trading-coin-futures';
import inquirer from 'inquirer';
import {
    decodeSelectedEntities,
    getConfigurationRestAPI,
    getUserAgent,
    isEmpty,
    readStdinObj,
    getParsedArgs,
} from './utils';

const parsedArgs = getParsedArgs();
const isFullDescription = parsedArgs?.['full-description'] ?? false;

const getClient = () => {
    if (!process.env?.LOG_LEVEL) {
        process.env.LOG_LEVEL = 'ERROR';
    }
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('derivatives-trading-coin-futures');

    let basePath = DERIVATIVES_TRADING_COIN_FUTURES_REST_API_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'futures-coin');

    if (process.env.BINANCE_FUTURES_COIN_BASE_PATH) {
        basePath = process.env.BINANCE_FUTURES_COIN_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['basePath']) {
        basePath = configurationRestAPI['basePath'];
    } else if (configurationRestAPI && configurationRestAPI['env']) {
        switch (configurationRestAPI['env']) {
            case 'demo':
            case 'testnet':
                basePath = DERIVATIVES_TRADING_COIN_FUTURES_REST_API_TESTNET_URL;
                break;
        }
    }

    let client;
    let hasConfig = false;
    if (configurationRestAPI !== null) {
        hasConfig = true;
        client = new DerivativesTradingCoinFutures({
            configurationRestAPI: { ...configurationRestAPI, basePath },
        });
    } else {
        client = new DerivativesTradingCoinFutures({
            configurationRestAPI: {
                apiKey: '',
                basePath,
            },
        });
    }

    return { client, hasConfig };
};

const derivativesTradingCoinFuturesCommands: any[] = [];

derivativesTradingCoinFuturesCommands.push({
    command: 'account-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get current account information.

* for One-way Mode user, the &quot;positions&quot; will only show the &quot;BOTH&quot; positions
* for Hedge Mode user, the &quot;positions&quot; will show &quot;BOTH&quot;, &quot;LONG&quot;, and &quot;SHORT&quot; positions.

Weight: 5`,
            isFullDescription
        ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
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

derivativesTradingCoinFuturesCommands.push({
    command: 'futures-account-balance',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Check futures account balance

Weight: 1`,
            isFullDescription
        ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'futures-account-balance is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.futuresAccountBalance(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-current-position-mode',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get user&#39;s position mode (Hedge Mode or One-way Mode ) on ***EVERY symbol***

Weight: 30`,
            isFullDescription
        ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-current-position-mode is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCurrentPositionMode(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-download-id-for-futures-order-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Download Id For Futures Order History

* Request Limitation is 10 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 5`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'start-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-download-id-for-futures-order-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getDownloadIdForFuturesOrderHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-download-id-for-futures-trade-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get download id for futures trade history

* Request Limitation is 5 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 5`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'start-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-download-id-for-futures-trade-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getDownloadIdForFuturesTradeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-download-id-for-futures-transaction-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get download id for futures transaction history

* Request Limitation is 5 times per month, shared by front end download page and rest api
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 5`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'start-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities('Timestamp in ms'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-download-id-for-futures-transaction-history is signed. Please create a profile using `binance-cli profile create`.'
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
                await client.restAPI.getDownloadIdForFuturesTransactionHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-futures-order-history-download-link-by-id',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get futures order history download link by Id

* Download link expiration: 24h

Weight: 5`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-futures-order-history-download-link-by-id is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getFuturesOrderHistoryDownloadLinkById(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-futures-trade-download-link-by-id',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get futures trade download link by Id

* Download link expiration: 24h

Weight: 5`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-futures-trade-download-link-by-id is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getFuturesTradeDownloadLinkById(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-futures-transaction-history-download-link-by-id',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get futures transaction history download link by Id

* Download link expiration: 24h

Weight: 5`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-futures-transaction-history-download-link-by-id is signed. Please create a profile using `binance-cli profile create`.'
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
            const response =
                await client.restAPI.getFuturesTransactionHistoryDownloadLinkById(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-income-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get income history

* If &#x60;incomeType &#x60; is not sent, all kinds of flow will be returned
* &quot;trandId&quot; is unique in the same &quot;incomeType&quot; for a user
* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can not be longer than 1 year

Weight: 20`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'income-type': {
                describe: decodeSelectedEntities(
                    '\&quot;TRANSFER\&quot;,\&quot;WELCOME_BONUS\&quot;, \&quot;FUNDING_FEE\&quot;, \&quot;REALIZED_PNL\&quot;, \&quot;COMMISSION\&quot;, \&quot;INSURANCE_CLEAR\&quot;, and \&quot;DELIVERED_SETTELMENT\&quot;'
                ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-income-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getIncomeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'notional-bracket-for-pair',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `**Not recommended to continue using this v1 endpoint**

Get the pair&#39;s default notional bracket list, may return ambiguous values when there have been multiple different &#x60;symbol&#x60; brackets under the &#x60;pair&#x60;, suggest using the following &#x60;GET /dapi/v2/leverageBracket&#x60; query instead to get the specific &#x60;symbol&#x60; notional bracket list.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'notional-bracket-for-pair is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.notionalBracketForPair(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'notional-bracket-for-symbol',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get the symbol&#39;s notional bracket list.

Weight: 1`,
            isFullDescription
        ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'notional-bracket-for-symbol is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.notionalBracketForSymbol(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'user-commission-rate',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query user commission rate

Weight: 20`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'user-commission-rate is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.userCommissionRate(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'basis',
    describe: decodeSelectedEntities(
        `Query basis

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities('BTCUSD'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'contract-type': {
                    describe: decodeSelectedEntities(
                        'ALL, CURRENT_QUARTER, NEXT_QUARTER, PERPETUAL'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['pair'] && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.['contractType'] && !options?.interactive) {
                    requiredParams.push('contractType');
                }

                if (!options?.['period'] && !options?.interactive) {
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.basis(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'check-server-time',
    describe: decodeSelectedEntities(
        `Test connectivity to the Rest API and get the current server time.

Weight: 1`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

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

derivativesTradingCoinFuturesCommands.push({
    command: 'compressed-aggregate-trades-list',
    describe: decodeSelectedEntities(
        `Get compressed, aggregate trades. Market trades that fill in 100ms with the same price and the same taking side will have the quantity aggregated.

* support querying futures trade histories that are not older than one year
* If both &#x60;startTime&#x60; and &#x60;endTime&#x60; are sent, time between &#x60;startTime&#x60; and &#x60;endTime&#x60; must be less than 1 hour.
* If &#x60;fromId&#x60;, &#x60;startTime&#x60;, and &#x60;endTime&#x60; are not sent, the most recent aggregate trades will be returned.
* Only market trades will be aggregated and returned, which means the insurance fund trades and ADL trades won&#39;t be aggregated.
* Sending both &#x60;startTime&#x60;/&#x60;endTime&#x60; and &#x60;fromId&#x60; might cause response timeout, please send either &#x60;fromId&#x60; or &#x60;startTime&#x60;/&#x60;endTime&#x60;

Weight: 20`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-id': {
                    describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
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
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.compressedAggregateTradesList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'continuous-contract-kline-candlestick-data',
    describe: decodeSelectedEntities(
        `Kline/candlestick bars for a specific contract type.
Klines are uniquely identified by their open time.

* Contract type:
* PERPETUAL
* CURRENT_QUARTER
* NEXT_QUARTER


1000 | 10
* The difference between &#x60;startTime&#x60; and &#x60;endTime&#x60; can only be up to 200 days
* Between &#x60;startTime&#x60; and &#x60;endTime&#x60;, the most recent &#x60;limit&#x60; data from &#x60;endTime&#x60; will be returned:
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are not sent, current timestamp will be set as &#x60;endTime&#x60;, and the most recent data will be returned.
* If &#x60;startTime&#x60; is sent only, the timestamp of 200 days after &#x60;startTime&#x60; will be set as &#x60;endTime&#x60;(up to the current time)
* If &#x60;endTime&#x60; is sent only, the timestamp of 200 days before &#x60;endTime&#x60; will be set as &#x60;startTime&#x60;

Weight: based on parameter LIMIT
LIMIT | weight
---|---
[1,100) | 1
[100, 500) | 2
[500, 1000] | 5
&gt; 1000 | 10`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities('BTCUSD'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'contract-type': {
                    describe: decodeSelectedEntities(
                        'ALL, CURRENT_QUARTER, NEXT_QUARTER, PERPETUAL'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['pair'] && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.['contractType'] && !options?.interactive) {
                    requiredParams.push('contractType');
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.continuousContractKlineCandlestickData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'exchange-information',
    describe: decodeSelectedEntities(
        `Current exchange trading rules and symbol information

Weight: 1`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

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

derivativesTradingCoinFuturesCommands.push({
    command: 'get-funding-rate-history-of-perpetual-futures',
    describe: decodeSelectedEntities(
        `Get Funding Rate History of Perpetual Futures

* empty array will be returned for delivery symbols.

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.getFundingRateHistoryOfPerpetualFutures(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-funding-rate-info',
    describe: decodeSelectedEntities(
        `Query funding rate info for symbols that had FundingRateCap/ FundingRateFloor / fundingIntervalHours adjustment

Weight: 0`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

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

derivativesTradingCoinFuturesCommands.push({
    command: 'index-price-and-mark-price',
    describe: decodeSelectedEntities(
        `Query index price and mark price

Weight: 10`,
        isFullDescription
    ),
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
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.indexPriceAndMarkPrice(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'index-price-kline-candlestick-data',
    describe: decodeSelectedEntities(
        `Kline/candlestick bars for the index price of a pair. Klines are uniquely identified by their open time.


1000 | 10
* The difference between &#x60;startTime&#x60; and &#x60;endTime&#x60; can only be up to 200 days
* Between &#x60;startTime&#x60; and &#x60;endTime&#x60;, the most recent &#x60;limit&#x60; data from &#x60;endTime&#x60; will be returned:
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are not sent, current timestamp will be set as &#x60;endTime&#x60;, and the most recent data will be returned.
* If &#x60;startTime&#x60; is sent only, the timestamp of 200 days after &#x60;startTime&#x60; will be set as &#x60;endTime&#x60;(up to the current time)
* If &#x60;endTime&#x60; is sent only, the timestamp of 200 days before &#x60;endTime&#x60; will be set as &#x60;startTime&#x60;

Weight: based on parameter LIMIT
LIMIT | weight
---|---
[1,100) | 1
[100, 500) | 2
[500, 1000] | 5
&gt; 1000 | 10`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities('BTCUSD'),
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['pair'] && !options?.interactive) {
                    requiredParams.push('pair');
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.indexPriceKlineCandlestickData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'kline-candlestick-data',
    describe: decodeSelectedEntities(
        `Kline/candlestick bars for a symbol.
Klines are uniquely identified by their open time.

1000 | 10
* The difference between &#x60;startTime&#x60; and &#x60;endTime&#x60; can only be up to 200 days
* Between &#x60;startTime&#x60; and &#x60;endTime&#x60;, the most recent &#x60;limit&#x60; data from &#x60;endTime&#x60; will be returned:
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are not sent, current timestamp will be set as &#x60;endTime&#x60;, and the most recent data will be returned.
* If &#x60;startTime&#x60; is sent only, the timestamp of 200 days after &#x60;startTime&#x60; will be set as &#x60;endTime&#x60;(up to the current time)
* If &#x60;endTime&#x60; is sent only, the timestamp of 200 days before &#x60;endTime&#x60; will be set as &#x60;startTime&#x60;

Weight: based on parameter LIMIT
LIMIT | weight
---|---
[1,100) | 1
[100, 500) | 2
[500, 1000] | 5
&gt; 1000 | 10`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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

derivativesTradingCoinFuturesCommands.push({
    command: 'long-short-ratio',
    describe: decodeSelectedEntities(
        `Query symbol Long/Short Ratio

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities('BTCUSD'),
                    type: 'string',
                    group: 'Command Options:',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['pair'] && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.['period'] && !options?.interactive) {
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

        if (options.interactive && !options.pair) {
            questions.push({
                type: 'input',
                name: 'pair',
                message: 'Input pair:',
                validate: (input: string) => (input ? true : 'pair cannot be empty'),
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
            const response = await client.restAPI.longShortRatio(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'mark-price-kline-candlestick-data',
    describe: decodeSelectedEntities(
        `Kline/candlestick bars for the mark price of a symbol.
Klines are uniquely identified by their open time.


1000 | 10
* The difference between &#x60;startTime&#x60; and &#x60;endTime&#x60; can only be up to 200 days
* Between &#x60;startTime&#x60; and &#x60;endTime&#x60;, the most recent &#x60;limit&#x60; data from &#x60;endTime&#x60; will be returned:
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are not sent, current timestamp will be set as &#x60;endTime&#x60;, and the most recent data will be returned.
* If &#x60;startTime&#x60; is sent only, the timestamp of 200 days after &#x60;startTime&#x60; will be set as &#x60;endTime&#x60;(up to the current time)
* If &#x60;endTime&#x60; is sent only, the timestamp of 200 days before &#x60;endTime&#x60; will be set as &#x60;startTime&#x60;

Weight: based on parameter LIMIT
LIMIT | weight
---|---
[1,100) | 1
[100, 500) | 2
[500, 1000] | 5
&gt; 1000 | 10`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.markPriceKlineCandlestickData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'old-trades-lookup',
    describe: decodeSelectedEntities(
        `Get older market historical trades.

* Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won&#39;t be returned.

Weight: 20`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-id': {
                    describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.oldTradesLookup(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'open-interest',
    describe: decodeSelectedEntities(
        `Get present open interest of a specific symbol.

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.openInterest(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'open-interest-statistics',
    describe: decodeSelectedEntities(
        `Query open interest stats


* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities('BTCUSD'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'contract-type': {
                    describe: decodeSelectedEntities(
                        'ALL, CURRENT_QUARTER, NEXT_QUARTER, PERPETUAL'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['pair'] && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.['contractType'] && !options?.interactive) {
                    requiredParams.push('contractType');
                }

                if (!options?.['period'] && !options?.interactive) {
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.openInterestStatistics(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'order-book',
    describe: decodeSelectedEntities(
        `Query orderbook on specific symbol

Weight: Adjusted based on the limit:
Limit | Weight
------------ | ------------
5, 10, 20, 50 | 2
100 | 5
500 | 10
1000 | 20`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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

derivativesTradingCoinFuturesCommands.push({
    command: 'premium-index-kline-data',
    describe: decodeSelectedEntities(
        `Premium index kline bars of a symbol. Klines are uniquely identified by their open time.


* If startTime and endTime are not sent, the most recent klines are returned.

Weight: based on parameter LIMIT
| LIMIT       | weight |
| ----------- | ------ |
| [1,100)     | 1      |
| [100, 500)  | 2      |
| [500, 1000] | 5      |
| &gt; 1000      | 10     |`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.premiumIndexKlineData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'query-index-price-constituents',
    describe: decodeSelectedEntities(
        `Query index price constituents

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.queryIndexPriceConstituents(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'recent-trades-list',
    describe: decodeSelectedEntities(
        `Get recent market trades

* Market trades means trades filled in the order book. Only market trades will be returned, which means the insurance fund trades and ADL trades won&#39;t be returned.

Weight: 5`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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

derivativesTradingCoinFuturesCommands.push({
    command: 'symbol-order-book-ticker',
    describe: decodeSelectedEntities(
        `Best price/qty on the order book for a symbol or symbols.

* Symbol and pair cannot be sent together
* If a pair is sent,tickers for all symbols of the pair will be returned
* If either a pair or symbol is sent, tickers for all symbols of all pairs will be returned

Weight: 2 for a single symbol, 5 when the symbol parameter is omitted`,
        isFullDescription
    ),
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
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.symbolOrderBookTicker(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'symbol-price-ticker',
    describe: decodeSelectedEntities(
        `Latest price for a symbol or symbols.

* Symbol and pair cannot be sent together
* If a pair is sent,tickers for all symbols of the pair will be returned
* If either a pair or symbol is sent, tickers for all symbols of all pairs will be returned

Weight: 1 for a single symbol, 2 when the symbol parameter is omitted`,
        isFullDescription
    ),
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
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.symbolPriceTicker(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'taker-buy-sell-volume',
    describe: decodeSelectedEntities(
        `Taker Buy Volume: the total volume of buy orders filled by takers within the period.
Taker Sell Volume: the total volume of sell orders filled by takers within the period.

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities('BTCUSD'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'contract-type': {
                    describe: decodeSelectedEntities(
                        'ALL, CURRENT_QUARTER, NEXT_QUARTER, PERPETUAL'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['pair'] && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.['contractType'] && !options?.interactive) {
                    requiredParams.push('contractType');
                }

                if (!options?.['period'] && !options?.interactive) {
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.takerBuySellVolume(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'test-connectivity',
    describe: decodeSelectedEntities(
        `Test connectivity to the Rest API.

Weight: 1`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

        try {
            await client.restAPI.testConnectivity();
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'ticker24hr-price-change-statistics',
    describe: decodeSelectedEntities(
        `24 hour rolling window price change statistics.

* Symbol and pair cannot be sent together
* If a pair is sent,tickers for all symbols of the pair will be returned
* If either a pair or symbol is sent, tickers for all symbols of all pairs will be returned

Weight: 1 for a single symbol, 40 when the symbol parameter is omitted
Careful when accessing this with no symbol.`,
        isFullDescription
    ),
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
            json: {
                describe: 'Send all fields as JSON',
                type: 'string',
                group: 'JSON Options:',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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

derivativesTradingCoinFuturesCommands.push({
    command: 'top-trader-long-short-ratio-accounts',
    describe: decodeSelectedEntities(
        `The proportion of net long and net short accounts to total accounts of the top 20% users with the highest margin balance. Each account is counted once only.
Long Account % &#x3D; Accounts of top traders with net long positions / Total accounts of top traders with open positions
Short Account % &#x3D; Accounts of top traders with net short positions / Total accounts of top traders with open positions
Long/Short Ratio (Accounts) &#x3D; Long Account % / Short Account %

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['period'] && !options?.interactive) {
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

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
            const response = await client.restAPI.topTraderLongShortRatioAccounts(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'top-trader-long-short-ratio-positions',
    describe: decodeSelectedEntities(
        `The proportion of net long and net short positions to total open positions of the top 20% users with the highest margin balance.
Long Position % &#x3D; Long positions of top traders / Total open positions of top traders
Short Position % &#x3D; Short positions of top traders / Total open positions of top traders
Long/Short Ratio (Positions) &#x3D; Long Position % / Short Position %

* If startTime and endTime are not sent, the most recent data is returned.
* Only the data of the latest 30 days is available.

Weight: 1`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                pair: {
                    describe: decodeSelectedEntities('BTCUSD'),
                    type: 'string',
                    group: 'Command Options:',
                },
                period: {
                    describe: decodeSelectedEntities(
                        '\&quot;5m\&quot;,\&quot;15m\&quot;,\&quot;30m\&quot;,\&quot;1h\&quot;,\&quot;2h\&quot;,\&quot;4h\&quot;,\&quot;6h\&quot;,\&quot;12h\&quot;,\&quot;1d\&quot;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100; max 1000'),
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
                json: {
                    describe: 'Send all fields as JSON',
                    type: 'string',
                    group: 'JSON Options:',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['pair'] && !options?.interactive) {
                    requiredParams.push('pair');
                }

                if (!options?.['period'] && !options?.interactive) {
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client } = getClient();

        if (options.interactive && !options.pair) {
            questions.push({
                type: 'input',
                name: 'pair',
                message: 'Input pair:',
                validate: (input: string) => (input ? true : 'pair cannot be empty'),
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
            const response = await client.restAPI.topTraderLongShortRatioPositions(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'classic-portfolio-margin-account-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Classic Portfolio Margin current account information.

* maxWithdrawAmount is for asset transfer out to the spot wallet.

Weight: 5`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'classic-portfolio-margin-account-information is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.classicPortfolioMarginAccountInformation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'account-trade-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get trades for a specific account and symbol.


* Either symbol or pair must be sent
* Symbol and pair cannot be sent together
* Pair and fromId cannot be sent together
* OrderId can only be sent together with symbol
* If a pair is sent,tickers for all symbols of the pair will be returned
* The parameter &#x60;fromId&#x60; cannot be sent with &#x60;startTime&#x60; or &#x60;endTime&#x60;
* If startTime and endTime are both not sent, then the last 7 days&#39; data will be returned.
* The time between startTime and endTime cannot be longer than 7 days.

Weight: 20 with symbol，40 with pair`,
            isFullDescription
        ),
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
            'order-id': {
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
            'from-id': {
                describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
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

derivativesTradingCoinFuturesCommands.push({
    command: 'all-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get all account orders; active, canceled, or filled.

* These orders will not be found:
* order status is CANCELED or EXPIRED AND order has NO filled trade AND created time + 3 days &lt; current time
* order create time + 90 days &lt; current time


* Either &#x60;symbol&#x60; or &#x60;pair&#x60; must be sent.
* &#x60;pair&#x60; can&#39;t be sent with &#x60;orderId&#x60;
* If &#x60;orderId&#x60; is set, it will get orders &gt;&#x3D; that &#x60;orderId&#x60;. Otherwise most recent orders are returned.
* If orderId is set, it will get orders &gt;&#x3D; that orderId. Otherwise most recent orders are returned.
* The query time period must be less then 7 days( default as the recent 7 days).

Weight: 20 with symbol, 40 with pair`,
            isFullDescription
        ),
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
            'order-id': {
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'all-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.allOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'auto-cancel-all-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Cancel all open orders of the specified symbol at the end of the specified countdown. This rest endpoint means to ensure your open orders are canceled in case of an outage. The endpoint should be called repeatedly as heartbeats so that the existing countdown time can be canceled and repalced by a new one. The system will check all countdowns **approximately every 10 milliseconds**, so please note that sufficient redundancy should be considered when using this function. We do not recommend setting the countdown time to be too precise or too small.

* Example usage:
Call this endpoint at 30s intervals with an countdownTime of 120000 (120s).
If this endpoint is not called within 120 seconds, all your orders of the specified symbol will be automatically canceled.
If this endpoint is called with an countdownTime of 0, the countdown timer will be stopped.

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'auto-cancel-all-open-orders is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.autoCancelAllOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'cancel-all-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Cancel All Open Orders

Weight: 1`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'cancel-all-open-orders is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.cancelAllOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'cancel-multiple-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Cancel Multiple Orders

* Either &#x60;orderIdList&#x60; or &#x60;origClientOrderIdList &#x60; must be sent.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id-list': {
                    describe: decodeSelectedEntities(
                        'max length 10 &lt;br /&gt; e.g. [1234567,2345678]'
                    ),
                    type: 'array',
                    group: 'Command Options:',
                },
                'orig-client-order-id-list': {
                    describe: decodeSelectedEntities(
                        'max length 10&lt;br /&gt; e.g. [\&quot;my_id_1\&quot;,\&quot;my_id_2\&quot;], encode the double quotes. No space after comma.'
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'cancel-multiple-orders is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.cancelMultipleOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'cancel-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Cancel an active order.


* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.

Weight: 1`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'cancel-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.cancelOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'change-initial-leverage',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Change user&#39;s initial leverage in the specific symbol market.
For Hedge Mode, LONG and SHORT positions of one symbol use the same initial leverage and share a total notional value.

Weight: 1`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'change-initial-leverage is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.changeInitialLeverage(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'change-margin-type',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Change user&#39;s margin type in the specific symbol market.For Hedge Mode, LONG and SHORT positions of one symbol use the same margin type.
With ISOLATED margin type, margins of the LONG and SHORT positions are isolated from each other.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'margin-type': {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['marginType'] && !options?.interactive) {
                    requiredParams.push('marginType');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'change-margin-type is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['marginType']) {
            questions.push({
                type: 'input',
                name: 'marginType',
                message: 'Input marginType:',
                validate: (input: string) => (input ? true : 'marginType cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeMarginType(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'change-position-mode',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Change user&#39;s position mode (Hedge Mode or One-way Mode ) on ***EVERY symbol***

Weight: 1`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'change-position-mode is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.changePositionMode(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'current-all-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get all open orders on a symbol. **Careful** when accessing this with no symbol.

Weight: 1 for a single symbol, 40 for mutltiple symbols`,
            isFullDescription
        ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'current-all-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.currentAllOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-order-modify-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get order modification history


* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent, and the &#x60;orderId&#x60; will prevail if both are sent.
* Order modify history longer than 3 month is not avaliable

Weight: 1`,
            isFullDescription
        ),
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
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
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
            })
            .check((options: any) => {
                const requiredParams: any = [];
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-order-modify-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getOrderModifyHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'get-position-margin-change-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get position margin change history

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                type: {
                    describe: decodeSelectedEntities(
                        '1: Add position margin,2: Reduce position margin'
                    ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-position-margin-change-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getPositionMarginChangeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'modify-isolated-position-margin',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Modify Isolated Position Margin

* Only for isolated symbol

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'position-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                type: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'modify-isolated-position-margin is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['amount']) {
            questions.push({
                type: 'input',
                name: 'amount',
                message: 'Input amount:',
                validate: (input: string) => (input ? true : 'amount cannot be empty'),
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
            const response = await client.restAPI.modifyIsolatedPositionMargin(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'modify-multiple-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Modify Multiple Orders

* Parameter rules are same with &#x60;Modify Order&#x60;
* Batch modify orders are processed concurrently, and the order of matching is not guaranteed.
* The order of returned contents for batch modify orders is the same as the order of the order list.
* One order can only be modfied for less than 10000 times

Weight: 5`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'batch-orders': {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['batchOrders'] && !options?.interactive) {
                    requiredParams.push('batchOrders');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'modify-multiple-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['batchOrders']) {
            questions.push({
                type: 'input',
                name: 'batchOrders',
                message: 'Input batchOrders:',
                validate: (input: string) => (input ? true : 'batchOrders cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.modifyMultipleOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'modify-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Order modify function, currently only LIMIT order modification is supported, modified orders will be reordered in the match queue

* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent, and the &#x60;orderId&#x60; will prevail if both are sent.
* Either &#x60;quantity&#x60; or &#x60;price&#x60; must be sent.
* When the new &#x60;quantity&#x60; or &#x60;price&#x60; doesn&#39;t satisfy PRICE_FILTER / PERCENT_FILTER / LOT_SIZE, amendment will be rejected and the order will stay as it is.
* However the order will be cancelled by the amendment in the following situations:
* when the order is in partially filled status and the new &#x60;quantity&#x60; &lt;&#x3D; &#x60;executedQty&#x60;
* When the order is &#x60;GTX&#x60; and the new price will cause it to be executed immediately
* One order can only be modfied for less than 10000 times

Weight: 1`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'modify-order is signed. Please create a profile using `binance-cli profile create`.'
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.modifyOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'new-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Send in a new order.


* Order with type &#x60;STOP&#x60;,  parameter &#x60;timeInForce&#x60; can be sent ( default &#x60;GTC&#x60;).
* Order with type &#x60;TAKE_PROFIT&#x60;,  parameter &#x60;timeInForce&#x60; can be sent ( default &#x60;GTC&#x60;).
* Condition orders will be triggered when:

* If parameter&#x60;priceProtect&#x60;is sent as true:
* when price reaches the &#x60;stopPrice&#x60; ，the difference rate between &quot;MARK_PRICE&quot; and &quot;CONTRACT_PRICE&quot; cannot be larger than the &quot;triggerProtect&quot; of the symbol
* &quot;triggerProtect&quot; of a symbol can be got from &#x60;GET /dapi/v1/exchangeInfo&#x60;

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
* If triggered,**close all** current long position( if &#x60;SELL&#x60;) or current short position( if &#x60;BUY&#x60;).
* Cannot be used with &#x60;quantity&#x60; parameter
* Cannot be used with &#x60;reduceOnly&#x60; parameter
* In Hedge Mode,cannot be used with &#x60;BUY&#x60; orders in &#x60;LONG&#x60; position side. and cannot be used with &#x60;SELL&#x60; orders in &#x60;SHORT&#x60; position side
* &#x60;selfTradePreventionMode&#x60; is only effective when &#x60;timeInForce&#x60; set to &#x60;IOC&#x60; or &#x60;GTC&#x60;.

Weight: 1 on 1min order rate limit(X-MBX-ORDER-COUNT-1M)\
0 on IP rate limit(x-mbx-used-weight-1m)`,
            isFullDescription
        ),
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
                'stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'close-position': {
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
                'working-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'price-protect': {
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
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

derivativesTradingCoinFuturesCommands.push({
    command: 'place-multiple-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Place multiple orders

* Parameter rules are same with &#x60;New Order&#x60;
* Batch orders are processed concurrently, and the order of matching is not guaranteed.
* The order of returned contents for batch orders is the same as the order of the order list.

Weight: 5`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'batch-orders': {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['batchOrders'] && !options?.interactive) {
                    requiredParams.push('batchOrders');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions: any = [];
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'place-multiple-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['batchOrders']) {
            questions.push({
                type: 'input',
                name: 'batchOrders',
                message: 'Input batchOrders:',
                validate: (input: string) => (input ? true : 'batchOrders cannot be empty'),
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

derivativesTradingCoinFuturesCommands.push({
    command: 'position-adl-quantile-estimation',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query position ADL quantile estimation

* Values update every 30s.
* Values 0, 1, 2, 3, 4 shows the queue position and possibility of ADL from low to high.
* For positions of the symbol are in One-way Mode or isolated margined in Hedge Mode, &quot;LONG&quot;, &quot;SHORT&quot;, and &quot;BOTH&quot; will be returned to show the positions&#39; adl quantiles of different position sides.
* If the positions of the symbol are crossed margined in Hedge Mode:
* &quot;HEDGE&quot; as a sign will be returned instead of &quot;BOTH&quot;;
* A same value caculated on unrealized pnls on long and short sides&#39; positions will be shown for &quot;LONG&quot; and &quot;SHORT&quot; when there are positions in both of long and short sides.

Weight: 5`,
            isFullDescription
        ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'position-adl-quantile-estimation is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.positionAdlQuantileEstimation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'position-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get current account information.

* If neither &#x60;marginAsset&#x60; nor &#x60;pair&#x60; is sent, positions of all symbols with &#x60;TRADING&#x60; status will be returned.
* for One-way Mode user, the response  will only show the &quot;BOTH&quot; positions
* for Hedge Mode user, the response will show &quot;BOTH&quot;, &quot;LONG&quot;, and &quot;SHORT&quot; positions.
Please use with user data stream &#x60;ACCOUNT_UPDATE&#x60; to meet your timeliness and accuracy needs.

Weight: 1`,
            isFullDescription
        ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'position-information is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.positionInformation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'query-current-open-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Current Open Order

* Either&#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent
* If the queried order has been filled or cancelled, the error message &quot;Order does not exist&quot; will be returned.

Weight: 1`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'query-current-open-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryCurrentOpenOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'query-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Check an order&#39;s status.

* These orders will not be found:
* order status is CANCELED or EXPIRED AND order has NO filled trade AND created time + 3 days &lt; current time
* order create time + 90 days &lt; current time


* Either &#x60;orderId&#x60; or &#x60;origClientOrderId&#x60; must be sent.

Weight: 1`,
            isFullDescription
        ),
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
                const stdinObj: any = readStdinObj();

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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'query-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'users-force-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `User&#39;s Force Orders

* If &quot;autoCloseType&quot; is not sent, orders with both of the types will be returned
* If &quot;startTime&quot; is not sent, data within 200 days before &quot;endTime&quot; can be queried

Weight: 20 with symbol, 50 without symbol`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'auto-close-type': {
                describe: decodeSelectedEntities(
                    '\&quot;LIQUIDATION\&quot; for liquidation orders, \&quot;ADL\&quot; for ADL orders.'
                ),
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
        const stdinObj: any = readStdinObj();

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'users-force-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.usersForceOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'close-user-data-stream',
    describe: decodeSelectedEntities(
        `Close out a user data stream.

Weight: 1`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

        try {
            await client.restAPI.closeUserDataStream();
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'keepalive-user-data-stream',
    describe: decodeSelectedEntities(
        `Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes.

Weight: 1`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

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

derivativesTradingCoinFuturesCommands.push({
    command: 'start-user-data-stream',
    describe: decodeSelectedEntities(
        `Start a new user data stream. The stream will close after 60 minutes unless a keepalive is sent. If the account has an active &#x60;listenKey&#x60;, that &#x60;listenKey&#x60; will be returned and its validity will be extended for 60 minutes.

Weight: 1`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

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
    command: 'futures-coin',
    description: 'Binance Derivatives Trading COIN Futures REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli futures-coin <command> [options]');
        derivativesTradingCoinFuturesCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
