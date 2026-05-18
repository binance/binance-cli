import { Alpha, ALPHA_REST_API_PROD_URL } from '@binance/alpha';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('alpha');

    let basePath = ALPHA_REST_API_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'alpha');

    if (process.env.BINANCE_ALPHA_BASE_PATH) {
        basePath = process.env.BINANCE_ALPHA_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['basePath']) {
        basePath = configurationRestAPI['basePath'];
    }

    let client;
    let hasConfig = false;
    if (configurationRestAPI !== null) {
        hasConfig = true;
        client = new Alpha({
            configurationRestAPI: { ...configurationRestAPI, basePath },
        });
    } else {
        client = new Alpha({
            configurationRestAPI: {
                apiKey: '',
                basePath,
            },
        });
    }

    return { client, hasConfig };
};

const alphaCommands: any[] = [];

alphaCommands.push({
    command: 'aggregated-trades',
    describe: decodeSelectedEntities(
        `Retrieves compressed, aggregated historical trades for a specific symbol. Useful for recent trade history.

Weight: 0`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(
                        'e.g., \&quot;ALPHA_175USDT\&quot; – use token ID from Token List'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-id': {
                    describe: decodeSelectedEntities('starting trade ID to fetch from'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities('start timestamp (milliseconds)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities('end timestamp (milliseconds)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'number of results to return (default 500, max 1000)'
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
            const response = await client.restAPI.aggregatedTrades(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

alphaCommands.push({
    command: 'get-exchange-info',
    describe: decodeSelectedEntities(
        `Fetches general exchange information, such as supported symbols, rate limits, and server time.

Weight: 0`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

        try {
            const response = await client.restAPI.getExchangeInfo();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

alphaCommands.push({
    command: 'klines',
    describe: decodeSelectedEntities(
        `Fetches Kline/candlestick bars for a symbol, which include open/high/low/close prices and volume over intervals. Useful for charting and analysis.

Weight: 0`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(
                        'e.g., \&quot;ALPHA_175USDT\&quot; – use token ID from Token List'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
                    describe: decodeSelectedEntities(
                        'e.g., \&quot;1h\&quot; – supported intervals: 1s, 15s, 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities(
                        'number of results to return (default 500, max 1000)'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities('start timestamp (milliseconds)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities('end timestamp (milliseconds)'),
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
            const response = await client.restAPI.klines(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

alphaCommands.push({
    command: 'ticker',
    describe: decodeSelectedEntities(
        `Gets the 24-hour rolling window price change statistics for a symbol, including volume and price changes.

Weight: 0`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(
                        'e.g., \&quot;ALPHA_175USDT\&quot; – use token ID from Token List'
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
            const response = await client.restAPI.ticker(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

alphaCommands.push({
    command: 'token-list',
    describe: decodeSelectedEntities(
        `Retrieves a list of all available ALPHA tokens, including their IDs and symbols. Use this to find the token ID for constructing symbols in other endpoints.

Weight: 0`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

        try {
            const response = await client.restAPI.tokenList();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'alpha',
    description: 'Binance Alpha REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli alpha <command> [options]');
        alphaCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
