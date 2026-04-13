import {
    getConfigDir,
    getCurrentProfile,
    getExistingProfiles,
    getProfileConfig,
    validateProfileName,
    validateProfileNameMessage,
} from './utils';
import inquirer from 'inquirer';
import fs from 'fs';

const profileCommands: any[] = [];

const configDir = getConfigDir();

profileCommands.push({
    command: 'create',
    describe: 'Create or update a profile',
    builder: (yargs: any) => {
        return yargs
            .option('name', {
                describe: 'Profile name',
            })
            .option('env', {
                describe: 'Environment name: prod, testnet or demo.',
            })
            .option('api-key', {
                describe: 'Enter your API key',
            })
            .option('api-secret', {
                describe: 'Enter your API secret or path to private key',
            })
            .option('select', {
                type: 'boolean',
                describe: 'Select and use the new profile',
            })
            .option('force', {
                alias: 'f',
                type: 'boolean',
                describe: 'Overwrite profile if it already exists',
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
                validate: (input: string) => {
                    if (input) {
                        if (!validateProfileName(input)) {
                            return validateProfileNameMessage(input);
                        }
                        return true;
                    }
                    return 'profile name cannot be empty';
                },
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

        if (options.interactive && !options['select']) {
            questions.push({
                type: 'confirm',
                // Default to Yes if there is no active profile
                default: !fs.existsSync(`${configDir}/active_profile`),
                name: 'select',
                message: 'Select and use the new profile:',
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        if (
            options.interactive &&
            fs.existsSync(`${configDir}/${options.name}`) &&
            !options?.force
        ) {
            const answers = await inquirer.prompt([
                {
                    type: 'confirm',
                    name: 'force',
                    default: false,
                    message: `A profile named "${options.name}" already exists ⚠️. Please confirm if you would like to overwrite it:`,
                },
            ]);
            options = { ...options, ...answers };
        }

        if (!fs.existsSync(configDir)) {
            fs.mkdirSync(configDir);
        }

        if (!validateProfileName(options.name)) {
            console.error(validateProfileNameMessage(options.name));
            process.exitCode = 1;
            return;
        }

        if (!fs.existsSync(`${configDir}/${options.name}`) || options?.force) {
            fs.writeFileSync(
                `${configDir}/${options.name}`,
                `api-key=${options['api-key']}\napi-secret=${String(options['api-secret']).replace(/\n/g, '\\n')}\nenv=${options.env}`
            );
            console.log(`Profile ${options.name} was created successfully ✅`);

            // Check if the user explicilty answered No to select the profile
            if (
                options?.select !== false &&
                (options?.select || !fs.existsSync(`${configDir}/active_profile`))
            ) {
                fs.writeFileSync(`${configDir}/active_profile`, `name=${options.name}`);
                console.log(`Profile ${options.name} was selected successfully ✅`);
            }
        } else {
            console.error(`Profile ${options.name} already exists, use -f to overwrite it ⚠️`);
            process.exitCode = 1;
        }
    },
});

profileCommands.push({
    command: ['select', 'change'],
    describe: 'Select profile',
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

        if (!profiles || profiles.length === 0) {
            console.error('There is no profile found ❌');
            process.exitCode = 1;
            return;
        }

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

        if (!validateProfileName(options.name)) {
            console.error(validateProfileNameMessage(options.name));
            process.exitCode = 1;
            return;
        }

        if (profiles.includes(options.name)) {
            fs.writeFileSync(`${configDir}/active_profile`, `name=${options.name}`);
            console.log(`Profile ${options.name} was selected successfully ✅`);
        } else {
            console.error(`Profile ${options.name} was not found ❌`);
            process.exitCode = 1;
        }
    },
});

profileCommands.push({
    command: 'delete',
    describe: 'Delete profiles',
    builder: (yargs: any) => {
        return yargs
            .option('names', {
                type: 'array',
                describe: 'Profile names',
            })
            .check((options: any) => {
                const requiredParams = [];
                if (!options.names && !options.interactive) {
                    requiredParams.push('names');
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

        if (!profiles || profiles.length === 0) {
            console.error('There is no profile found ❌');
            process.exitCode = 1;
            return;
        }

        if (options.interactive && !options.names) {
            questions.push({
                type: 'checkbox',
                name: 'names',
                choices: profiles.map((profile) => {
                    const profileConfig = getProfileConfig(profile);
                    return {
                        name: `${profile} (${profileConfig && profileConfig['env'] ? profileConfig['env'] : 'prod'})`,
                        value: profile,
                    };
                }),
                message: 'Please select the profile names:',
                validate: (input: string) => (input ? true : 'names cannot be empty'),
            });
        }

        if (questions.length > 0) {
            const answers = await inquirer.prompt(questions);
            options = { ...options, ...answers };
        }

        if (options?.names) {
            const profile = getCurrentProfile();
            for (let i = 0; i < options.names.length; i++) {
                const name = options.names[i];

                if (!validateProfileName(name)) {
                    console.error(validateProfileNameMessage(name));
                    process.exitCode = 1;
                    return;
                }

                if (profiles.includes(name)) {
                    fs.unlinkSync(`${configDir}/${name}`);
                    if (name === profile) {
                        fs.unlinkSync(`${configDir}/active_profile`);
                        console.log(`The active profile ${name} was deleted ⚠️`);
                    } else {
                        console.log(`Profile ${name} was deleted successfully ✅`);
                    }
                } else {
                    console.error(`Profile ${name} was not found ❌`);
                    process.exitCode = 1;
                }
            }
        }
    },
});

profileCommands.push({
    command: 'list',
    describe: 'List all the profiles',
    handler: async () => {
        const profiles = getExistingProfiles();

        if (!profiles || profiles.length === 0) {
            return;
        }

        let output = '';
        const currentProfile = getCurrentProfile();
        profiles.forEach((profile, index) => {
            const profileConfig = getProfileConfig(profile);
            const active = currentProfile === profile ? ' *' : '';
            output = `${output}${profile} (${profileConfig && profileConfig['env'] ? profileConfig['env'] : 'prod'})${active}`;
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
            console.error(
                'There is no active profile found, please create one using "binance-cli profile create"'
            );
            process.exitCode = 1;
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
    description: 'Profile commands (create, select, view, list, delete)',
    builder: (yargs: any) => {
        profileCommands.forEach((command: any) => {
            yargs.command(command);
        });
    },
};
