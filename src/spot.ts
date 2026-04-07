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
    describe: decodeSelectedEntities(`Get current account commission rates.
Weight: 20`),
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
        if (isEmpty(configurationRestAPI)) {
            console.log('account-commission is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.accountCommission({ ...stdinObj, ...options });
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
        decodeSelectedEntities(`Retrieves all order lists based on provided optional parameters.

Note that the time between &#x60;startTime&#x60; and &#x60;endTime&#x60; can&#39;t be longer than 24 hours.
Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'from-id': {
                describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
                type: 'string',
            },
            'start-time': {
                describe: decodeSelectedEntities(
                    'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                ),
                type: 'string',
            },
            'end-time': {
                describe: decodeSelectedEntities(
                    'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                ),
                type: 'string',
            },
            limit: {
                describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                ),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('all-order-list is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.allOrderList({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Get all account orders; active, canceled, or filled.
Weight: 20`),
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
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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

spotCommands.push({
    command: 'get-account',
    describe: decodeSelectedEntities(`Get current account information.
Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'omit-zero-balances': {
                describe: decodeSelectedEntities(
                    'When set to &#x60;true&#x60;, emits only the non-zero balances of an account. &lt;br&gt;Default value: &#x60;false&#x60;'
                ),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                ),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('get-account is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getAccount({ ...stdinObj, ...options });
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
        decodeSelectedEntities(`Get all open orders on a symbol. **Careful** when accessing this with no symbol.
Weight: 6 for a single symbol; **80** when the symbol parameter is omitted`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                ),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('get-open-orders is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getOpenOrders({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Check an order&#39;s status.
Weight: 4`),
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
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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
            console.log('get-order is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.getOrder({ ...stdinObj, ...options });
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
        decodeSelectedEntities(`Retrieves a specific order list based on provided optional parameters.
Weight: 4`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-list-id': {
                describe: decodeSelectedEntities(
                    'Either &#x60;orderListId&#x60; or &#x60;listClientOrderId&#x60; must be provided'
                ),
                type: 'string',
            },
            'orig-client-order-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                ),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('get-order-list is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getOrderList({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Retrieves allocations resulting from SOR order placement.
Weight: 20`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'from-allocation-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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
            console.log('my-allocations is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.myAllocations({ ...stdinObj, ...options });
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
        decodeSelectedEntities(`Retrieves the list of [filters](filters.md) relevant to an account on a given symbol. This is the only endpoint that shows if an account has &#x60;MAX_ASSET&#x60; filters applied to it.
Weight: 40`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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
            console.log('my-filters is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.myFilters({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Displays the list of orders that were expired due to STP.

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
                },
                'prevented-match-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'from-prevented-match-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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
            console.log('my-prevented-matches is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.myPreventedMatches({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Get trades for a specific account and symbol.
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
                },
                'order-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'from-id': {
                    describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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
            console.log('my-trades is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.myTrades({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`
Weight: 6`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                ),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('open-order-list is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.openOrderList({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Queries all amendments of a single order.
Weight: 4`),
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
                'from-execution-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default:500; Maximum: 1000 '),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
                    type: 'string',
                },
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!options?.symbol && !stdinObj?.symbol && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.orderId && !stdinObj?.orderId && !options?.interactive) {
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
        if (isEmpty(configurationRestAPI)) {
            console.log('order-amendments is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.orderAmendments({ ...stdinObj, ...options });
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
        decodeSelectedEntities(`Displays the user&#39;s unfilled order count for all intervals.
Weight: 40`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities(
                    'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                ),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log('rate-limit-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.rateLimitOrder({ ...stdinObj, ...options });
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
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
            },
            permissions: {
                describe: decodeSelectedEntities('List of permissions to query'),
                type: 'array',
            },
            'show-permission-sets': {
                describe: decodeSelectedEntities(
                    'Controls whether the content of the &#x60;permissionSets&#x60; field is populated or not. Defaults to &#x60;true&#x60;'
                ),
                type: 'string',
            },
            'symbol-status': {
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
            const response = await client.restAPI.exchangeInfo({ ...stdinObj, ...options });
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
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
            },
            'symbol-status': {
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
            const response = await client.restAPI.executionRules({ ...stdinObj, ...options });
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
                },
                'from-id': {
                    describe: decodeSelectedEntities('ID to get aggregate trades from INCLUSIVE.'),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.aggTrades({ ...stdinObj, ...options });
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
            const response = await client.restAPI.avgPrice({ ...stdinObj, ...options });
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
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
                    type: 'string',
                },
                'symbol-status': {
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
            const response = await client.restAPI.depth({ ...stdinObj, ...options });
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
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.getTrades({ ...stdinObj, ...options });
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
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.historicalTrades({ ...stdinObj, ...options });
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
                },
                interval: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'time-zone': {
                    describe: decodeSelectedEntities('Default: 0 (UTC)'),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.klines({ ...stdinObj, ...options });
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
            const response = await client.restAPI.referencePrice({ ...stdinObj, ...options });
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
                },
                'symbol-status': {
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
            const response = await client.restAPI.referencePriceCalculation({
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

spotCommands.push({
    command: 'ticker',
    describe: decodeSelectedEntities(`
Weight: 4 for each requested &lt;tt&gt;symbol&lt;/tt&gt; regardless of &lt;tt&gt;windowSize&lt;/tt&gt;. &lt;br/&gt;&lt;br/&gt; The weight for this request will cap at 200 once the number of &#x60;symbols&#x60; in the request is more than 50.`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Symbol to query'),
                type: 'string',
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
            },
            'window-size': {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            type: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'symbol-status': {
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
            const response = await client.restAPI.ticker({ ...stdinObj, ...options });
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
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
            },
            type: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'symbol-status': {
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
            const response = await client.restAPI.ticker24hr({ ...stdinObj, ...options });
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
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
            },
            'symbol-status': {
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
            const response = await client.restAPI.tickerBookTicker({ ...stdinObj, ...options });
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
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
            },
            'symbol-status': {
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
            const response = await client.restAPI.tickerPrice({ ...stdinObj, ...options });
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
            },
            symbols: {
                describe: decodeSelectedEntities('List of symbols to query'),
                type: 'array',
            },
            'time-zone': {
                describe: decodeSelectedEntities('Default: 0 (UTC)'),
                type: 'string',
            },
            type: {
                describe: decodeSelectedEntities(''),
                type: 'string',
            },
            'symbol-status': {
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
            const response = await client.restAPI.tickerTradingDay({ ...stdinObj, ...options });
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
                },
                interval: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'start-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades from INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'Timestamp in ms to get aggregate trades until INCLUSIVE.'
                    ),
                    type: 'string',
                },
                'time-zone': {
                    describe: decodeSelectedEntities('Default: 0 (UTC)'),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 500; Maximum: 1000.'),
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
            const response = await client.restAPI.uiKlines({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Cancels all active orders on a symbol.
This includes orders that are part of an order list.
Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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
            console.log('delete-open-orders is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.deleteOpenOrders({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Cancel an active order.
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
                'new-client-order-id': {
                    describe: decodeSelectedEntities(
                        'A unique id among open orders. Automatically generated if not sent.&lt;br/&gt; Orders with the same &#x60;newClientOrderID&#x60; can be accepted only when the previous one is filled, otherwise the order will be rejected.'
                    ),
                    type: 'string',
                },
                'cancel-restrictions': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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
            console.log('delete-order is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.deleteOrder({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Cancel an entire Order list
Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'order-list-id': {
                    describe: decodeSelectedEntities(
                        'Either &#x60;orderListId&#x60; or &#x60;listClientOrderId&#x60; must be provided'
                    ),
                    type: 'string',
                },
                'list-client-order-id': {
                    describe: decodeSelectedEntities('A unique Id for the entire orderList'),
                    type: 'string',
                },
                'new-client-order-id': {
                    describe: decodeSelectedEntities(
                        'A unique id among open orders. Automatically generated if not sent.&lt;br/&gt; Orders with the same &#x60;newClientOrderID&#x60; can be accepted only when the previous one is filled, otherwise the order will be rejected.'
                    ),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(
                        'The value cannot be greater than &#x60;60000&#x60;. &lt;br&gt; Supports up to three decimal places of precision (e.g., 6000.346) so that microseconds may be specified.'
                    ),
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
            console.log('delete-order-list is signed. Please login using `binance-cli login`');
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
            const response = await client.restAPI.deleteOrderList({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Send in a new order.

This adds 1 order to the &#x60;EXCHANGE_MAX_ORDERS&#x60; filter and the &#x60;MAX_NUM_ORDERS&#x60; filter.
Weight: 1`),
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
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
    describe: decodeSelectedEntities(`Reduce the quantity of an existing open order.

This adds 0 orders to the &#x60;EXCHANGE_MAX_ORDERS&#x60; filter and the &#x60;MAX_NUM_ORDERS&#x60; filter.

Read [Order Amend Keep Priority FAQ](faqs/order_amend_keep_priority.md) to learn more.
Weight: 4`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'orderAmendKeepPriorityRequest: ',
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
                'order-amend-keep-priority is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderAmendKeepPriorityRequest:',
                validate: (input: string) =>
                    input ? true : 'orderAmendKeepPriorityRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderAmendKeepPriority(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
        decodeSelectedEntities(`* Cancels an existing order and places a new order on the same symbol.
* Filters and Order Count are evaluated before the processing of the cancellation and order placement occurs.
* A new order that was not attempted (i.e. when &#x60;newOrderResult: NOT_ATTEMPTED&#x60;), will still increase the unfilled order count by 1.
* You can only cancel an individual order from an orderList using this endpoint, but the result is the same as canceling the entire orderList.
Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'orderCancelReplaceRequest: ',
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
            console.log('order-cancel-replace is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderCancelReplaceRequest:',
                validate: (input: string) =>
                    input ? true : 'orderCancelReplaceRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderCancelReplace(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
                json: {
                    describe: 'orderListOcoRequest: ',
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
            console.log('order-list-oco is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderListOcoRequest:',
                validate: (input: string) => (input ? true : 'orderListOcoRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderListOco(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
    describe: decodeSelectedEntities(`Place an [OPO](./faqs/opo.md).

* OPOs add 2 orders to the EXCHANGE_MAX_NUM_ORDERS filter and MAX_NUM_ORDERS filter.
Weight: 1

Unfilled Order Count: 2`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'orderListOpoRequest: ',
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
            console.log('order-list-opo is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderListOpoRequest:',
                validate: (input: string) => (input ? true : 'orderListOpoRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderListOpo(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
    describe: decodeSelectedEntities(`Place an [OPOCO](./faqs/opo.md).
Weight: 1

Unfilled Order Count: 3`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'orderListOpocoRequest: ',
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
            console.log('order-list-opoco is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderListOpocoRequest:',
                validate: (input: string) =>
                    input ? true : 'orderListOpocoRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderListOpoco(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
    describe: decodeSelectedEntities(`Place an OTO.

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
                json: {
                    describe: 'orderListOtoRequest: ',
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
            console.log('order-list-oto is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderListOtoRequest:',
                validate: (input: string) => (input ? true : 'orderListOtoRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderListOto(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
    describe: decodeSelectedEntities(`Place an OTOCO.

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
                json: {
                    describe: 'orderListOtocoRequest: ',
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
            console.log('order-list-otoco is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderListOtocoRequest:',
                validate: (input: string) =>
                    input ? true : 'orderListOtocoRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderListOtoco(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
    describe: decodeSelectedEntities(`Send in a new OCO.

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
                json: {
                    describe: 'orderOcoRequest: ',
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
            console.log('order-oco is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderOcoRequest:',
                validate: (input: string) => (input ? true : 'orderOcoRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderOco(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
    describe: decodeSelectedEntities(`Test new order creation and signature/recvWindow long.
Creates and validates a new order but does not send it into the matching engine.
Weight: |Condition| Request Weight|
|------------           | ------------ |
|Without &#x60;computeCommissionRates&#x60;| 1|
|With &#x60;computeCommissionRates&#x60;|20|`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'orderTestRequest: ',
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
            console.log('order-test is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input orderTestRequest:',
                validate: (input: string) => (input ? true : 'orderTestRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderTest(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
    describe: decodeSelectedEntities(`Places an order using smart order routing (SOR).

This adds 1 order to the &#x60;EXCHANGE_MAX_ORDERS&#x60; filter and the &#x60;MAX_NUM_ORDERS&#x60; filter.

Read [SOR FAQ](faqs/sor_faq.md) to learn more.
Weight: 1

Unfilled Order Count: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'sorOrderRequest: ',
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
            console.log('sor-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input sorOrderRequest:',
                validate: (input: string) => (input ? true : 'sorOrderRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.sorOrder(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
        decodeSelectedEntities(`Test new order creation and signature/recvWindow using smart order routing (SOR).
Creates and validates a new order but does not send it into the matching engine.
Weight: | Condition | Request Weight |
| --------- | -------------- |
| Without &#x60;computeCommissionRates&#x60;  |  1 |
| With &#x60;computeCommissionRates&#x60;     | 20 |`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'sorOrderTestRequest: ',
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
            console.log('sor-order-test is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input sorOrderTestRequest:',
                validate: (input: string) => (input ? true : 'sorOrderTestRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.sorOrderTest(
                !isEmpty(stdinObj) ? stdinObj : options.json ? JSON.parse(options.json) : options
            );
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
