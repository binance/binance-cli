import { SimpleEarn, SIMPLE_EARN_REST_API_PROD_URL } from '@binance/simple-earn';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('simple-earn');

    let basePath = SIMPLE_EARN_REST_API_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'simple-earn');

    if (process.env.BINANCE_SIMPLE_EARN_BASE_PATH) {
        basePath = process.env.BINANCE_SIMPLE_EARN_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['basePath']) {
        basePath = configurationRestAPI['basePath'];
    }

    let client;
    let hasConfig = false;
    if (configurationRestAPI !== null) {
        hasConfig = true;
        client = new SimpleEarn({
            configurationRestAPI: { ...configurationRestAPI, basePath },
        });
    } else {
        client = new SimpleEarn({
            configurationRestAPI: {
                apiKey: '',
                basePath,
            },
        });
    }

    return { client, hasConfig };
};

const simpleEarnCommands: any[] = [];

simpleEarnCommands.push({
    command: 'get-bfusd-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get BFUSD account information.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-bfusd-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBfusdAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-bfusd-quota-details',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get BFUSD quota details including subscription quota, fast redemption quota and standard redemption quota.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-bfusd-quota-details is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBfusdQuotaDetails(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-bfusd-rate-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get BFUSD rate history sorted by descending order.

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 6 months.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, &#x60;endTime&#x60; will default to current time, and results from &#x60;startTime&#x60; onward will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, &#x60;startTime&#x60; defaults to the current time minus one month, and data between &#x60;startTime&#x60; and &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
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
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-bfusd-rate-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBfusdRateHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-bfusd-redemption-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get BFUSD redemption history.

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 6 months.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, &#x60;endTime&#x60; will default to current time, and results from &#x60;startTime&#x60; onward will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, &#x60;startTime&#x60; defaults to the current time minus one month, and data between &#x60;startTime&#x60; and &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
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
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-bfusd-redemption-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBfusdRedemptionHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-bfusd-rewards-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get BFUSD rewards history.

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 6 months.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, &#x60;endTime&#x60; will default to current time, and results from &#x60;startTime&#x60; onward will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, &#x60;startTime&#x60; defaults to the current time minus one month, and data between &#x60;startTime&#x60; and &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
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
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-bfusd-rewards-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBfusdRewardsHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-bfusd-subscription-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get BFUSD subscription history

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 6 months.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, &#x60;endTime&#x60; will default to current time, and results from &#x60;startTime&#x60; onward will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, &#x60;startTime&#x60; defaults to the current time advanced by one month, and data between &#x60;startTime&#x60; and &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
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
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-bfusd-subscription-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getBfusdSubscriptionHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'redeem-bfusd',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Redeem BFUSD to USDT

* You need to open Enable Spot &amp; Margin Trading permission for the API Key which requests this endpoint.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
                'redeem-bfusd is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
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
            const response = await client.restAPI.redeemBfusd(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'subscribe-bfusd',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Subscribe BFUSD

* You need to open Enable Spot &amp; Margin Trading permission for the API Key which requests this endpoint.

Weight: 150`,
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
                'subscribe-bfusd is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.subscribeBfusd(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-collateral-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Collateral Record

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 30 days.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, the next 30 days&#39; data beginning from &#x60;startTime&#x60; will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, the 30 days&#39; data before &#x60;endTime&#x60; will be returned.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'product-id': {
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
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-collateral-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCollateralRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-flexible-personal-left-quota',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Flexible Personal Left Quota

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'product-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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

                if (!options?.['productId'] && !options?.interactive) {
                    requiredParams.push('productId');
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
                'get-flexible-personal-left-quota is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.productId) {
            questions.push({
                type: 'input',
                name: 'productId',
                message: 'Input productId:',
                validate: (input: string) => (input ? true : 'productId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexiblePersonalLeftQuota(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-flexible-product-position',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Flexible Product Position

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
                type: 'string',
                group: 'Command Options:',
            },
            'product-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-flexible-product-position is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleProductPosition(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-flexible-redemption-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Flexible Redemption Record

*	The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 30 days.
*	If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
*	If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, the next 30 days&#39; data beginning from &#x60;startTime&#x60; will be returned.
*	If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, the 30 days&#39; data before &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'product-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'redeem-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
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
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-flexible-redemption-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleRedemptionRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-flexible-rewards-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Flexible Rewards History

*	The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 30 days.
*	If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
*	If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, the next 30 days&#39; data beginning from &#x60;startTime&#x60; will be returned.
*	If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, the 30 days&#39; data before &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                type: {
                    describe: decodeSelectedEntities(
                        '&#x60;BONUS&#x60; - Bonus tiered APR, &#x60;REALTIME&#x60; Real-time APR, &#x60;REWARDS&#x60; Historical rewards,&#x60;ALL&#x60;(set to default)'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'product-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
                    describe: decodeSelectedEntities('USDC or USDT'),
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
                        'Currently querying page. Starts from 1. Default: 1'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                size: {
                    describe: decodeSelectedEntities(
                        'Number of results per page. Default: 10, Max: 100'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-flexible-rewards-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
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
            const response = await client.restAPI.getFlexibleRewardsHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-flexible-subscription-preview',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Flexible Subscription Preview

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'product-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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

                if (!options?.['productId'] && !options?.interactive) {
                    requiredParams.push('productId');
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
                'get-flexible-subscription-preview is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.productId) {
            questions.push({
                type: 'input',
                name: 'productId',
                message: 'Input productId:',
                validate: (input: string) => (input ? true : 'productId cannot be empty'),
            });
        }
        if (options.interactive && !options.amount) {
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
            const response = await client.restAPI.getFlexibleSubscriptionPreview(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-flexible-subscription-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Flexible Subscription Record

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 30 days.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, the next 30 days&#39; data beginning from &#x60;startTime&#x60; will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, the 30 days&#39; data before &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'product-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'purchase-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
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
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-flexible-subscription-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getFlexibleSubscriptionRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-locked-personal-left-quota',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Locked Personal Left Quota

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'project-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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

                if (!options?.['projectId'] && !options?.interactive) {
                    requiredParams.push('projectId');
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
                'get-locked-personal-left-quota is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.projectId) {
            questions.push({
                type: 'input',
                name: 'projectId',
                message: 'Input projectId:',
                validate: (input: string) => (input ? true : 'projectId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLockedPersonalLeftQuota(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-locked-product-position',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Locked Product Position

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
                type: 'string',
                group: 'Command Options:',
            },
            'position-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'project-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-locked-product-position is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLockedProductPosition(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-locked-redemption-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Locked Redemption Record

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 30 days.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, the next 30 days&#39; data beginning from &#x60;startTime&#x60; will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, the 30 days&#39; data before &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'position-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'redeem-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
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
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-locked-redemption-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLockedRedemptionRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-locked-rewards-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Locked Rewards History

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 30 days.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, the next 30 days&#39; data beginning from &#x60;startTime&#x60; will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, the 30 days&#39; data before &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'position-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
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
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-locked-rewards-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLockedRewardsHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-locked-subscription-preview',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Locked Subscription Preview

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'project-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-subscribe': {
                    describe: decodeSelectedEntities('true or false, default true.'),
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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

                if (!options?.['projectId'] && !options?.interactive) {
                    requiredParams.push('projectId');
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
                'get-locked-subscription-preview is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.projectId) {
            questions.push({
                type: 'input',
                name: 'projectId',
                message: 'Input projectId:',
                validate: (input: string) => (input ? true : 'projectId cannot be empty'),
            });
        }
        if (options.interactive && !options.amount) {
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
            const response = await client.restAPI.getLockedSubscriptionPreview(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-locked-subscription-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Locked Subscription Record

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 30 days.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, the next 30 days&#39; data beginning from &#x60;startTime&#x60; will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, the 30 days&#39; data before &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'purchase-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
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
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-locked-subscription-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getLockedSubscriptionRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-rate-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Rate History

* The time between startTime and endTime cannot be longer than 1 year.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, the next 30 days&#39; data beginning from &#x60;startTime&#x60; will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, the 30 days&#39; data before &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'product-id': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'apr-period': {
                    describe: decodeSelectedEntities(
                        '\&quot;DAY\&quot;,\&quot;YEAR\&quot;,default\&quot;DAY\&quot;'
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
                current: {
                    describe: decodeSelectedEntities(
                        'Currently querying page. Starts from 1. Default: 1'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                size: {
                    describe: decodeSelectedEntities(
                        'Number of results per page. Default: 10, Max: 100'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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

                if (!options?.['productId'] && !options?.interactive) {
                    requiredParams.push('productId');
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
                'get-rate-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.productId) {
            questions.push({
                type: 'input',
                name: 'productId',
                message: 'Input productId:',
                validate: (input: string) => (input ? true : 'productId cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getRateHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-simple-earn-flexible-product-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get available Simple Earn flexible product list

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
                type: 'string',
                group: 'Command Options:',
            },
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-simple-earn-flexible-product-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSimpleEarnFlexibleProductList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-simple-earn-locked-product-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Simple Earn Locked Product List

* Get available Simple Earn locked product list

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
                type: 'string',
                group: 'Command Options:',
            },
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-simple-earn-locked-product-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSimpleEarnLockedProductList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'redeem-flexible-product',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Redeem Flexible Product

* You need to open &#x60;Enable Spot &amp; Margin Trading&#x60; permission for the API Key which requests this endpoint.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'product-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'redeem-all': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'dest-account': {
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

                if (!options?.['productId'] && !options?.interactive) {
                    requiredParams.push('productId');
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
                'redeem-flexible-product is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['productId']) {
            questions.push({
                type: 'input',
                name: 'productId',
                message: 'Input productId:',
                validate: (input: string) => (input ? true : 'productId cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.redeemFlexibleProduct(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'redeem-locked-product',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Redeem Locked Product

* You need to open &#x60;Enable Spot &amp; Margin Trading&#x60; permission for the API Key which requests this endpoint.

Weight: 1/3s per account`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'position-id': {
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
                'redeem-locked-product is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.redeemLockedProduct(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'set-flexible-auto-subscribe',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Set Flexible Auto Subscribe

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'product-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-subscribe': {
                    type: 'boolean',
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

                if (!options?.['productId'] && !options?.interactive) {
                    requiredParams.push('productId');
                }

                if (!options?.['autoSubscribe'] && !options?.interactive) {
                    requiredParams.push('autoSubscribe');
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
                'set-flexible-auto-subscribe is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['productId']) {
            questions.push({
                type: 'input',
                name: 'productId',
                message: 'Input productId:',
                validate: (input: string) => (input ? true : 'productId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['autoSubscribe']) {
            questions.push({
                type: 'input',
                name: 'autoSubscribe',
                message: 'Input autoSubscribe:',
                validate: (input: string) => (input ? true : 'autoSubscribe cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.setFlexibleAutoSubscribe(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'set-locked-auto-subscribe',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Set locked auto subscribe

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'position-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-subscribe': {
                    type: 'boolean',
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

                if (!options?.['autoSubscribe'] && !options?.interactive) {
                    requiredParams.push('autoSubscribe');
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
                'set-locked-auto-subscribe is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['autoSubscribe']) {
            questions.push({
                type: 'input',
                name: 'autoSubscribe',
                message: 'Input autoSubscribe:',
                validate: (input: string) => (input ? true : 'autoSubscribe cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.setLockedAutoSubscribe(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'set-locked-product-redeem-option',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Set redeem option for Locked product

Weight: 50`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'position-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'redeem-to': {
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

                if (!options?.['redeemTo'] && !options?.interactive) {
                    requiredParams.push('redeemTo');
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
                'set-locked-product-redeem-option is signed. Please create a profile using `binance-cli profile create`.'
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

        if (options.interactive && !options?.['redeemTo']) {
            questions.push({
                type: 'input',
                name: 'redeemTo',
                message: 'Input redeemTo:',
                validate: (input: string) => (input ? true : 'redeemTo cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.setLockedProductRedeemOption(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'simple-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Simple Account query

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'simple-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.simpleAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'subscribe-flexible-product',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Subscribe Flexible Product

* You need to open &#x60;Enable Spot &amp; Margin Trading&#x60; permission for the API Key which requests this endpoint.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'product-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-subscribe': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'source-account': {
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

                if (!options?.['productId'] && !options?.interactive) {
                    requiredParams.push('productId');
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
                'subscribe-flexible-product is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['productId']) {
            questions.push({
                type: 'input',
                name: 'productId',
                message: 'Input productId:',
                validate: (input: string) => (input ? true : 'productId cannot be empty'),
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
            const response = await client.restAPI.subscribeFlexibleProduct(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'subscribe-locked-product',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Subscribe Locked Product

* You need to open &#x60;Enable Spot &amp; Margin Trading&#x60; permission for the API Key which requests this endpoint.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'project-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'auto-subscribe': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'source-account': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'redeem-to': {
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

                if (!options?.['projectId'] && !options?.interactive) {
                    requiredParams.push('projectId');
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
                'subscribe-locked-product is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['projectId']) {
            questions.push({
                type: 'input',
                name: 'projectId',
                message: 'Input projectId:',
                validate: (input: string) => (input ? true : 'projectId cannot be empty'),
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
            const response = await client.restAPI.subscribeLockedProduct(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-rwusd-account',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get RWUSD account information.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-rwusd-account is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getRwusdAccount(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-rwusd-quota-details',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get RWUSD quota details including subscription quota, fast redemption quota, and standard redemption quota.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-rwusd-quota-details is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getRwusdQuotaDetails(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-rwusd-rate-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get RWUSD rate history sorted by descending order.

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 6 months.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, &#x60;endTime&#x60; will default to current time, and results from &#x60;startTime&#x60; onward will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, &#x60;startTime&#x60; defaults to the current time minus one month, and data between &#x60;startTime&#x60; and &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
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
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-rwusd-rate-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getRwusdRateHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-rwusd-redemption-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get RWUSD redemption history.

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 6 months.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, &#x60;endTime&#x60; will default to current time, and results from &#x60;startTime&#x60; onward will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, &#x60;startTime&#x60; defaults to the current time minus one month, and data between &#x60;startTime&#x60; and &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
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
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-rwusd-redemption-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getRwusdRedemptionHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-rwusd-rewards-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get RWUSD rewards history.

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 6 months.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, &#x60;endTime&#x60; will default to current time, and results from &#x60;startTime&#x60; onward will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, &#x60;startTime&#x60; defaults to the current time minus one month, and data between &#x60;startTime&#x60; and &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
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
            current: {
                describe: decodeSelectedEntities(
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-rwusd-rewards-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getRwusdRewardsHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-rwusd-subscription-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get RWUSD subscription history

* The time between &#x60;startTime&#x60; and &#x60;endTime&#x60; cannot be longer than 6 months.
* If &#x60;startTime&#x60; and &#x60;endTime&#x60; are both not sent, then the last 30 days&#39; data will be returned.
* If &#x60;startTime&#x60; is sent but &#x60;endTime&#x60; is not sent, &#x60;endTime&#x60; will default to current time, and results from &#x60;startTime&#x60; onward will be returned.
* If &#x60;endTime&#x60; is sent but &#x60;startTime&#x60; is not sent, &#x60;startTime&#x60; defaults to the current time advanced by one month, and data between &#x60;startTime&#x60; and &#x60;endTime&#x60; will be returned.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                describe: decodeSelectedEntities('USDC or USDT'),
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
                    'Currently querying page. Starts from 1. Default: 1'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            size: {
                describe: decodeSelectedEntities(
                    'Number of results per page. Default: 10, Max: 100'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-rwusd-subscription-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getRwusdSubscriptionHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'redeem-rwusd',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Redeem RWUSD to USDC

* You need to open Enable Spot &amp; Margin Trading permission for the API Key which requests this endpoint.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
                'redeem-rwusd is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
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
            const response = await client.restAPI.redeemRwusd(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'subscribe-rwusd',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Subscribe RWUSD

* You need to open Enable Spot &amp; Margin Trading permission for the API Key which requests this endpoint.

Weight: 150`,
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
                'subscribe-rwusd is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.subscribeRwusd(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

simpleEarnCommands.push({
    command: 'get-yield-arena-activities',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get the list of Earn Yield Arena giveaway activities currently available to the user.

Supported locales: &#x60;en&#x60;, &#x60;en-GB&#x60;, &#x60;en-AU&#x60;, &#x60;cn&#x60;, &#x60;zh&#x60;, &#x60;zh-CN&#x60;, &#x60;tw&#x60;, &#x60;zh-TW&#x60;, &#x60;zh-HK&#x60;, &#x60;ja&#x60;, &#x60;ja-JP&#x60;, &#x60;ru&#x60;, &#x60;ru-RU&#x60;, &#x60;es&#x60;, &#x60;es-ES&#x60;, &#x60;es-LA&#x60;, &#x60;pt&#x60;, &#x60;pt-BR&#x60;, &#x60;pt-PT&#x60;, &#x60;fr&#x60;, &#x60;fr-FR&#x60;, &#x60;de&#x60;, &#x60;de-DE&#x60;, &#x60;it&#x60;, &#x60;it-IT&#x60;, &#x60;id&#x60;, &#x60;id-ID&#x60;, &#x60;vi&#x60;, &#x60;vi-VN&#x60;, &#x60;ar&#x60;, &#x60;ar-SA&#x60;, &#x60;pl&#x60;, &#x60;pl-PL&#x60;, &#x60;uk&#x60;, &#x60;uk-UA&#x60;, &#x60;cs&#x60;, &#x60;cs-CZ&#x60;, &#x60;ro&#x60;, &#x60;ro-RO&#x60;, &#x60;sv&#x60;, &#x60;sv-SE&#x60;, &#x60;bg&#x60;, &#x60;bg-BG&#x60;, &#x60;da&#x60;, &#x60;da-DK&#x60;, &#x60;el&#x60;, &#x60;el-GR&#x60;, &#x60;hu&#x60;, &#x60;hu-HU&#x60;, &#x60;lv&#x60;, &#x60;lv-LV&#x60;, &#x60;sk&#x60;, &#x60;sk-SK&#x60;, &#x60;sl&#x60;, &#x60;sl-SI&#x60;.

Weight: 150`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
                describe: decodeSelectedEntities('The value cannot be greater than 60000 (ms)'),
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
                'get-yield-arena-activities is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getYieldArenaActivities(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'simple-earn',
    description: 'Binance Simple Earn REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli simple-earn <command> [options]');
        simpleEarnCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
