import {
    Spot,
    SPOT_WS_STREAMS_PROD_URL,
    SPOT_WS_STREAMS_TESTNET_URL,
    SPOT_WS_STREAMS_DEMO_URL,
} from '@binance/spot';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('spot');

    let basePath = SPOT_WS_STREAMS_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'spot');

    if (process.env.BINANCE_SPOT_WS_STREAMS_BASE_PATH) {
        basePath = process.env.BINANCE_SPOT_WS_STREAMS_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['wsStreamsBasePath']) {
        basePath = configurationRestAPI['wsStreamsBasePath'];
    } else if (configurationRestAPI && configurationRestAPI['env']) {
        switch (configurationRestAPI['env']) {
            case 'testnet':
                basePath = SPOT_WS_STREAMS_TESTNET_URL;
                break;
            case 'demo':
                basePath = SPOT_WS_STREAMS_DEMO_URL;
                break;
        }
    }

    const configurationWebsocketStreams = {
        wsURL: basePath,
    };

    return {
        client: new Spot({ configurationWebsocketStreams }),
    };
};
const spotCommands: any[] = [];

spotCommands.push({
    command: 'agg-trade',
    describe: decodeSelectedEntities(
        'The Aggregate Trade Streams push trade information that is aggregated for a single taker order.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.aggTrade(options);
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

spotCommands.push({
    command: 'all-market-rolling-window-ticker',
    describe: decodeSelectedEntities(
        `Rolling window ticker statistics for all market symbols, computed over multiple windows.
Note that only tickers that have changed will be present in the array.`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'window-size': {
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

                if (!options?.['windowSize'] && !options?.interactive) {
                    requiredParams.push('windowSize');
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

        if (options.interactive && !options?.['windowSize']) {
            questions.push({
                type: 'input',
                name: 'windowSize',
                message: 'Input windowSize:',
                validate: (input: string) => (input ? true : 'windowSize cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.allMarketRollingWindowTicker(options);
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

spotCommands.push({
    command: 'all-mini-ticker',
    describe: decodeSelectedEntities(
        '24hr rolling window mini-ticker statistics for all symbols that changed in an array. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs. Note that only tickers that have changed will be present in the array.',
        isFullDescription
    ),
    handler: async (options: any) => {
        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.allMiniTicker();
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

spotCommands.push({
    command: 'avg-price',
    describe: decodeSelectedEntities(
        'Average price streams push changes in the average price over a fixed time interval.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.avgPrice(options);
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

spotCommands.push({
    command: 'block-trade',
    describe: decodeSelectedEntities('', isFullDescription),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.blockTrade(options);
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

spotCommands.push({
    command: 'book-ticker',
    describe: decodeSelectedEntities(
        `Pushes any update to the best bid or ask&#39;s price or quantity in real-time for a specified symbol.
Multiple &#x60;&lt;symbol&gt;@bookTicker&#x60; streams can be subscribed to over one connection.`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.bookTicker(options);
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

spotCommands.push({
    command: 'diff-book-depth',
    describe: decodeSelectedEntities(
        'Order book price and quantity depth updates used to locally manage an order book.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.diffBookDepth(options);
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

spotCommands.push({
    command: 'kline',
    describe: decodeSelectedEntities(
        `The Kline/Candlestick Stream push updates to the current klines/candlestick every second in &#x60;UTC+0&#x60; timezone

&lt;a id&#x3D;&quot;kline-intervals&quot;&gt;&lt;/a&gt;`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.kline(options);
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

spotCommands.push({
    command: 'kline-offset',
    describe: decodeSelectedEntities(
        'The Kline/Candlestick Stream push updates to the current klines/candlestick every second in &#x60;UTC+8&#x60; timezone',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.klineOffset(options);
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

spotCommands.push({
    command: 'mini-ticker',
    describe: decodeSelectedEntities(
        '24hr rolling window mini-ticker statistics. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.miniTicker(options);
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

spotCommands.push({
    command: 'partial-book-depth',
    describe: decodeSelectedEntities(
        'Top **\&lt;levels\&gt;** bids and asks, pushed every second. Valid **\&lt;levels\&gt;** are 5, 10, or 20.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.partialBookDepth(options);
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

spotCommands.push({
    command: 'reference-price',
    describe: decodeSelectedEntities('', isFullDescription),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.referencePrice(options);
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

spotCommands.push({
    command: 'rolling-window-ticker',
    describe: decodeSelectedEntities(
        'Rolling window ticker statistics for a single symbol, computed over multiple windows.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'window-size': {
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

                if (!options?.['windowSize'] && !options?.interactive) {
                    requiredParams.push('windowSize');
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

        if (options.interactive && !options?.['windowSize']) {
            questions.push({
                type: 'input',
                name: 'windowSize',
                message: 'Input windowSize:',
                validate: (input: string) => (input ? true : 'windowSize cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.rollingWindowTicker(options);
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

spotCommands.push({
    command: 'ticker',
    describe: decodeSelectedEntities(
        '24hr rolling window ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.ticker(options);
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

spotCommands.push({
    command: 'trade',
    describe: decodeSelectedEntities(
        'The Trade Streams push raw trade information; each trade has a unique buyer and seller.',
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
            const stream = connection.trade(options);
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

export default {
    command: 'spot-streams',
    description: 'Binance Spot WebSocket Streams',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli spot-streams <command> [options]').options({
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
        spotCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
