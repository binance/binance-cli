import { getConfigDir, getCurrentProfile, getExistingProfiles, getProfileConfig } from './utils';
import inquirer from 'inquirer';
import fs from 'fs';
const profileCommands: any[] = [];

const configDir = getConfigDir();

profileCommands.push({
    command: 'create',
    describe: 'Create a new profile',
    builder: (yargs: any) => {
        return yargs
            .option('name', {
                describe: 'Profile name',
            })
            .option('env', {
                describe: 'Environment name',
            })
            .option('api-key', {
                describe: 'Enter your API key',
            })
            .option('api-secret', {
                describe: 'Enter your API secret or path to private key',
            })
            .check((options: any) => {
                const requiredParams = [];
                if (!options.name && !options.interactive) {
                    requiredParams.push('name');
                }
                if (!options.env && !options.interactive) {
                    requiredParams.push('env');
                }
                if (!options['api-key'] && !options.interactive) {
                    requiredParams.push('api-key');
                }
                if (!options['api-secret'] && !options.interactive) {
                    requiredParams.push('api-secret');
                }
                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions = [];

        if (options.interactive && !options.name) {
            questions.push({
                type: 'input',
                name: 'name',
                message: 'Please choose the profile name:',
                validate: (input: string) => (input ? true : 'profile name cannot be empty'),
            });
        }

        if (options.interactive && !options.env) {
            questions.push({
                type: 'rawlist',
                name: 'env',
                message: 'Please choose the environment:',
                choices: ['prod', 'testnet', 'demo'],
                validate: (input: string) => (input ? true : 'env cannot be empty'),
            });
        }

        if (options.interactive && !options['api-key']) {
            questions.push({
                type: 'input',
                name: 'api-key',
                message: 'Please input your API Key:',
                validate: (input: string) => (input ? true : 'api-key cannot be empty'),
            });
        }

        if (options.interactive && !options['api-secret']) {
            questions.push({
                type: 'password',
                name: 'api-secret',
                message: 'Please input your API Secret:',
                mask: '*',
                validate: (input: string) => (input ? true : 'api-secret cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }
        if (!fs.existsSync(configDir)) {
            fs.mkdirSync(configDir);
        }
        fs.writeFileSync(
            `${configDir}/${options.name}`,
            `api-key=${options['api-key']}\napi-secret=${options['api-secret'].replace(/\n/g, '\\n')}\nenv=${options.env}`
        );
        console.log(`Profile ${options.name} was created successfully ✅`);
        fs.writeFileSync(`${configDir}/active_profile`, `name=${options.name}`);
        console.log(`Profile ${options.name} was selected successfully ✅`);
    },
});

profileCommands.push({
    command: 'change',
    describe: 'Change profile',
    builder: (yargs: any) => {
        return yargs
            .option('name', {
                describe: 'Profile name',
            })
            .check((options: any) => {
                const requiredParams = [];
                if (!options.name && !options.interactive) {
                    requiredParams.push('name');
                }

                if (requiredParams.length > 0) {
                    return `Following arguments are required: ${requiredParams.join(', ')}`;
                }

                return true;
            });
    },
    handler: async (options: any) => {
        const questions = [];
        const profiles = getExistingProfiles();

        if (options.interactive && !options.name) {
            questions.push({
                type: 'rawlist',
                name: 'name',
                choices: profiles.map((profile) => {
                    const profileConfig = getProfileConfig(profile);
                    return {
                        name: `${profile} (${profileConfig && profileConfig['env'] ? profileConfig['env'] : 'prod'})`,
                        value: profile,
                    };
                }),
                message: 'Please input the profile name:',
                validate: (input: string) => (input ? true : 'name cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        if (profiles.includes(options.name)) {
            fs.writeFileSync(`${configDir}/active_profile`, `name=${options.name}`);
            console.log(`Profile ${options.name} was selected successfully ✅`);
        } else {
            console.log(`Profile ${options.name} was not found ❌`);
        }
    },
});

profileCommands.push({
    command: 'list',
    describe: 'List all the profiles',
    handler: async () => {
        const profiles = getExistingProfiles();
        let output = '';
        profiles.forEach((profile, index) => {
            const profileConfig = getProfileConfig(profile);
            output = `${output}${profile} (${profileConfig && profileConfig['env'] ? profileConfig['env'] : 'prod'})`;
            if (index !== profiles.length - 1) {
                output = `${output}\n`;
            }
        });
        console.log(output);
    },
});

profileCommands.push({
    command: 'view',
    describe: 'View current active profile',
    handler: async () => {
        const profile = getCurrentProfile();
        if (!profile) {
            console.log(
                'There is no active profile found, please create one using "binance-cli profile create"'
            );
        } else {
            const profileConfig = getProfileConfig(profile);
            console.log(
                `The current active profile is: ${profile}${profileConfig && profileConfig['env'] ? ` (${profileConfig['env']})` : 'prod'}`
            );
        }
    },
});

export default {
    command: 'profile',
    description: 'Profile commands (create, change, view, list)',
    builder: (yargs: any) => {
        profileCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
