import {
    DerivativesTradingOptions,
    DERIVATIVES_TRADING_OPTIONS_WS_STREAMS_PROD_URL,
    DERIVATIVES_TRADING_OPTIONS_WS_STREAMS_TESTNET_URL,
} from '@binance/derivatives-trading-options';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('derivatives-trading-options');

    let basePath = DERIVATIVES_TRADING_OPTIONS_WS_STREAMS_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(
        parsedArgs?.profile,
        'derivatives-options'
    );

    if (process.env.BINANCE_DERIVATIVES_OPTIONS_WS_STREAMS_BASE_PATH) {
        basePath = process.env.BINANCE_DERIVATIVES_OPTIONS_WS_STREAMS_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['wsStreamsBasePath']) {
        basePath = configurationRestAPI['wsStreamsBasePath'];
    } else if (configurationRestAPI && configurationRestAPI['env']) {
        switch (configurationRestAPI['env']) {
            case 'demo':
            case 'testnet':
                basePath = DERIVATIVES_TRADING_OPTIONS_WS_STREAMS_TESTNET_URL;
                break;
        }
    }

    const configurationWebsocketStreams = {
        wsURL: basePath,
    };

    return {
        client: new DerivativesTradingOptions({ configurationWebsocketStreams }),
    };
};
const derivativesTradingOptionsCommands: any[] = [];

derivativesTradingOptionsCommands.push({
    command: 'index-price-streams',
    describe: decodeSelectedEntities(
        `Underlying(e.g ETHUSDT) index stream.

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
            const stream = connection.indexPriceStreams(options);
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

derivativesTradingOptionsCommands.push({
    command: 'kline-candlestick-streams',
    describe: decodeSelectedEntities(
        `The Kline/Candlestick Stream push updates to the current klines/candlestick every 1000 milliseconds (if existing).

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

derivativesTradingOptionsCommands.push({
    command: 'mark-price',
    describe: decodeSelectedEntities(
        `The mark price for all option symbols on specific underlying asset. E.g.[btcusdt@optionMarkPrice](wss://fstream.binance.com/market/stream?streams&#x3D;btcusdt@optionMarkPrice)

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
                underlying: {
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

                if (!options?.['underlying'] && !options?.interactive) {
                    requiredParams.push('underlying');
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

        if (options.interactive && !options?.['underlying']) {
            questions.push({
                type: 'input',
                name: 'underlying',
                message: 'Input underlying:',
                validate: (input: string) => (input ? true : 'underlying cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.markPrice(options);
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

derivativesTradingOptionsCommands.push({
    command: 'new-symbol-info',
    describe: decodeSelectedEntities(
        `New symbol listing stream.

Update Speed: 50ms`,
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
            const stream = connection.newSymbolInfo(options);
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

derivativesTradingOptionsCommands.push({
    command: 'open-interest',
    describe: decodeSelectedEntities(
        `Option open interest for specific underlying asset on specific expiration date. E.g.[ethusdt@openInterest@221125](wss://fstream.binance.com/market/stream?streams&#x3D;ethusdt@openInterest@221125)

Update Speed: 60s`,
        isFullDescription
    ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'expiration-date': {
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

                if (!options?.['expirationDate'] && !options?.interactive) {
                    requiredParams.push('expirationDate');
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

        if (options.interactive && !options?.['expirationDate']) {
            questions.push({
                type: 'input',
                name: 'expirationDate',
                message: 'Input expirationDate:',
                validate: (input: string) => (input ? true : 'expirationDate cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const { client } = getClient();
            const connection = await client.websocketStreams.connect();
            const stream = connection.openInterest(options);
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

derivativesTradingOptionsCommands.push({
    command: 'diff-book-depth-streams',
    describe: decodeSelectedEntities(
        `Bids and asks, pushed every 500 milliseconds, 100 milliseconds (if existing)

Update Speed: 100ms or 500ms`,
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

derivativesTradingOptionsCommands.push({
    command: 'individual-symbol-book-ticker-streams',
    describe: decodeSelectedEntities(
        `Pushes any update to the best bid or ask&#39;s price or quantity in real-time for a specified symbol.

Update Speed: Real-Time`,
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

derivativesTradingOptionsCommands.push({
    command: 'partial-book-depth-streams',
    describe: decodeSelectedEntities(
        `Top **&lt;levels\&gt;** bids and asks, Valid levels are **&lt;levels\&gt;** are 5, 10, 20.

Update Speed: 100ms or 500ms`,
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
                level: {
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

                if (!options?.['level'] && !options?.interactive) {
                    requiredParams.push('level');
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

        if (options.interactive && !options?.['level']) {
            questions.push({
                type: 'input',
                name: 'level',
                message: 'Input level:',
                validate: (input: string) => (input ? true : 'level cannot be empty'),
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

derivativesTradingOptionsCommands.push({
    command: 'ticker24-hour',
    describe: decodeSelectedEntities(
        `24hr ticker info for all symbols. Only symbols whose ticker info changed will be sent.

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
            const stream = connection.ticker24Hour(options);
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

derivativesTradingOptionsCommands.push({
    command: 'trade-streams',
    describe: decodeSelectedEntities(
        `The Trade Streams push raw trade information for specific symbol or underlying asset. E.g.[btcusdt@optionTrade](wss://fstream.binance.com/public/stream?streams&#x3D;btcusdt@optionTrade)

Update Speed: 50ms`,
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
            const stream = connection.tradeStreams(options);
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

derivativesTradingOptionsCommands.push({
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
    command: 'derivatives-options-streams',
    description: 'Binance Derivatives Trading Options WebSocket Market Streams',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli derivatives-options-streams <command> [options]').options({
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
        derivativesTradingOptionsCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
