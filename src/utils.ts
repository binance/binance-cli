import fs from 'fs';
import path from 'path';
import os, { platform, arch } from 'os';
import { convert } from 'html-to-text';
import { markdownToTxt } from 'markdown-to-txt';
import type { ConfigurationRestAPI } from '@binance/common';

let stdin: any = null;
const homeDir = os.homedir();
const BINANCE_LOGIN_DIR = path.join(homeDir, '.binance');

export const validHTTPMethods = new Set([
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
    env: string | null;
};

export const getCurrentProfile = (): string | null => {
    if (fs.existsSync(`${BINANCE_LOGIN_DIR}/active_profile`)) {
        const session: Record<string, string> = {};
        const sessionData = fs.readFileSync(`${BINANCE_LOGIN_DIR}/active_profile`, 'utf-8');
        sessionData.split('\n').forEach((line) => {
            const trimmed = line.trim();
            if (trimmed && trimmed.includes('=')) {
                const [key, value] = trimmed.split('=');
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
            const [key, value] = trimmed.split('=');
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
        ...creds,
    };
};

export const getSessionCreds = (
    profile: string,
    packageName: string = ''
): CliConfiguration | null => {
    if (process.env.BINANCE_API_KEY && process.env.BINANCE_SECRET_KEY) {
        return {
            apiKey: process.env.BINANCE_API_KEY,
            apiSecret: process.env.BINANCE_SECRET_KEY,
            env: process.env.BINANCE_API_ENV ?? 'prod',
        };
    } else {
        let profileName;
        if (profile === undefined) {
            profileName = getCurrentProfile();
        } else {
            profileName = profile;
        }

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
        const input = fs.readFileSync('/dev/stdin', 'utf-8');
        return input;
    } catch (err) {
        console.error('Error reading stdin:', err);
        return null;
    }
}

export const readStdinObj = (): any => {
    if (stdin === null) {
        stdin = readStdinSync();
    }
    if (stdin !== null && stdin.length > 0) {
        return JSON.parse(stdin);
    }
    return {};
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

export const decodeSelectedEntities = (str: string) => {
    const options = {
        selectors: [
            { selector: 'table', format: 'dataTable' }, // Critical for visual table alignment
        ],
    };
    return convert(
        markdownToTxt(str)
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

export const getUserAgent = (product: string = 'unkown'): string => {
    let clientType = 'cli';
    if (isAIAgent()) {
        clientType = 'skill';
    }

    return `binance-${clientType}/${product}/1.1.0 (Node.js/${process.version}; ${platform()}; ${arch()})`;
};
