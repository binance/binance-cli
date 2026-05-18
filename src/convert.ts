import { Convert, CONVERT_REST_API_PROD_URL } from '@binance/convert';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('convert');

    let basePath = CONVERT_REST_API_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'convert');

    if (process.env.BINANCE_CONVERT_BASE_PATH) {
        basePath = process.env.BINANCE_CONVERT_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['basePath']) {
        basePath = configurationRestAPI['basePath'];
    }

    let client;
    let hasConfig = false;
    if (configurationRestAPI !== null) {
        hasConfig = true;
        client = new Convert({
            configurationRestAPI: { ...configurationRestAPI, basePath },
        });
    } else {
        client = new Convert({
            configurationRestAPI: {
                apiKey: '',
                basePath,
            },
        });
    }

    return { client, hasConfig };
};

const convertCommands: any[] = [];

convertCommands.push({
    command: 'list-all-convert-pairs',
    describe: decodeSelectedEntities(
        `Query for all convertible token pairs and the tokens’ respective upper/lower limits

* User needs to supply either or both of the input parameter
* If not defined for both fromAsset and toAsset, only partial token pairs will be returned

Weight: 3000(IP)`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'from-asset': {
                describe: decodeSelectedEntities('User spends coin'),
                type: 'string',
                group: 'Command Options:',
            },
            'to-asset': {
                describe: decodeSelectedEntities('User receives coin'),
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
            const response = await client.restAPI.listAllConvertPairs(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

convertCommands.push({
    command: 'query-order-quantity-precision-per-asset',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query for supported asset’s precision information

Weight: 100(IP)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000'),
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
                'query-order-quantity-precision-per-asset is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryOrderQuantityPrecisionPerAsset(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

convertCommands.push({
    command: 'accept-quote',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Accept the offered quote by quote ID.

Weight: 500(UID)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'quote-id': {
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

                if (!options?.['quoteId'] && !options?.interactive) {
                    requiredParams.push('quoteId');
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
                'accept-quote is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['quoteId']) {
            questions.push({
                type: 'input',
                name: 'quoteId',
                message: 'Input quoteId:',
                validate: (input: string) => (input ? true : 'quoteId cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.acceptQuote(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

convertCommands.push({
    command: 'cancel-limit-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Enable users to cancel a limit order

Weight: 200(UID)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'order-id': {
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
                'cancel-limit-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['orderId']) {
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
            const response = await client.restAPI.cancelLimitOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

convertCommands.push({
    command: 'get-convert-trade-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Convert Trade History

* The max interval between startTime and endTime is 30 days.

Weight: 3000`,
            isFullDescription
        ),
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
                limit: {
                    describe: decodeSelectedEntities('Default 100, Max 1000'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000'),
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
                'get-convert-trade-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getConvertTradeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

convertCommands.push({
    command: 'order-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query order status by order ID.

Weight: 100(UID)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-id': {
                describe: decodeSelectedEntities('Either orderId or quoteId is required'),
                type: 'string',
                group: 'Command Options:',
            },
            'quote-id': {
                describe: decodeSelectedEntities('Either orderId or quoteId is required'),
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
                'order-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.orderStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

convertCommands.push({
    command: 'place-limit-order',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Enable users to place a limit order

* &#x60;baseAsset&#x60; or &#x60;quoteAsset&#x60; can be determined via &#x60;exchangeInfo&#x60; endpoint.
* Limit price is defined from &#x60;baseAsset&#x60; to &#x60;quoteAsset&#x60;.
* Either &#x60;baseAmount&#x60; or &#x60;quoteAmount&#x60; is used.

Weight: 500(UID)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'base-asset': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'quote-asset': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'limit-price': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'base-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'quote-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                side: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'wallet-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'expired-type': {
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

                if (!options?.['baseAsset'] && !options?.interactive) {
                    requiredParams.push('baseAsset');
                }

                if (!options?.['quoteAsset'] && !options?.interactive) {
                    requiredParams.push('quoteAsset');
                }

                if (!options?.['limitPrice'] && !options?.interactive) {
                    requiredParams.push('limitPrice');
                }

                if (!options?.['side'] && !options?.interactive) {
                    requiredParams.push('side');
                }

                if (!options?.['expiredType'] && !options?.interactive) {
                    requiredParams.push('expiredType');
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
                'place-limit-order is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['baseAsset']) {
            questions.push({
                type: 'input',
                name: 'baseAsset',
                message: 'Input baseAsset:',
                validate: (input: string) => (input ? true : 'baseAsset cannot be empty'),
            });
        }

        if (options.interactive && !options?.['quoteAsset']) {
            questions.push({
                type: 'input',
                name: 'quoteAsset',
                message: 'Input quoteAsset:',
                validate: (input: string) => (input ? true : 'quoteAsset cannot be empty'),
            });
        }

        if (options.interactive && !options?.['limitPrice']) {
            questions.push({
                type: 'input',
                name: 'limitPrice',
                message: 'Input limitPrice:',
                validate: (input: string) => (input ? true : 'limitPrice cannot be empty'),
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

        if (options.interactive && !options?.['expiredType']) {
            questions.push({
                type: 'input',
                name: 'expiredType',
                message: 'Input expiredType:',
                validate: (input: string) => (input ? true : 'expiredType cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.placeLimitOrder(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

convertCommands.push({
    command: 'query-limit-open-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Request a quote for the requested token pairs

Weight: 3000(UID)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000'),
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
                'query-limit-open-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryLimitOpenOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

convertCommands.push({
    command: 'send-quote-request',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Request a quote for the requested token pairs

* Either fromAmount or toAmount should be sent
* &#x60;quoteId&#x60; will be returned only if you have enough funds to convert

Weight: 200(UID)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'from-asset': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-asset': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'wallet-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'valid-time': {
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

                if (!options?.['fromAsset'] && !options?.interactive) {
                    requiredParams.push('fromAsset');
                }

                if (!options?.['toAsset'] && !options?.interactive) {
                    requiredParams.push('toAsset');
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
                'send-quote-request is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['fromAsset']) {
            questions.push({
                type: 'input',
                name: 'fromAsset',
                message: 'Input fromAsset:',
                validate: (input: string) => (input ? true : 'fromAsset cannot be empty'),
            });
        }

        if (options.interactive && !options?.['toAsset']) {
            questions.push({
                type: 'input',
                name: 'toAsset',
                message: 'Input toAsset:',
                validate: (input: string) => (input ? true : 'toAsset cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.sendQuoteRequest(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'convert',
    description: 'Binance Convert REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli convert <command> [options]');
        convertCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
