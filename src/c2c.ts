import { C2C, C2C_REST_API_PROD_URL } from '@binance/c2c';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('c2c');

const stdinObj: any = readStdinObj();

let basePath = C2C_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'c2c');

if (process.env.BINANCE_C2C_BASE_PATH) {
    basePath = process.env.BINANCE_C2C_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new C2C({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new C2C({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const c2cCommands: any[] = [];

c2cCommands.push({
    command: 'get-c2-c-trade-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get C2C Trade History

* The max interval between startTimestamp and endTimestamp is 30 days.
* If startTimestamp and endTimestamp are not sent, the recent 30 days&#39; data will be returned.
* You can only view data from the past 6 months. To see all C2C orders, please check https://c2c.binance.com/en/fiatOrder

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'trade-type': {
                describe: decodeSelectedEntities('BUY, SELL'),
                type: 'string',
                group: 'Command Options:',
            },
            'start-timestamp': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'end-timestamp': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            page: {
                describe: decodeSelectedEntities('Default 1'),
                type: 'string',
                group: 'Command Options:',
            },
            rows: {
                describe: decodeSelectedEntities('default 100, max 100'),
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
                'get-c2-c-trade-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getC2CTradeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'c2c',
    description: 'Binance C2C REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli c2c <command> [options]');
        c2cCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
