import { Mining, MINING_REST_API_PROD_URL } from '@binance/mining';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('mining');

const stdinObj: any = readStdinObj();

let basePath = MINING_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'mining');

if (process.env.BINANCE_MINING_BASE_PATH) {
    basePath = process.env.BINANCE_MINING_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new Mining({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new Mining({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const miningCommands: any[] = [];

miningCommands.push({
    command: 'account-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Account List

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                algo: {
                    describe: decodeSelectedEntities('Algorithm(sha256) sha256'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'user-name': {
                    describe: decodeSelectedEntities('Mining account test'),
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

                if (!options?.['algo'] && !options?.interactive) {
                    requiredParams.push('algo');
                }

                if (!options?.['userName'] && !options?.interactive) {
                    requiredParams.push('userName');
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
            console.log(
                'account-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algo) {
            questions.push({
                type: 'input',
                name: 'algo',
                message: 'Input algo:',
                validate: (input: string) => (input ? true : 'algo cannot be empty'),
            });
        }
        if (options.interactive && !options.userName) {
            questions.push({
                type: 'input',
                name: 'userName',
                message: 'Input userName:',
                validate: (input: string) => (input ? true : 'userName cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'acquiring-algorithm',
    describe: decodeSelectedEntities(`Acquiring Algorithm

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.acquiringAlgorithm();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'acquiring-coinname',
    describe: decodeSelectedEntities(`Acquiring CoinName

Weight: 1`),
    handler: async () => {
        try {
            const response = await client.restAPI.acquiringCoinname();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'cancel-hashrate-resale-configuration',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`
Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'config-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'user-name': {
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

                if (!options?.['configId'] && !options?.interactive) {
                    requiredParams.push('configId');
                }

                if (!options?.['userName'] && !options?.interactive) {
                    requiredParams.push('userName');
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
            console.log(
                'cancel-hashrate-resale-configuration is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['configId']) {
            questions.push({
                type: 'input',
                name: 'configId',
                message: 'Input configId:',
                validate: (input: string) => (input ? true : 'configId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['userName']) {
            questions.push({
                type: 'input',
                name: 'userName',
                message: 'Input userName:',
                validate: (input: string) => (input ? true : 'userName cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.cancelHashrateResaleConfiguration(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'earnings-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Query Earnings List

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                algo: {
                    describe: decodeSelectedEntities('Algorithm(sha256) sha256'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'user-name': {
                    describe: decodeSelectedEntities('Mining account test'),
                    type: 'string',
                    group: 'Command Options:',
                },
                coin: {
                    describe: decodeSelectedEntities('Coin Name '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-date': {
                    describe: decodeSelectedEntities('Millisecond timestamp '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-date': {
                    describe: decodeSelectedEntities('Millisecond timestamp '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-index': {
                    describe: decodeSelectedEntities(
                        'Page number, empty default first page, starting from 1 '
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-size': {
                    describe: decodeSelectedEntities('Min 10,Max 200 '),
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

                if (!options?.['algo'] && !options?.interactive) {
                    requiredParams.push('algo');
                }

                if (!options?.['userName'] && !options?.interactive) {
                    requiredParams.push('userName');
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
            console.log(
                'earnings-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algo) {
            questions.push({
                type: 'input',
                name: 'algo',
                message: 'Input algo:',
                validate: (input: string) => (input ? true : 'algo cannot be empty'),
            });
        }
        if (options.interactive && !options.userName) {
            questions.push({
                type: 'input',
                name: 'userName',
                message: 'Input userName:',
                validate: (input: string) => (input ? true : 'userName cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.earningsList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'extra-bonus-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Extra Bonus List

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                algo: {
                    describe: decodeSelectedEntities('Algorithm(sha256) sha256'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'user-name': {
                    describe: decodeSelectedEntities('Mining account test'),
                    type: 'string',
                    group: 'Command Options:',
                },
                coin: {
                    describe: decodeSelectedEntities('Coin Name '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-date': {
                    describe: decodeSelectedEntities('Millisecond timestamp '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-date': {
                    describe: decodeSelectedEntities('Millisecond timestamp '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-index': {
                    describe: decodeSelectedEntities(
                        'Page number, empty default first page, starting from 1 '
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-size': {
                    describe: decodeSelectedEntities('Min 10,Max 200 '),
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

                if (!options?.['algo'] && !options?.interactive) {
                    requiredParams.push('algo');
                }

                if (!options?.['userName'] && !options?.interactive) {
                    requiredParams.push('userName');
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
            console.log(
                'extra-bonus-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algo) {
            questions.push({
                type: 'input',
                name: 'algo',
                message: 'Input algo:',
                validate: (input: string) => (input ? true : 'algo cannot be empty'),
            });
        }
        if (options.interactive && !options.userName) {
            questions.push({
                type: 'input',
                name: 'userName',
                message: 'Input userName:',
                validate: (input: string) => (input ? true : 'userName cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.extraBonusList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'hashrate-resale-detail',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Hashrate Resale Detail(USER_DATA)

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'config-id': {
                    describe: decodeSelectedEntities('Mining ID 168'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-index': {
                    describe: decodeSelectedEntities(
                        'Page number, empty default first page, starting from 1 '
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-size': {
                    describe: decodeSelectedEntities('Min 10,Max 200 '),
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

                if (!options?.['configId'] && !options?.interactive) {
                    requiredParams.push('configId');
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
            console.log(
                'hashrate-resale-detail is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.configId) {
            questions.push({
                type: 'input',
                name: 'configId',
                message: 'Input configId:',
                validate: (input: string) => (input ? true : 'configId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.hashrateResaleDetail(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'hashrate-resale-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Hashrate Resale List

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'page-index': {
                describe: decodeSelectedEntities(
                    'Page number, empty default first page, starting from 1 '
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'page-size': {
                describe: decodeSelectedEntities('Min 10,Max 200 '),
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
                'hashrate-resale-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.hashrateResaleList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'hashrate-resale-request',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Hashrate Resale Request

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'user-name': {
                    type: 'string',
                    group: 'Command Options:',
                },
                algo: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-date': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-date': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-pool-user': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'hash-rate': {
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

                if (!options?.['userName'] && !options?.interactive) {
                    requiredParams.push('userName');
                }

                if (!options?.['algo'] && !options?.interactive) {
                    requiredParams.push('algo');
                }

                if (!options?.['endDate'] && !options?.interactive) {
                    requiredParams.push('endDate');
                }

                if (!options?.['startDate'] && !options?.interactive) {
                    requiredParams.push('startDate');
                }

                if (!options?.['toPoolUser'] && !options?.interactive) {
                    requiredParams.push('toPoolUser');
                }

                if (!options?.['hashRate'] && !options?.interactive) {
                    requiredParams.push('hashRate');
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
            console.log(
                'hashrate-resale-request is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['userName']) {
            questions.push({
                type: 'input',
                name: 'userName',
                message: 'Input userName:',
                validate: (input: string) => (input ? true : 'userName cannot be empty'),
            });
        }

        if (options.interactive && !options?.['algo']) {
            questions.push({
                type: 'input',
                name: 'algo',
                message: 'Input algo:',
                validate: (input: string) => (input ? true : 'algo cannot be empty'),
            });
        }

        if (options.interactive && !options?.['endDate']) {
            questions.push({
                type: 'input',
                name: 'endDate',
                message: 'Input endDate:',
                validate: (input: string) => (input ? true : 'endDate cannot be empty'),
            });
        }

        if (options.interactive && !options?.['startDate']) {
            questions.push({
                type: 'input',
                name: 'startDate',
                message: 'Input startDate:',
                validate: (input: string) => (input ? true : 'startDate cannot be empty'),
            });
        }

        if (options.interactive && !options?.['toPoolUser']) {
            questions.push({
                type: 'input',
                name: 'toPoolUser',
                message: 'Input toPoolUser:',
                validate: (input: string) => (input ? true : 'toPoolUser cannot be empty'),
            });
        }

        if (options.interactive && !options?.['hashRate']) {
            questions.push({
                type: 'input',
                name: 'hashRate',
                message: 'Input hashRate:',
                validate: (input: string) => (input ? true : 'hashRate cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.hashrateResaleRequest(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'mining-account-earning',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Mining Account Earning

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                algo: {
                    describe: decodeSelectedEntities('Algorithm(sha256) sha256'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-date': {
                    describe: decodeSelectedEntities('Millisecond timestamp '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-date': {
                    describe: decodeSelectedEntities('Millisecond timestamp '),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-index': {
                    describe: decodeSelectedEntities(
                        'Page number, empty default first page, starting from 1 '
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-size': {
                    describe: decodeSelectedEntities('Min 10,Max 200 '),
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

                if (!options?.['algo'] && !options?.interactive) {
                    requiredParams.push('algo');
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
            console.log(
                'mining-account-earning is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algo) {
            questions.push({
                type: 'input',
                name: 'algo',
                message: 'Input algo:',
                validate: (input: string) => (input ? true : 'algo cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.miningAccountEarning(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'request-for-detail-miner-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Request for Detail Miner List

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                algo: {
                    describe: decodeSelectedEntities('Algorithm(sha256) sha256'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'user-name': {
                    describe: decodeSelectedEntities('Mining account test'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'worker-name': {
                    describe: decodeSelectedEntities('Miner’s name(required) bhdc1.16A10404B'),
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

                if (!options?.['algo'] && !options?.interactive) {
                    requiredParams.push('algo');
                }

                if (!options?.['userName'] && !options?.interactive) {
                    requiredParams.push('userName');
                }

                if (!options?.['workerName'] && !options?.interactive) {
                    requiredParams.push('workerName');
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
            console.log(
                'request-for-detail-miner-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algo) {
            questions.push({
                type: 'input',
                name: 'algo',
                message: 'Input algo:',
                validate: (input: string) => (input ? true : 'algo cannot be empty'),
            });
        }
        if (options.interactive && !options.userName) {
            questions.push({
                type: 'input',
                name: 'userName',
                message: 'Input userName:',
                validate: (input: string) => (input ? true : 'userName cannot be empty'),
            });
        }
        if (options.interactive && !options.workerName) {
            questions.push({
                type: 'input',
                name: 'workerName',
                message: 'Input workerName:',
                validate: (input: string) => (input ? true : 'workerName cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.requestForDetailMinerList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'request-for-miner-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Request for Miner List

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                algo: {
                    describe: decodeSelectedEntities('Algorithm(sha256) sha256'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'user-name': {
                    describe: decodeSelectedEntities('Mining account test'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'page-index': {
                    describe: decodeSelectedEntities(
                        'Page number, empty default first page, starting from 1 '
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                sort: {
                    describe: decodeSelectedEntities(
                        'sort sequence(default&#x3D;0)0 positive sequence，1 negative sequence'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'sort-column': {
                    describe: decodeSelectedEntities(
                        'Sort by( default 1): &lt;br&gt;&lt;/br&gt;1: miner name, &lt;br&gt;&lt;/br&gt;2: real-time computing power, &lt;br&gt;&lt;/br&gt;3: daily average computing power, &lt;br&gt;&lt;/br&gt;4: real-time rejection rate, &lt;br&gt;&lt;/br&gt;5: last submission time'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'worker-status': {
                    describe: decodeSelectedEntities(
                        'miners status(default&#x3D;0),0 all，1 valid，2 invalid，3 failure'
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

                if (!options?.['algo'] && !options?.interactive) {
                    requiredParams.push('algo');
                }

                if (!options?.['userName'] && !options?.interactive) {
                    requiredParams.push('userName');
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
            console.log(
                'request-for-miner-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algo) {
            questions.push({
                type: 'input',
                name: 'algo',
                message: 'Input algo:',
                validate: (input: string) => (input ? true : 'algo cannot be empty'),
            });
        }
        if (options.interactive && !options.userName) {
            questions.push({
                type: 'input',
                name: 'userName',
                message: 'Input userName:',
                validate: (input: string) => (input ? true : 'userName cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.requestForMinerList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

miningCommands.push({
    command: 'statistic-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`Statistic List

Weight: 5`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                algo: {
                    describe: decodeSelectedEntities('Algorithm(sha256) sha256'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'user-name': {
                    describe: decodeSelectedEntities('Mining account test'),
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

                if (!options?.['algo'] && !options?.interactive) {
                    requiredParams.push('algo');
                }

                if (!options?.['userName'] && !options?.interactive) {
                    requiredParams.push('userName');
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
            console.log(
                'statistic-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.algo) {
            questions.push({
                type: 'input',
                name: 'algo',
                message: 'Input algo:',
                validate: (input: string) => (input ? true : 'algo cannot be empty'),
            });
        }
        if (options.interactive && !options.userName) {
            questions.push({
                type: 'input',
                name: 'userName',
                message: 'Input userName:',
                validate: (input: string) => (input ? true : 'userName cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.statisticList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'mining',
    description: 'Binance Mining REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli mining <command> [options]');
        miningCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
