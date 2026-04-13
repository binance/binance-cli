import { CopyTrading, COPY_TRADING_REST_API_PROD_URL } from '@binance/copy-trading';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('copy-trading');

const stdinObj: any = readStdinObj();

let basePath = COPY_TRADING_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'copy-trading');

if (process.env.BINANCE_COPY_TRADING_BASE_PATH) {
    basePath = process.env.BINANCE_COPY_TRADING_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new CopyTrading({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new CopyTrading({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const copyTradingCommands: any[] = [];

copyTradingCommands.push({
    command: 'get-futures-lead-trader-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Futures Lead Trader Status

Weight: 20`),
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
            console.error(
                'get-futures-lead-trader-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFuturesLeadTraderStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

copyTradingCommands.push({
    command: 'get-futures-lead-trading-symbol-whitelist',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Futures Lead Trading Symbol Whitelist

Weight: 20`),
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
            console.error(
                'get-futures-lead-trading-symbol-whitelist is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFuturesLeadTradingSymbolWhitelist(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'copy-trading',
    description: 'Binance Copy Trading REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli copy-trading <command> [options]');
        copyTradingCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
