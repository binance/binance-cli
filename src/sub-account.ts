import { SubAccount, SUB_ACCOUNT_REST_API_PROD_URL } from '@binance/sub-account';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('sub-account');

    let basePath = SUB_ACCOUNT_REST_API_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'sub-account');

    if (process.env.BINANCE_SUB_ACCOUNT_BASE_PATH) {
        basePath = process.env.BINANCE_SUB_ACCOUNT_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['basePath']) {
        basePath = configurationRestAPI['basePath'];
    }

    let client;
    let hasConfig = false;
    if (configurationRestAPI !== null) {
        hasConfig = true;
        client = new SubAccount({
            configurationRestAPI: { ...configurationRestAPI, basePath },
        });
    } else {
        client = new SubAccount({
            configurationRestAPI: {
                apiKey: '',
                basePath,
            },
        });
    }

    return { client, hasConfig };
};

const subAccountCommands: any[] = [];

subAccountCommands.push({
    command: 'create-a-virtual-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Create a Virtual Sub-account

* This request will generate a virtual sub account under your master account.
* You need to enable &quot;trade&quot; option for the API Key which requests this endpoint.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'sub-account-string': {
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

                if (!options?.['subAccountString'] && !options?.interactive) {
                    requiredParams.push('subAccountString');
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
                'create-a-virtual-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['subAccountString']) {
            questions.push({
                type: 'input',
                name: 'subAccountString',
                message: 'Input subAccountString:',
                validate: (input: string) => (input ? true : 'subAccountString cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.createAVirtualSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'enable-futures-for-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Enable Futures for Sub-account for Master Account

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'enable-futures-for-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['email']) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.enableFuturesForSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'enable-options-for-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Enable Options for Sub-account (For Master Account).

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'enable-options-for-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['email']) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.enableOptionsForSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-futures-position-risk-of-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Futures Position-Risk of Sub-account

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'get-futures-position-risk-of-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFuturesPositionRiskOfSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-futures-position-risk-of-sub-account-v2',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Futures Position-Risk of Sub-account V2

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'futures-type': {
                    describe: decodeSelectedEntities(
                        '1:USDT-margined Futures，2: Coin-margined Futures'
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['futuresType'] && !options?.interactive) {
                    requiredParams.push('futuresType');
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
                'get-futures-position-risk-of-sub-account-v2 is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.futuresType) {
            questions.push({
                type: 'input',
                name: 'futuresType',
                message: 'Input futuresType:',
                validate: (input: string) => (input ? true : 'futuresType cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFuturesPositionRiskOfSubAccountV2(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-sub-accounts-status-on-margin-or-futures',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Sub-account&#39;s Status on Margin Or Futures

* If no email sent, all sub-accounts&#39; information will be returned.

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            email: {
                describe: decodeSelectedEntities('Managed sub-account email'),
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
                'get-sub-accounts-status-on-margin-or-futures is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSubAccountsStatusOnMarginOrFutures(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-sub-account-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Sub-account List

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            email: {
                describe: decodeSelectedEntities('Managed sub-account email'),
                type: 'string',
                group: 'Command Options:',
            },
            'is-freeze': {
                describe: decodeSelectedEntities('true or false'),
                type: 'string',
                group: 'Command Options:',
            },
            page: {
                describe: decodeSelectedEntities('Default value: 1'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
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
                'query-sub-account-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySubAccountList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-sub-account-transaction-statistics',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Sub-account Transaction statistics (For Master Account).

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            email: {
                describe: decodeSelectedEntities('Managed sub-account email'),
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
                'query-sub-account-transaction-statistics is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySubAccountTransactionStatistics(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'add-ip-restriction-for-sub-account-api-key',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Add IP Restriction for Sub-Account API key

* You need to enable Enable Spot &amp; Margin Trading option for the api key which requests this endpoint

Weight: 3000`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'sub-account-api-key': {
                    type: 'string',
                    group: 'Command Options:',
                },
                status: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'ip-address': {
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['subAccountApiKey'] && !options?.interactive) {
                    requiredParams.push('subAccountApiKey');
                }

                if (!options?.['status'] && !options?.interactive) {
                    requiredParams.push('status');
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
                'add-ip-restriction-for-sub-account-api-key is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['email']) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }

        if (options.interactive && !options?.['subAccountApiKey']) {
            questions.push({
                type: 'input',
                name: 'subAccountApiKey',
                message: 'Input subAccountApiKey:',
                validate: (input: string) => (input ? true : 'subAccountApiKey cannot be empty'),
            });
        }

        if (options.interactive && !options?.['status']) {
            questions.push({
                type: 'input',
                name: 'status',
                message: 'Input status:',
                validate: (input: string) => (input ? true : 'status cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.addIpRestrictionForSubAccountApiKey(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'delete-ip-list-for-a-sub-account-api-key',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Delete IP List For a Sub-account API Key

* You need to enable Enable Spot &amp; Margin Trading option for the api key which requests this endpoint

Weight: 3000`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'sub-account-api-key': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'ip-address': {
                    describe: decodeSelectedEntities(
                        'IPs to be deleted. Can be added in batches, separated by commas'
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['subAccountApiKey'] && !options?.interactive) {
                    requiredParams.push('subAccountApiKey');
                }

                if (!options?.['ipAddress'] && !options?.interactive) {
                    requiredParams.push('ipAddress');
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
                'delete-ip-list-for-a-sub-account-api-key is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.subAccountApiKey) {
            questions.push({
                type: 'input',
                name: 'subAccountApiKey',
                message: 'Input subAccountApiKey:',
                validate: (input: string) => (input ? true : 'subAccountApiKey cannot be empty'),
            });
        }
        if (options.interactive && !options.ipAddress) {
            questions.push({
                type: 'input',
                name: 'ipAddress',
                message: 'Input ipAddress:',
                validate: (input: string) => (input ? true : 'ipAddress cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.deleteIpListForASubAccountApiKey(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-ip-restriction-for-a-sub-account-api-key',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get IP Restriction for a Sub-account API Key

Weight: 3000`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'sub-account-api-key': {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['subAccountApiKey'] && !options?.interactive) {
                    requiredParams.push('subAccountApiKey');
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
                'get-ip-restriction-for-a-sub-account-api-key is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.subAccountApiKey) {
            questions.push({
                type: 'input',
                name: 'subAccountApiKey',
                message: 'Input subAccountApiKey:',
                validate: (input: string) => (input ? true : 'subAccountApiKey cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getIpRestrictionForASubAccountApiKey(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'futures-transfer-for-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Futures Transfer for Sub-account

* You need to open Enable Spot &amp; Margin Trading permission for the API Key which requests this endpoint.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                type: {
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
                }

                if (!options?.['type'] && !options?.interactive) {
                    requiredParams.push('type');
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
                'futures-transfer-for-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['email']) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
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

        if (options.interactive && !options?.['type']) {
            questions.push({
                type: 'input',
                name: 'type',
                message: 'Input type:',
                validate: (input: string) => (input ? true : 'type cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.futuresTransferForSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-detail-on-sub-accounts-futures-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Detail on Sub-account&#39;s Futures Account

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'get-detail-on-sub-accounts-futures-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getDetailOnSubAccountsFuturesAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-detail-on-sub-accounts-futures-account-v2',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Detail on Sub-account&#39;s Futures Account

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'futures-type': {
                    describe: decodeSelectedEntities(
                        '1:USDT-margined Futures，2: Coin-margined Futures'
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['futuresType'] && !options?.interactive) {
                    requiredParams.push('futuresType');
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
                'get-detail-on-sub-accounts-futures-account-v2 is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.futuresType) {
            questions.push({
                type: 'input',
                name: 'futuresType',
                message: 'Input futuresType:',
                validate: (input: string) => (input ? true : 'futuresType cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getDetailOnSubAccountsFuturesAccountV2(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-detail-on-sub-accounts-margin-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Detail on Sub-account&#39;s Margin Account

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'get-detail-on-sub-accounts-margin-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getDetailOnSubAccountsMarginAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-move-position-history-for-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query move position history

* If &#x60;startTime&#x60; and &#x60;endTime&#x60; not sent, return records of the last 90 days by default with 1000 maximum limits
* If &#x60;startTime&#x60; is sent and &#x60;endTime&#x60; is not sent, return records of [max(startTime, now-90d), now].
* If &#x60;startTime&#x60; is not sent and &#x60;endTime&#x60; is sent, return records of [max(now,endTime-90d), endTime].

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                symbol: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                page: {
                    describe: decodeSelectedEntities('Page'),
                    type: 'string',
                    group: 'Command Options:',
                },
                rows: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['symbol'] && !options?.interactive) {
                    requiredParams.push('symbol');
                }

                if (!options?.['page'] && !options?.interactive) {
                    requiredParams.push('page');
                }

                if (!options?.['rows'] && !options?.interactive) {
                    requiredParams.push('rows');
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
                'get-move-position-history-for-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.symbol) {
            questions.push({
                type: 'input',
                name: 'symbol',
                message: 'Input symbol:',
                validate: (input: string) => (input ? true : 'symbol cannot be empty'),
            });
        }
        if (options.interactive && !options.page) {
            questions.push({
                type: 'input',
                name: 'page',
                message: 'Input page:',
                validate: (input: string) => (input ? true : 'page cannot be empty'),
            });
        }
        if (options.interactive && !options.rows) {
            questions.push({
                type: 'input',
                name: 'rows',
                message: 'Input rows:',
                validate: (input: string) => (input ? true : 'rows cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getMovePositionHistoryForSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-sub-account-deposit-address',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch sub-account deposit address

* &#x60;amount&#x60; needs to be sent if using LIGHTNING network

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                coin: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                network: {
                    describe: decodeSelectedEntities(
                        'networks can be found in &#x60;GET /sapi/v1/capital/deposit/address&#x60;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['coin'] && !options?.interactive) {
                    requiredParams.push('coin');
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
                'get-sub-account-deposit-address is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.coin) {
            questions.push({
                type: 'input',
                name: 'coin',
                message: 'Input coin:',
                validate: (input: string) => (input ? true : 'coin cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSubAccountDepositAddress(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-sub-account-deposit-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch sub-account deposit history

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                coin: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                status: {
                    describe: decodeSelectedEntities(
                        '0(0:pending,6: credited but cannot withdraw,7:Wrong Deposit,8:Waiting User confirm,1:success)'
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
                    describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
                    type: 'string',
                    group: 'Command Options:',
                },
                offset: {
                    describe: decodeSelectedEntities('default:0'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'tx-id': {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'get-sub-account-deposit-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSubAccountDepositHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-summary-of-sub-accounts-futures-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Summary of Sub-account&#39;s Futures Account

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                page: {
                    describe: decodeSelectedEntities('Page'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Limit (Max: 500)'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['page'] && !options?.interactive) {
                    requiredParams.push('page');
                }

                if (!options?.['limit'] && !options?.interactive) {
                    requiredParams.push('limit');
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
                'get-summary-of-sub-accounts-futures-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.page) {
            questions.push({
                type: 'input',
                name: 'page',
                message: 'Input page:',
                validate: (input: string) => (input ? true : 'page cannot be empty'),
            });
        }
        if (options.interactive && !options.limit) {
            questions.push({
                type: 'input',
                name: 'limit',
                message: 'Input limit:',
                validate: (input: string) => (input ? true : 'limit cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSummaryOfSubAccountsFuturesAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-summary-of-sub-accounts-futures-account-v2',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Summary of Sub-account&#39;s Futures Account

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'futures-type': {
                    describe: decodeSelectedEntities(
                        '1:USDT-margined Futures，2: Coin-margined Futures'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                page: {
                    describe: decodeSelectedEntities('Default value: 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['futuresType'] && !options?.interactive) {
                    requiredParams.push('futuresType');
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
                'get-summary-of-sub-accounts-futures-account-v2 is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.futuresType) {
            questions.push({
                type: 'input',
                name: 'futuresType',
                message: 'Input futuresType:',
                validate: (input: string) => (input ? true : 'futuresType cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSummaryOfSubAccountsFuturesAccountV2(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-summary-of-sub-accounts-margin-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Summary of Sub-account&#39;s Margin Account

Weight: 10`,
            isFullDescription
        ),
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
                'get-summary-of-sub-accounts-margin-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSummaryOfSubAccountsMarginAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'margin-transfer-for-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Margin Transfer for Sub-account

* You need to open Enable Spot &amp; Margin Trading permission for the API Key which requests this endpoint.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                type: {
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
                }

                if (!options?.['type'] && !options?.interactive) {
                    requiredParams.push('type');
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
                'margin-transfer-for-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['email']) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
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

        if (options.interactive && !options?.['type']) {
            questions.push({
                type: 'input',
                name: 'type',
                message: 'Input type:',
                validate: (input: string) => (input ? true : 'type cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.marginTransferForSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'move-position-for-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Move position between sub-master, master-sub, or sub-sub accounts when necessary

* You need to Enable Trading permission for the API Key which requests this endpoint.
* This function only support VIP level 7-9.
* Only master account can use the function
* Quantity should be positive number only
* The function support normal account, PM PRO, PM PRO SPAN and PM Retail.
* Only support for from account has positions
* For all orders in the same orderArgs request, if any symbol’s total close position quantity is bigger than the symbol’s current position quantity, all batch orders in the same list will fail simultaneously.
* Only support cross margin mode
* The price for move position is MarkPrice only.
* Not support for MSA.
* Not support for the symbol under Reduce-Only.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'from-user-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-user-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'product-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'order-args': {
                    type: 'array',
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

                if (!options?.['fromUserEmail'] && !options?.interactive) {
                    requiredParams.push('fromUserEmail');
                }

                if (!options?.['toUserEmail'] && !options?.interactive) {
                    requiredParams.push('toUserEmail');
                }

                if (!options?.['productType'] && !options?.interactive) {
                    requiredParams.push('productType');
                }

                if (!options?.['orderArgs'] && !options?.interactive) {
                    requiredParams.push('orderArgs');
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
                'move-position-for-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['fromUserEmail']) {
            questions.push({
                type: 'input',
                name: 'fromUserEmail',
                message: 'Input fromUserEmail:',
                validate: (input: string) => (input ? true : 'fromUserEmail cannot be empty'),
            });
        }

        if (options.interactive && !options?.['toUserEmail']) {
            questions.push({
                type: 'input',
                name: 'toUserEmail',
                message: 'Input toUserEmail:',
                validate: (input: string) => (input ? true : 'toUserEmail cannot be empty'),
            });
        }

        if (options.interactive && !options?.['productType']) {
            questions.push({
                type: 'input',
                name: 'productType',
                message: 'Input productType:',
                validate: (input: string) => (input ? true : 'productType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['orderArgs']) {
            questions.push({
                type: 'input',
                name: 'orderArgs',
                message: 'Input orderArgs:',
                validate: (input: string) => (input ? true : 'orderArgs cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.movePositionForSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-sub-account-assets',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch sub-account assets

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'query-sub-account-assets is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySubAccountAssets(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-sub-account-assets-asset-management',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch sub-account assets

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'query-sub-account-assets-asset-management is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySubAccountAssetsAssetManagement(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-sub-account-futures-asset-transfer-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Sub-account Futures Asset Transfer History

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'futures-type': {
                    describe: decodeSelectedEntities(
                        '1:USDT-margined Futures，2: Coin-margined Futures'
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
                page: {
                    describe: decodeSelectedEntities('Default value: 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['futuresType'] && !options?.interactive) {
                    requiredParams.push('futuresType');
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
                'query-sub-account-futures-asset-transfer-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.futuresType) {
            questions.push({
                type: 'input',
                name: 'futuresType',
                message: 'Input futuresType:',
                validate: (input: string) => (input ? true : 'futuresType cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.querySubAccountFuturesAssetTransferHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-sub-account-spot-asset-transfer-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Sub-account Spot Asset Transfer History

* fromEmail and toEmail cannot be sent at the same time.
* Return fromEmail equal master account email by default.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'from-email': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'to-email': {
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
            page: {
                describe: decodeSelectedEntities('Default value: 1'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
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
                'query-sub-account-spot-asset-transfer-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySubAccountSpotAssetTransferHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-sub-account-spot-assets-summary',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get BTC valued asset summary of subaccounts.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            email: {
                describe: decodeSelectedEntities('Managed sub-account email'),
                type: 'string',
                group: 'Command Options:',
            },
            page: {
                describe: decodeSelectedEntities('Default value: 1'),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities('default 10, max 20'),
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
                'query-sub-account-spot-assets-summary is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.querySubAccountSpotAssetsSummary(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-universal-transfer-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Universal Transfer History

* fromEmail and toEmail cannot be sent at the same time.
* Return fromEmail equal master account email by default.
* The query time period must be less than 7 days.
* If startTime and endTime not sent, return records of the last 7 days by default.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'from-email': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'to-email': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'client-tran-id': {
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
            page: {
                describe: decodeSelectedEntities('Default value: 1'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
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
                'query-universal-transfer-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUniversalTransferHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'sub-account-futures-asset-transfer',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Sub-account Futures Asset Transfer


* Master account can transfer max 2000 times a minute
* There must be sufficient margin balance in futures wallet to execute transferring.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'from-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'futures-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['fromEmail'] && !options?.interactive) {
                    requiredParams.push('fromEmail');
                }

                if (!options?.['toEmail'] && !options?.interactive) {
                    requiredParams.push('toEmail');
                }

                if (!options?.['futuresType'] && !options?.interactive) {
                    requiredParams.push('futuresType');
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
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
                'sub-account-futures-asset-transfer is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['fromEmail']) {
            questions.push({
                type: 'input',
                name: 'fromEmail',
                message: 'Input fromEmail:',
                validate: (input: string) => (input ? true : 'fromEmail cannot be empty'),
            });
        }

        if (options.interactive && !options?.['toEmail']) {
            questions.push({
                type: 'input',
                name: 'toEmail',
                message: 'Input toEmail:',
                validate: (input: string) => (input ? true : 'toEmail cannot be empty'),
            });
        }

        if (options.interactive && !options?.['futuresType']) {
            questions.push({
                type: 'input',
                name: 'futuresType',
                message: 'Input futuresType:',
                validate: (input: string) => (input ? true : 'futuresType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
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
            const response = await client.restAPI.subAccountFuturesAssetTransfer(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'sub-account-transfer-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Sub-account Transfer History

* If type is not sent, the records of type 2: transfer out will be returned by default.
* If startTime and endTime are not sent, the recent 30-day data will be returned.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities(
                    'If not sent, result of all assets will be returned'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            type: {
                describe: decodeSelectedEntities('1: transfer in, 2: transfer out'),
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
                describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
                type: 'string',
                group: 'Command Options:',
            },
            'return-fail-history': {
                describe: decodeSelectedEntities(
                    'Default &#x60;False&#x60;, return PROCESS and SUCCESS status history; If &#x60;True&#x60;,return PROCESS and SUCCESS and FAILURE status history'
                ),
                type: 'boolean',
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
                'sub-account-transfer-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.subAccountTransferHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'transfer-to-master',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Transfer to Master

* You need to open Enable Spot &amp; Margin Trading permission for the API Key which requests this endpoint.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
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
                'transfer-to-master is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
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
            const response = await client.restAPI.transferToMaster(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'transfer-to-sub-account-of-same-master',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Transfer to Sub-account of Same Master

* You need to open Enable Spot &amp; Margin Trading permission for the API Key which requests this endpoint.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'to-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['toEmail'] && !options?.interactive) {
                    requiredParams.push('toEmail');
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
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
                'transfer-to-sub-account-of-same-master is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['toEmail']) {
            questions.push({
                type: 'input',
                name: 'toEmail',
                message: 'Input toEmail:',
                validate: (input: string) => (input ? true : 'toEmail cannot be empty'),
            });
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
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
            const response = await client.restAPI.transferToSubAccountOfSameMaster(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'universal-transfer',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Universal Transfer

* You need to enable &quot;internal transfer&quot; option for the api key which requests this endpoint.
* Transfer from master account by default if fromEmail is not sent.
* Transfer to master account by default if toEmail is not sent.
* At least either fromEmail or toEmail need to be sent when the fromAccountType and the toAccountType are the same.
* Supported transfer scenarios:
* &#x60;SPOT&#x60; transfer to &#x60;SPOT&#x60;, &#x60;USDT_FUTURE&#x60;, &#x60;COIN_FUTURE&#x60; (regardless of master or sub)
* &#x60;SPOT&#x60;, &#x60;USDT_FUTURE&#x60;, &#x60;COIN_FUTURE&#x60; transfer to &#x60;SPOT&#x60;  (regardless of master or sub)
* Master account &#x60;SPOT&#x60; transfer to sub-account &#x60;MARGIN(Cross)&#x60;, &#x60;ISOLATED_MARGIN&#x60;
* Sub-account &#x60;MARGIN(Cross)&#x60;, &#x60;ISOLATED_MARGIN&#x60; transfer to master account &#x60;SPOT&#x60;
* Sub-account &#x60;MARGIN(Cross)&#x60; transfer to Sub-account &#x60;MARGIN(Cross)&#x60;
* &#x60;ALPHA&#x60; to &#x60;ALPHA&#x60;  (regardless of master or sub)

Weight: 360`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'from-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-account-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-account-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-tran-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                symbol: {
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['fromAccountType'] && !options?.interactive) {
                    requiredParams.push('fromAccountType');
                }

                if (!options?.['toAccountType'] && !options?.interactive) {
                    requiredParams.push('toAccountType');
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
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
                'universal-transfer is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['fromAccountType']) {
            questions.push({
                type: 'input',
                name: 'fromAccountType',
                message: 'Input fromAccountType:',
                validate: (input: string) => (input ? true : 'fromAccountType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['toAccountType']) {
            questions.push({
                type: 'input',
                name: 'toAccountType',
                message: 'Input toAccountType:',
                validate: (input: string) => (input ? true : 'toAccountType cannot be empty'),
            });
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
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
            const response = await client.restAPI.universalTransfer(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'deposit-assets-into-the-managed-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Deposit Assets Into The Managed Sub-account

* You need to enable &#x60;Enable Spot &amp; Margin Trading&#x60; option for the api key which requests this endpoint

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'to-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['toEmail'] && !options?.interactive) {
                    requiredParams.push('toEmail');
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
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
                'deposit-assets-into-the-managed-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['toEmail']) {
            questions.push({
                type: 'input',
                name: 'toEmail',
                message: 'Input toEmail:',
                validate: (input: string) => (input ? true : 'toEmail cannot be empty'),
            });
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
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
            const response = await client.restAPI.depositAssetsIntoTheManagedSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'get-managed-sub-account-deposit-address',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get investor&#39;s managed sub-account deposit address.

* If &#x60;network&#x60; is not send, return with default &#x60;network&#x60; of the &#x60;coin&#x60;.
* * &#x60;amount&#x60; needs to be sent if using LIGHTNING network

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                coin: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                network: {
                    describe: decodeSelectedEntities(
                        'networks can be found in &#x60;GET /sapi/v1/capital/deposit/address&#x60;'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['coin'] && !options?.interactive) {
                    requiredParams.push('coin');
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
                'get-managed-sub-account-deposit-address is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.coin) {
            questions.push({
                type: 'input',
                name: 'coin',
                message: 'Input coin:',
                validate: (input: string) => (input ? true : 'coin cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getManagedSubAccountDepositAddress(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-managed-sub-account-asset-details',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Managed Sub-account Asset Details

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'query-managed-sub-account-asset-details is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryManagedSubAccountAssetDetails(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-managed-sub-account-futures-asset-details',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Investor can use this api to query managed sub account futures asset details

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'account-type': {
                    describe: decodeSelectedEntities(
                        'No input or input \&quot;MARGIN\&quot; to get Cross Margin account details. Input \&quot;ISOLATED_MARGIN\&quot; to get Isolated Margin account details.'
                    ),
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'query-managed-sub-account-futures-asset-details is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.queryManagedSubAccountFuturesAssetDetails(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-managed-sub-account-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get investor&#39;s managed sub-account list.

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            email: {
                describe: decodeSelectedEntities('Managed sub-account email'),
                type: 'string',
                group: 'Command Options:',
            },
            page: {
                describe: decodeSelectedEntities('Default value: 1'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
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
                'query-managed-sub-account-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryManagedSubAccountList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-managed-sub-account-margin-asset-details',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Investor can use this api to query managed sub account margin asset details

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'account-type': {
                    describe: decodeSelectedEntities(
                        'No input or input \&quot;MARGIN\&quot; to get Cross Margin account details. Input \&quot;ISOLATED_MARGIN\&quot; to get Isolated Margin account details.'
                    ),
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
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
                'query-managed-sub-account-margin-asset-details is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryManagedSubAccountMarginAssetDetails(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-managed-sub-account-snapshot',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Managed Sub-account Snapshot

* The query time period must be less then 30 days
* Support query within the last one month only
* If startTimeand endTime not sent, return records of the last 7 days by default

Weight: 2400`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                type: {
                    describe: decodeSelectedEntities(
                        '\&quot;SPOT\&quot;, \&quot;MARGIN\&quot;（cross）, \&quot;FUTURES\&quot;（UM）'
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
                    describe: decodeSelectedEntities('Default value: 1, Max value: 200'),
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['type'] && !options?.interactive) {
                    requiredParams.push('type');
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
                'query-managed-sub-account-snapshot is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.type) {
            questions.push({
                type: 'input',
                name: 'type',
                message: 'Input type:',
                validate: (input: string) => (input ? true : 'type cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryManagedSubAccountSnapshot(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-managed-sub-account-transfer-log-master-account-investor',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Investor can use this api to query managed sub account transfer log. This endpoint is available for investor of Managed Sub-Account. A Managed Sub-Account is an account type for investors who value flexibility in asset allocation and account application, while delegating trades to a professional trading team.
Please refer to [link](https://www.binance.com/en/support/faq/how-to-get-started-with-managed-sub-account-functions-and-frequently-asked-questions-0594748722704383a7c369046e489459)

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities('Start Time'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'End Time (The start time and end time interval cannot exceed half a year)'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                page: {
                    describe: decodeSelectedEntities('Page'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Limit (Max: 500)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                transfers: {
                    describe: decodeSelectedEntities('Transfer Direction (FROM/TO)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'transfer-function-account-type': {
                    describe: decodeSelectedEntities(
                        'Transfer function account type (SPOT/MARGIN/ISOLATED_MARGIN/USDT_FUTURE/COIN_FUTURE)'
                    ),
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['startTime'] && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.['endTime'] && !options?.interactive) {
                    requiredParams.push('endTime');
                }

                if (!options?.['page'] && !options?.interactive) {
                    requiredParams.push('page');
                }

                if (!options?.['limit'] && !options?.interactive) {
                    requiredParams.push('limit');
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
                'query-managed-sub-account-transfer-log-master-account-investor is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.startTime) {
            questions.push({
                type: 'input',
                name: 'startTime',
                message: 'Input startTime:',
                validate: (input: string) => (input ? true : 'startTime cannot be empty'),
            });
        }
        if (options.interactive && !options.endTime) {
            questions.push({
                type: 'input',
                name: 'endTime',
                message: 'Input endTime:',
                validate: (input: string) => (input ? true : 'endTime cannot be empty'),
            });
        }
        if (options.interactive && !options.page) {
            questions.push({
                type: 'input',
                name: 'page',
                message: 'Input page:',
                validate: (input: string) => (input ? true : 'page cannot be empty'),
            });
        }
        if (options.interactive && !options.limit) {
            questions.push({
                type: 'input',
                name: 'limit',
                message: 'Input limit:',
                validate: (input: string) => (input ? true : 'limit cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.queryManagedSubAccountTransferLogMasterAccountInvestor(
                    options
                );
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-managed-sub-account-transfer-log-master-account-trading',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Trading team can use this api to query managed sub account transfer log. This endpoint is available for trading team of Managed Sub-Account. A Managed Sub-Account is an account type for investors who value flexibility in asset allocation and account application, while delegating trades to a professional trading team.
Please refer to [link](https://www.binance.com/en/support/faq/how-to-get-started-with-managed-sub-account-functions-and-frequently-asked-questions-0594748722704383a7c369046e489459)

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
                    describe: decodeSelectedEntities('[Sub-account email](#email-address)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'start-time': {
                    describe: decodeSelectedEntities('Start Time'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'End Time (The start time and end time interval cannot exceed half a year)'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                page: {
                    describe: decodeSelectedEntities('Page'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Limit (Max: 500)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                transfers: {
                    describe: decodeSelectedEntities('Transfer Direction (FROM/TO)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'transfer-function-account-type': {
                    describe: decodeSelectedEntities(
                        'Transfer function account type (SPOT/MARGIN/ISOLATED_MARGIN/USDT_FUTURE/COIN_FUTURE)'
                    ),
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

                if (!options?.['email'] && !options?.interactive) {
                    requiredParams.push('email');
                }

                if (!options?.['startTime'] && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.['endTime'] && !options?.interactive) {
                    requiredParams.push('endTime');
                }

                if (!options?.['page'] && !options?.interactive) {
                    requiredParams.push('page');
                }

                if (!options?.['limit'] && !options?.interactive) {
                    requiredParams.push('limit');
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
                'query-managed-sub-account-transfer-log-master-account-trading is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.email) {
            questions.push({
                type: 'input',
                name: 'email',
                message: 'Input email:',
                validate: (input: string) => (input ? true : 'email cannot be empty'),
            });
        }
        if (options.interactive && !options.startTime) {
            questions.push({
                type: 'input',
                name: 'startTime',
                message: 'Input startTime:',
                validate: (input: string) => (input ? true : 'startTime cannot be empty'),
            });
        }
        if (options.interactive && !options.endTime) {
            questions.push({
                type: 'input',
                name: 'endTime',
                message: 'Input endTime:',
                validate: (input: string) => (input ? true : 'endTime cannot be empty'),
            });
        }
        if (options.interactive && !options.page) {
            questions.push({
                type: 'input',
                name: 'page',
                message: 'Input page:',
                validate: (input: string) => (input ? true : 'page cannot be empty'),
            });
        }
        if (options.interactive && !options.limit) {
            questions.push({
                type: 'input',
                name: 'limit',
                message: 'Input limit:',
                validate: (input: string) => (input ? true : 'limit cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.queryManagedSubAccountTransferLogMasterAccountTrading(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'query-managed-sub-account-transfer-log-sub-account-trading',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Managed Sub Account Transfer Log (For Trading Team Sub Account)

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'start-time': {
                    describe: decodeSelectedEntities('Start Time'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'end-time': {
                    describe: decodeSelectedEntities(
                        'End Time (The start time and end time interval cannot exceed half a year)'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                page: {
                    describe: decodeSelectedEntities('Page'),
                    type: 'string',
                    group: 'Command Options:',
                },
                limit: {
                    describe: decodeSelectedEntities('Limit (Max: 500)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                transfers: {
                    describe: decodeSelectedEntities('Transfer Direction (FROM/TO)'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'transfer-function-account-type': {
                    describe: decodeSelectedEntities(
                        'Transfer function account type (SPOT/MARGIN/ISOLATED_MARGIN/USDT_FUTURE/COIN_FUTURE)'
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
                const stdinObj: any = readStdinObj();

                if (!isEmpty(stdinObj)) {
                    options = { ...options, ...stdinObj };
                }

                if (options.json) {
                    options = { ...options, ...JSON.parse(options.json) };
                }

                if (!options?.['startTime'] && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.['endTime'] && !options?.interactive) {
                    requiredParams.push('endTime');
                }

                if (!options?.['page'] && !options?.interactive) {
                    requiredParams.push('page');
                }

                if (!options?.['limit'] && !options?.interactive) {
                    requiredParams.push('limit');
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
                'query-managed-sub-account-transfer-log-sub-account-trading is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.startTime) {
            questions.push({
                type: 'input',
                name: 'startTime',
                message: 'Input startTime:',
                validate: (input: string) => (input ? true : 'startTime cannot be empty'),
            });
        }
        if (options.interactive && !options.endTime) {
            questions.push({
                type: 'input',
                name: 'endTime',
                message: 'Input endTime:',
                validate: (input: string) => (input ? true : 'endTime cannot be empty'),
            });
        }
        if (options.interactive && !options.page) {
            questions.push({
                type: 'input',
                name: 'page',
                message: 'Input page:',
                validate: (input: string) => (input ? true : 'page cannot be empty'),
            });
        }
        if (options.interactive && !options.limit) {
            questions.push({
                type: 'input',
                name: 'limit',
                message: 'Input limit:',
                validate: (input: string) => (input ? true : 'limit cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.queryManagedSubAccountTransferLogSubAccountTrading(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

subAccountCommands.push({
    command: 'withdrawl-assets-from-the-managed-sub-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Withdrawl Assets From The Managed Sub-account

* You need to enable &#x60;Enable Spot &amp; Margin Trading&#x60; option for the api key which requests this endpoint

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'from-email': {
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'transfer-date': {
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

                if (!options?.['fromEmail'] && !options?.interactive) {
                    requiredParams.push('fromEmail');
                }

                if (!options?.['asset'] && !options?.interactive) {
                    requiredParams.push('asset');
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
                'withdrawl-assets-from-the-managed-sub-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['fromEmail']) {
            questions.push({
                type: 'input',
                name: 'fromEmail',
                message: 'Input fromEmail:',
                validate: (input: string) => (input ? true : 'fromEmail cannot be empty'),
            });
        }

        if (options.interactive && !options?.['asset']) {
            questions.push({
                type: 'input',
                name: 'asset',
                message: 'Input asset:',
                validate: (input: string) => (input ? true : 'asset cannot be empty'),
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
            const response = await client.restAPI.withdrawlAssetsFromTheManagedSubAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'sub-account',
    description: 'Binance Sub Account REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli sub-account <command> [options]');
        subAccountCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
