import { Convert, CONVERT_REST_API_PROD_URL } from '@binance/convert';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('convert');

const stdinObj: any = readStdinObj();

let basePath = CONVERT_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'convert');

if (process.env.BINANCE_CONVERT_BASE_PATH) {
    basePath = process.env.BINANCE_CONVERT_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
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

const convertCommands: any[] = [];

convertCommands.push({
    command: 'list-all-convert-pairs',
    describe:
        decodeSelectedEntities(`Query for all convertible token pairs and the tokens’ respective upper/lower limits

* User needs to supply either or both of the input parameter
* If not defined for both fromAsset and toAsset, only partial token pairs will be returned

Weight: 3000(IP)`),
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

convertCommands.push({
    command: 'query-order-quantity-precision-per-asset',
    describe: decodeSelectedEntities(`Query for supported asset’s precision information

Weight: 100(IP)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000'),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'query-order-quantity-precision-per-asset is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryOrderQuantityPrecisionPerAsset({
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

convertCommands.push({
    command: 'accept-quote',
    describe: decodeSelectedEntities(`Accept the offered quote by quote ID.

Weight: 500(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'acceptQuoteRequest: ',
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
            console.log('accept-quote is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input acceptQuoteRequest:',
                validate: (input: string) => (input ? true : 'acceptQuoteRequest cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.acceptQuote(
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

convertCommands.push({
    command: 'cancel-limit-order',
    describe: decodeSelectedEntities(`Enable users to cancel a limit order

Weight: 200(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'cancelLimitOrderRequest: ',
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
            console.log('cancel-limit-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input cancelLimitOrderRequest:',
                validate: (input: string) =>
                    input ? true : 'cancelLimitOrderRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelLimitOrder(
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

convertCommands.push({
    command: 'get-convert-trade-history',
    describe: decodeSelectedEntities(`Get Convert Trade History

* The max interval between startTime and endTime is 30 days.

Weight: 3000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'start-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                'end-time': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                },
                limit: {
                    describe: decodeSelectedEntities('Default 100, Max 1000'),
                    type: 'string',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000'),
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
                'get-convert-trade-history is signed. Please login using `binance-cli login`'
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
            const response = await client.restAPI.getConvertTradeHistory({
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

convertCommands.push({
    command: 'order-status',
    describe: decodeSelectedEntities(`Query order status by order ID.

Weight: 100(UID)`),
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

convertCommands.push({
    command: 'place-limit-order',
    describe: decodeSelectedEntities(`Enable users to place a limit order

* &#x60;baseAsset&#x60; or &#x60;quoteAsset&#x60; can be determined via &#x60;exchangeInfo&#x60; endpoint.
* Limit price is defined from &#x60;baseAsset&#x60; to &#x60;quoteAsset&#x60;.
* Either &#x60;baseAmount&#x60; or &#x60;quoteAmount&#x60; is used.

Weight: 500(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                json: {
                    describe: 'placeLimitOrderRequest: ',
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
            console.log('place-limit-order is signed. Please login using `binance-cli login`');
            return;
        }

        if (options.interactive && !options.json) {
            questions.push({
                type: 'input',
                name: 'json',
                message: 'Input placeLimitOrderRequest:',
                validate: (input: string) =>
                    input ? true : 'placeLimitOrderRequest cannot be empty',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.placeLimitOrder(
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

convertCommands.push({
    command: 'query-limit-open-orders',
    describe: decodeSelectedEntities(`Request a quote for the requested token pairs

Weight: 3000(UID)`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000'),
                type: 'string',
            },
        });
    },
    handler: async (options: any) => {
        const questions: any = [];
        if (isEmpty(configurationRestAPI)) {
            console.log(
                'query-limit-open-orders is signed. Please login using `binance-cli login`'
            );
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryLimitOpenOrders({ ...stdinObj, ...options });
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
    describe: decodeSelectedEntities(`Request a quote for the requested token pairs

* Either fromAmount or toAmount should be sent
* &#x60;quoteId&#x60; will be returned only if you have enough funds to convert

Weight: 200(UID)`),
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
