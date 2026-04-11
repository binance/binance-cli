import { MarginTrading, MARGIN_TRADING_REST_API_PROD_URL } from '@binance/margin-trading';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('margin-trading');

const stdinObj: any = readStdinObj();

let basePath = MARGIN_TRADING_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'margin-trading');

if (process.env.BINANCE_MARGIN_TRADING_BASE_PATH) {
    basePath = process.env.BINANCE_MARGIN_TRADING_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new MarginTrading({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new MarginTrading({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const marginTradingCommands: any[] = [];

marginTradingCommands.push({
    command: 'adjust-cross-margin-max-leverage',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Adjust cross margin max leverage

* The margin level need higher than the initial risk ratio of adjusted leverage, the initial risk ratio of 3x is 1.5 , the initial risk ratio of 5x is 1.25;  The detail conditions on how to switch between Cross Margin Classic and Cross Margin Pro can refer to [the FAQ](https://www.binance.com/en/support/faq/how-to-activate-the-cross-margin-pro-mode-on-binance-e27786da05e743a694b8c625b3bc475d).

Weight: 3000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'max-leverage': {
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

                if (!options?.['maxLeverage'] && !options?.interactive) {
                    requiredParams.push('maxLeverage');
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
                'adjust-cross-margin-max-leverage is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['maxLeverage']) {
            questions.push({
                type: 'input',
                name: 'maxLeverage',
                message: 'Input maxLeverage:',
                validate: (input: string) => (input ? true : 'maxLeverage cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.adjustCrossMarginMaxLeverage(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'disable-isolated-margin-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Disable isolated margin account for a specific symbol. Each trading pair can only be deactivated once every 24
hours.

Weight: 300(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'disable-isolated-margin-account is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.disableIsolatedMarginAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'enable-isolated-margin-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Enable isolated margin account for a specific symbol(Only supports activation of previously disabled accounts).

Weight: 300(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
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
                'enable-isolated-margin-account is signed. Please create a profile using `binance-cli profile create`.'
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.enableIsolatedMarginAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-bnb-burn-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get BNB Burn Status

Weight: 1(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'get-bnb-burn-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBnbBurnStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-summary-of-margin-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get personal margin level information

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'get-summary-of-margin-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSummaryOfMarginAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-cross-isolated-margin-capital-flow',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Cross Isolated Margin Capital Flow

Weight: 100(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            type: {
                describe: decodeSelectedEntities('Transfer Type: ROLL_IN, ROLL_OUT'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities(
                    'Only supports querying data from the past 90 days.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'from-id': {
                describe: decodeSelectedEntities(
                    'If &#x60;fromId&#x60; is set, data with &#x60;id&#x60; greater than &#x60;fromId&#x60; will be returned. Otherwise, the latest data will be returned.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities(
                    'Limit on the number of data records returned per request. Default: 500; Maximum: 1000.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-cross-isolated-margin-capital-flow is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCrossIsolatedMarginCapitalFlow(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-cross-margin-account-details',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Cross Margin Account Details

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-cross-margin-account-details is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCrossMarginAccountDetails(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-cross-margin-fee-data',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get cross margin fee data collection with any vip level or user&#39;s current specific data as https://www.binance.com/en/margin-fee

Weight: 1 when coin is specified;(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'vip-level': {
                describe: decodeSelectedEntities(
                    'User\&#39;s current specific margin data will be returned if vipLevel is omitted'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            coin: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-cross-margin-fee-data is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCrossMarginFeeData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-enabled-isolated-margin-account-limit',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query enabled isolated margin account limit.

Weight: 1(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-enabled-isolated-margin-account-limit is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryEnabledIsolatedMarginAccountLimit(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-isolated-margin-account-info',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Isolated Margin Account Info

* If &quot;symbols&quot; is not sent, all isolated assets will be returned.
* If &quot;symbols&quot; is sent, only the isolated assets of the sent symbols will be returned.

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbols: {
                describe: decodeSelectedEntities(
                    'Max 5 symbols can be sent; separated by \&quot;,\&quot;. e.g. \&quot;BTCUSDT,BNBUSDT,ADAUSDT\&quot;'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-isolated-margin-account-info is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryIsolatedMarginAccountInfo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-isolated-margin-fee-data',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get isolated margin fee data collection with any vip level or user&#39;s current specific data as https://www.binance.com/en/margin-fee

Weight: 1 when a single is specified;(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'vip-level': {
                describe: decodeSelectedEntities(
                    'User\&#39;s current specific margin data will be returned if vipLevel is omitted'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-isolated-margin-fee-data is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryIsolatedMarginFeeData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-future-hourly-interest-rate',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get future hourly interest rate

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                assets: {
                    describe: decodeSelectedEntities(
                        'List of assets, separated by commas, up to 20'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    describe: decodeSelectedEntities(
                        'for isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;'
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

                if (!options?.['assets'] && !options?.interactive) {
                    requiredParams.push('assets');
                }

                if (!options?.['isIsolated'] && !options?.interactive) {
                    requiredParams.push('isIsolated');
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
                'get-future-hourly-interest-rate is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.assets) {
            questions.push({
                type: 'input',
                name: 'assets',
                message: 'Input assets:',
                validate: (input: string) => (input ? true : 'assets cannot be empty'),
            });
        }
        if (options.interactive && !options.isIsolated) {
            questions.push({
                type: 'input',
                name: 'isIsolated',
                message: 'Input isIsolated:',
                validate: (input: string) => (input ? true : 'isIsolated cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFutureHourlyInterestRate(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-interest-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Interest History

* Response in descending order
* If isolatedSymbol is not sent, crossed margin data will be returned
* The max interval between &#x60;startTime&#x60; and &#x60;endTime&#x60; is 30 days.  It is a MUST to ensure data correctness.
* If &#x60;startTime&#x60;and &#x60;endTime&#x60; not sent, return records of the last 7 days by default.
* If &#x60;startTime&#x60; is sent and &#x60;endTime&#x60; is not sent, return records of [max(&#x60;startTime&#x60;, now-30d), now].
* If &#x60;startTime&#x60; is not sent and &#x60;endTime&#x60; is sent, return records of [&#x60;endTime&#x60;-7, &#x60;endTime&#x60;]
* &#x60;type&#x60; in response has 4 enums:
* &#x60;PERIODIC&#x60; interest charged per hour
* &#x60;ON_BORROW&#x60; first interest charged on borrow
* &#x60;PERIODIC_CONVERTED&#x60; interest charged per hour converted into BNB
* &#x60;ON_BORROW_CONVERTED&#x60; first interest charged on borrow converted into BNB
* &#x60;PORTFOLIO&#x60; interest charged daily on the portfolio margin negative balance

Weight: 1(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'isolated-symbol': {
                describe: decodeSelectedEntities('isolated symbol'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities(
                    'Only supports querying data from the past 90 days.'
                ),
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
                describe: decodeSelectedEntities('No more than 60000'),
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
                'get-interest-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getInterestHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'margin-account-borrow-repay',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Margin account borrow/repay(MARGIN)

Weight: 1500`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
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

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
                }

                if (!options?.['isIsolated'] && !options?.interactive) {
                    requiredParams.push('isIsolated');
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

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'margin-account-borrow-repay is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['isIsolated']) {
            questions.push({
                type: 'input',
                name: 'isIsolated',
                message: 'Input isIsolated:',
                validate: (input: string) => (input ? true : 'isIsolated cannot be empty'),
            });
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
            const response = await client.restAPI.marginAccountBorrowRepay(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-borrow-repay-records-in-margin-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query borrow/repay records in Margin account

* &#x60;txId&#x60; or &#x60;startTime&#x60; must be sent. &#x60;txId&#x60; takes precedence.
* If an asset is sent, data within 30 days before &#x60;endTime&#x60;; If an asset is not sent, data within 7 days before &#x60;endTime&#x60;
* If neither &#x60;startTime&#x60; nor &#x60;endTime&#x60; is sent, the recent 7-day data will be returned.
* &#x60;startTime&#x60; set as &#x60;endTime&#x60; - 7days by default, &#x60;endTime&#x60; set as current time by default

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                type: {
                    describe: decodeSelectedEntities('MARGIN,ISOLATED'),
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'isolated-symbol': {
                    describe: decodeSelectedEntities('isolated symbol'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'tx-id': {
                    describe: decodeSelectedEntities(
                        '&#x60;tranId&#x60; in &#x60;POST /sapi/v1/margin/loan&#x60;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Only supports querying data from the past 90 days.'
                    ),
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
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'query-borrow-repay-records-in-margin-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.type) {
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
            const response = await client.restAPI.queryBorrowRepayRecordsInMarginAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-margin-interest-rate-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Interest Rate History

Weight: 1(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'vip-level': {
                    describe: decodeSelectedEntities(
                        'User\&#39;s current specific margin data will be returned if vipLevel is omitted'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Only supports querying data from the past 90 days.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'query-margin-interest-rate-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryMarginInterestRateHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-max-borrow',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Max Borrow

* If isolatedSymbol is not sent, crossed margin data will be sent.
* &#x60;borrowLimit&#x60; is also available from [https://www.binance.com/en/margin-fee](https://www.binance.com/en/margin-fee)

Weight: 50(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'isolated-symbol': {
                    describe: decodeSelectedEntities('isolated symbol'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'query-max-borrow is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryMaxBorrow(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'cross-margin-collateral-ratio',
    describe: decodeSelectedEntities(`Cross margin collateral ratio

Weight: 100(IP)`),
    handler: async () => {
        try {
            const response = await client.restAPI.crossMarginCollateralRatio();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-all-cross-margin-pairs',
    describe: decodeSelectedEntities(`Get All Cross Margin Pairs

Weight: 1(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
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
            const response = await client.restAPI.getAllCrossMarginPairs(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-all-isolated-margin-symbol',
    describe: decodeSelectedEntities(`Get All Isolated Margin Symbol

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
            const response = await client.restAPI.getAllIsolatedMarginSymbol(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-all-margin-assets',
    describe: decodeSelectedEntities(`Get All Margin Assets.

Weight: 1(IP)`),
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
            const response = await client.restAPI.getAllMarginAssets(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-delist-schedule',
    describe:
        decodeSelectedEntities(`Get tokens or symbols delist schedule for cross margin and isolated margin

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
            const response = await client.restAPI.getDelistSchedule(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-limit-price-pairs',
    describe: decodeSelectedEntities(`Query trading pairs with restriction on limit price range.
In margin trading, you can place orders with limit price. Limit price should be within (-15%, 15%) of current index price for a list of margin trading pairs. This rule only impacts limit sell orders with limit price that is lower than current index price and limit buy orders with limit price that is higher than current index price.

- Buy order: Your order will be rejected with an error message notification if the limit price is 15% above the index price.
- Sell order: Your order will be rejected with an error message notification if the limit price is 15% below the index price.
Please review the limit price order placing strategy, backtest and calibrate the planned order size with the trading volume and order book depth to prevent trading loss.

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.getLimitPricePairs();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-list-schedule',
    describe:
        decodeSelectedEntities(`Get the upcoming tokens or symbols listing schedule for Cross Margin and Isolated Margin.

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
            const response = await client.restAPI.getListSchedule(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-margin-asset-risk-based-liquidation-ratio',
    describe: decodeSelectedEntities(`Get Margin Asset Risk-Based Liquidation Ratio

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.getMarginAssetRiskBasedLiquidationRatio();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-margin-restricted-assets',
    describe: decodeSelectedEntities(`Get Margin Restricted Assets

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.getMarginRestrictedAssets();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-isolated-margin-tier-data',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get isolated margin tier data collection with any tier as https://www.binance.com/en/margin-data

Weight: 1(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                tier: {
                    describe: decodeSelectedEntities(
                        'All margin tier data will be returned if tier is omitted'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'query-isolated-margin-tier-data is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryIsolatedMarginTierData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-liability-coin-leverage-bracket-in-cross-margin-pro-mode',
    describe: decodeSelectedEntities(`Liability Coin Leverage Bracket in Cross Margin Pro Mode

Weight: 1`),
    handler: async () => {
        try {
            const response =
                await client.restAPI.queryLiabilityCoinLeverageBracketInCrossMarginProMode();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-margin-available-inventory',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Margin available Inventory query

Weight: 50`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                type: {
                    describe: decodeSelectedEntities('MARGIN,ISOLATED'),
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
                'query-margin-available-inventory is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.type) {
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
            const response = await client.restAPI.queryMarginAvailableInventory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-margin-priceindex',
    describe: decodeSelectedEntities(`Query Margin PriceIndex

Weight: 10(IP)`),
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
            const response = await client.restAPI.queryMarginPriceindex(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'close-user-data-stream',
    describe: decodeSelectedEntities(`Close out a user data stream.

Weight: 3000`),
    handler: async () => {
        try {
            await client.restAPI.closeUserDataStream();
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'keepalive-user-data-stream',
    describe: decodeSelectedEntities(`Keepalive a user data stream to prevent a time out.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'listen-key': {
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

                if (!options?.['listenKey'] && !options?.interactive) {
                    requiredParams.push('listenKey');
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

        if (options.interactive && !options?.['listenKey']) {
            questions.push({
                type: 'input',
                name: 'listenKey',
                message: 'Input listenKey:',
                validate: (input: string) => (input ? true : 'listenKey cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            await client.restAPI.keepaliveUserDataStream(options);
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'start-user-data-stream',
    describe: decodeSelectedEntities(`Start a new user data stream.

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

marginTradingCommands.push({
    command: 'create-special-key',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`- Binance Margin offers low-latency trading through a [special key](https://www.binance.com/en/support/faq/frequently-asked-questions-on-margin-special-api-key-3208663e900d4d2e9fec4140e1832f4e), available exclusively to users with VIP level 7 or higher.
- If you are VIP level 6 or below, please contact your VIP manager for eligibility criterias.

**Supported Products:**

- Cross Margin
- Isolated Margin
- Portfolio Margin Pro

**Unsupported Products:**

- Portfolio Margin

We support several types of API keys:

* Ed25519 (recommended)
* HMAC
* RSA

We recommend to **use Ed25519 API keys** as it should provide the best performance and security out of all supported key types. We accept PKCS#8 (BEGIN PUBLIC KEY). For how to generate an RSA key pair to send API requests on Binance. Please refer to the document below [FAQ](https://www.binance.com/en/support/faq/how-to-generate-an-rsa-key-pair-to-send-api-requests-on-binance-2b79728f331e43079b27440d9d15c5db) .

Weight: 1(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'api-name': {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                ip: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'public-key': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'permission-mode': {
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

                if (!options?.['apiName'] && !options?.interactive) {
                    requiredParams.push('apiName');
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
                'create-special-key is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['apiName']) {
            questions.push({
                type: 'input',
                name: 'apiName',
                message: 'Input apiName:',
                validate: (input: string) => (input ? true : 'apiName cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.createSpecialKey(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'delete-special-key',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This only applies to Special Key for Low Latency Trading.

If apiKey is given, apiName will be ignored. If apiName is given with no apiKey, all apikeys with given apiName will be deleted.

You need to enable Permits “Enable Spot &amp; Margin Trading” option for the API Key which requests this endpoint.

Weight: 1(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'api-name': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'delete-special-key is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            await client.restAPI.deleteSpecialKey(options);
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'edit-ip-for-special-key',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Edit ip restriction. This only applies to Special Key for Low Latency Trading.

You need to enable Permits “Enable Spot &amp; Margin Trading” option for the API Key which requests this endpoint.

Weight: 1(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                ip: {
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

                if (!options?.['ip'] && !options?.interactive) {
                    requiredParams.push('ip');
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
                'edit-ip-for-special-key is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['ip']) {
            questions.push({
                type: 'input',
                name: 'ip',
                message: 'Input ip:',
                validate: (input: string) => (input ? true : 'ip cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            await client.restAPI.editIpForSpecialKey(options);
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-force-liquidation-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Force Liquidation Record

* Response in descending order

Weight: 1(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'start-time': {
                describe: decodeSelectedEntities(
                    'Only supports querying data from the past 90 days.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'isolated-symbol': {
                describe: decodeSelectedEntities('isolated symbol'),
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
                describe: decodeSelectedEntities('No more than 60000'),
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
                'get-force-liquidation-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getForceLiquidationRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-small-liability-exchange-coin-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query the coins which can be small liability exchange

Weight: 100`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'get-small-liability-exchange-coin-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSmallLiabilityExchangeCoinList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-small-liability-exchange-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Small liability Exchange History

Weight: 100(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                current: {
                    describe: decodeSelectedEntities(
                        'Currently querying page. Start from 1. Default:1'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                size: {
                    describe: decodeSelectedEntities('Default:10, Max:100'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Only supports querying data from the past 90 days.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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

                if (!options?.['current'] && !options?.interactive) {
                    requiredParams.push('current');
                }

                if (!options?.['size'] && !options?.interactive) {
                    requiredParams.push('size');
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
                'get-small-liability-exchange-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.current) {
            questions.push({
                type: 'input',
                name: 'current',
                message: 'Input current:',
                validate: (input: string) => (input ? true : 'current cannot be empty'),
            });
        }
        if (options.interactive && !options.size) {
            questions.push({
                type: 'input',
                name: 'size',
                message: 'Input size:',
                validate: (input: string) => (input ? true : 'size cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSmallLiabilityExchangeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'margin-account-cancel-all-open-orders-on-a-symbol',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancels all active orders on a symbol for margin account.&lt;br&gt;&lt;/br&gt;
This includes OCO orders.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    describe: decodeSelectedEntities(
                        'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'margin-account-cancel-all-open-orders-on-a-symbol is signed. Please create a profile using `binance-cli profile create`.'
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
                await client.restAPI.marginAccountCancelAllOpenOrdersOnASymbol(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'margin-account-cancel-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an entire Order List for a margin account.

* Canceling an individual leg will cancel the entire OCO

Weight: 1(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    describe: decodeSelectedEntities(
                        'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                    ),
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
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'margin-account-cancel-oco is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.marginAccountCancelOco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'margin-account-cancel-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an active order for margin account.

* Either orderId or origClientOrderId must be sent.

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    describe: decodeSelectedEntities(
                        'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                    ),
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
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'margin-account-cancel-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.marginAccountCancelOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'margin-account-new-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send in a new OCO for a margin account

* autoRepayAtCancel is suggested to set as “FALSE” to keep liability unrepaid under high frequent new order/cancel order execution

Weight: 6(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
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

marginTradingCommands.push({
    command: 'margin-account-new-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Post a new order for margin account.

* autoRepayAtCancel is suggested to set as “FALSE” to keep liability unrepaid under high frequent new order/cancel order execution

Weight: 6(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
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
                'iceberg-qty': {
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
                'margin-account-new-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.marginAccountNewOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'margin-account-new-oto',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Post a new OTO order for margin account:

- An OTO (One-Triggers-the-Other) is an order list comprised of 2 orders.
- The first order is called the **working order** and must be &#x60;LIMIT&#x60; or &#x60;LIMIT_MAKER&#x60;. Initially, only the working order goes on the order book.
- The second order is called the **pending order**. It can be any order type except for &#x60;MARKET&#x60; orders using parameter &#x60;quoteOrderQty&#x60;. The pending order is only placed on the order book when the working order gets **fully filled**.
- If either the working order or the pending order is cancelled individually, the other order in the order list will also be canceled or expired.
- When the order list is placed, if the working order gets **immediately fully filled**, the placement response will show the working order as &#x60;FILLED&#x60; but the pending order will still appear as &#x60;PENDING_NEW&#x60;. You need to query the status of the pending order again to see its updated status.
- OTOs add **2 orders** to the unfilled order count, &#x60;EXCHANGE_MAX_NUM_ORDERS&#x60; filter and &#x60;MAX_NUM_ORDERS&#x60; filter.

* autoRepayAtCancel is suggested to set as “FALSE” to keep liability unrepaid under high frequent new order/cancel order execution
* Depending on the &#x60;pendingType&#x60; or &#x60;workingType&#x60;, some optional parameters will become mandatory:

Weight: 6(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'list-client-order-id': {
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
                'self-trade-prevention-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-repay-at-cancel': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'working-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-quantity': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-trailing-delta': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-quantity': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-time-in-force': {
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

                if (!options?.['workingType'] && !options?.interactive) {
                    requiredParams.push('workingType');
                }

                if (!options?.['workingSide'] && !options?.interactive) {
                    requiredParams.push('workingSide');
                }

                if (!options?.['workingPrice'] && !options?.interactive) {
                    requiredParams.push('workingPrice');
                }

                if (!options?.['workingQuantity'] && !options?.interactive) {
                    requiredParams.push('workingQuantity');
                }

                if (!options?.['workingIcebergQty'] && !options?.interactive) {
                    requiredParams.push('workingIcebergQty');
                }

                if (!options?.['pendingType'] && !options?.interactive) {
                    requiredParams.push('pendingType');
                }

                if (!options?.['pendingSide'] && !options?.interactive) {
                    requiredParams.push('pendingSide');
                }

                if (!options?.['pendingQuantity'] && !options?.interactive) {
                    requiredParams.push('pendingQuantity');
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
                'margin-account-new-oto is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['workingType']) {
            questions.push({
                type: 'input',
                name: 'workingType',
                message: 'Input workingType:',
                validate: (input: string) => (input ? true : 'workingType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['workingSide']) {
            questions.push({
                type: 'input',
                name: 'workingSide',
                message: 'Input workingSide:',
                validate: (input: string) => (input ? true : 'workingSide cannot be empty'),
            });
        }

        if (options.interactive && !options?.['workingPrice']) {
            questions.push({
                type: 'input',
                name: 'workingPrice',
                message: 'Input workingPrice:',
                validate: (input: string) => (input ? true : 'workingPrice cannot be empty'),
            });
        }

        if (options.interactive && !options?.['workingQuantity']) {
            questions.push({
                type: 'input',
                name: 'workingQuantity',
                message: 'Input workingQuantity:',
                validate: (input: string) => (input ? true : 'workingQuantity cannot be empty'),
            });
        }

        if (options.interactive && !options?.['workingIcebergQty']) {
            questions.push({
                type: 'input',
                name: 'workingIcebergQty',
                message: 'Input workingIcebergQty:',
                validate: (input: string) => (input ? true : 'workingIcebergQty cannot be empty'),
            });
        }

        if (options.interactive && !options?.['pendingType']) {
            questions.push({
                type: 'input',
                name: 'pendingType',
                message: 'Input pendingType:',
                validate: (input: string) => (input ? true : 'pendingType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['pendingSide']) {
            questions.push({
                type: 'input',
                name: 'pendingSide',
                message: 'Input pendingSide:',
                validate: (input: string) => (input ? true : 'pendingSide cannot be empty'),
            });
        }

        if (options.interactive && !options?.['pendingQuantity']) {
            questions.push({
                type: 'input',
                name: 'pendingQuantity',
                message: 'Input pendingQuantity:',
                validate: (input: string) => (input ? true : 'pendingQuantity cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.marginAccountNewOto(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'margin-account-new-otoco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Post a new OTOCO order for margin account：

- An OTOCO (One-Triggers-One-Cancels-the-Other) is an order list comprised of 3 orders.
- The first order is called the **working order** and must be &#x60;LIMIT&#x60; or &#x60;LIMIT_MAKER&#x60;. Initially, only the working order goes on the order book.
- The behavior of the working order is the same as the OTO.
- OTOCO has 2 pending orders (pending above and pending below), forming an OCO pair. The pending orders are only placed on the order book when the working order gets **fully filled**.
- The rules of the pending above and pending below follow the same rules as the [Order List OCO](https://developers.binance.com/docs/margin_trading/trade/Margin-Account-New-OCO).
- OTOCOs add **3 orders** against the unfilled order count, &#x60;EXCHANGE_MAX_NUM_ORDERS&#x60; filter, and &#x60;MAX_NUM_ORDERS&#x60; filter.

* autoRepayAtCancel is suggested to set as “FALSE” to keep liability unrepaid under high frequent new order/cancel order execution
* Depending on the &#x60;pendingAboveType&#x60;/&#x60;pendingBelowType&#x60; or &#x60;workingType&#x60;, some optional parameters will become mandatory:

Weight: 6(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'side-effect-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-repay-at-cancel': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'list-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-order-resp-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'self-trade-prevention-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-quantity': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-side': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-quantity': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-trailing-delta': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-trailing-delta': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-time-in-force': {
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

                if (!options?.['workingType'] && !options?.interactive) {
                    requiredParams.push('workingType');
                }

                if (!options?.['workingSide'] && !options?.interactive) {
                    requiredParams.push('workingSide');
                }

                if (!options?.['workingPrice'] && !options?.interactive) {
                    requiredParams.push('workingPrice');
                }

                if (!options?.['workingQuantity'] && !options?.interactive) {
                    requiredParams.push('workingQuantity');
                }

                if (!options?.['pendingSide'] && !options?.interactive) {
                    requiredParams.push('pendingSide');
                }

                if (!options?.['pendingQuantity'] && !options?.interactive) {
                    requiredParams.push('pendingQuantity');
                }

                if (!options?.['pendingAboveType'] && !options?.interactive) {
                    requiredParams.push('pendingAboveType');
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
                'margin-account-new-otoco is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['workingType']) {
            questions.push({
                type: 'input',
                name: 'workingType',
                message: 'Input workingType:',
                validate: (input: string) => (input ? true : 'workingType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['workingSide']) {
            questions.push({
                type: 'input',
                name: 'workingSide',
                message: 'Input workingSide:',
                validate: (input: string) => (input ? true : 'workingSide cannot be empty'),
            });
        }

        if (options.interactive && !options?.['workingPrice']) {
            questions.push({
                type: 'input',
                name: 'workingPrice',
                message: 'Input workingPrice:',
                validate: (input: string) => (input ? true : 'workingPrice cannot be empty'),
            });
        }

        if (options.interactive && !options?.['workingQuantity']) {
            questions.push({
                type: 'input',
                name: 'workingQuantity',
                message: 'Input workingQuantity:',
                validate: (input: string) => (input ? true : 'workingQuantity cannot be empty'),
            });
        }

        if (options.interactive && !options?.['pendingSide']) {
            questions.push({
                type: 'input',
                name: 'pendingSide',
                message: 'Input pendingSide:',
                validate: (input: string) => (input ? true : 'pendingSide cannot be empty'),
            });
        }

        if (options.interactive && !options?.['pendingQuantity']) {
            questions.push({
                type: 'input',
                name: 'pendingQuantity',
                message: 'Input pendingQuantity:',
                validate: (input: string) => (input ? true : 'pendingQuantity cannot be empty'),
            });
        }

        if (options.interactive && !options?.['pendingAboveType']) {
            questions.push({
                type: 'input',
                name: 'pendingAboveType',
                message: 'Input pendingAboveType:',
                validate: (input: string) => (input ? true : 'pendingAboveType cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.marginAccountNewOtoco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'margin-manual-liquidation',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Margin Manual Liquidation

* This endpoint can support Cross Margin Classic Mode and Pro Mode.
* And only support Isolated Margin for restricted region.

Weight: 3000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                type: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
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
                'margin-manual-liquidation is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
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
            const response = await client.restAPI.marginManualLiquidation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-current-margin-order-count-usage',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Displays the user&#39;s current margin order count usage for all intervals.

Weight: 20(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'is-isolated': {
                describe: decodeSelectedEntities(
                    'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-current-margin-order-count-usage is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentMarginOrderCountUsage(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-margin-accounts-all-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Retrieves all OCO for a specific margin account based on provided optional parameters

Weight: 200(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'is-isolated': {
                describe: decodeSelectedEntities(
                    'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'from-id': {
                describe: decodeSelectedEntities(
                    'If &#x60;fromId&#x60; is set, data with &#x60;id&#x60; greater than &#x60;fromId&#x60; will be returned. Otherwise, the latest data will be returned.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities(
                    'Only supports querying data from the past 90 days.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities(
                    'Limit on the number of data records returned per request. Default: 500; Maximum: 1000.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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

marginTradingCommands.push({
    command: 'query-margin-accounts-all-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Account&#39;s All Orders

* If orderId is set, it will get orders &gt;&#x3D; that orderId. Otherwise the orders within 24 hours are returned.
* For some historical orders cummulativeQuoteQty will be &lt; 0, meaning the data is not available at this time.
* Less than 24 hours between startTime and endTime.

Weight: 200(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    describe: decodeSelectedEntities(
                        'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                    ),
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
                        'Only supports querying data from the past 90 days.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'Limit on the number of data records returned per request. Default: 500; Maximum: 1000.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'query-margin-accounts-all-orders is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryMarginAccountsAllOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-margin-accounts-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Retrieves a specific OCO based on provided optional parameters

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'is-isolated': {
                describe: decodeSelectedEntities(
                    'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
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
            'orig-client-order-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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

marginTradingCommands.push({
    command: 'query-margin-accounts-open-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Account&#39;s Open OCO

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'is-isolated': {
                describe: decodeSelectedEntities(
                    'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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

marginTradingCommands.push({
    command: 'query-margin-accounts-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Account&#39;s Open Orders

* If the symbol is not sent, orders for all symbols will be returned in an array.
* When all symbols are returned, the number of requests counted against the rate limiter is equal to the number of symbols currently trading on the exchange.
* If isIsolated &#x3D;&quot;TRUE&quot;, symbol must be sent.

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'is-isolated': {
                describe: decodeSelectedEntities(
                    'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-margin-accounts-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryMarginAccountsOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-margin-accounts-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Account&#39;s Order

* Either orderId or origClientOrderId must be sent.
* For some historical orders cummulativeQuoteQty will be &lt; 0, meaning the data is not available at this time.

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    describe: decodeSelectedEntities(
                        'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                    ),
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
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'query-margin-accounts-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryMarginAccountsOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-margin-accounts-trade-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Margin Account&#39;s Trade List

* If fromId is set, it will get trades &gt;&#x3D; that fromId. Otherwise the trades within 24 hours are returned.
* Less than 24 hours between startTime and endTime.

Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    describe: decodeSelectedEntities(
                        'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
                    ),
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
                        'Only supports querying data from the past 90 days.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-id': {
                    describe: decodeSelectedEntities(
                        'If &#x60;fromId&#x60; is set, data with &#x60;id&#x60; greater than &#x60;fromId&#x60; will be returned. Otherwise, the latest data will be returned.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'Limit on the number of data records returned per request. Default: 500; Maximum: 1000.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'query-margin-accounts-trade-list is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryMarginAccountsTradeList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-prevented-matches',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`
Weight: 10(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'prevented-match-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-prevented-match-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-isolated': {
                    describe: decodeSelectedEntities(
                        'For isolated margin or not, \&quot;TRUE\&quot;, \&quot;FALSE\&quot;, default \&quot;FALSE\&quot;'
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

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'query-prevented-matches is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryPreventedMatches(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-special-key',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Special Key Information.

This only applies to Special Key for Low Latency Trading.

Weight: 1(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-special-key is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySpecialKey(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-special-key-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This only applies to Special Key for Low Latency Trading.

Weight: 1(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('isolated margin pair'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'query-special-key-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySpecialKeyList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'small-liability-exchange',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Small Liability Exchange

* Only convert once within 6 hours
* Only liability valuation less than 10 USDT are supported
* The maximum number of coin is 10

Weight: 3000(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'asset-names': {
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

                if (!options?.['assetNames'] && !options?.interactive) {
                    requiredParams.push('assetNames');
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
                'small-liability-exchange is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['assetNames']) {
            questions.push({
                type: 'input',
                name: 'assetNames',
                message: 'Input assetNames:',
                validate: (input: string) => (input ? true : 'assetNames cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            await client.restAPI.smallLiabilityExchange(options);
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'get-cross-margin-transfer-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Cross Margin Transfer History

* Response in descending order
* The max interval between &#x60;startTime&#x60; and &#x60;endTime&#x60; is 30 days.
* Returns data for last 7 days by default

Weight: 1(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            type: {
                describe: decodeSelectedEntities('Transfer Type: ROLL_IN, ROLL_OUT'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities(
                    'Only supports querying data from the past 90 days.'
                ),
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
            'isolated-symbol': {
                describe: decodeSelectedEntities('isolated symbol'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('No more than 60000'),
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
                'get-cross-margin-transfer-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCrossMarginTransferHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

marginTradingCommands.push({
    command: 'query-max-transfer-out-amount',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Max Transfer-Out Amount

* If isolatedSymbol is not sent, crossed margin data will be sent.

Weight: 50(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'isolated-symbol': {
                    describe: decodeSelectedEntities('isolated symbol'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('No more than 60000'),
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
                'query-max-transfer-out-amount is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryMaxTransferOutAmount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'margin-trading',
    description: 'Binance Margin Trading REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli margin-trading <command> [options]');
        marginTradingCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
