import { CryptoLoan, CRYPTO_LOAN_REST_API_PROD_URL } from '@binance/crypto-loan';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('crypto-loan');

const stdinObj: any = readStdinObj();

let basePath = CRYPTO_LOAN_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'crypto-loan');

if (process.env.BINANCE_CRYPTO_LOAN_BASE_PATH) {
    basePath = process.env.BINANCE_CRYPTO_LOAN_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new CryptoLoan({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new CryptoLoan({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const cryptoLoanCommands: any[] = [];

cryptoLoanCommands.push({
    command: 'check-collateral-repay-rate',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`
Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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

                if (!options?.['collateralCoin'] && !options?.interactive) {
                    requiredParams.push('collateralCoin');
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
                'check-collateral-repay-rate is signed. Please create a profile using `binance-cli profile create`.'
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
        if (options.interactive && !options.collateralCoin) {
            questions.push({
                type: 'input',
                name: 'collateralCoin',
                message: 'Input collateralCoin:',
                validate: (input: string) => (input ? true : 'collateralCoin cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.checkCollateralRepayRate(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'flexible-loan-adjust-ltv',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Flexible Loan Adjust LTV

* API Key needs Spot &amp; Margin Trading permission for this endpoint

Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'loan-coin': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'collateral-coin': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'adjustment-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                direction: {
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

                if (!options?.['loanCoin'] && !options?.interactive) {
                    requiredParams.push('loanCoin');
                }

                if (!options?.['collateralCoin'] && !options?.interactive) {
                    requiredParams.push('collateralCoin');
                }

                if (!options?.['adjustmentAmount'] && !options?.interactive) {
                    requiredParams.push('adjustmentAmount');
                }

                if (!options?.['direction'] && !options?.interactive) {
                    requiredParams.push('direction');
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
                'flexible-loan-adjust-ltv is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['loanCoin']) {
            questions.push({
                type: 'input',
                name: 'loanCoin',
                message: 'Input loanCoin:',
                validate: (input: string) => (input ? true : 'loanCoin cannot be empty'),
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

        if (options.interactive && !options?.['adjustmentAmount']) {
            questions.push({
                type: 'input',
                name: 'adjustmentAmount',
                message: 'Input adjustmentAmount:',
                validate: (input: string) => (input ? true : 'adjustmentAmount cannot be empty'),
            });
        }

        if (options.interactive && !options?.['direction']) {
            questions.push({
                type: 'input',
                name: 'direction',
                message: 'Input direction:',
                validate: (input: string) => (input ? true : 'direction cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.flexibleLoanAdjustLtv(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'flexible-loan-borrow',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Borrow Flexible Loan


* This API endpoint is available for both the master account and the sub-account.
* You can customize LTV by entering loanAmount and collateralAmount.

Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'loan-coin': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'loan-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'collateral-coin': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'collateral-amount': {
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

                if (!options?.['loanCoin'] && !options?.interactive) {
                    requiredParams.push('loanCoin');
                }

                if (!options?.['collateralCoin'] && !options?.interactive) {
                    requiredParams.push('collateralCoin');
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
                'flexible-loan-borrow is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['loanCoin']) {
            questions.push({
                type: 'input',
                name: 'loanCoin',
                message: 'Input loanCoin:',
                validate: (input: string) => (input ? true : 'loanCoin cannot be empty'),
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.flexibleLoanBorrow(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'flexible-loan-repay',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Flexible Loan Repay


* repayAmount is mandatory even fullRepayment &#x3D; FALSE

Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'loan-coin': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'collateral-coin': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'repay-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'collateral-return': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'full-repayment': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'repayment-type': {
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

                if (!options?.['loanCoin'] && !options?.interactive) {
                    requiredParams.push('loanCoin');
                }

                if (!options?.['collateralCoin'] && !options?.interactive) {
                    requiredParams.push('collateralCoin');
                }

                if (!options?.['repayAmount'] && !options?.interactive) {
                    requiredParams.push('repayAmount');
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
                'flexible-loan-repay is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['loanCoin']) {
            questions.push({
                type: 'input',
                name: 'loanCoin',
                message: 'Input loanCoin:',
                validate: (input: string) => (input ? true : 'loanCoin cannot be empty'),
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

        if (options.interactive && !options?.['repayAmount']) {
            questions.push({
                type: 'input',
                name: 'repayAmount',
                message: 'Input repayAmount:',
                validate: (input: string) => (input ? true : 'repayAmount cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.flexibleLoanRepay(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-flexible-loan-assets-data',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get interest rate and borrow limit of flexible loanable assets. The borrow limit is shown in USD value.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'loan-coin': {
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
                'get-flexible-loan-assets-data is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleLoanAssetsData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-flexible-loan-borrow-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Flexible Loan Borrow History

* If startTime and endTime are not sent, the recent 90-day data will be returned.
* The max interval between startTime and endTime is 180 days.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
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
                'get-flexible-loan-borrow-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleLoanBorrowHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-flexible-loan-collateral-assets-data',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get LTV information and collateral limit of flexible loan&#39;s collateral assets. The collateral limit is shown in USD value.

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
                'get-flexible-loan-collateral-assets-data is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleLoanCollateralAssetsData(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-flexible-loan-interest-rate-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Check Flexible Loan interest rate history

* If startTime and endTime are not sent, the recent 90-day data will be returned
* The max interval between startTime and endTime is 90 days.
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
                'get-flexible-loan-interest-rate-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.getFlexibleLoanInterestRateHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-flexible-loan-liquidation-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`
Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
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
                'get-flexible-loan-liquidation-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleLoanLiquidationHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-flexible-loan-ltv-adjustment-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Flexible Loan LTV Adjustment History

* If startTime and endTime are not sent, the recent 90-day data will be returned.
* The max interval between startTime and endTime is 180 days.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
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
                'get-flexible-loan-ltv-adjustment-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleLoanLtvAdjustmentHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-flexible-loan-ongoing-orders',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Flexible Loan Ongoing Orders

Weight: 300`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
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
                'get-flexible-loan-ongoing-orders is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleLoanOngoingOrders(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-flexible-loan-repayment-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Flexible Loan Repayment History

* If startTime and endTime are not sent, the recent 90-day data will be returned.
* The max interval between startTime and endTime is 180 days.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
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
                'get-flexible-loan-repayment-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleLoanRepaymentHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'check-collateral-repay-rate-stable-rate',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get the the rate of collateral coin / loan coin when using collateral repay, the rate will be valid within 8 second.

Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
                'repay-amount': {
                    describe: decodeSelectedEntities('repay amount of loanCoin'),
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

                if (!options?.['collateralCoin'] && !options?.interactive) {
                    requiredParams.push('collateralCoin');
                }

                if (!options?.['repayAmount'] && !options?.interactive) {
                    requiredParams.push('repayAmount');
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
                'check-collateral-repay-rate-stable-rate is signed. Please create a profile using `binance-cli profile create`.'
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
        if (options.interactive && !options.collateralCoin) {
            questions.push({
                type: 'input',
                name: 'collateralCoin',
                message: 'Input collateralCoin:',
                validate: (input: string) => (input ? true : 'collateralCoin cannot be empty'),
            });
        }
        if (options.interactive && !options.repayAmount) {
            questions.push({
                type: 'input',
                name: 'repayAmount',
                message: 'Input repayAmount:',
                validate: (input: string) => (input ? true : 'repayAmount cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.checkCollateralRepayRateStableRate(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-crypto-loans-income-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Crypto Loans Income History

* If startTime and endTime are not sent, the recent 7-day data will be returned.
* The max interval between startTime and endTime is 30 days.

Weight: 6000`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            type: {
                describe: decodeSelectedEntities(
                    'All types will be returned by default. Enum：&#x60;borrowIn&#x60; ,&#x60;collateralSpent&#x60;, &#x60;repayAmount&#x60;, &#x60;collateralReturn&#x60;(Collateral return after repayment), &#x60;addCollateral&#x60;, &#x60;removeCollateral&#x60;, &#x60;collateralReturnAfterLiquidation&#x60;'
                ),
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
                'get-crypto-loans-income-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCryptoLoansIncomeHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-loan-borrow-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Loan Borrow History

* If startTime and endTime are not sent, the recent 90-day data will be returned.
* The max interval between startTime and endTime is 180 days.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-id': {
                describe: decodeSelectedEntities(
                    'orderId in &#x60;POST /sapi/v1/loan/borrow&#x60;'
                ),
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
                'get-loan-borrow-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLoanBorrowHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-loan-ltv-adjustment-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Loan LTV Adjustment History

* If startTime and endTime are not sent, the recent 90-day data will be returned.
* The max interval between startTime and endTime is 180 days.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-id': {
                describe: decodeSelectedEntities(
                    'orderId in &#x60;POST /sapi/v1/loan/borrow&#x60;'
                ),
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
                'get-loan-ltv-adjustment-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLoanLtvAdjustmentHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

cryptoLoanCommands.push({
    command: 'get-loan-repayment-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Get Loan Repayment History

* If startTime and endTime are not sent, the recent 90-day data will be returned.
* The max interval between startTime and endTime is 180 days.

Weight: 400`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'order-id': {
                describe: decodeSelectedEntities(
                    'orderId in &#x60;POST /sapi/v1/loan/borrow&#x60;'
                ),
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
                'get-loan-repayment-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLoanRepaymentHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'crypto-loan',
    description: 'Binance Crypto Loan REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli crypto-loan <command> [options]');
        cryptoLoanCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
