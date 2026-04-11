import { Algo, ALGO_REST_API_PROD_URL } from '@binance/algo';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('algo');

const stdinObj: any = readStdinObj();

let basePath = ALGO_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'algo');

if (process.env.BINANCE_ALGO_BASE_PATH) {
    basePath = process.env.BINANCE_ALGO_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new Algo({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new Algo({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const algoCommands: any[] = [];

algoCommands.push({
    command: 'cancel-algo-order-future-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an active order.

* You need to enable &#x60;Futures Trading Permission&#x60; for the api key which requests this endpoint.
* Base URL: https://api.binance.com

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'algo-id': {
                    describe: decodeSelectedEntities('eg. 14511'),
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

                if (!options?.['algoId'] && !options?.interactive) {
                    requiredParams.push('algoId');
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
                'cancel-algo-order-future-algo is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algoId) {
            questions.push({
                type: 'input',
                name: 'algoId',
                message: 'Input algoId:',
                validate: (input: string) => (input ? true : 'algoId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelAlgoOrderFutureAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'query-current-algo-open-orders-future-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Current Algo Open Orders

* You need to enable &#x60;Futures Trading Permission&#x60; for the api key which requests this endpoint.
* Base URL: https://api.binance.com

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
                'query-current-algo-open-orders-future-algo is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentAlgoOpenOrdersFutureAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'query-historical-algo-orders-future-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Historical Algo Order

* You need to enable &#x60;Futures Trading Permission&#x60; for the api key which requests this endpoint.
* Base URL: https://api.binance.com

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Trading symbol eg. BTCUSDT'),
                type: 'string',
                group: 'Command Options:',
            },
            side: {
                describe: decodeSelectedEntities('BUY or SELL'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('in milliseconds  eg.1641522717552'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('in milliseconds  eg.1641522526562'),
                type: 'string',
                group: 'Command Options:',
            },
            page: {
                describe: decodeSelectedEntities('Default is 1'),
                type: 'string',
                group: 'Command Options:',
            },
            'page-size': {
                describe: decodeSelectedEntities('MIN 1, MAX 100; Default 100'),
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
                'query-historical-algo-orders-future-algo is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryHistoricalAlgoOrdersFutureAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'query-sub-orders-future-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get respective sub orders for a specified algoId

* You need to enable &#x60;Futures Trading Permission&#x60; for the api key which requests this endpoint.
* Base URL: https://api.binance.com

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'algo-id': {
                    describe: decodeSelectedEntities('eg. 14511'),
                    type: 'string',
                    group: 'Command Options:',
                },
                page: {
                    describe: decodeSelectedEntities('Default is 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-size': {
                    describe: decodeSelectedEntities('MIN 1, MAX 100; Default 100'),
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

                if (!options?.['algoId'] && !options?.interactive) {
                    requiredParams.push('algoId');
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
                'query-sub-orders-future-algo is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algoId) {
            questions.push({
                type: 'input',
                name: 'algoId',
                message: 'Input algoId:',
                validate: (input: string) => (input ? true : 'algoId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySubOrdersFutureAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'time-weighted-average-price-future-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send in a Twap new order.
Only support on USDⓈ-M Contracts.

* Total Algo open orders max allowed: &#x60;30&#x60; orders.
* Leverage of symbols and position mode will be the same as your futures account settings. You can set up through the trading page or fapi.
* Receiving &#x60;&quot;success&quot;: true&#x60; does not mean that your order will be executed. Please use the query order endpoints（&#x60;GET sapi/v1/algo/futures/openOrders&#x60; or &#x60;GET sapi/v1/algo/futures/historicalOrders&#x60;） to check the order status.
For example: Your futures balance is insufficient, or open position with reduce only or position side is inconsistent with your own setting. In these cases you will receive &#x60;&quot;success&quot;: true&#x60;, but the order status will be &#x60;expired&#x60; after we check it.
* &#x60;quantity&#x60; * 60 / &#x60;duration&#x60; should be larger than minQty
* &#x60;duration&#x60; cannot be less than 5 mins or more than 24 hours.
* For delivery contracts, TWAP end time should be one hour earlier than the delivery time of the symbol.
* You need to enable &#x60;Futures Trading Permission&#x60; for the api key which requests this endpoint.
* Base URL: https://api.binance.com

Weight: 3000`),
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
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                duration: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-algo-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'reduce-only': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'limit-price': {
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

                if (!options?.['duration'] && !options?.interactive) {
                    requiredParams.push('duration');
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
                'time-weighted-average-price-future-algo is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['duration']) {
            questions.push({
                type: 'input',
                name: 'duration',
                message: 'Input duration:',
                validate: (input: string) => (input ? true : 'duration cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.timeWeightedAveragePriceFutureAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'volume-participation-future-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Send in a VP new order.
Only support on USDⓈ-M Contracts.

* Total Algo open orders max allowed: &#x60;10&#x60; orders.
* Leverage of symbols and position mode will be the same as your futures account settings. You can set up through the trading page or fapi.
* Receiving &#x60;&quot;success&quot;: true&#x60; does not mean that your order will be executed. Please use the query order endpoints（&#x60;GET sapi/v1/algo/futures/openOrders&#x60; or &#x60;GET sapi/v1/algo/futures/historicalOrders&#x60;） to check the order status.
For example: Your futures balance is insufficient, or open position with reduce only or position side is inconsistent with your own setting. In these cases you will receive &#x60;&quot;success&quot;: true&#x60;, but the order status will be &#x60;expired&#x60; after we check it.
* You need to enable &#x60;Futures Trading Permission&#x60; for the api key which requests this endpoint.
* Base URL: https://api.binance.com

Weight: 300`),
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
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                urgency: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-algo-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'reduce-only': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'limit-price': {
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

                if (!options?.['urgency'] && !options?.interactive) {
                    requiredParams.push('urgency');
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
                'volume-participation-future-algo is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['urgency']) {
            questions.push({
                type: 'input',
                name: 'urgency',
                message: 'Input urgency:',
                validate: (input: string) => (input ? true : 'urgency cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.volumeParticipationFutureAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'cancel-algo-order-spot-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Cancel an open TWAP order

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'algo-id': {
                    describe: decodeSelectedEntities('eg. 14511'),
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

                if (!options?.['algoId'] && !options?.interactive) {
                    requiredParams.push('algoId');
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
                'cancel-algo-order-spot-algo is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algoId) {
            questions.push({
                type: 'input',
                name: 'algoId',
                message: 'Input algoId:',
                validate: (input: string) => (input ? true : 'algoId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelAlgoOrderSpotAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'query-current-algo-open-orders-spot-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all open SPOT TWAP orders

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
                'query-current-algo-open-orders-spot-algo is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryCurrentAlgoOpenOrdersSpotAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'query-historical-algo-orders-spot-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get all historical SPOT TWAP orders

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
                describe: decodeSelectedEntities('Trading symbol eg. BTCUSDT'),
                type: 'string',
                group: 'Command Options:',
            },
            side: {
                describe: decodeSelectedEntities('BUY or SELL'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-time': {
                describe: decodeSelectedEntities('in milliseconds  eg.1641522717552'),
                type: 'string',
                group: 'Command Options:',
            },
            'end-time': {
                describe: decodeSelectedEntities('in milliseconds  eg.1641522526562'),
                type: 'string',
                group: 'Command Options:',
            },
            page: {
                describe: decodeSelectedEntities('Default is 1'),
                type: 'string',
                group: 'Command Options:',
            },
            'page-size': {
                describe: decodeSelectedEntities('MIN 1, MAX 100; Default 100'),
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
                'query-historical-algo-orders-spot-algo is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryHistoricalAlgoOrdersSpotAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'query-sub-orders-spot-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get respective sub orders for a specified algoId

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'algo-id': {
                    describe: decodeSelectedEntities('eg. 14511'),
                    type: 'string',
                    group: 'Command Options:',
                },
                page: {
                    describe: decodeSelectedEntities('Default is 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-size': {
                    describe: decodeSelectedEntities('MIN 1, MAX 100; Default 100'),
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

                if (!options?.['algoId'] && !options?.interactive) {
                    requiredParams.push('algoId');
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
                'query-sub-orders-spot-algo is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algoId) {
            questions.push({
                type: 'input',
                name: 'algoId',
                message: 'Input algoId:',
                validate: (input: string) => (input ? true : 'algoId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySubOrdersSpotAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

algoCommands.push({
    command: 'time-weighted-average-price-spot-algo',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Place a new spot TWAP order with Algo service.

* Total Algo open orders max allowed: &#x60;20&#x60; orders.

Weight: 3000`),
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
                quantity: {
                    type: 'string',
                    group: 'Command Options:',
                },
                duration: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-algo-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'limit-price': {
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

                if (!options?.['duration'] && !options?.interactive) {
                    requiredParams.push('duration');
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
                'time-weighted-average-price-spot-algo is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['duration']) {
            questions.push({
                type: 'input',
                name: 'duration',
                message: 'Input duration:',
                validate: (input: string) => (input ? true : 'duration cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.timeWeightedAveragePriceSpotAlgo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'algo',
    description: 'Binance Algo REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli algo <command> [options]');
        algoCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
