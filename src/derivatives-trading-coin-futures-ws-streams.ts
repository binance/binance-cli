import {
    DerivativesTradingCoinFutures,
    DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_PROD_URL,
    DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_TESTNET_URL,
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

    let basePath = DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'futures-coin');

    if (process.env.BINANCE_FUTURES_COIN_WS_STREAMS_BASE_PATH) {
        basePath = process.env.BINANCE_FUTURES_COIN_WS_STREAMS_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['wsStreamsBasePath']) {
        basePath = configurationRestAPI['wsStreamsBasePath'];
    } else if (configurationRestAPI && configurationRestAPI['env']) {
        switch (configurationRestAPI['env']) {
            case 'demo':
            case 'testnet':
                basePath = DERIVATIVES_TRADING_COIN_FUTURES_WS_STREAMS_TESTNET_URL;
                break;
        }
    }

    const configurationWebsocketStreams = {
        wsURL: basePath,
    };

    return {
        client: new DerivativesTradingCoinFutures({ configurationWebsocketStreams }),
    };
};
const derivativesTradingCoinFuturesCommands: any[] = [];

derivativesTradingCoinFuturesCommands.push({
    command: 'aggregate-trade-streams',
    describe: decodeSelectedEntities(
        `The Aggregate Trade Streams push market trade information that is aggregated for fills with same price and taking side every 100 milliseconds.

Update Speed: 100ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.aggregateTradeStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'all-book-tickers-stream',
    describe: decodeSelectedEntities(
        `Pushes any update to the best bid or ask&#39;s price or quantity in real-time for all symbols.

Update Speed: Real-time`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            id: {
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.allBookTickersStream(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'all-market-liquidation-order-streams',
    describe: decodeSelectedEntities(
        `The All Liquidation Order Snapshot Streams push force liquidation order information for all symbols in the market.
For each symbol，only the latest one liquidation order within 1000ms will be pushed as the snapshot. If no liquidation happens in the interval of 1000ms, no stream will be pushed.

Update Speed: 1000ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            id: {
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.allMarketLiquidationOrderStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'all-market-mini-tickers-stream',
    describe: decodeSelectedEntities(
        `24hr rolling window mini-ticker statistics for all symbols. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before. Note that only tickers that have changed will be present in the array.

Update Speed: 1000ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            id: {
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.allMarketMiniTickersStream(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'all-market-tickers-streams',
    describe: decodeSelectedEntities(
        `24hr rolling window ticker statistics for all symbols. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before. Note that only tickers that have changed will be present in the array.

Update Speed: 1000ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            id: {
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.allMarketTickersStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'continuous-contract-kline-candlestick-streams',
    describe: decodeSelectedEntities(
        `Kline update every second

Update Speed: 250ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                pair: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'contract-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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

        if (options.interactive && !options?.['pair']) {
            questions.push({
                type: 'input',
                name: 'pair',
                message: 'Input pair:',
                validate: (input: string) => (input ? true : 'pair cannot be empty'),
            });
        }

        if (options.interactive && !options?.['contractType']) {
            questions.push({
                type: 'input',
                name: 'contractType',
                message: 'Input contractType:',
                validate: (input: string) => (input ? true : 'contractType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['interval']) {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.continuousContractKlineCandlestickStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'contract-info-stream',
    describe: decodeSelectedEntities(
        `ContractInfo stream pushes when contract info updates(listing/settlement/contract bracket update). &#x60;bks&#x60; field only shows up when bracket gets updated.

Update Speed: Real-time`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            id: {
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.contractInfoStream(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'diff-book-depth-streams',
    describe: decodeSelectedEntities(
        `Bids and asks, pushed every 250 milliseconds, 500 milliseconds, or 100 milliseconds

Update Speed: 250ms or 500ms or 100ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'update-speed': {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.diffBookDepthStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'index-kline-candlestick-streams',
    describe: decodeSelectedEntities(
        `Index Kline/Candlestick Streams

Update Speed: 250ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                pair: {
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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

        if (options.interactive && !options?.['pair']) {
            questions.push({
                type: 'input',
                name: 'pair',
                message: 'Input pair:',
                validate: (input: string) => (input ? true : 'pair cannot be empty'),
            });
        }

        if (options.interactive && !options?.['interval']) {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.indexKlineCandlestickStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'index-price-stream',
    describe: decodeSelectedEntities(
        `Index Price Stream

Update Speed: 3000ms OR 1000ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                pair: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'update-speed': {
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

        if (options.interactive && !options?.['pair']) {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.indexPriceStream(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'individual-symbol-book-ticker-streams',
    describe: decodeSelectedEntities(
        `Pushes any update to the best bid or ask&#39;s price or quantity in real-time for a specified symbol.

Update Speed: Real-time`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.individualSymbolBookTickerStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'individual-symbol-mini-ticker-stream',
    describe: decodeSelectedEntities(
        `24hr rolling window mini-ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before.

Update Speed: 500ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.individualSymbolMiniTickerStream(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'individual-symbol-ticker-streams',
    describe: decodeSelectedEntities(
        `24hr rolling window ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window from requestTime to 24hrs before.

Update Speed: 500ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.individualSymbolTickerStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'kline-candlestick-streams',
    describe: decodeSelectedEntities(
        `The Kline/Candlestick Stream push updates to the current klines/candlestick every 250 milliseconds (if existing).

Update Speed: 250ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['interval']) {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.klineCandlestickStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'liquidation-order-streams',
    describe: decodeSelectedEntities(
        `The Liquidation Order Snapshot Streams push force liquidation order information for specific symbol.

For each symbol，only the latest one liquidation order within 1000ms will be pushed as the snapshot. If no liquidation happens in the interval of 1000ms, no stream will be pushed.

Update Speed: 1000ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.liquidationOrderStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'mark-price-kline-candlestick-streams',
    describe: decodeSelectedEntities(
        `Mark Price Kline/Candlestick Streams

Update Speed: 250ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                interval: {
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

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['interval']) {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.markPriceKlineCandlestickStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'mark-price-of-all-symbols-of-a-pair',
    describe: decodeSelectedEntities(
        `Mark Price of All Symbols of a Pair

Update Speed: 3000ms OR 1000ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                pair: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'update-speed': {
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

        if (options.interactive && !options?.['pair']) {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.markPriceOfAllSymbolsOfAPair(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'mark-price-stream',
    describe: decodeSelectedEntities(
        `Mark price update stream

Update Speed: 3000ms OR 1000ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'update-speed': {
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
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.markPriceStream(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'partial-book-depth-streams',
    describe: decodeSelectedEntities(
        `Top **&lt;levels\&gt;** bids and asks, Valid **&lt;levels\&gt;** are 5, 10, or 20.

Update Speed: 250ms, 500ms or 100ms`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                levels: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'update-speed': {
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

                if (!options?.['levels'] && !options?.interactive) {
                    requiredParams.push('levels');
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

        if (options.interactive && !options?.['symbol']) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }

        if (options.interactive && !options?.['levels']) {
            questions.push({
                type: 'input',
                name: 'levels',
                message: 'Input levels:',
                validate: (input: string) => (input ? true : 'levels cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.partialBookDepthStreams(options);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount >= limit - 1 || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

derivativesTradingCoinFuturesCommands.push({
    command: 'user-data',
    describe: decodeSelectedEntities(
        'Subscribes to the user data WebSocket stream using the provided listen key.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'listen-key': {
                    type: 'string',
                    group: 'Command Options:',
                },
                id: {
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

                if (!options?.['listen-key'] && !options?.interactive) {
                    requiredParams.push('listen-key');
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

        if (options.interactive && !options?.['listen-key']) {
            questions.push({
                type: 'input',
                name: 'listen-key',
                message: 'Input listenKey:',
                validate: (input: string) => (input ? true : 'listen-key cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.userData(options?.['listen-key'], options?.['id']);
            const limit = options?.['stream-limit'];
            const startTime = Date.now();
            const durationMs = options?.['stream-duration'];
            let eventCount = 0;
            stream.on('message', (data: any) => {
                if (options?.['pretty']) {
                    console.info(data);
                } else {
                    console.info(JSON.stringify(data));
                }
                if (eventCount > limit || Date.now() - startTime > durationMs) {
                    connection.disconnect();
                    return;
                }
                eventCount = eventCount + 1;
            });
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'futures-coin-streams',
    description: 'Binance Derivatives Trading COIN Futures WebSocket Market Streams',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli futures-coin-streams <command> [options]').options({
            pretty: {
                type: 'boolean',
                group: 'Stream Options:',
                describe: 'Pretty-print the JSON output',
            },
            'stream-limit': {
                type: 'number',
                group: 'Stream Options:',
                describe: 'Stop listening after the given number of messages',
            },
            'stream-duration': {
                type: 'number',
                group: 'Stream Options:',
                describe: 'Stop listening after the given duration (in ms)',
            },
        });
        derivativesTradingCoinFuturesCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
