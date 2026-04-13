#!/usr/bin/env node

import { ConfigurationRestAPI, sendRequest, TimeUnit } from '@binance/common';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import {
    getConfigurationRestAPI,
    getUserAgent,
    isEmpty,
    readStdinObj,
    VALID_HTTP_METHODS,
} from './utils';
import profileCommands from './profile';

import algoCommands from './algo';
import alphaCommands from './alpha';
import c2cCommands from './c2c';
import convertCommands from './convert';
import copyTradingCommands from './copy-trading';
import cryptoLoanCommands from './crypto-loan';
import futuresCoinCommands from './derivatives-trading-coin-futures';
import derivativesTradingOptionsCommands from './derivatives-trading-options';
import derivativesTradingPortfolioMarginCommands from './derivatives-trading-portfolio-margin';
import derivativesTradingPortfolioMarginProCommands from './derivatives-trading-portfolio-margin-pro';
import futuresUsdsCommands from './derivatives-trading-usds-futures';
import dualInvestmentCommands from './dual-investment';
import fiatCommands from './fiat';
import giftCardCommands from './gift-card';
import marginTradingCommands from './margin-trading';
import miningCommands from './mining';
import payCommands from './pay';
import rebateCommands from './rebate';
import simpleEarnCommands from './simple-earn';
import spotCommands from './spot';
import stakingCommands from './staking';
import subAccountCommands from './sub-account';
import vipLoanCommands from './vip-loan';
import walletCommands from './wallet';

const yargsInstance = yargs(hideBin(process.argv));

interface EnvVar {
    name: string;
    description?: string;
}

const envVars: EnvVar[] = [
    {
        name: 'BINANCE_API_KEY',
    },
    {
        name: 'BINANCE_SECRET_KEY',
    },
    {
        name: 'BINANCE_API_ENV',
        description: 'API Environment (prod | testnet | demo)',
    },
    {
        name: 'BINANCE_<PRODUCT>_BASE_PATH',
        description: 'Base path of the product (e.g. "https://api.binance.com" for Spot)',
    },
];

const allCommands = yargsInstance
    .usage('Usage: binance-cli <command> [options]')
    .scriptName('')
    .group('help', 'Global Options:')
    .group('version', 'Global Options:')
    .epilog(formatEnvVarsSection(envVars, Math.min(150, yargsInstance.terminalWidth())))
    .wrap(Math.min(150, yargsInstance.terminalWidth()));

(BigInt.prototype as any).toJSON = function () {
    return this.toString();
};

const commandModules = [
    algoCommands,
    alphaCommands,
    c2cCommands,
    convertCommands,
    copyTradingCommands,
    cryptoLoanCommands,
    derivativesTradingOptionsCommands,
    derivativesTradingPortfolioMarginCommands,
    derivativesTradingPortfolioMarginProCommands,
    dualInvestmentCommands,
    fiatCommands,
    futuresCoinCommands,
    futuresUsdsCommands,
    giftCardCommands,
    marginTradingCommands,
    miningCommands,
    payCommands,
    rebateCommands,
    simpleEarnCommands,
    spotCommands,
    stakingCommands,
    subAccountCommands,
    vipLoanCommands,
    walletCommands,
    profileCommands,
];

for (let i = 0; i < commandModules.length; i++) {
    if (!isEmpty(commandModules[i])) {
        allCommands.command({
            ...commandModules[i],
            handler: () => {
                allCommands.showHelp('log');
            },
        } as any);
    }
}

allCommands.command({
    command: 'request <method> <url>',
    describe: 'Custom request',
    builder: (options: any) => {
        return options
            .positional('method', {
                describe: 'HTTP Method (GET | POST | PUT | DELETE)',
                demandOption: true,
            })
            .positional('url', {
                describe: 'Request\'s URL (e.g. "https://api.binance.com/api/v3/exchangeInfo")',
                demandOption: true,
            })
            .option('signed', {
                type: 'boolean',
                describe: 'For signed endpoints',
            })
            .option('time-unit', {
                describe: 'Time Unit: MILLISECOND (default) or MICROSECOND',
                default: TimeUnit.MILLISECOND,
            })
            .check((options: any) => {
                if (
                    typeof options?.method !== 'string' ||
                    !VALID_HTTP_METHODS.has(options?.method.toUpperCase())
                ) {
                    return `${options?.method} is not a valid HTTP method.`;
                }
                return true;
            });
    },
    handler: async (options: any) => {
        const url = new URL(options.url);
        let requestParams;

        const stdinObj: any = readStdinObj();
        if (!isEmpty(stdinObj)) {
            requestParams = stdinObj;
        } else {
            requestParams = { ...options };
            // Remove params used by the cli
            delete requestParams.method;
            delete requestParams.url;
            delete requestParams.signed;
            delete requestParams.profile;
            delete requestParams['time-unit'];
            delete requestParams['timeUnit'];
            delete requestParams['$0'];
            delete requestParams['_'];
        }

        let configurationRestAPI = getConfigurationRestAPI(requestParams?.profile);
        if (configurationRestAPI === null) {
            configurationRestAPI = {
                apiKey: '',
                env: null,
            };
        }

        configurationRestAPI = {
            ...configurationRestAPI,
            basePath: url.protocol + '//' + url.hostname + `${url.port ? `:${url.port}` : ''}`,
        };

        const configRestAPI = new ConfigurationRestAPI(
            configurationRestAPI
        ) as ConfigurationRestAPI & {
            baseOptions: Record<string, unknown>;
        };

        configRestAPI.baseOptions = configRestAPI.baseOptions || {};
        configRestAPI.baseOptions.headers = {
            ...(configRestAPI.baseOptions.headers || {}),
            'User-Agent': getUserAgent(),
        };

        const response = await sendRequest(
            configRestAPI,
            url.pathname,
            options.method,
            requestParams,
            {},
            options['time-unit'],
            { isSigned: options['signed'] }
        );
        const responseData = await response.data();
        console.log(JSON.stringify(responseData, null, 2));
    },
});

if (hideBin(process.argv).length == 0) {
    allCommands.showHelp('log');
} else {
    allCommands
        .option('interactive', {
            group: 'Global Options:',
            alias: 'i',
            type: 'boolean',
            description: 'Run in interactive mode',
        })
        .option('profile', {
            group: 'Global Options:',
            type: 'string',
            description: 'Choose the API profile',
        })
        .completion('completion', 'Generate completion script')
        .parse();
}

function formatEnvVarsSection(vars: EnvVar[], totalWidth: number = 150): string {
    const indent = '  ';
    const typeCol = '[string]'.length;
    const maxNameLength = Math.max(...vars.map(({ name }) => name.length));

    const lines = vars.map(({ name, description }: EnvVar) => {
        const namePadding = ' '.repeat(maxNameLength - name.length + 7);
        const nameAndDesc = description
            ? `${name}${namePadding}${description}`
            : `${name}${namePadding}`;
        const typePadding = ' '.repeat(
            Math.max(2, totalWidth - indent.length - nameAndDesc.length - typeCol)
        );
        return `${indent}${nameAndDesc}${typePadding}`;
    });

    return `Environment Variables:\n${lines.join('\n')}`;
}
