import {
    Spot,
    SPOT_REST_API_PROD_URL,
    SPOT_REST_API_DEMO_URL,
    SPOT_REST_API_TESTNET_URL,
} from '@binance/spot';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('spot');

const stdinObj: any = readStdinObj();

let basePath = SPOT_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'spot');

if (process.env.BINANCE_SPOT_BASE_PATH) {
    basePath = process.env.BINANCE_SPOT_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
} else if (configurationRestAPI && configurationRestAPI['env']) {
    switch (configurationRestAPI['env']) {
        case 'testnet':
            basePath = SPOT_REST_API_TESTNET_URL;
            break;
        case 'demo':
            basePath = SPOT_REST_API_DEMO_URL;
            break;
    }
}

let client;
if (configurationRestAPI !== null) {
    client = new Spot({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new Spot({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const spotCommands: any[] = [];

spotCommands.push({
    command: 'account-commission',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current account commission rates.
Weight: 20`),
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

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'account-commission is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.accountCommission(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'all-order-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Retrieves all order lists based on provided optional parameters.

Note that the time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can&#39;t be longer than 24 hours.
Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'from-id': {
                describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities(
                    'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities(
                    'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'all-order-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.allOrderList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'all-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all account orders; active, canceled, or filled.
Weight: 20`),
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
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'all-orders is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.allOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'get-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get current account information.
Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'omit-zero-balances': {
                describe: decodeSelectedEntities(
                    'When set to &#x60;true&#x60;, emits only the non-zero balances of an account. &lt;br&gt;Default value: &#x60;false&#x60;'
                ),
                type: 'boolean',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'get-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all open orders on a symbol. **Careful** when accessing this with no symbol.
Weight: 6 for a single symbol; **80** when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'get-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Check an order&#39;s status.
Weight: 4`),
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
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'get-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'get-order-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Retrieves a specific order list based on provided optional parameters.
Weight: 4`),
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
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'get-order-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getOrderList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'my-allocations',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Retrieves allocations resulting from SOR order placement.
Weight: 20`),
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
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-allocation-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'my-allocations is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.myAllocations(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'my-filters',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Retrieves the list of [filters](filters.md) relevant to an account on a given symbol. This is the only endpoint that shows if an account has &#x60;MAX_ASSET&#x60; filters applied to it.
Weight: 40`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'my-filters is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.myFilters(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'my-prevented-matches',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Displays the list of orders that were expired due to STP.

These are the combinations supported:

* &#x60;symbol&#x60; + &#x60;preventedMatchId&#x60;
* &#x60;symbol&#x60; + &#x60;orderId&#x60;
* &#x60;symbol&#x60; + &#x60;orderId&#x60; + &#x60;fromPreventedMatchId&#x60; (&#x60;limit&#x60; will default to 500)
* &#x60;symbol&#x60; + &#x60;orderId&#x60; + &#x60;fromPreventedMatchId&#x60; + &#x60;limit&#x60;
Weight: Case                            | Weight
----                            | -----
If &#x60;symbol&#x60; is invalid          | 2
Querying by &#x60;preventedMatchId&#x60;  | 2
Querying by &#x60;orderId&#x60;           | 20`),
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
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'my-prevented-matches is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.myPreventedMatches(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'my-trades',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get trades for a specific account and symbol.
Weight: Condition| Weight|
---| ---
|Without orderId|20|
|With orderId|5|`),
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
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-id': {
                    describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'my-trades is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.myTrades(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'open-order-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`
Weight: 6`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'open-order-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.openOrderList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-amendments',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Queries all amendments of a single order.
Weight: 4`),
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
                'from-execution-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default:500; Maximum: 1000 '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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

                if (!options?.['orderId'] && !options?.interactive) {
                    requiredParams.push('orderId');
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
                'order-amendments is signed. Please create a profile using `binance-cli profile create`.'
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
        if (options.interactive && !options.orderId) {
            questions.push({
                type: 'input',
                name: 'orderId',
                message: 'Input orderId:',
                validate: (input: string) => (input ? true : 'orderId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderAmendments(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'rate-limit-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Displays the user&#39;s unfilled order count for all intervals.
Weight: 40`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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

        if (isEmpty(configurationRestAPI)) {
            console.log(
                'rate-limit-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.rateLimitOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'exchange-info',
    describe: decodeSelectedEntities(`Current exchange trading rules and symbol information
Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
                group: 'Command Options:',
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
                group: 'Command Options:',
            },
            permissions: {
                describe: decodeSelectedEntities('List of permissions to query'),
                type: 'array',
                group: 'Command Options:',
            },
            'show-permission-sets': {
                describe: decodeSelectedEntities(
                    'Controls whether the content of the &#x60;permissionSets&#x60; field is populated or not. Defaults to &#x60;true&#x60;'
                ),
                type: 'boolean',
                group: 'Command Options:',
            },
            'symbol-status': {
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
            const response = await client.restAPI.exchangeInfo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'execution-rules',
    describe: decodeSelectedEntities(`
Weight: Parameter | Weight|
---        | ---
&#x60;symbol&#x60;  | 2
&#x60;symbols&#x60; | 2 for each &#x60;symbol&#x60;, capped at a max of 40|
&#x60;symbolStatus&#x60; |40|
None            |40|`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
                group: 'Command Options:',
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
                group: 'Command Options:',
            },
            'symbol-status': {
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
            const response = await client.restAPI.executionRules(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'ping',
    describe: decodeSelectedEntities(`Test connectivity to the Rest API.
Weight: 1`),
    handler: async () => {
        try {
            await client.restAPI.ping();
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'time',
    describe:
        decodeSelectedEntities(`Test connectivity to the Rest API and get the current server time.
Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.time();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'agg-trades',
    describe:
        decodeSelectedEntities(`Get compressed, aggregate trades. Trades that fill at the time, from the same taker order, with the same price will have the quantity aggregated.
Weight: 4`),
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
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.aggTrades(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'avg-price',
    describe: decodeSelectedEntities(`Current average price for a symbol.
Weight: 2`),
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
            const response = await client.restAPI.avgPrice(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'depth',
    describe: decodeSelectedEntities(`
Weight: Adjusted based on the limit:

|Limit|Request Weight
------|-------
1-100|  5
101-500| 25
501-1000| 50
1001-5000| 250`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'symbol-status': {
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
            const response = await client.restAPI.depth(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'get-trades',
    describe: decodeSelectedEntities(`Get recent trades.
Weight: 25`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.getTrades(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'historical-trades',
    describe: decodeSelectedEntities(`Get older trades.
Weight: 25`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.historicalTrades(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'klines',
    describe: decodeSelectedEntities(`Kline/candlestick bars for a symbol.
Klines are uniquely identified by their open time.
Weight: 2`),
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
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'time-zone': {
                    describe: decodeSelectedEntities('Default: 0 (UTC)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.klines(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'reference-price',
    describe: decodeSelectedEntities(`
Weight: 2`),
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
            const response = await client.restAPI.referencePrice(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'reference-price-calculation',
    describe:
        decodeSelectedEntities(`Describes how reference price is calculated for a given symbol.
Weight: 2`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'symbol-status': {
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
            const response = await client.restAPI.referencePriceCalculation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'ticker',
    describe: decodeSelectedEntities(`
Weight: 4 for each requested &lt;tt&gt;symbol&lt;/tt&gt; regardless of &lt;tt&gt;windowSize&lt;/tt&gt;. &lt;br/&gt;&lt;br/&gt; The weight for this request will cap at 200 once the number of &#x60;symbols&#x60; in the request is more than 50.`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
                group: 'Command Options:',
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
                group: 'Command Options:',
            },
            'window-size': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            type: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'symbol-status': {
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
            const response = await client.restAPI.ticker(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'ticker24hr',
    describe:
        decodeSelectedEntities(`24 hour rolling window price change statistics. **Careful** when accessing this with no symbol.
Weight: &lt;table&gt;
&lt;thead&gt;
    &lt;tr&gt;
        &lt;th&gt;Parameter&lt;/th&gt;
        &lt;th&gt;Symbols Provided&lt;/th&gt;
        &lt;th&gt;Weight&lt;/th&gt;
    &lt;/tr&gt;
&lt;/thead&gt;
&lt;tbody&gt;
    &lt;tr&gt;
        &lt;td rowspan&#x3D;&quot;2&quot;&gt;symbol&lt;/td&gt;
        &lt;td&gt;1&lt;/td&gt;
        &lt;td&gt;2&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td&gt;symbol parameter is omitted&lt;/td&gt;
        &lt;td&gt;80&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td rowspan&#x3D;&quot;4&quot;&gt;symbols&lt;/td&gt;
        &lt;td&gt;1-20&lt;/td&gt;
        &lt;td&gt;2&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td&gt;21-100&lt;/td&gt;
        &lt;td&gt;40&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td&gt;101 or more&lt;/td&gt;
        &lt;td&gt;80&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td&gt;symbols parameter is omitted&lt;/td&gt;
        &lt;td&gt;80&lt;/td&gt;
    &lt;/tr&gt;
&lt;/tbody&gt;
&lt;/table&gt;`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
                group: 'Command Options:',
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
                group: 'Command Options:',
            },
            type: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'symbol-status': {
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
            const response = await client.restAPI.ticker24hr(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'ticker-book-ticker',
    describe: decodeSelectedEntities(`Best price/qty on the order book for a symbol or symbols.
Weight: &lt;table&gt;
&lt;thead&gt;
    &lt;tr&gt;
        &lt;th&gt;Parameter&lt;/th&gt;
        &lt;th&gt;Symbols Provided&lt;/th&gt;
        &lt;th&gt;Weight&lt;/th&gt;
    &lt;/tr&gt;
&lt;/thead&gt;
&lt;tbody&gt;
    &lt;tr&gt;
        &lt;td rowspan&#x3D;&quot;2&quot;&gt;symbol&lt;/td&gt;
        &lt;td&gt;1&lt;/td&gt;
        &lt;td&gt;2&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td&gt;symbol parameter is omitted&lt;/td&gt;
        &lt;td&gt;4&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td&gt;symbols&lt;/td&gt;
        &lt;td&gt;Any&lt;/td&gt;
        &lt;td&gt;4&lt;/td&gt;
    &lt;/tr&gt;
&lt;/tbody&gt;
&lt;/table&gt;`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
                group: 'Command Options:',
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
                group: 'Command Options:',
            },
            'symbol-status': {
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
            const response = await client.restAPI.tickerBookTicker(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'ticker-price',
    describe: decodeSelectedEntities(`Latest price for a symbol or symbols.
Weight: &lt;table&gt;
&lt;thead&gt;
    &lt;tr&gt;
        &lt;th&gt;Parameter&lt;/th&gt;
        &lt;th&gt;Symbols Provided&lt;/th&gt;
        &lt;th&gt;Weight&lt;/th&gt;
    &lt;/tr&gt;
&lt;/thead&gt;
&lt;tbody&gt;
    &lt;tr&gt;
        &lt;td rowspan&#x3D;&quot;2&quot;&gt;symbol&lt;/td&gt;
        &lt;td&gt;1&lt;/td&gt;
        &lt;td&gt;2&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td&gt;symbol parameter is omitted&lt;/td&gt;
        &lt;td&gt;4&lt;/td&gt;
    &lt;/tr&gt;
    &lt;tr&gt;
        &lt;td&gt;symbols&lt;/td&gt;
        &lt;td&gt;Any&lt;/td&gt;
        &lt;td&gt;4&lt;/td&gt;
    &lt;/tr&gt;
&lt;/tbody&gt;
&lt;/table&gt;`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
                group: 'Command Options:',
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
                group: 'Command Options:',
            },
            'symbol-status': {
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
            const response = await client.restAPI.tickerPrice(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'ticker-trading-day',
    describe: decodeSelectedEntities(`Price change statistics for a trading day.
Weight: 4 for each requested &lt;tt&gt;symbol&lt;/tt&gt;. &lt;br/&gt;&lt;br/&gt; The weight for this request will cap at 200 once the number of &#x60;symbols&#x60; in the request is more than 50.`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
                group: 'Command Options:',
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
                group: 'Command Options:',
            },
            'time-zone': {
                describe: decodeSelectedEntities('Default: 0 (UTC)'),
                type: 'string',
                group: 'Command Options:',
            },
            type: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'symbol-status': {
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
            const response = await client.restAPI.tickerTradingDay(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'ui-klines',
    describe:
        decodeSelectedEntities(`The request is similar to klines having the same parameters and response.

&#x60;uiKlines&#x60; return modified kline data, optimized for presentation of candlestick charts.
Weight: 2`),
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
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'time-zone': {
                    describe: decodeSelectedEntities('Default: 0 (UTC)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.uiKlines(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'delete-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancels all active orders on a symbol.
This includes orders that are part of an order list.
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
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'delete-open-orders is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.deleteOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'delete-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an active order.
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
                'new-client-order-id': {
                    describe: decodeSelectedEntities(
                        'A unique id among open orders. Automatically generated if not sent.&lt;br/&gt; Orders with the same &#x60;newClientOrderID&#x60; can be accepted only when the previous one is filled, otherwise the order will be rejected.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'cancel-restrictions': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'delete-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.deleteOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'delete-order-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an entire Order list
Weight: 1`),
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
                    describe: decodeSelectedEntities('A unique Id for the entire orderList'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    describe: decodeSelectedEntities(
                        'A unique id among open orders. Automatically generated if not sent.&lt;br/&gt; Orders with the same &#x60;newClientOrderID&#x60; can be accepted only when the previous one is filled, otherwise the order will be rejected.'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
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
                'delete-order-list is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.deleteOrderList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'new-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send in a new order.

This adds 1 order to the &#x60;EXCHANGE_MAX_ORDERS&#x60; filter and the &#x60;MAX_NUM_ORDERS&#x60; filter.
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
                'time-in-force': {
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
                'new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'trailing-delta': {
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
                'self-trade-prevention-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-offset-value': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-offset-type': {
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

spotCommands.push({
    command: 'order-amend-keep-priority',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Reduce the quantity of an existing open order.

This adds 0 orders to the &#x60;EXCHANGE_MAX_ORDERS&#x60; filter and the &#x60;MAX_NUM_ORDERS&#x60; filter.

Read [Order Amend Keep Priority FAQ](faqs/order_amend_keep_priority.md) to learn more.
Weight: 4`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'orig-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-qty': {
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

                if (!options?.['newQty'] && !options?.interactive) {
                    requiredParams.push('newQty');
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
                'order-amend-keep-priority is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['newQty']) {
            questions.push({
                type: 'input',
                name: 'newQty',
                message: 'Input newQty:',
                validate: (input: string) => (input ? true : 'newQty cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderAmendKeepPriority(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-cancel-replace',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`* Cancels an existing order and places a new order on the same symbol.
* Filters and Order Count are evaluated before the processing of the cancellation and order placement occurs.
* A new order that was not attempted (i.e. when &#x60;newOrderResult: NOT_ATTEMPTED&#x60;), will still increase the unfilled order count by 1.
* You can only cancel an individual order from an orderList using this endpoint, but the result is the same as canceling the entire orderList.
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
                'cancel-replace-mode': {
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
                'quote-order-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'cancel-new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'cancel-orig-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'cancel-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'trailing-delta': {
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
                'self-trade-prevention-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'cancel-restrictions': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-rate-limit-exceeded-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-offset-value': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-offset-type': {
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

                if (!options?.['cancelReplaceMode'] && !options?.interactive) {
                    requiredParams.push('cancelReplaceMode');
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
                'order-cancel-replace is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['cancelReplaceMode']) {
            questions.push({
                type: 'input',
                name: 'cancelReplaceMode',
                message: 'Input cancelReplaceMode:',
                validate: (input: string) => (input ? true : 'cancelReplaceMode cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderCancelReplace(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-list-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send in an one-cancels-the-other (OCO) pair, where activation of one order immediately cancels the other.

* An OCO has 2 orders called the **above order** and **below order**.
* One of the orders must be a &#x60;LIMIT_MAKER/TAKE_PROFIT/TAKE_PROFIT_LIMIT&#x60; order and the other must be &#x60;STOP_LOSS&#x60; or &#x60;STOP_LOSS_LIMIT&#x60; order.
* Price restrictions
  * If the OCO is on the &#x60;SELL&#x60; side:
    * &#x60;LIMIT_MAKER/TAKE_PROFIT_LIMIT&#x60; &#x60;price&#x60; &gt; Last Traded Price &gt;  &#x60;STOP_LOSS/STOP_LOSS_LIMIT&#x60; &#x60;stopPrice&#x60;
    * &#x60;TAKE_PROFIT stopPrice&#x60; &gt; Last Traded Price &gt; &#x60;STOP_LOSS/STOP_LOSS_LIMIT stopPrice&#x60;
  * If the OCO is on the &#x60;BUY&#x60; side:
    * &#x60;LIMIT_MAKER/TAKE_PROFIT_LIMIT price&#x60; &lt; Last Traded Price &lt; &#x60;stopPrice&#x60;
    * &#x60;TAKE_PROFIT stopPrice&#x60; &lt; Last Traded Price &lt; &#x60;STOP_LOSS/STOP_LOSS_LIMIT stopPrice&#x60;
* OCOs add **2 orders** to the &#x60;EXCHANGE_MAX_ORDERS&#x60; filter and the &#x60;MAX_NUM_ORDERS&#x60; filter.
Weight: 1

Unfilled Order Count: 2`),
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
                'above-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-trailing-delta': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'above-peg-offset-value': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-trailing-delta': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'below-peg-offset-value': {
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

                if (!options?.['aboveType'] && !options?.interactive) {
                    requiredParams.push('aboveType');
                }

                if (!options?.['belowType'] && !options?.interactive) {
                    requiredParams.push('belowType');
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
                'order-list-oco is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['aboveType']) {
            questions.push({
                type: 'input',
                name: 'aboveType',
                message: 'Input aboveType:',
                validate: (input: string) => (input ? true : 'aboveType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['belowType']) {
            questions.push({
                type: 'input',
                name: 'belowType',
                message: 'Input belowType:',
                validate: (input: string) => (input ? true : 'belowType cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderListOco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-list-opo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Place an [OPO](./faqs/opo.md).

* OPOs add 2 orders to the EXCHANGE_MAX_NUM_ORDERS filter and MAX_NUM_ORDERS filter.
Weight: 1

Unfilled Order Count: 2`),
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
                'working-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-offset-value': {
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
                'pending-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-time-in-force': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-peg-offset-value': {
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

                if (!options?.['pendingType'] && !options?.interactive) {
                    requiredParams.push('pendingType');
                }

                if (!options?.['pendingSide'] && !options?.interactive) {
                    requiredParams.push('pendingSide');
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
                'order-list-opo is signed. Please create a profile using `binance-cli profile create`.'
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderListOpo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-list-opoco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Place an [OPOCO](./faqs/opo.md).
Weight: 1

Unfilled Order Count: 3`),
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
                'working-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-offset-value': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-side': {
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
                'pending-above-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-peg-offset-value': {
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
                'pending-below-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-peg-offset-value': {
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
                'order-list-opoco is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.orderListOpoco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-list-oto',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Place an OTO.

* An OTO (One-Triggers-the-Other) is an order list comprised of 2 orders.
* The first order is called the **working order** and must be &#x60;LIMIT&#x60; or &#x60;LIMIT_MAKER&#x60;. Initially, only the working order goes on the order book.
* The second order is called the **pending order**. It can be any order type except for &#x60;MARKET&#x60; orders using parameter &#x60;quoteOrderQty&#x60;. The pending order is only placed on the order book when the working order gets **fully filled**.
* If either the working order or the pending order is cancelled individually, the other order in the order list will also be canceled or expired.
* When the order list is placed, if the working order gets **immediately fully filled**, the placement response will show the working order as &#x60;FILLED&#x60; but the pending order will still appear as &#x60;PENDING_NEW&#x60;. You need to query the status of the pending order again to see its updated status.
* OTOs add **2 orders** to the &#x60;EXCHANGE_MAX_NUM_ORDERS&#x60; filter and &#x60;MAX_NUM_ORDERS&#x60; filter.
Weight: 1

Unfilled Order Count: 2`),
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
                'working-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-offset-value': {
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
                'pending-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-peg-offset-value': {
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
                'order-list-oto is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.orderListOto(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-list-otoco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Place an OTOCO.

* An OTOCO (One-Triggers-One-Cancels-the-Other) is an order list comprised of 3 orders.
* The first order is called the **working order** and must be &#x60;LIMIT&#x60; or &#x60;LIMIT_MAKER&#x60;. Initially, only the working order goes on the order book.
  * The behavior of the working order is the same as the [OTO](#new-order-list---oto-trade).
* OTOCO has 2 pending orders (pending above and pending below), forming an OCO pair. The pending orders are only placed on the order book when the working order gets **fully filled**.
    * The rules of the pending above and pending below follow the same rules as the [Order list OCO](#new-order-list---oco-trade).
* OTOCOs add **3 orders** to the &#x60;EXCHANGE_MAX_NUM_ORDERS&#x60; filter and &#x60;MAX_NUM_ORDERS&#x60; filter.
Weight: 1

Unfilled Order Count: 3`),
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
                'working-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'working-peg-offset-value': {
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
                'pending-above-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-above-peg-offset-value': {
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
                'pending-below-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-peg-offset-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'pending-below-peg-offset-value': {
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
                'order-list-otoco is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.orderListOtoco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-oco',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send in a new OCO.

* Price Restrictions:
    * &#x60;SELL&#x60;: Limit Price &gt; Last Price &gt; Stop Price
    * &#x60;BUY&#x60;: Limit Price &lt; Last Price &lt; Stop Price
* Quantity Restrictions:
    * Both legs must have the same quantity.
    * &#x60;ICEBERG&#x60; quantities however do not have to be the same
* &#x60;OCO&#x60; adds **2 orders** to the &#x60;EXCHANGE_MAX_ORDERS&#x60; filter and the &#x60;MAX_NUM_ORDERS&#x60; filter.
Weight: 1

Unfilled Order Count: 2`),
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
                'limit-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'limit-strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'limit-iceberg-qty': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'trailing-delta': {
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
                'stop-strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-strategy-type': {
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
                'order-oco is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.orderOco(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'order-test',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Test new order creation and signature/recvWindow long.
Creates and validates a new order but does not send it into the matching engine.
Weight: |Condition| Request Weight|
|------------           | ------------ |
|Without &#x60;computeCommissionRates&#x60;| 1|
|With &#x60;computeCommissionRates&#x60;|20|`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'compute-commission-rates': {
                    type: 'boolean',
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
                'quote-order-qty': {
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
                'strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'stop-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'trailing-delta': {
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
                'self-trade-prevention-mode': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-price-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-offset-value': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'peg-offset-type': {
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
                'order-test is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.orderTest(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'sor-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Places an order using smart order routing (SOR).

This adds 1 order to the &#x60;EXCHANGE_MAX_ORDERS&#x60; filter and the &#x60;MAX_NUM_ORDERS&#x60; filter.

Read [SOR FAQ](faqs/sor_faq.md) to learn more.
Weight: 1

Unfilled Order Count: 1`),
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
                'time-in-force': {
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
                'new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-type': {
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
            console.log(
                'sor-order is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.sorOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

spotCommands.push({
    command: 'sor-order-test',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Test new order creation and signature/recvWindow using smart order routing (SOR).
Creates and validates a new order but does not send it into the matching engine.
Weight: | Condition | Request Weight |
| --------- | -------------- |
| Without &#x60;computeCommissionRates&#x60;  |  1 |
| With &#x60;computeCommissionRates&#x60;     | 20 |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'compute-commission-rates': {
                    type: 'boolean',
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
                price: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'new-client-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'strategy-type': {
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
            console.log(
                'sor-order-test is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.sorOrderTest(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'spot',
    description: 'Binance Spot REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli spot <command> [options]');
        spotCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
