#!/usr/bin/env node

import { ConfigurationRestAPI, sendRequest, TimeUnit } from '@binance/common';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { getConfigurationRestAPI, isEmpty, readStdinObj } from './utils';
import spotCommands from './spot';
import convertCommands from './convert';
import futuresUsdsCommands from './derivatives-trading-usds-futures';
import profileCommands from './profile';

const yargsInstance = yargs(hideBin(process.argv));

const allCommands = yargsInstance
    .usage('Usage: binance-cli <command> [options]')
    .scriptName('')
    .group('help', 'Global Options:')
    .group('version', 'Global Options:')
    .wrap(Math.min(150, yargsInstance.terminalWidth()));

(BigInt.prototype as any).toJSON = function () {
    return this.toString();
};

const commandModules = [profileCommands, spotCommands, convertCommands, futuresUsdsCommands];
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

allCommands.command(
    'request <method> <url>',
    'Custom request',
    (options: any) => {
        return options
            .positional('method', {
                describe: 'HTTP Method',
                demandOption: true,
            })
            .positional('url', {
                describe: "Request's URL",
                demandOption: true,
            })
            .option('signed', {
                type: 'boolean',
                describe: 'True if it is a signed request',
            })
            .option('time-unit', {
                describe: 'Time Unit: MILLISECOND (default) or MICROSECOND',
                default: TimeUnit.MILLISECOND,
            });
    },
    async (options: any) => {
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
            basePath: url.protocol + '//' + url.hostname,
        };

        const response = await sendRequest(
            new ConfigurationRestAPI(configurationRestAPI),
            url.pathname,
            options.method,
            requestParams,
            {},
            options['time-unit'],
            { isSigned: options['signed'] }
        );
        const responseData = await response.data();
        console.log(JSON.stringify(responseData, null, 2));
    }
);

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
