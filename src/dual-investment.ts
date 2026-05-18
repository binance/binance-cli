import { DualInvestment, DUAL_INVESTMENT_REST_API_PROD_URL } from '@binance/dual-investment';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('dual-investment');

    let basePath = DUAL_INVESTMENT_REST_API_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'dual-investment');

    if (process.env.BINANCE_DUAL_INVESTMENT_BASE_PATH) {
        basePath = process.env.BINANCE_DUAL_INVESTMENT_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['basePath']) {
        basePath = configurationRestAPI['basePath'];
    }

    let client;
    let hasConfig = false;
    if (configurationRestAPI !== null) {
        hasConfig = true;
        client = new DualInvestment({
            configurationRestAPI: { ...configurationRestAPI, basePath },
        });
    } else {
        client = new DualInvestment({
            configurationRestAPI: {
                apiKey: '',
                basePath,
            },
        });
    }

    return { client, hasConfig };
};

const dualInvestmentCommands: any[] = [];

dualInvestmentCommands.push({
    command: 'get-dual-investment-product-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Dual Investment product list

Weight: 1(IP)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'option-type': {
                    describe: decodeSelectedEntities('Input CALL or PUT'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'exercised-coin': {
                    describe: decodeSelectedEntities(
                        'Target exercised asset, e.g.: if you subscribe to a high sell product (call option), you should input: &#x60;optionType&#x60;:CALL,&#x60;exercisedCoin&#x60;:USDT,&#x60;investCoin&#x60;:BNB; if you subscribe to a low buy product (put option), you should input: &#x60;optionType&#x60;:PUT,&#x60;exercisedCoin&#x60;:BNB,&#x60;investCoin&#x60;:USDT'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'invest-coin': {
                    describe: decodeSelectedEntities(
                        'Asset used for subscribing, e.g.: if you subscribe to a high sell product (call option), you should input: &#x60;optionType&#x60;:CALL,&#x60;exercisedCoin&#x60;:USDT,&#x60;investCoin&#x60;:BNB; if you subscribe to a low buy product (put option), you should input: &#x60;optionType&#x60;:PUT,&#x60;exercisedCoin&#x60;:BNB,&#x60;investCoin&#x60;:USDT'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-size': {
                    describe: decodeSelectedEntities('Default: 10, Maximum: 100'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-index': {
                    describe: decodeSelectedEntities('Default: 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000'),
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

                if (!options?.['optionType'] && !options?.interactive) {
                    requiredParams.push('optionType');
                }

                if (!options?.['exercisedCoin'] && !options?.interactive) {
                    requiredParams.push('exercisedCoin');
                }

                if (!options?.['investCoin'] && !options?.interactive) {
                    requiredParams.push('investCoin');
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

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-dual-investment-product-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.optionType) {
            questions.push({
                type: 'input',
                name: 'optionType',
                message: 'Input optionType:',
                validate: (input: string) => (input ? true : 'optionType cannot be empty'),
            });
        }
        if (options.interactive && !options.exercisedCoin) {
            questions.push({
                type: 'input',
                name: 'exercisedCoin',
                message: 'Input exercisedCoin:',
                validate: (input: string) => (input ? true : 'exercisedCoin cannot be empty'),
            });
        }
        if (options.interactive && !options.investCoin) {
            questions.push({
                type: 'input',
                name: 'investCoin',
                message: 'Input investCoin:',
                validate: (input: string) => (input ? true : 'investCoin cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getDualInvestmentProductList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

dualInvestmentCommands.push({
    command: 'change-auto-compound-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Change Auto-Compound status

Weight: 1(IP)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'position-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-compound-plan': {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['positionId'] && !options?.interactive) {
                    requiredParams.push('positionId');
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

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'change-auto-compound-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['positionId']) {
            questions.push({
                type: 'input',
                name: 'positionId',
                message: 'Input positionId:',
                validate: (input: string) => (input ? true : 'positionId cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.changeAutoCompoundStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

dualInvestmentCommands.push({
    command: 'check-dual-investment-accounts',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Check Dual Investment accounts

Weight: 1(IP)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000'),
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

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'check-dual-investment-accounts is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.checkDualInvestmentAccounts(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

dualInvestmentCommands.push({
    command: 'get-dual-investment-positions',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Dual Investment positions (batch)

Weight: 1(IP)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            status: {
                describe: decodeSelectedEntities(
                    '&#x60;PENDING&#x60;:Products are purchasing, will give results later;&#x60;PURCHASE_SUCCESS&#x60;:purchase successfully;&#x60;SETTLED&#x60;: Products are finish settling;&#x60;PURCHASE_FAIL&#x60;:fail to purchase;&#x60;REFUNDING&#x60;:refund ongoing;&#x60;REFUND_SUCCESS&#x60;:refund to spot account successfully; &#x60;SETTLING&#x60;:Products are settling. If don\&#39;t fill this field, will response all the position status.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'page-size': {
                describe: decodeSelectedEntities('Default: 10, Maximum: 100'),
                type: 'string',
                group: 'Command Options:',
            },
            'page-index': {
                describe: decodeSelectedEntities('Default: 1'),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000'),
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

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-dual-investment-positions is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getDualInvestmentPositions(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

dualInvestmentCommands.push({
    command: 'subscribe-dual-investment-products',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Subscribe Dual Investment products

* Products are not available. // this means APR changes to lower value, or orders are not unavailable.
* Failed. This means System or network errors.

Weight: 1(IP)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                id: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'deposit-amount': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-compound-plan': {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['id'] && !options?.interactive) {
                    requiredParams.push('id');
                }

                if (!options?.['orderId'] && !options?.interactive) {
                    requiredParams.push('orderId');
                }

                if (!options?.['depositAmount'] && !options?.interactive) {
                    requiredParams.push('depositAmount');
                }

                if (!options?.['autoCompoundPlan'] && !options?.interactive) {
                    requiredParams.push('autoCompoundPlan');
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

        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'subscribe-dual-investment-products is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['id']) {
            questions.push({
                type: 'input',
                name: 'id',
                message: 'Input id:',
                validate: (input: string) => (input ? true : 'id cannot be empty'),
            });
        }

        if (options.interactive && !options?.['orderId']) {
            questions.push({
                type: 'input',
                name: 'orderId',
                message: 'Input orderId:',
                validate: (input: string) => (input ? true : 'orderId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['depositAmount']) {
            questions.push({
                type: 'input',
                name: 'depositAmount',
                message: 'Input depositAmount:',
                validate: (input: string) => (input ? true : 'depositAmount cannot be empty'),
            });
        }

        if (options.interactive && !options?.['autoCompoundPlan']) {
            questions.push({
                type: 'input',
                name: 'autoCompoundPlan',
                message: 'Input autoCompoundPlan:',
                validate: (input: string) => (input ? true : 'autoCompoundPlan cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.subscribeDualInvestmentProducts(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'dual-investment',
    description: 'Binance Dual Investment REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli dual-investment <command> [options]');
        dualInvestmentCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
