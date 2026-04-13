import { Fiat, FIAT_REST_API_PROD_URL } from '@binance/fiat';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('fiat');

const stdinObj: any = readStdinObj();

let basePath = FIAT_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'fiat');

if (process.env.BINANCE_FIAT_BASE_PATH) {
    basePath = process.env.BINANCE_FIAT_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new Fiat({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new Fiat({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const fiatCommands: any[] = [];

fiatCommands.push({
    command: 'deposit',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Submit deposit request, in this version, we only support BRL deposit via pix.



For BRL deposit via pix, you need to place an order before making a transfer from your bank.

Before calling this api, please make sure you have already completed your KYC or KYB, and already activated your fiat service on our website.

Weight: 45000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'recv-window': {
                    type: 'string',
                    group: 'Command Options:',
                },
                currency: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'api-payment-method': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                ext: {
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

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['currency'] && !options?.interactive) {
                    requiredParams.push('currency');
                }

                if (!options?.['apiPaymentMethod'] && !options?.interactive) {
                    requiredParams.push('apiPaymentMethod');
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
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
                'deposit is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['currency']) {
            questions.push({
                type: 'input',
                name: 'currency',
                message: 'Input currency:',
                validate: (input: string) => (input ? true : 'currency cannot be empty'),
            });
        }

        if (options.interactive && !options?.['apiPaymentMethod']) {
            questions.push({
                type: 'input',
                name: 'apiPaymentMethod',
                message: 'Input apiPaymentMethod:',
                validate: (input: string) => (input ? true : 'apiPaymentMethod cannot be empty'),
            });
        }

        if (options.interactive && !options?.['amount']) {
            questions.push({
                type: 'input',
                name: 'amount',
                message: 'Input amount:',
                validate: (input: string) => (input ? true : 'amount cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.deposit(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

fiatCommands.push({
    command: 'fiat-withdraw',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Submit withdraw request, in this version, we support BRL,ARS,MXN withdrawal via bank_transfer.

You need to call this api first, and call query order detail api in a loop to get the status of the order until this order is successful.

Before calling this API, please ensure you have completed your KYC or KYB, activated your fiat service, and verified your destination bank account on our website.

you need to bind your bank account on web/app before using the corresponding account number

Weight: 45000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'recv-window': {
                    type: 'string',
                    group: 'Command Options:',
                },
                currency: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'api-payment-method': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'account-info': {
                    type: 'string',
                    group: 'Command Options:',
                },
                ext: {
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

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['currency'] && !options?.interactive) {
                    requiredParams.push('currency');
                }

                if (!options?.['apiPaymentMethod'] && !options?.interactive) {
                    requiredParams.push('apiPaymentMethod');
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
                }

                if (!options?.['accountInfo'] && !options?.interactive) {
                    requiredParams.push('accountInfo');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
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
                'fiat-withdraw is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['currency']) {
            questions.push({
                type: 'input',
                name: 'currency',
                message: 'Input currency:',
                validate: (input: string) => (input ? true : 'currency cannot be empty'),
            });
        }

        if (options.interactive && !options?.['apiPaymentMethod']) {
            questions.push({
                type: 'input',
                name: 'apiPaymentMethod',
                message: 'Input apiPaymentMethod:',
                validate: (input: string) => (input ? true : 'apiPaymentMethod cannot be empty'),
            });
        }

        if (options.interactive && !options?.['amount']) {
            questions.push({
                type: 'input',
                name: 'amount',
                message: 'Input amount:',
                validate: (input: string) => (input ? true : 'amount cannot be empty'),
            });
        }

        if (options.interactive && !options?.['accountInfo']) {
            questions.push({
                type: 'input',
                name: 'accountInfo',
                message: 'Input accountInfo:',
                validate: (input: string) => (input ? true : 'accountInfo cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.fiatWithdraw(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

fiatCommands.push({
    command: 'get-fiat-deposit-withdraw-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Fiat Deposit/Withdraw History

* If beginTime and endTime are not sent, the recent 30-day data will be returned.

Weight: 45000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'transaction-type': {
                    describe: decodeSelectedEntities('0-buy,1-sell'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'begin-time': {
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
                    describe: decodeSelectedEntities('default 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                rows: {
                    describe: decodeSelectedEntities('default 100, max 500'),
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['transactionType'] && !options?.interactive) {
                    requiredParams.push('transactionType');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
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
                'get-fiat-deposit-withdraw-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.transactionType) {
            questions.push({
                type: 'input',
                name: 'transactionType',
                message: 'Input transactionType:',
                validate: (input: string) => (input ? true : 'transactionType cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFiatDepositWithdrawHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

fiatCommands.push({
    command: 'get-fiat-payments-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Fiat Deposit/Withdraw History

* If beginTime and endTime are not sent, the recent 30-day data will be returned.
* paymentMethod: Only when requesting payments history for buy (transactionType&#x3D;0), response contains paymentMethod representing the way of purchase. Now we have:
* Cash Balance
* Credit Card
* Online Banking
* Bank Transfer

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'transaction-type': {
                    describe: decodeSelectedEntities('0-buy,1-sell'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'begin-time': {
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
                    describe: decodeSelectedEntities('default 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                rows: {
                    describe: decodeSelectedEntities('default 100, max 500'),
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['transactionType'] && !options?.interactive) {
                    requiredParams.push('transactionType');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
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
                'get-fiat-payments-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.transactionType) {
            questions.push({
                type: 'input',
                name: 'transactionType',
                message: 'Input transactionType:',
                validate: (input: string) => (input ? true : 'transactionType cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFiatPaymentsHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

fiatCommands.push({
    command: 'get-order-detail',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Order Detail

Before calling this api, please make sure you have already completed your KYC or KYB, and already activated your fiat service on our website.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'order-no': {
                    describe: decodeSelectedEntities(
                        'order id retrieved from the api call of withdrawal'
                    ),
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
            })
            .check((options: any) => {
                const requiredParams: any = [];

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['orderNo'] && !options?.interactive) {
                    requiredParams.push('orderNo');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
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
                'get-order-detail is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.orderNo) {
            questions.push({
                type: 'input',
                name: 'orderNo',
                message: 'Input orderNo:',
                validate: (input: string) => (input ? true : 'orderNo cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getOrderDetail(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'fiat',
    description: 'Binance Fiat REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli fiat <command> [options]');
        fiatCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
