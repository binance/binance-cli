import { Wallet, WALLET_REST_API_PROD_URL } from '@binance/wallet';
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
    process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('wallet');

    let basePath = WALLET_REST_API_PROD_URL;

    const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'wallet');

    if (process.env.BINANCE_WALLET_BASE_PATH) {
        basePath = process.env.BINANCE_WALLET_BASE_PATH;
    } else if (configurationRestAPI && configurationRestAPI['basePath']) {
        basePath = configurationRestAPI['basePath'];
    }

    let client;
    let hasConfig = false;
    if (configurationRestAPI !== null) {
        hasConfig = true;
        client = new Wallet({
            configurationRestAPI: { ...configurationRestAPI, basePath },
        });
    } else {
        client = new Wallet({
            configurationRestAPI: {
                apiKey: '',
                basePath,
            },
        });
    }

    return { client, hasConfig };
};

const walletCommands: any[] = [];

walletCommands.push({
    command: 'account-api-trading-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch account api trading status detail.

Weight: 1`,
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
                'account-api-trading-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountApiTradingStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'account-info',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch account info detail.

Weight: 1`,
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
                'account-info is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountInfo(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'account-status',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch account status detail.

Weight: 1`,
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
                'account-status is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.accountStatus(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'daily-account-snapshot',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Daily account snapshot

* The query time period must be less then 30 days
* Support query within the last one month only
* If startTimeand endTime not sent, return records of the last 7 days by default

Weight: 2400`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                type: {
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
                limit: {
                    describe: decodeSelectedEntities('min 7, max 30, default 7'),
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
                'daily-account-snapshot is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.dailyAccountSnapshot(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'disable-fast-withdraw-switch',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `
Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
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
                'disable-fast-withdraw-switch is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            await client.restAPI.disableFastWithdrawSwitch(options);
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'enable-fast-withdraw-switch',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Enable Fast Withdraw Switch (USER_DATA)

* This request will enable fastwithdraw switch under your  account. &lt;br&gt;&lt;/br&gt;
* When Fast Withdraw Switch is on, transferring funds to a Binance account will be done instantly. There is no on-chain transaction, no transaction ID and no withdrawal fee.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'recv-window': {
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
                'enable-fast-withdraw-switch is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            await client.restAPI.enableFastWithdrawSwitch(options);
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'get-api-key-permission',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get API Key Permission

Weight: 1`,
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
                'get-api-key-permission is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getApiKeyPermission(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'asset-detail',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch details of assets supported on Binance.


* Please get network and other deposit or withdraw details from &#x60;&#x60;GET /sapi/v1/capital/config/getall&#x60;&#x60;.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
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
                'asset-detail is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.assetDetail(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'asset-dividend-record',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query asset dividend record.


* There cannot be more than 180 days between parameter &#x60;startTime&#x60; and &#x60;endTime&#x60;.

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
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
            limit: {
                describe: decodeSelectedEntities('min 7, max 30, default 7'),
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
                'asset-dividend-record is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.assetDividendRecord(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'dust-convert',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Convert dust assets

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'account-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'target-asset': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'third-party-client-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'dust-quota-asset-to-target-asset-price': {
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
                'dust-convert is signed. Please create a profile using `binance-cli profile create`.'
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.dustConvert(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'dust-convertible-assets',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query dust convertible assets

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'account-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'target-asset': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'dust-quota-asset-to-target-asset-price': {
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

                if (!options?.['targetAsset'] && !options?.interactive) {
                    requiredParams.push('targetAsset');
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
                'dust-convertible-assets is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['targetAsset']) {
            questions.push({
                type: 'input',
                name: 'targetAsset',
                message: 'Input targetAsset:',
                validate: (input: string) => (input ? true : 'targetAsset cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.dustConvertibleAssets(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'dust-transfer',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Convert dust assets to BNB.

* You need to open&#x60;Enable Spot &amp; Margin Trading&#x60; permission for the API Key which requests this endpoint.

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                asset: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'account-type': {
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
                'dust-transfer is signed. Please create a profile using `binance-cli profile create`.'
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

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.dustTransfer(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'dustlog',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Dustlog

* Only return last 100 records
* Only return records after 2020/12/01

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'account-type': {
                describe: decodeSelectedEntities(
                    '&#x60;SPOT&#x60;or&#x60;MARGIN&#x60;,default&#x60;SPOT&#x60;'
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
                'dustlog is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.dustlog(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'funding-wallet',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query Funding Wallet


* Currently supports querying the following business assets：Binance Pay, Binance Card, Binance Gift Card, Stock Token

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                type: 'string',
                group: 'Command Options:',
            },
            'need-btc-valuation': {
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
                'funding-wallet is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.fundingWallet(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'get-assets-that-can-be-converted-into-bnb',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get Assets That Can Be Converted Into BNB

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'account-type': {
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
                'get-assets-that-can-be-converted-into-bnb is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getAssetsThatCanBeConvertedIntoBnb(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'get-cloud-mining-payment-and-refund-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `The query of Cloud-Mining payment and refund history

* Just return the SUCCESS records of payment and refund.
* For response, type &#x3D; 248 means payment, type &#x3D; 249 means refund, status &#x3D;S means SUCCESS.

Weight: 600`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
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
                'tran-id': {
                    describe: decodeSelectedEntities('The transaction id'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'client-tran-id': {
                    describe: decodeSelectedEntities('The unique flag'),
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                current: {
                    describe: decodeSelectedEntities('current page, default 1, the min value is 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                size: {
                    describe: decodeSelectedEntities('page size, default 10, the max value is 100'),
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
                'get-cloud-mining-payment-and-refund-history is signed. Please create a profile using `binance-cli profile create`.'
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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getCloudMiningPaymentAndRefundHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'get-open-symbol-list',
    describe: decodeSelectedEntities(
        `Get the list of symbols that are scheduled to be opened for trading in the market.

Weight: 100`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

        try {
            const response = await client.restAPI.getOpenSymbolList();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'query-user-delegation-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query User Delegation History

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                email: {
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
                type: {
                    describe: decodeSelectedEntities('Delegate/Undelegate'),
                    type: 'string',
                    group: 'Command Options:',
                },
                asset: {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                current: {
                    describe: decodeSelectedEntities('current page, default 1, the min value is 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                size: {
                    describe: decodeSelectedEntities('page size, default 10, the max value is 100'),
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

                if (!options?.['startTime'] && !options?.interactive) {
                    requiredParams.push('startTime');
                }

                if (!options?.['endTime'] && !options?.interactive) {
                    requiredParams.push('endTime');
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
                'query-user-delegation-history is signed. Please create a profile using `binance-cli profile create`.'
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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUserDelegationHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'query-user-universal-transfer-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query User Universal Transfer History


*  &#x60;fromSymbol&#x60; must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
*  &#x60;toSymbol&#x60; must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
* Support query within the last 6 months only
* If &#x60;startTime&#x60;and &#x60;endTime&#x60; not sent, return records of the last 7 days by default

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                type: {
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
                    describe: decodeSelectedEntities('current page, default 1, the min value is 1'),
                    type: 'string',
                    group: 'Command Options:',
                },
                size: {
                    describe: decodeSelectedEntities('page size, default 10, the max value is 100'),
                    type: 'string',
                    group: 'Command Options:',
                },
                'from-symbol': {
                    describe: decodeSelectedEntities(''),
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-symbol': {
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
                'query-user-universal-transfer-history is signed. Please create a profile using `binance-cli profile create`.'
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
            const response = await client.restAPI.queryUserUniversalTransferHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'query-user-wallet-balance',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query User Wallet Balance

Weight: 60`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'quote-asset': {
                describe: decodeSelectedEntities(
                    '&#x60;USDT&#x60;, &#x60;ETH&#x60;, &#x60;USDC&#x60;, &#x60;BNB&#x60;, etc. default &#x60;BTC&#x60;'
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
                'query-user-wallet-balance is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.queryUserWalletBalance(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'toggle-bnb-burn-on-spot-trade-and-margin-interest',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Toggle BNB Burn On Spot Trade And Margin Interest

* &quot;spotBNBBurn&quot; and &quot;interestBNBBurn&quot; should be sent at least one.

Weight: 1(IP)`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'spot-bnb-burn': {
                type: 'string',
                group: 'Command Options:',
            },
            'interest-bnb-burn': {
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
                'toggle-bnb-burn-on-spot-trade-and-margin-interest is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response =
                await client.restAPI.toggleBnbBurnOnSpotTradeAndMarginInterest(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'trade-fee',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch trade fee

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            symbol: {
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
                'trade-fee is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.tradeFee(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'user-asset',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get user assets, just for positive data.

* If asset is set, then return this asset, otherwise return all assets positive.
* If needBtcValuation is set, then return btcValudation.

Weight: 5`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            asset: {
                type: 'string',
                group: 'Command Options:',
            },
            'need-btc-valuation': {
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
                'user-asset is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.userAsset(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'user-universal-transfer',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `user universal transfer

*  &#x60;fromSymbol&#x60; must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
*  &#x60;toSymbol&#x60; must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
* ENUM of transfer types:
* MAIN_UMFUTURE   Spot account transfer to USDⓈ-M Futures account
* MAIN_CMFUTURE   Spot account transfer to COIN-M Futures account
* MAIN_MARGIN   Spot account transfer to Margin（cross）account
* UMFUTURE_MAIN   USDⓈ-M Futures account transfer to Spot account
* UMFUTURE_MARGIN   USDⓈ-M Futures account transfer to Margin（cross）account
* CMFUTURE_MAIN   COIN-M Futures account transfer to Spot account
* CMFUTURE_MARGIN   COIN-M Futures account transfer to Margin(cross) account
* MARGIN_MAIN   Margin（cross）account transfer to Spot account
* MARGIN_UMFUTURE   Margin（cross）account transfer to USDⓈ-M Futures
* MARGIN_CMFUTURE   Margin（cross）account transfer to COIN-M Futures
* ISOLATEDMARGIN_MARGIN   Isolated margin account transfer to Margin(cross) account
* MARGIN_ISOLATEDMARGIN   Margin(cross) account transfer to Isolated margin account
* ISOLATEDMARGIN_ISOLATEDMARGIN   Isolated margin account transfer to Isolated margin account
* MAIN_FUNDING   Spot account transfer to Funding account
* FUNDING_MAIN   Funding account transfer to Spot account
* FUNDING_UMFUTURE   Funding account transfer to UMFUTURE account
* UMFUTURE_FUNDING   UMFUTURE account transfer to Funding account
* MARGIN_FUNDING   MARGIN account transfer to Funding account
* FUNDING_MARGIN   Funding account transfer to Margin account
* FUNDING_CMFUTURE   Funding account transfer to CMFUTURE account
* CMFUTURE_FUNDING   CMFUTURE account transfer to Funding account
* MAIN_OPTION  Spot account transfer to Options account
* OPTION_MAIN  Options account transfer to Spot account
* UMFUTURE_OPTION USDⓈ-M Futures account transfer to Options account
* OPTION_UMFUTURE Options account transfer to USDⓈ-M Futures account
* MARGIN_OPTION  Margin（cross）account transfer to Options account
* OPTION_MARGIN  Options account transfer to Margin（cross）account
* FUNDING_OPTION   Funding account transfer to Options account
* OPTION_FUNDING   Options account transfer to Funding account
* MAIN_PORTFOLIO_MARGIN  Spot account transfer to Portfolio Margin account
* PORTFOLIO_MARGIN_MAIN  Portfolio Margin account transfer to Spot account

Weight: 900`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                type: {
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
                'from-symbol': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'to-symbol': {
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

                if (!options?.['type'] && !options?.interactive) {
                    requiredParams.push('type');
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
                'user-universal-transfer is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['type']) {
            questions.push({
                type: 'input',
                name: 'type',
                message: 'Input type:',
                validate: (input: string) => (input ? true : 'type cannot be empty'),
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
            const response = await client.restAPI.userUniversalTransfer(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'all-coins-information',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Get information of coins (available for deposit and withdraw) for user.

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
                'all-coins-information is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.allCoinsInformation(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'deposit-address',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch deposit address with network.

* If &#x60;network&#x60; is not send, return with default network of the coin.
* You can get &#x60;network&#x60; and &#x60;isDefault&#x60; in &#x60;networkList&#x60; in the response of &#x60;Get /sapi/v1/capital/config/getall (HMAC SHA256)&#x60;.
* &#x60;amount&#x60; needs to be sent if using LIGHTNING network

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                coin: {
                    describe: decodeSelectedEntities(
                        '&#x60;coin&#x60; refers to the parent network address format that the address is using'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                network: {
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
                'deposit-address is signed. Please create a profile using `binance-cli profile create`.'
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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.depositAddress(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'deposit-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch deposit history.


* Please notice the default &#x60;startTime&#x60; and &#x60;endTime&#x60; to make sure that time interval is within 0-90 days.
* If both &#x60;&#x60;startTime&#x60;&#x60; and &#x60;&#x60;endTime&#x60;&#x60; are sent, time between &#x60;&#x60;startTime&#x60;&#x60; and &#x60;&#x60;endTime&#x60;&#x60; must be less than 90 days.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'include-source': {
                describe: decodeSelectedEntities(
                    'Default: &#x60;false&#x60;, return &#x60;sourceAddress&#x60;field when set to &#x60;true&#x60;'
                ),
                type: 'boolean',
                group: 'Command Options:',
            },
            coin: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            status: {
                describe: decodeSelectedEntities(
                    '0(0:Email Sent, 2:Awaiting Approval 3:Rejected 4:Processing 6:Completed)'
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
            offset: {
                describe: decodeSelectedEntities('Default: 0'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('min 7, max 30, default 7'),
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
                'deposit-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.depositHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'fetch-deposit-address-list-with-network',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch deposit address list with network.


* If network is not send, return with default network of the coin.
* You can get network and isDefault in networkList in the response of &#x60;Get /sapi/v1/capital/config/getall&#x60;.

Weight: 10`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                coin: {
                    describe: decodeSelectedEntities(
                        '&#x60;coin&#x60; refers to the parent network address format that the address is using'
                    ),
                    type: 'string',
                    group: 'Command Options:',
                },
                network: {
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
                'fetch-deposit-address-list-with-network is signed. Please create a profile using `binance-cli profile create`.'
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
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.fetchDepositAddressListWithNetwork(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'fetch-withdraw-address-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch withdraw address list

Weight: 10`,
            isFullDescription
        ),
    handler: async () => {
        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'fetch-withdraw-address-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        try {
            const response = await client.restAPI.fetchWithdrawAddressList();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'fetch-withdraw-quota',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch withdraw quota

Weight: 10`,
            isFullDescription
        ),
    handler: async () => {
        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'fetch-withdraw-quota is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        try {
            const response = await client.restAPI.fetchWithdrawQuota();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'one-click-arrival-deposit-apply',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Apply deposit credit for expired address (One click arrival)

* Params need to be in the POST body

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'deposit-id': {
                type: 'string',
                group: 'Command Options:',
            },
            'tx-id': {
                type: 'string',
                group: 'Command Options:',
            },
            'sub-account-id': {
                type: 'string',
                group: 'Command Options:',
            },
            'sub-user-id': {
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
                'one-click-arrival-deposit-apply is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.oneClickArrivalDepositApply(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'withdraw',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Submit a withdraw request.


* If &#x60;network&#x60; not send, return with default network of the coin.
* You can get &#x60;network&#x60; and &#x60;isDefault&#x60; in &#x60;networkList&#x60; of a coin in the response of &#x60;Get /sapi/v1/capital/config/getall (HMAC SHA256)&#x60;.
* To check if travel rule is required, by using  &#x60;GET /sapi/v1/localentity/questionnaire-requirements&#x60; and if it returns anything other than &#x60;NIL&#x60; you will need update SAPI to &#x60;POST /sapi/v1/localentity/withdraw/apply&#x60; else you can continue &#x60;POST /sapi/v1/capital/withdraw/apply&#x60;. Please note that if you are required to comply to travel rule please refer to the Travel Rule SAPI.
* For networks that do not support memo/tag, submitting a withdrawal request with a non-empty &#x60;addressTag&#x60; will return error &#x60;-4106 TAG_NOT_SUPPORTED_FOR_NETWORK&#x60;. Please omit the &#x60;addressTag&#x60; field for such networks. You can check whether a network requires a tag via &#x60;GET /sapi/v1/capital/config/getall&#x60;:
* If &#x60;withdrawTag&#x60; &#x3D; &#x60;true&#x60; → memo/tag is required.
* If &#x60;withdrawTag&#x60; &#x3D; &#x60;false&#x60; → memo/tag is not supported; omit &#x60;addressTag&#x60;.

Weight: 900`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                coin: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'withdraw-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                network: {
                    type: 'string',
                    group: 'Command Options:',
                },
                address: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'address-tag': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'transaction-fee-flag': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                name: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'wallet-type': {
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

                if (!options?.['coin'] && !options?.interactive) {
                    requiredParams.push('coin');
                }

                if (!options?.['address'] && !options?.interactive) {
                    requiredParams.push('address');
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
                'withdraw is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['coin']) {
            questions.push({
                type: 'input',
                name: 'coin',
                message: 'Input coin:',
                validate: (input: string) => (input ? true : 'coin cannot be empty'),
            });
        }

        if (options.interactive && !options?.['address']) {
            questions.push({
                type: 'input',
                name: 'address',
                message: 'Input address:',
                validate: (input: string) => (input ? true : 'address cannot be empty'),
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
            const response = await client.restAPI.withdraw(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'withdraw-history',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch withdraw history.

* &#x60;network&#x60; may not be in the response for old withdraw.
* Please notice the default &#x60;startTime&#x60; and &#x60;endTime&#x60; to make sure that time interval is within 0-90 days.
* If both &#x60;startTime&#x60; and &#x60;endTime&#x60;are sent, time between &#x60;startTime&#x60;and &#x60;endTime&#x60;must be less than 90 days.
* If &#x60;withdrawOrderId&#x60; is sent, time between &#x60;startTime&#x60; and &#x60;endTime&#x60; must be less than 7 days.
* If &#x60;withdrawOrderId&#x60; is sent, &#x60;startTime&#x60; and &#x60;endTime&#x60; are not sent, will return last 7 days records by default.
* Maximum support &#x60;idList&#x60; number is 45.

Weight: 18000
Request limit: 10 requests per second`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            coin: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'withdraw-order-id': {
                describe: decodeSelectedEntities(
                    'client side id for withdrawal, if provided in POST &#x60;/sapi/v1/capital/withdraw/apply&#x60;, can be used here for query.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            status: {
                describe: decodeSelectedEntities(
                    '0(0:Email Sent, 2:Awaiting Approval 3:Rejected 4:Processing 6:Completed)'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            offset: {
                describe: decodeSelectedEntities('Default: 0'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('min 7, max 30, default 7'),
                type: 'string',
                group: 'Command Options:',
            },
            'id-list': {
                describe: decodeSelectedEntities(
                    'id list returned in the response of POST &#x60;/sapi/v1/capital/withdraw/apply&#x60;, separated by &#x60;,&#x60;'
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
                'withdraw-history is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.withdrawHistory(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'get-symbols-delist-schedule-for-spot',
    describe: decodeSelectedEntities(
        `Get symbols delist schedule for spot

Weight: 100`,
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

        const { client } = getClient();

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getSymbolsDelistScheduleForSpot(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'system-status',
    describe: decodeSelectedEntities(
        `Fetch system status.

Weight: 1`,
        isFullDescription
    ),
    handler: async () => {
        const { client } = getClient();

        try {
            const response = await client.restAPI.systemStatus();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'broker-withdraw',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Submit a withdrawal request for brokers of local entities that required travel rule.

* If &#x60;network&#x60; not send, return with default network of the coin, but if the address could not match default network, the withdraw will be rejected.
* You can get &#x60;network&#x60; in &#x60;networkList&#x60; of a coin in the response
* Questionnaire is different for each local entity, please refer to
* If getting error like &#x60;Questionnaire format not valid.&#x60; or &#x60;Questionnaire must not be blank&#x60;,

Weight: 600`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                address: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'address-tag': {
                    type: 'string',
                    group: 'Command Options:',
                },
                network: {
                    type: 'string',
                    group: 'Command Options:',
                },
                coin: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'address-name': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'withdraw-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'transaction-fee-flag': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                'wallet-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                questionnaire: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'originator-pii': {
                    type: 'string',
                    group: 'Command Options:',
                },
                signature: {
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

                if (!options?.['address'] && !options?.interactive) {
                    requiredParams.push('address');
                }

                if (!options?.['coin'] && !options?.interactive) {
                    requiredParams.push('coin');
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
                }

                if (!options?.['withdrawOrderId'] && !options?.interactive) {
                    requiredParams.push('withdrawOrderId');
                }

                if (!options?.['questionnaire'] && !options?.interactive) {
                    requiredParams.push('questionnaire');
                }

                if (!options?.['originatorPii'] && !options?.interactive) {
                    requiredParams.push('originatorPii');
                }

                if (!options?.['signature'] && !options?.interactive) {
                    requiredParams.push('signature');
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
                'broker-withdraw is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['address']) {
            questions.push({
                type: 'input',
                name: 'address',
                message: 'Input address:',
                validate: (input: string) => (input ? true : 'address cannot be empty'),
            });
        }

        if (options.interactive && !options?.['coin']) {
            questions.push({
                type: 'input',
                name: 'coin',
                message: 'Input coin:',
                validate: (input: string) => (input ? true : 'coin cannot be empty'),
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

        if (options.interactive && !options?.['withdrawOrderId']) {
            questions.push({
                type: 'input',
                name: 'withdrawOrderId',
                message: 'Input withdrawOrderId:',
                validate: (input: string) => (input ? true : 'withdrawOrderId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['questionnaire']) {
            questions.push({
                type: 'input',
                name: 'questionnaire',
                message: 'Input questionnaire:',
                validate: (input: string) => (input ? true : 'questionnaire cannot be empty'),
            });
        }

        if (options.interactive && !options?.['originatorPii']) {
            questions.push({
                type: 'input',
                name: 'originatorPii',
                message: 'Input originatorPii:',
                validate: (input: string) => (input ? true : 'originatorPii cannot be empty'),
            });
        }

        if (options.interactive && !options?.['signature']) {
            questions.push({
                type: 'input',
                name: 'signature',
                message: 'Input signature:',
                validate: (input: string) => (input ? true : 'signature cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.brokerWithdraw(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'check-questionnaire-requirements',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `This API will return user-specific Travel Rule questionnaire requirement information in reference to the current API key.

Weight: 1`,
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
                'check-questionnaire-requirements is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.checkQuestionnaireRequirements(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'deposit-history-travel-rule',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch deposit history for local entities that required travel rule.

* Please notice the default &#x60;startTime&#x60; and &#x60;endTime&#x60; to make sure that time interval is within
* If both &#x60;&#x60;startTime&#x60;&#x60; and &#x60;&#x60;endTime&#x60;&#x60; are sent, time between &#x60;&#x60;startTime&#x60;&#x60; and &#x60;&#x60;endTime&#x60;&#x60; must
* Please, note that due to network-specific characteristics, the returned source address may be inaccurate. If multiple source addresses are found, only the first one will be returned.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'tr-id': {
                describe: decodeSelectedEntities(
                    'Comma(,) separated list of travel rule record Ids.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'tx-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'tran-id': {
                describe: decodeSelectedEntities('Comma(,) separated list of wallet tran Ids.'),
                type: 'string',
                group: 'Command Options:',
            },
            network: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            coin: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'travel-rule-status': {
                describe: decodeSelectedEntities('0:Completed,1:Pending,2:Failed'),
                type: 'string',
                group: 'Command Options:',
            },
            'pending-questionnaire': {
                describe: decodeSelectedEntities(
                    'true: Only return records that pending deposit questionnaire. false/not provided: return all records.'
                ),
                type: 'boolean',
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
            offset: {
                describe: decodeSelectedEntities('Default: 0'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('min 7, max 30, default 7'),
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
                'deposit-history-travel-rule is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.depositHistoryTravelRule(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'deposit-history-v2',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch deposit history for local entities that with required travel rule information.

* Please notice the default &#x60;startTime&#x60; and &#x60;endTime&#x60; to make sure that time interval is within
* If both &#x60;&#x60;startTime&#x60;&#x60; and &#x60;&#x60;endTime&#x60;&#x60; are sent, time between &#x60;&#x60;startTime&#x60;&#x60; and &#x60;&#x60;endTime&#x60;&#x60; must
* Please, note that due to network-specific characteristics, the returned source address may be inaccurate. If multiple source addresses are found, only the first one will be returned.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'deposit-id': {
                describe: decodeSelectedEntities('Comma(,) separated list of wallet tran Ids.'),
                type: 'string',
                group: 'Command Options:',
            },
            'tx-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            network: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            coin: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'retrieve-questionnaire': {
                describe: decodeSelectedEntities(
                    'true: return &#x60;questionnaire&#x60; within response.'
                ),
                type: 'boolean',
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
            offset: {
                describe: decodeSelectedEntities('Default: 0'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('min 7, max 30, default 7'),
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
                'deposit-history-v2 is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.depositHistoryV2(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'fetch-address-verification-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch address verification list for user to check on status and other details for the addresses stored in Address Book.

Weight: 1`,
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
                'fetch-address-verification-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.fetchAddressVerificationList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'get-country-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query the active country list for travel rule questionnaires.

Weight: 1`,
            isFullDescription
        ),
    handler: async () => {
        const { client, hasConfig } = getClient();
        if (!hasConfig) {
            console.error(
                'get-country-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        try {
            const response = await client.restAPI.getCountryList();
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'get-region-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Query the active region/city list for a given country.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'country-code': {
                    describe: decodeSelectedEntities(
                        'ISO 2-digit country code (from &#x60;Country List&#x60; API).'
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

                if (!options?.['countryCode'] && !options?.interactive) {
                    requiredParams.push('countryCode');
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
                'get-region-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.countryCode) {
            questions.push({
                type: 'input',
                name: 'countryCode',
                message: 'Input countryCode:',
                validate: (input: string) => (input ? true : 'countryCode cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.getRegionList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'submit-deposit-questionnaire',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Submit questionnaire for brokers of local entities that require travel rule.
The questionnaire is only applies to transactions from un-hosted wallets or VASPs that are not
yet onboarded with GTR.

* Questionnaire is different for each local entity, please refer
* If getting error like &#x60;Questionnaire format not valid.&#x60; or &#x60;Questionnaire must not be blank&#x60;,

Weight: 600`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'sub-account-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'deposit-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                questionnaire: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'beneficiary-pii': {
                    type: 'string',
                    group: 'Command Options:',
                },
                network: {
                    type: 'string',
                    group: 'Command Options:',
                },
                coin: {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                address: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'address-tag': {
                    type: 'string',
                    group: 'Command Options:',
                },
                signature: {
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

                if (!options?.['subAccountId'] && !options?.interactive) {
                    requiredParams.push('subAccountId');
                }

                if (!options?.['depositId'] && !options?.interactive) {
                    requiredParams.push('depositId');
                }

                if (!options?.['questionnaire'] && !options?.interactive) {
                    requiredParams.push('questionnaire');
                }

                if (!options?.['beneficiaryPii'] && !options?.interactive) {
                    requiredParams.push('beneficiaryPii');
                }

                if (!options?.['signature'] && !options?.interactive) {
                    requiredParams.push('signature');
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
                'submit-deposit-questionnaire is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['subAccountId']) {
            questions.push({
                type: 'input',
                name: 'subAccountId',
                message: 'Input subAccountId:',
                validate: (input: string) => (input ? true : 'subAccountId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['depositId']) {
            questions.push({
                type: 'input',
                name: 'depositId',
                message: 'Input depositId:',
                validate: (input: string) => (input ? true : 'depositId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['questionnaire']) {
            questions.push({
                type: 'input',
                name: 'questionnaire',
                message: 'Input questionnaire:',
                validate: (input: string) => (input ? true : 'questionnaire cannot be empty'),
            });
        }

        if (options.interactive && !options?.['beneficiaryPii']) {
            questions.push({
                type: 'input',
                name: 'beneficiaryPii',
                message: 'Input beneficiaryPii:',
                validate: (input: string) => (input ? true : 'beneficiaryPii cannot be empty'),
            });
        }

        if (options.interactive && !options?.['signature']) {
            questions.push({
                type: 'input',
                name: 'signature',
                message: 'Input signature:',
                validate: (input: string) => (input ? true : 'signature cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.submitDepositQuestionnaire(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'submit-deposit-questionnaire-travel-rule',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Submit questionnaire for local entities that require travel rule.
The questionnaire is only applies to transactions from unhosted wallets or VASPs that are not
yet onboarded with GTR.

* Questionnaire is different for each local entity, please refer
* If getting error like &#x60;Questionnaire format not valid.&#x60; or &#x60;Questionnaire must not be blank&#x60;,

Weight: 600`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'tran-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                questionnaire: {
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

                if (!options?.['tranId'] && !options?.interactive) {
                    requiredParams.push('tranId');
                }

                if (!options?.['questionnaire'] && !options?.interactive) {
                    requiredParams.push('questionnaire');
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
                'submit-deposit-questionnaire-travel-rule is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['tranId']) {
            questions.push({
                type: 'input',
                name: 'tranId',
                message: 'Input tranId:',
                validate: (input: string) => (input ? true : 'tranId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['questionnaire']) {
            questions.push({
                type: 'input',
                name: 'questionnaire',
                message: 'Input questionnaire:',
                validate: (input: string) => (input ? true : 'questionnaire cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.submitDepositQuestionnaireTravelRule(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'submit-deposit-questionnaire-v2',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Submit questionnaire for local entities that require travel rule.
The questionnaire is only applies to transactions from unhosted wallets or VASPs that are not
yet onboarded with GTR.

* Questionnaire is different for each local entity, please refer
* If getting error like &#x60;Questionnaire format not valid.&#x60; or &#x60;Questionnaire must not be blank&#x60;,

Weight: 600`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'deposit-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                questionnaire: {
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

                if (!options?.['depositId'] && !options?.interactive) {
                    requiredParams.push('depositId');
                }

                if (!options?.['questionnaire'] && !options?.interactive) {
                    requiredParams.push('questionnaire');
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
                'submit-deposit-questionnaire-v2 is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['depositId']) {
            questions.push({
                type: 'input',
                name: 'depositId',
                message: 'Input depositId:',
                validate: (input: string) => (input ? true : 'depositId cannot be empty'),
            });
        }

        if (options.interactive && !options?.['questionnaire']) {
            questions.push({
                type: 'input',
                name: 'questionnaire',
                message: 'Input questionnaire:',
                validate: (input: string) => (input ? true : 'questionnaire cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.submitDepositQuestionnaireV2(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'vasp-list',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch the VASP list for local entities.

Weight: 1`,
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
                'vasp-list is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.vaspList(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'withdraw-history-v1',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch withdraw history for local entities that required travel rule.

* &#x60;network&#x60; may not be in the response for old withdraw.
* Please notice the default &#x60;startTime&#x60; and &#x60;endTime&#x60; to make sure that time interval is within
* If both &#x60;startTime&#x60; and &#x60;endTime&#x60;are sent, time between &#x60;startTime&#x60;and &#x60;endTime&#x60;must be less

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'tr-id': {
                describe: decodeSelectedEntities(
                    'Comma(,) separated list of travel rule record Ids.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'tx-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'withdraw-order-id': {
                describe: decodeSelectedEntities(
                    'client side id for withdrawal, if provided in POST &#x60;/sapi/v1/capital/withdraw/apply&#x60;, can be used here for query.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            network: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            coin: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'travel-rule-status': {
                describe: decodeSelectedEntities('0:Completed,1:Pending,2:Failed'),
                type: 'string',
                group: 'Command Options:',
            },
            offset: {
                describe: decodeSelectedEntities('Default: 0'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('min 7, max 30, default 7'),
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
                'withdraw-history-v1 is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.withdrawHistoryV1(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'withdraw-history-v2',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Fetch withdraw history for local entities that required travel rule.

* &#x60;network&#x60; may not be in the response for old withdraw.
* Withdrawal made through /sapi/v1/capital/withdraw/apply may not be in the response.
* Please notice the default &#x60;startTime&#x60; and &#x60;endTime&#x60; to make sure that time interval is within
* If both &#x60;startTime&#x60; and &#x60;endTime&#x60;are sent, time between &#x60;startTime&#x60;and &#x60;endTime&#x60;must be less
* If withdrawOrderId is sent, time between startTime and endTime must be less than 7 days.
* If withdrawOrderId is sent, startTime and endTime are not sent, will return last 7 days records by default.
* Maximum support trId,txId number is 45.
* WithdrawOrderId only support 1.
* If responsible does not include withdrawalStatus, please input trId or txId retrieve the data.

Weight: 1`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd.options({
            'tr-id': {
                describe: decodeSelectedEntities(
                    'Comma(,) separated list of travel rule record Ids.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            'tx-id': {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'withdraw-order-id': {
                describe: decodeSelectedEntities(
                    'client side id for withdrawal, if provided in POST &#x60;/sapi/v1/capital/withdraw/apply&#x60;, can be used here for query.'
                ),
                type: 'string',
                group: 'Command Options:',
            },
            network: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            coin: {
                describe: decodeSelectedEntities(''),
                type: 'string',
                group: 'Command Options:',
            },
            'travel-rule-status': {
                describe: decodeSelectedEntities('0:Completed,1:Pending,2:Failed'),
                type: 'string',
                group: 'Command Options:',
            },
            offset: {
                describe: decodeSelectedEntities('Default: 0'),
                type: 'string',
                group: 'Command Options:',
            },
            limit: {
                describe: decodeSelectedEntities('min 7, max 30, default 7'),
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
                'withdraw-history-v2 is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.withdrawHistoryV2(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

walletCommands.push({
    command: 'withdraw-travel-rule',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(
            `Submit a withdrawal request for local entities that required travel rule.

* If &#x60;network&#x60; not send, return with default network of the coin, but if the address could not match default network, the withdraw will be rejected.
* You can get &#x60;network&#x60; and &#x60;isDefault&#x60; in &#x60;networkList&#x60; of a coin in the response
* Questionnaire is different for each local entity, please refer to
* If getting error like &#x60;Questionnaire format not valid.&#x60; or &#x60;Questionnaire must not be blank&#x60;,

Weight: 600`,
            isFullDescription
        ),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                coin: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'withdraw-order-id': {
                    type: 'string',
                    group: 'Command Options:',
                },
                network: {
                    type: 'string',
                    group: 'Command Options:',
                },
                address: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'address-tag': {
                    type: 'string',
                    group: 'Command Options:',
                },
                amount: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'transaction-fee-flag': {
                    type: 'boolean',
                    group: 'Command Options:',
                },
                name: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'wallet-type': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'recv-window': {
                    type: 'string',
                    group: 'Command Options:',
                },
                questionnaire: {
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

                if (!options?.['coin'] && !options?.interactive) {
                    requiredParams.push('coin');
                }

                if (!options?.['address'] && !options?.interactive) {
                    requiredParams.push('address');
                }

                if (!options?.['amount'] && !options?.interactive) {
                    requiredParams.push('amount');
                }

                if (!options?.['questionnaire'] && !options?.interactive) {
                    requiredParams.push('questionnaire');
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
                'withdraw-travel-rule is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['coin']) {
            questions.push({
                type: 'input',
                name: 'coin',
                message: 'Input coin:',
                validate: (input: string) => (input ? true : 'coin cannot be empty'),
            });
        }

        if (options.interactive && !options?.['address']) {
            questions.push({
                type: 'input',
                name: 'address',
                message: 'Input address:',
                validate: (input: string) => (input ? true : 'address cannot be empty'),
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

        if (options.interactive && !options?.['questionnaire']) {
            questions.push({
                type: 'input',
                name: 'questionnaire',
                message: 'Input questionnaire:',
                validate: (input: string) => (input ? true : 'questionnaire cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.withdrawTravelRule(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'wallet',
    description: 'Binance Wallet REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli wallet <command> [options]');
        walletCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
