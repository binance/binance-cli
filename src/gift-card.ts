import { GiftCard, GIFT_CARD_REST_API_PROD_URL } from '@binance/giftcard';
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

process.env.BINANCE_CONNECTOR_JS_USER_AGENT = getUserAgent('gift-card');

const stdinObj: any = readStdinObj();

let basePath = GIFT_CARD_REST_API_PROD_URL;

const configurationRestAPI = getConfigurationRestAPI(parsedArgs?.profile, 'gift-card');

if (process.env.BINANCE_GIFT_CARD_BASE_PATH) {
    basePath = process.env.BINANCE_GIFT_CARD_BASE_PATH;
} else if (configurationRestAPI && configurationRestAPI['basePath']) {
    basePath = configurationRestAPI['basePath'];
}

let client;
if (configurationRestAPI !== null) {
    client = new GiftCard({
        configurationRestAPI: { ...configurationRestAPI, basePath },
    });
} else {
    client = new GiftCard({
        configurationRestAPI: {
            apiKey: '',
            basePath,
        },
    });
}

const giftCardCommands: any[] = [];

giftCardCommands.push({
    command: 'create-a-dual-token-gift-card',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`* This API is for creating a dual-token ( stablecoin-denominated) Binance Gift Card. You may create a gift card using USDT as baseToken, that is redeemable to another designated token (faceToken). For example, you can create a fixed-value BTC gift card and pay with 100 USDT plus minting fee. This gift card can keep the value fixed at 100 USDT before redemption, and will be redeemable to BTC equivalent to 100 USDT upon redemption.
* Once successfully created, the amount of baseToken (e.g. USDT) in the fixed-value gift card along with the fee would be deducted from your funding wallet.


* To get started with, please make sure:
* You have a Binance account
* You have passed KYB
* You have a sufﬁcient balance(Gift Card amount and fee amount) in your Binance funding wallet
* You need Enable Withdrawals for the API Key which requests this endpoint.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'base-token': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'face-token': {
                    type: 'string',
                    group: 'Command Options:',
                },
                'base-token-amount': {
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

                if (!options?.['baseToken'] && !options?.interactive) {
                    requiredParams.push('baseToken');
                }

                if (!options?.['faceToken'] && !options?.interactive) {
                    requiredParams.push('faceToken');
                }

                if (!options?.['baseTokenAmount'] && !options?.interactive) {
                    requiredParams.push('baseTokenAmount');
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
                'create-a-dual-token-gift-card is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['baseToken']) {
            questions.push({
                type: 'input',
                name: 'baseToken',
                message: 'Input baseToken:',
                validate: (input: string) => (input ? true : 'baseToken cannot be empty'),
            });
        }

        if (options.interactive && !options?.['faceToken']) {
            questions.push({
                type: 'input',
                name: 'faceToken',
                message: 'Input faceToken:',
                validate: (input: string) => (input ? true : 'faceToken cannot be empty'),
            });
        }

        if (options.interactive && !options?.['baseTokenAmount']) {
            questions.push({
                type: 'input',
                name: 'baseTokenAmount',
                message: 'Input baseTokenAmount:',
                validate: (input: string) => (input ? true : 'baseTokenAmount cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.createADualTokenGiftCard(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

giftCardCommands.push({
    command: 'create-a-single-token-gift-card',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This API is for creating a Binance Gift Card.

To get started with, please make sure:

* You have a Binance account
* You have passed KYB
* You have a sufﬁcient balance(Gift Card amount and fee amount) in your Binance funding wallet
* You need &#x60;Enable Withdrawals&#x60; for the API Key which requests this endpoint.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                token: {
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

                if (!options?.['token'] && !options?.interactive) {
                    requiredParams.push('token');
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
                'create-a-single-token-gift-card is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['token']) {
            questions.push({
                type: 'input',
                name: 'token',
                message: 'Input token:',
                validate: (input: string) => (input ? true : 'token cannot be empty'),
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
            const response = await client.restAPI.createASingleTokenGiftCard(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

giftCardCommands.push({
    command: 'fetch-rsa-public-key',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This API is for fetching the RSA Public Key.
This RSA Public key will be used to encrypt the card code.

**Please note that the RSA Public key fetched is valid only for the current day.**

Weight: 1`),
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

        if (!isEmpty(stdinObj)) {
            options = { ...options, ...stdinObj };
        }

        if (options.json) {
            options = { ...options, ...JSON.parse(options.json) };
            delete options.json;
        }

        if (isEmpty(configurationRestAPI)) {
            console.error(
                'fetch-rsa-public-key is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.fetchRsaPublicKey(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

giftCardCommands.push({
    command: 'fetch-token-limit',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This API is to help you verify which tokens are available for you to create Stablecoin-Denominated gift cards as mentioned in section 2 and its’ limitation.

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'base-token': {
                    describe: decodeSelectedEntities('The token you want to pay, example: BUSD'),
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

                if (!options?.['baseToken'] && !options?.interactive) {
                    requiredParams.push('baseToken');
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
                'fetch-token-limit is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.baseToken) {
            questions.push({
                type: 'input',
                name: 'baseToken',
                message: 'Input baseToken:',
                validate: (input: string) => (input ? true : 'baseToken cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.fetchTokenLimit(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

giftCardCommands.push({
    command: 'redeem-a-binance-gift-card',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This API is for redeeming a Binance Gift Card
Once redeemed, the coins will be deposited in your funding wallet.

* Parameter code can be sent in two formats:
* Plaintext
* Encrypted

* Sending code in Encrypted format provides more security than sending it as a plaintext. To send card code in encrypted format the following steps must be followed:
* Fetch RSA public key from api stated below.
* Use the below algorithm to encrypt the card code using the RSA public key fetched above: &#x60;RSA/ECB/OAEPWithSHA-256AndMGF1Padding&#x60;
**A sample code snippet (JAVA) is stated below for reference, the same approach can be used for different languages like C#, PERL, PYTHON, SHELL etc.:**

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                code: {
                    type: 'string',
                    group: 'Command Options:',
                },
                'external-uid': {
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

                if (!options?.['code'] && !options?.interactive) {
                    requiredParams.push('code');
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
                'redeem-a-binance-gift-card is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options?.['code']) {
            questions.push({
                type: 'input',
                name: 'code',
                message: 'Input code:',
                validate: (input: string) => (input ? true : 'code cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.redeemABinanceGiftCard(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

giftCardCommands.push({
    command: 'verify-binance-gift-card-by-gift-card-number',
    describe:
        'Authentication required. ' +
        decodeSelectedEntities(`This API is for verifying whether the Binance Gift Card is valid or not by entering Gift Card Number.

**Please note that if you enter the wrong Gift Card Number 5 times within an hour, you will no longer be able to verify any Gift Card Number for that hour.**

Weight: 1`),
    builder: (yargsCmd: any) => {
        return yargsCmd
            .options({
                'reference-no': {
                    describe: decodeSelectedEntities('Enter the Gift Card Number'),
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

                if (!options?.['referenceNo'] && !options?.interactive) {
                    requiredParams.push('referenceNo');
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
                'verify-binance-gift-card-by-gift-card-number is signed. Please create a profile using `binance-cli profile create`.'
            );
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.referenceNo) {
            questions.push({
                type: 'input',
                name: 'referenceNo',
                message: 'Input referenceNo:',
                validate: (input: string) => (input ? true : 'referenceNo cannot be empty'),
            });
        }
        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        try {
            const response = await client.restAPI.verifyBinanceGiftCardByGiftCardNumber(options);
            const responseData = await response.data();
            console.log(JSON.stringify(responseData, null, 2));
        } catch (e: any) {
            console.log(e.message);
            return;
        }
    },
});

export default {
    command: 'gift-card',
    description: 'Binance Gift Card REST API',
    builder: (yargs: any) => {
        yargs.usage('Usage: binance-cli gift-card <command> [options]');
        giftCardCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
