import { Rebate, REBATE_REST_API_PROD_URL } from '@binance/rebate';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('rebate');

const stdinObj: any = readStdinObj();

let basePath = REBATE_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'rebate');

if (process.env.BINANCE_REBATE_BASE_PATH) {
    basePath = process.env.BINANCE_REBATE_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new Rebate({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new Rebate({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const rebateCommands: any[] = [];

rebateCommands.push({
    command: 'get-spot-rebate-history-records',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Spot Rebate History Records

* The max interval between startTime and endTime is 30 days.
* If startTime and endTime are not sent, the recent 7 days&#39; data will be returned.
* The earliest startTime is supported on June 10, 2020
* Return up to 200 records per request.

Weight: 12000`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
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
            page: {
                describe: decodeSelectedEntities('Default 1'),
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
            console.error(
                'get-spot-rebate-history-records is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSpotRebateHistoryRecords(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'rebate',
    description: 'Binance Rebate REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli rebate <command> [options]');
        rebateCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
