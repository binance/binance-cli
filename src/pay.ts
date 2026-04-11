import { Pay, PAY_REST_API_PROD_URL } from '@binance/pay';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('pay');

const stdinObj: any = readStdinObj();

let basePath = PAY_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'pay');

if (process.env.BINANCE_PAY_BASE_PATH) {
    basePath = process.env.BINANCE_PAY_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new Pay({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new Pay({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const payCommands: any[] = [];

payCommands.push({
    command: 'get-pay-trade-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Pay Trade History

* If startTime and endTime are not sent, the recent 90 days&#39; data will be returned.
* The max interval between startTime and endTime is 90 days.
* Support for querying orders within the last 18 months.
* For payerInfo and receiverInfo，there are different return values in different orderTypes.
* Sender&#39;s perspective when orderType is C2C
* payerInfo : binanceId
* receiverInfo : name, binanceId/accountId/email/countryCode/phoneNumber/mobileCode (based on user input)
* Receiver&#39;s perspective when orderType is C2C
* payerInfo : name
* receiverInfo : binanceId
* Sender&#39;s perspective when orderType is CRYPTO_BOX
* payerInfo : binanceId
* receiverInfo : name(the value is always &quot;Crypto Box&quot;)
* Receiver&#39;s perspective when orderType is CRYPTO_BOX
* payerInfo : name
* receiverInfo : binanceId
* Sender&#39;s perspective when orderType is PAY
* payerInfo : binanceId
* receiverInfo : name
* Receiver&#39;s perspective when orderType is PAY
* payerInfo : name
* receiverInfo : binanceId, name
* Sender&#39;s perspective when orderType is PAY_REFUND
* payerInfo : binanceId, name
* receiverInfo : name, accountId
* Receiver&#39;s perspective when orderType is PAY_REFUND
* payerInfo : name
* receiverInfo :  binanceId
* Sender&#39;s perspective when orderType is PAYOUT
* payerInfo : binanceId, name
* receiverInfo : name, accountId
* Receiver&#39;s perspective when orderType is PAYOUT
* payerInfo : name
* receiverInfo :  binanceId
* Receiver&#39;s perspective when orderType is CRYPTO_BOX_RF
* payerInfo : name(the value is always &quot;Crypto Box&quot;)
* receiverInfo : binanceId
* Sender&#39;s perspective when orderType is REMITTANCE
* payerInfo : binanceId
* receiverInfo : name, institutionName, cardNumber, digitalWalletId

Weight: 3000`),
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
            limit: {
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
                'get-pay-trade-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getPayTradeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'pay',
    description: 'Binance Pay REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli pay <command> [options]');
        payCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
