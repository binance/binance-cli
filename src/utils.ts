import fs from 'fs';
import path from 'path';
import os, { platform, arch } from 'os';
import { convert } from 'html-to-text';
import { markdownToTxt } from 'markdown-to-txt';
import type { ConfigurationRestAPI } from '@binance/common';
import { hideBin, Parser } from 'yargs/helpers';

const homeDir = os.homedir();
const BINANCE_LOGIN_DIR = path.join(homeDir, '.binance');

export const VALID_PROFILE_NAME = /^[a-zA-Z0-9][a-zA-Z0-9._-]{0,62}$/;

export const VALID_HTTP_METHODS = new Set([
    'GET',
    'POST',
    'PUT',
    'DELETE',
    'PATCH',
    'HEAD',
    'OPTIONS',
    'CONNECT',
    'TRACE',
]);

export function isHmacSecretKey(key: string): boolean {
    const regex = /^[A-Za-z0-9]{64}$/;
    return regex.test(key);
}

type CliConfiguration = ConfigurationRestAPI & {
    env?: string;
    wsStreamsBasePath?: string;
};

export const getCurrentProfile = (): string | null => {
    if (fs.existsSync(`${BINANCE_LOGIN_DIR}/active_profile`)) {
        const session: Record<string, string> = {};
        const sessionData = fs.readFileSync(`${BINANCE_LOGIN_DIR}/active_profile`, 'utf-8');
        sessionData.split('\n').forEach((line) => {
            const trimmed = line.trim();
            if (trimmed && trimmed.includes('=')) {
                const eqIndex = trimmed.indexOf('=');
                const key = trimmed.slice(0, eqIndex);
                const value = trimmed.slice(eqIndex + 1);
                if (key && value) {
                    session[key.trim()] = value.trim();
                }
            }
        });
        return session['name'] ?? null;
    }
    return null;
};

export const getProfileConfig = (profileName: string, packageName: string = '') => {
    const creds: Record<string, string> = {};

    if (!fs.existsSync(`${BINANCE_LOGIN_DIR}/${profileName}`)) {
        console.log(
            `The profile ${profileName} does not exist, please create it using "binance-cli profile create"`
        );
        return null;
    }

    const content = fs.readFileSync(`${BINANCE_LOGIN_DIR}/${profileName}`, 'utf-8');

    content.split('\n').forEach((line) => {
        const trimmed = line.trim();
        if (trimmed && trimmed.includes('=')) {
            const eqIndex = trimmed.indexOf('=');
            const key = trimmed.slice(0, eqIndex);
            const value = trimmed.slice(eqIndex + 1);
            if (key && value) {
                creds[key.trim()] = value.trim();
            }
        }
    });

    if (isEmpty(creds)) {
        return null;
    }

    return {
        apiKey: creds['api-key'] ?? '',
        apiSecret: creds['api-secret'] ? creds['api-secret'].replace(/\\n/g, '\n') : '',
        env: creds['env'] ?? process.env.BINANCE_API_ENV ?? 'prod',
        basePath:
            packageName && creds[`${packageName}-base-path`]
                ? creds[`${packageName}-base-path`]
                : '',
        wsStreamsBasePath:
            packageName && creds[`${packageName}-ws-streams-base-path`]
                ? creds[`${packageName}-ws-streams-base-path`]
                : '',
        ...creds,
    };
};

export const getSessionCreds = (
    profile: string,
    packageName: string = ''
): CliConfiguration | null => {
    if (profile !== undefined) {
        return getProfileConfig(profile, packageName);
    } else if (process.env.BINANCE_API_KEY && process.env.BINANCE_SECRET_KEY) {
        return {
            apiKey: process.env.BINANCE_API_KEY,
            apiSecret: process.env.BINANCE_SECRET_KEY,
            env: process.env.BINANCE_API_ENV ?? 'prod',
        };
    } else {
        const profileName = getCurrentProfile();

        if (!profileName) {
            return null;
        }

        return getProfileConfig(profileName, packageName);
    }
};

function readStdinSync(): string | null {
    try {
        // Check if stdin is a TTY (interactive terminal)
        if (process.stdin.isTTY) {
            // No piped input, stdin is from terminal, so return null
            return null;
        }

        // Read all data from stdin synchronously
        const input = fs.readFileSync(0, 'utf-8');
        return input;
    } catch (err) {
        console.error('Error reading stdin:', err);
        return null;
    }
}

let stdin: string | null = null;
let stdinObj: any | null = null;
let readStdinObjInitialized = false;

export const readStdinObj = (): any => {
    if (readStdinObjInitialized) {
        return stdinObj ?? {};
    }

    readStdinObjInitialized = true;

    if (stdin === null) {
        stdin = readStdinSync();
    }

    if (stdin !== null && stdin.length > 0) {
        try {
            stdinObj = JSON.parse(stdin);
            return stdinObj;
        } catch {
            const preview = stdin.length > 80 ? stdin.slice(0, 80) + '...' : stdin;
            console.error(
                'Error: stdin input is not valid JSON.\n' +
                    `Received: "${preview.trim()}"\n` +
                    'Expected: a JSON object, e.g. {"symbol":"BTCUSDT"}'
            );
            process.exit(1);
        }
    }

    stdinObj = {};
    return stdinObj;
};

export const getConfigurationRestAPI = (
    profile: string,
    packageName: string = ''
): CliConfiguration | null => {
    const creds = getSessionCreds(profile, packageName);
    if (creds === null) {
        return null;
    }

    if (creds.apiSecret && isHmacSecretKey(creds.apiSecret)) {
        return {
            ...creds,
        };
    } else {
        return {
            privateKey: creds.apiSecret,
            ...creds,
        };
    }
};

export const decodeSelectedEntities = (str: string, isFull = false) => {
    const options = {
        selectors: [
            { selector: 'table', format: 'dataTable' }, // Critical for visual table alignment
        ],
    };

    let description;
    if (isFull) {
        description = str;
    } else {
        description = str.split('\n\n')[0] ?? '';
    }

    return convert(
        markdownToTxt(description)
            .replace(/&#39;/g, "'")
            .replace(/&#x3D;/g, '=')
            .replace(/&#x60;/g, '`')
            .replace(/&gt;/g, '>')
            .replace(/&lt;/g, '<')
            .replace(/&quot;/g, '"'),
        options
    );
};

export const isEmpty = (obj: any): boolean => {
    return obj === null || Object.keys(obj).length === 0;
};

export const getConfigDir = (): string => {
    return BINANCE_LOGIN_DIR;
};

export const getExistingProfiles = (): string[] => {
    if (!fs.existsSync(getConfigDir())) {
        return [];
    }
    const files = fs.readdirSync(getConfigDir());
    const profiles = files.filter((a) => a != 'active_profile');
    return profiles;
};

export const isAIAgent = (): boolean => {
    if (process.env.AGENT) {
        return true;
    }
    if (process.env.AI_AGENT) {
        return true;
    }
    if (process.env.CLAUDECODE) {
        return true;
    }
    if (process.env.GEMINI_CLI) {
        return true;
    }
    if (process.env.CODEX_SANDBOX) {
        return true;
    }
    return false;
};

export const getUserAgent = (product: string = 'unknown'): string => {
    let clientType = 'cli';
    if (isAIAgent()) {
        clientType = 'skill';
    }

    return `binance-${clientType}/${product}/1.3.0 (Node.js/${process.version}; ${platform()}; ${arch()})`;
};

export const validateProfileName = (name: string): boolean => {
    if (!name || !VALID_PROFILE_NAME.test(name) || name.includes('..')) {
        return false;
    }
    return true;
};

export const validateProfileNameMessage = (name: string): string => {
    return (
        `Invalid profile name "${name}". ` +
        'Use only letters, numbers, hyphens, underscores, and dots. ' +
        'Must start with a letter or number. Max 63 characters.'
    );
};

let parsedArgs: ReturnType<typeof Parser> | undefined;

export function getParsedArgs() {
    if (parsedArgs === undefined) {
        parsedArgs = Parser(hideBin(process.argv));
    }
    return parsedArgs;
}
