import { VIPLoan, VIP_LOAN_REST_API_PROD_URL } from '@binance/vip-loan';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('vip-loan');

const stdinObj: any = readStdinObj();

let basePath = VIP_LOAN_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'vip-loan');

if (process.env.BINANCE_VIP_LOAN_BASE_PATH) {
    basePath = process.env.BINANCE_VIP_LOAN_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new VIPLoan({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new VIPLoan({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const vipLoanCommands: any[] = [];

vipLoanCommands.push({
    command: 'get-borrow-interest-rate',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Borrow Interest Rate

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'loan-coin': {
                    describe: decodeSelectedEntities(
                        'Max 10 assets, Multiple split by \&quot;,\&quot;'
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

                if (!options?.['loanCoin'] && !options?.interactive) {
                    requiredParams.push('loanCoin');
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
                'get-borrow-interest-rate is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.loanCoin) {
            questions.push({
                type: 'input',
                name: 'loanCoin',
                message: 'Input loanCoin:',
                validate: (input: string) => (input ? true : 'loanCoin cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBorrowInterestRate(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'get-collateral-asset-data',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Collateral Asset Data

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'collateral-coin': {
                describe: decodeSelectedEntities(''),
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
                'get-collateral-asset-data is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCollateralAssetData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'get-loanable-assets-data',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get interest rate and borrow limit of loanable assets. The borrow limit is shown in USD value.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'loan-coin': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'vip-level': {
                describe: decodeSelectedEntities('default:user\&#39;s vip level'),
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
                'get-loanable-assets-data is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLoanableAssetsData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'get-vip-loan-interest-rate-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Check VIP Loan flexible interest rate history

* If startTime and endTime are not sent, the recent 90-day data will be returned
* The max interval between startTime and end Time is 180 days.
* Time based on UTC+0.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                coin: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
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
                current: {
                    describe: decodeSelectedEntities(
                        'Current querying page. Start from 1; default: 1; max: 1000'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default: 10; max: 100'),
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

                if (!options?.['coin'] && !options?.interactive) {
                    requiredParams.push('coin');
                }

                if (!options?.['recvWindow'] && !options?.interactive) {
                    requiredParams.push('recvWindow');
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
                'get-vip-loan-interest-rate-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.coin) {
            questions.push({
                type: 'input',
                name: 'coin',
                message: 'Input coin:',
                validate: (input: string) => (input ? true : 'coin cannot be empty'),
            });
        }
        if (options.interactive && !options.recvWindow) {
            questions.push({
                type: 'input',
                name: 'recvWindow',
                message: 'Input recvWindow:',
                validate: (input: string) => (input ? true : 'recvWindow cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getVIPLoanInterestRateHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'vip-loan-borrow',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`VIP loan is available for VIP users only.

* loanAccountId refer to loan receiving account
* Only master account applications are supported
* loanAccountId and collateralAccountId under same master account
* loanTerm is mandatory if user choose stable rate

Weight: 0`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'loan-account-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'loan-coin': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'loan-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'collateral-account-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'collateral-coin': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'is-flexible-rate': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'loan-term': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
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

                if (!options?.['loanAccountId'] && !options?.interactive) {
                    requiredParams.push('loanAccountId');
                }

                if (!options?.['loanCoin'] && !options?.interactive) {
                    requiredParams.push('loanCoin');
                }

                if (!options?.['loanAmount'] && !options?.interactive) {
                    requiredParams.push('loanAmount');
                }

                if (!options?.['collateralAccountId'] && !options?.interactive) {
                    requiredParams.push('collateralAccountId');
                }

                if (!options?.['collateralCoin'] && !options?.interactive) {
                    requiredParams.push('collateralCoin');
                }

                if (!options?.['isFlexibleRate'] && !options?.interactive) {
                    requiredParams.push('isFlexibleRate');
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
                'vip-loan-borrow is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['loanAccountId']) {
            questions.push({
                type: 'input',
                name: 'loanAccountId',
                message: 'Input loanAccountId:',
                validate: (input: string) => (input ? true : 'loanAccountId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['loanCoin']) {
            questions.push({
                type: 'input',
                name: 'loanCoin',
                message: 'Input loanCoin:',
                validate: (input: string) => (input ? true : 'loanCoin cannot be empty'),
            });
        }

        if (options.interactive && !options?.['loanAmount']) {
            questions.push({
                type: 'input',
                name: 'loanAmount',
                message: 'Input loanAmount:',
                validate: (input: string) => (input ? true : 'loanAmount cannot be empty'),
            });
        }

        if (options.interactive && !options?.['collateralAccountId']) {
            questions.push({
                type: 'input',
                name: 'collateralAccountId',
                message: 'Input collateralAccountId:',
                validate: (input: string) => (input ? true : 'collateralAccountId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['collateralCoin']) {
            questions.push({
                type: 'input',
                name: 'collateralCoin',
                message: 'Input collateralCoin:',
                validate: (input: string) => (input ? true : 'collateralCoin cannot be empty'),
            });
        }

        if (options.interactive && !options?.['isFlexibleRate']) {
            questions.push({
                type: 'input',
                name: 'isFlexibleRate',
                message: 'Input isFlexibleRate:',
                validate: (input: string) => (input ? true : 'isFlexibleRate cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.vipLoanBorrow(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'vip-loan-renew',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`VIP loan is available for VIP users only.

Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'loan-term': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
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

                if (!options?.['orderId'] && !options?.interactive) {
                    requiredParams.push('orderId');
                }

                if (!options?.['loanTerm'] && !options?.interactive) {
                    requiredParams.push('loanTerm');
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
                'vip-loan-renew is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['orderId']) {
            questions.push({
                type: 'input',
                name: 'orderId',
                message: 'Input orderId:',
                validate: (input: string) => (input ? true : 'orderId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['loanTerm']) {
            questions.push({
                type: 'input',
                name: 'loanTerm',
                message: 'Input loanTerm:',
                validate: (input: string) => (input ? true : 'loanTerm cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.vipLoanRenew(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'vip-loan-repay',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`VIP loan is available for VIP users only.

Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
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

                if (!options?.['orderId'] && !options?.interactive) {
                    requiredParams.push('orderId');
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
                'vip-loan-repay is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['orderId']) {
            questions.push({
                type: 'input',
                name: 'orderId',
                message: 'Input orderId:',
                validate: (input: string) => (input ? true : 'orderId cannot be empty'),
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
            const response = await client.restAPI.vipLoanRepay(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'check-vip-loan-collateral-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`VIP loan is available for VIP users only

* If the login account is loan account, all collateral accounts under the loan account can be queried.
* If the login account is collateral account, only the current collateral account can be queried.

Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'collateral-account-id': {
                describe: decodeSelectedEntities(''),
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
                'check-vip-loan-collateral-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.checkVIPLoanCollateralAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'get-vip-loan-accrued-interest',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Check VIP Loan interest record

* If startTime and endTime are not sent, the recent 90-day data will be returned.
* The max interval between startTime and endTime is 90 days.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'loan-coin': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
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
            current: {
                describe: decodeSelectedEntities(
                    'Current querying page. Start from 1; default: 1; max: 1000'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default: 10; max: 100'),
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
                'get-vip-loan-accrued-interest is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getVIPLoanAccruedInterest(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'get-vip-loan-ongoing-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`VIP loan is available for VIP users only.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'collateral-account-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'loan-coin': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'collateral-coin': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            current: {
                describe: decodeSelectedEntities(
                    'Current querying page. Start from 1; default: 1; max: 1000'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default: 10; max: 100'),
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
                'get-vip-loan-ongoing-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getVIPLoanOngoingOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

vipLoanCommands.push({
    command: 'query-application-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Application Status

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            current: {
                describe: decodeSelectedEntities(
                    'Current querying page. Start from 1; default: 1; max: 1000'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default: 10; max: 100'),
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
                'query-application-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryApplicationStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'vip-loan',
    description: 'Binance VIP Loan REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli vip-loan <command> [options]');
        vipLoanCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
