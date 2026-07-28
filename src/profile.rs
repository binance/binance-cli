use anyhow::{Context, Result, bail};
use clap::{ArgAction, Args, Subcommand, ValueEnum};
use dialoguer::{Confirm, Input, MultiSelect, Password, Select};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct ProfileArgs {
    #[command(subcommand)]
    command: ProfileCommand,
}

#[derive(Subcommand)]
pub enum ProfileCommand {
    /// Create or update a profile
    Create(CreateArgs),

    /// Select profile
    #[command(alias = "change")]
    Select(SelectArgs),

    /// View current active profile
    View,

    /// List all profiles
    List,

    /// Delete profiles
    Delete(DeleteArgs),
}

#[derive(Args, Debug)]
struct CreateArgs {
    /// Profile name
    #[arg(long)]
    name: Option<String>,

    /// Environment name: prod, testnet or demo
    #[arg(long)]
    env: Option<ApiEnv>,

    /// Enter your API key
    #[arg(long = "api-key")]
    api_key: Option<String>,

    /// Enter your API secret or path to private key
    #[arg(long = "api-secret")]
    api_secret: Option<String>,

    /// Select and use the new profile
    ///
    /// Supports:
    /// --select
    /// --select true
    /// --select false
    #[arg(
        long,
        action = ArgAction::Set,
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    select: Option<bool>,

    /// Overwrite profile if it already exists
    #[arg(short = 'f', long)]
    force: bool,

    /// Interactive mode
    #[arg(short = 'i', long)]
    interactive: bool,
}

#[derive(Args, Debug)]
struct SelectArgs {
    /// Profile name
    #[arg(long)]
    name: Option<String>,

    /// Interactive mode
    #[arg(short = 'i', long)]
    interactive: bool,
}

#[derive(Args, Debug)]
struct DeleteArgs {
    /// Profile names
    #[arg(long, num_args = 1..)]
    names: Option<Vec<String>>,

    /// Interactive mode
    #[arg(short = 'i', long)]
    interactive: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ApiEnv {
    Prod,
    Testnet,
    Demo,
}

impl ApiEnv {
    fn as_str(self) -> &'static str {
        match self {
            ApiEnv::Prod => "prod",
            ApiEnv::Testnet => "testnet",
            ApiEnv::Demo => "demo",
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => ApiEnv::Prod,
            1 => ApiEnv::Testnet,
            2 => ApiEnv::Demo,
            _ => ApiEnv::Prod,
        }
    }
}

pub fn handle_profile_command(command: ProfileCommand) -> Result<()> {
    match command {
        ProfileCommand::Create(args) => create_profile(args),
        ProfileCommand::Select(args) => select_profile(args),
        ProfileCommand::View => view_profile(),
        ProfileCommand::List => list_profiles(),
        ProfileCommand::Delete(args) => delete_profiles(args),
    }
}

fn binance_login_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("Could not find home directory")?;
    Ok(home_dir.join(".binance"))
}

fn active_profile_path() -> Result<PathBuf> {
    Ok(binance_login_dir()?.join("active_profile"))
}

fn profile_path(profile_name: &str) -> Result<PathBuf> {
    Ok(binance_login_dir()?.join(profile_name))
}

fn parse_key_value_content(content: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            if !key.is_empty() && !value.is_empty() {
                result.insert(key.to_string(), value.to_string());
            }
        }
    }

    result
}

fn validate_profile_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    if name == "." || name == ".." {
        return false;
    }

    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn validate_profile_name_message(name: &str) -> String {
    format!("Invalid profile name: {name}. Only letters, numbers, underscore and dash are allowed.")
}

fn get_current_profile() -> Result<Option<String>> {
    let path = active_profile_path()?;

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path)?;
    let session = parse_key_value_content(&content);

    Ok(session.get("name").cloned())
}

fn get_profile_config(profile_name: &str) -> Result<Option<HashMap<String, String>>> {
    let path = profile_path(profile_name)?;

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path)?;
    let creds = parse_key_value_content(&content);

    if creds.is_empty() {
        Ok(None)
    } else {
        Ok(Some(creds))
    }
}

fn get_profile_env(profile_name: &str) -> Result<String> {
    let env = get_profile_config(profile_name)?
        .and_then(|config| config.get("env").cloned())
        .unwrap_or_else(|| "prod".to_string());

    Ok(env)
}

fn get_existing_profiles() -> Result<Vec<String>> {
    let dir = binance_login_dir()?;

    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut profiles = vec![];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name,
            None => continue,
        };

        if file_name == "active_profile" {
            continue;
        }

        profiles.push(file_name.to_string());
    }

    profiles.sort();

    Ok(profiles)
}

fn create_profile(mut args: CreateArgs) -> Result<()> {
    if args.interactive {
        if args.name.is_none() {
            let name: String = Input::new()
                .with_prompt("Please choose the profile name")
                .validate_with(|input: &String| {
                    if input.is_empty() {
                        Err("profile name cannot be empty")
                    } else if !validate_profile_name(input) {
                        Err("invalid profile name")
                    } else {
                        Ok(())
                    }
                })
                .interact_text()?;

            args.name = Some(name);
        }

        if args.env.is_none() {
            let choices = ["prod", "testnet", "demo"];

            let selected = Select::new()
                .with_prompt("Please choose the environment")
                .items(&choices)
                .default(0)
                .interact()?;

            args.env = Some(ApiEnv::from_index(selected));
        }

        if args.api_key.is_none() {
            let api_key: String = Input::new()
                .with_prompt("Please input your API Key")
                .validate_with(|input: &String| {
                    if input.is_empty() {
                        Err("api-key cannot be empty")
                    } else {
                        Ok(())
                    }
                })
                .interact_text()?;

            args.api_key = Some(api_key);
        }

        if args.api_secret.is_none() {
            let api_secret = Password::new()
                .with_prompt("Please input your API Secret")
                .interact()?;

            if api_secret.is_empty() {
                bail!("api-secret cannot be empty");
            }

            args.api_secret = Some(api_secret);
        }

        if args.select.is_none() {
            let default_select = !active_profile_path()?.exists();

            let select = Confirm::new()
                .with_prompt("Select and use the new profile")
                .default(default_select)
                .interact()?;

            args.select = Some(select);
        }
    }

    let name = args
        .name
        .context("Following arguments are required: name")?;
    let env = args.env.context("Following arguments are required: env")?;
    let api_key = args
        .api_key
        .context("Following arguments are required: api-key")?;
    let api_secret = args
        .api_secret
        .context("Following arguments are required: api-secret")?;

    if !validate_profile_name(&name) {
        bail!("{}", validate_profile_name_message(&name));
    }

    let dir = binance_login_dir()?;
    let path = profile_path(&name)?;

    if path.exists() && !args.force {
        if args.interactive {
            let overwrite = Confirm::new()
                .with_prompt(format!(
                    "A profile named \"{}\" already exists ⚠️. Please confirm if you would like to overwrite it",
                    name
                ))
                .default(false)
                .interact()?;

            if !overwrite {
                bail!("Profile {} already exists, use -f to overwrite it ⚠️", name);
            }
        } else {
            bail!("Profile {} already exists, use -f to overwrite it ⚠️", name);
        }
    }

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    let escaped_api_secret = api_secret.replace('\n', "\\n");

    let profile_content = format!(
        "api-key={}\napi-secret={}\nenv={}",
        api_key,
        escaped_api_secret,
        env.as_str()
    );

    fs::write(&path, profile_content)?;

    println!("Profile {} was created successfully ✅", name);

    let should_select = args.select != Some(false)
        && (args.select.unwrap_or(false) || !active_profile_path()?.exists());

    if should_select {
        fs::write(active_profile_path()?, format!("name={}", name))?;
        println!("Profile {} was selected successfully ✅", name);
    }

    Ok(())
}

fn select_profile(mut args: SelectArgs) -> Result<()> {
    let profiles = get_existing_profiles()?;

    if profiles.is_empty() {
        bail!("There is no profile found ❌");
    }

    if args.interactive && args.name.is_none() {
        let mut choices = vec![];

        for profile in &profiles {
            let env = get_profile_env(profile)?;
            choices.push(format!("{} ({})", profile, env));
        }

        let selected = Select::new()
            .with_prompt("Please input the profile name")
            .items(&choices)
            .default(0)
            .interact()?;

        args.name = Some(profiles[selected].clone());
    }

    let name = args
        .name
        .context("Following arguments are required: name")?;

    if !validate_profile_name(&name) {
        bail!("{}", validate_profile_name_message(&name));
    }

    if profiles.contains(&name) {
        fs::write(active_profile_path()?, format!("name={}", name))?;
        println!("Profile {} was selected successfully ✅", name);
    } else {
        bail!("Profile {} was not found ❌", name);
    }

    Ok(())
}

fn view_profile() -> Result<()> {
    let profile = get_current_profile()?;

    match profile {
        Some(profile) => {
            let env = get_profile_env(&profile)?;
            println!("The current active profile is: {} ({})", profile, env);
        }
        None => {
            bail!(
                "There is no active profile found, please create one using \"binance-cli profile create\""
            );
        }
    }

    Ok(())
}

fn list_profiles() -> Result<()> {
    let profiles = get_existing_profiles()?;

    if profiles.is_empty() {
        return Ok(());
    }

    let current_profile = get_current_profile()?;

    for profile in profiles {
        let env = get_profile_env(&profile)?;
        let active = if current_profile.as_deref() == Some(profile.as_str()) {
            " *"
        } else {
            ""
        };

        println!("{} ({}){}", profile, env, active);
    }

    Ok(())
}

fn delete_profiles(mut args: DeleteArgs) -> Result<()> {
    let profiles = get_existing_profiles()?;

    if profiles.is_empty() {
        bail!("There is no profile found ❌");
    }

    if args.interactive && args.names.is_none() {
        let mut choices = vec![];

        for profile in &profiles {
            let env = get_profile_env(profile)?;
            choices.push(format!("{} ({})", profile, env));
        }

        let selected = MultiSelect::new()
            .with_prompt("Please select the profile names")
            .items(&choices)
            .interact()?;

        if selected.is_empty() {
            bail!("names cannot be empty");
        }

        let names = selected
            .into_iter()
            .map(|index| profiles[index].clone())
            .collect::<Vec<_>>();

        args.names = Some(names);
    }

    let names = args
        .names
        .context("Following arguments are required: names")?;

    if names.is_empty() {
        bail!("Following arguments are required: names");
    }

    let current_profile = get_current_profile()?;

    for name in names {
        if !validate_profile_name(&name) {
            bail!("{}", validate_profile_name_message(&name));
        }

        if profiles.contains(&name) {
            fs::remove_file(profile_path(&name)?)?;

            if current_profile.as_deref() == Some(name.as_str()) {
                let active_path = active_profile_path()?;

                if active_path.exists() {
                    fs::remove_file(active_path)?;
                }

                println!("The active profile {} was deleted ⚠️", name);
            } else {
                println!("Profile {} was deleted successfully ✅", name);
            }
        } else {
            bail!("Profile {} was not found ❌", name);
        }
    }

    Ok(())
}
