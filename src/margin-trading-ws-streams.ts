import { MarginTrading, MARGIN_TRADING_WS_STREAMS_PROD_URL } from '@binance/margin-trading';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('margin-trading');

    let basePath = MARGIN_TRADING_WS_STREAMS_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'margin-trading');

    if (process.env.BINANCE_MARGIN_TRADING_WS_STREAMS_BASE_PATH) {
        basePath = process.env.BINANCE_MARGIN_TRADING_WS_STREAMS_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['wsStreamsBasePath']) {
        basePath = configurationRestAPI['wsStreamsBasePath'];
    }

    const configurationWebsocketStreams = {
        wsURL: basePath,
    };

    return {
        client: new MarginTrading({ configurationWebsocketStreams }),
    };
};
const marginTradingCommands: any[] = [];

marginTradingCommands.push({
    command: 'risk-data',
    describe: decodeSelectedEntities(
        'Subscribes to the risk data WebSocket stream using the provided listen key.',
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
            const stream = connection.riskData(options?.['listen-key'], options?.['id']);
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

marginTradingCommands.push({
    command: 'trade-data',
    describe: decodeSelectedEntities(
        'Subscribes to the trade data WebSocket stream using the provided listen key.',
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
            const stream = connection.tradeData(options?.['listen-key'], options?.['id']);
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
    command: 'margin-trading-streams',
    description: 'Binance Margin Trading WebSocket Market Streams',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli margin-trading-streams <command> [options]').options({
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
        marginTradingCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
