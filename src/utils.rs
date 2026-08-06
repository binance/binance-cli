use pulldown_cmark::{Options, Parser, html};

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::{env, fs, process};

static STDIN_RAW: OnceLock<String> = OnceLock::new();
static USER_AGENT_INIT: OnceLock<()> = OnceLock::new();

pub fn decode_selected_entities(input: &str, is_full: bool) -> String {
    let description = if is_full {
        input
    } else {
        input.split("\n\n").next().unwrap_or("")
    };

    let plain_text = markdown_to_text(description);

    plain_text
        .replace("&#39;", "'")
        .replace("&#x3D;", "=")
        .replace("&#x60;", "`")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
}

fn markdown_to_text(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html2text::from_read(html_output.as_bytes(), 80)
}
pub fn read_stdin_as<T>() -> Option<T>
where
    T: DeserializeOwned + Clone,
{
    let stdin = read_stdin_raw_once();

    if stdin.is_empty() {
        return None;
    }

    match serde_json::from_str::<T>(stdin) {
        Ok(value) => Some(value),
        Err(_) => {
            let preview = preview_stdin(stdin);

            eprintln!(
                "Error: stdin input is not valid JSON.\n\
                 Received: \"{}\"",
                preview.trim(),
            );

            process::exit(1);
        }
    }
}

pub fn read_json_as<T>(json: String) -> Option<T>
where
    T: DeserializeOwned + Clone,
{
    match serde_json::from_str::<T>(&json) {
        Ok(value) => Some(value),
        Err(_) => {
            let preview = preview_stdin(&json);

            eprintln!(
                "Error: json param is not valid JSON.\n\
                 Received: \"{}\"",
                preview.trim(),
            );

            process::exit(1);
        }
    }
}

fn read_stdin_raw_once() -> &'static str {
    STDIN_RAW.get_or_init(|| {
        let mut stdin_handle = io::stdin();

        // Avoid blocking if nothing is piped into stdin
        if stdin_handle.is_terminal() {
            return String::new();
        }

        let mut stdin = String::new();

        if let Err(err) = stdin_handle.read_to_string(&mut stdin) {
            eprintln!("Error: failed to read stdin: {err}");
            process::exit(1);
        }

        stdin
    })
}

fn preview_stdin(input: &str) -> String {
    let mut preview: String = input.chars().take(80).collect();

    if input.chars().count() > 80 {
        preview.push_str("...");
    }

    preview
}

fn binance_login_dir() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home).join(".binance");
    }

    if let Ok(userprofile) = env::var("USERPROFILE") {
        return PathBuf::from(userprofile).join(".binance");
    }

    panic!("Could not find home directory");
}

#[derive(Debug, Clone)]
pub struct CliConfiguration {
    pub api_key: String,
    pub api_secret: String,
    pub env: Option<String>,
    pub base_path: Option<String>,
    pub base_path_ws_streams: Option<String>,
    pub private_key: Option<String>,
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

pub fn get_current_profile() -> Option<String> {
    let active_profile_path = binance_login_dir().join("active_profile");

    if !active_profile_path.exists() {
        return None;
    }

    let session_data = fs::read_to_string(active_profile_path).ok()?;
    let session = parse_key_value_content(&session_data);

    session.get("name").cloned()
}

pub fn get_profile_config(profile_name: &str, package_name: &str) -> Option<CliConfiguration> {
    let profile_path = binance_login_dir().join(profile_name);

    if !profile_path.exists() {
        println!(
            "The profile {} does not exist, please create it using \"binance-cli profile create\"",
            profile_name
        );
        return None;
    }

    let content = fs::read_to_string(profile_path).ok()?;
    let creds = parse_key_value_content(&content);

    if creds.is_empty() {
        return None;
    }

    let api_key = creds.get("api-key").cloned().unwrap_or_default();

    let api_secret = creds
        .get("api-secret")
        .map(|value| value.replace("\\n", "\n"))
        .unwrap_or_default();

    let api_env = creds.get("env").cloned();

    let base_path = if !package_name.is_empty() {
        creds.get(&format!("{}-base-path", package_name)).cloned()
    } else {
        None
    };

    let base_path_ws_streams = if !package_name.is_empty() {
        creds
            .get(&format!("{}-ws-streams-base-path", package_name))
            .cloned()
    } else {
        None
    };

    Some(CliConfiguration {
        api_key,
        api_secret,
        env: api_env,
        base_path: std::env::var(format!("BINANCE_{}_BASE_PATH", package_name.to_uppercase()))
            .ok()
            .filter(|v| !v.is_empty())
            .or(base_path),
        base_path_ws_streams: std::env::var(format!(
            "BINANCE_{}_WS_STREAMS_BASE_PATH",
            package_name.to_uppercase()
        ))
        .ok()
        .filter(|v| !v.is_empty())
        .or(base_path_ws_streams),
        private_key: None,
    })
}

pub fn get_session_creds(profile: Option<&str>, package_name: &str) -> Option<CliConfiguration> {
    if let Some(profile_name) = profile {
        return get_profile_config(profile_name, package_name);
    }

    if let (Ok(api_key), Ok(api_secret)) =
        (env::var("BINANCE_API_KEY"), env::var("BINANCE_SECRET_KEY"))
    {
        return Some(CliConfiguration {
            api_key,
            api_secret,
            env: env::var("BINANCE_API_ENV").ok(),
            base_path: env::var(format!("BINANCE_{}_BASE_PATH", package_name)).ok(),
            base_path_ws_streams: env::var(format!(
                "BINANCE_{}_WS_STREAMS_BASE_PATH",
                package_name
            ))
            .ok(),
            private_key: None,
        });
    }

    let Some(profile_name) = get_current_profile() else {
        return Some(CliConfiguration {
            api_key: String::new(),
            api_secret: String::new(),
            env: env::var("BINANCE_API_ENV").ok(),
            base_path: env::var(format!("BINANCE_{}_BASE_PATH", package_name)).ok(),
            base_path_ws_streams: env::var(format!(
                "BINANCE_{}_WS_STREAMS_BASE_PATH",
                package_name
            ))
            .ok(),
            private_key: None,
        });
    };

    get_profile_config(&profile_name, package_name)
}

pub fn is_hmac_secret_key(key: &str) -> bool {
    key.len() == 64 && key.chars().all(|c| c.is_ascii_alphanumeric())
}

pub fn get_client_configuration(
    profile: Option<&str>,
    package_name: &str,
) -> Option<CliConfiguration> {
    let mut creds = get_session_creds(profile, package_name)?;

    if !creds.api_secret.is_empty() && is_hmac_secret_key(&creds.api_secret) {
        Some(creds)
    } else {
        creds.private_key = Some(creds.api_secret.clone());
        Some(creds)
    }
}

pub fn is_ai_agent() -> bool {
    std::env::var("AGENT").is_ok()
        || std::env::var("AI_AGENT").is_ok()
        || std::env::var("CLAUDECODE").is_ok()
        || std::env::var("GEMINI_CLI").is_ok()
        || std::env::var("CODEX_SANDBOX").is_ok()
}

pub fn build_user_agent(product: &str) -> String {
    format!(
        "binance-{}/{}/{} (Rust/{}; {}; {})",
        if is_ai_agent() { "skill" } else { "cli" },
        product,
        env!("CARGO_PKG_VERSION"),
        option_env!("RUSTC_VERSION").unwrap_or("unknown"),
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
}

pub fn init_user_agent(product: &str) {
    USER_AGENT_INIT.get_or_init(|| unsafe {
        std::env::set_var(
            "BINANCE_CONNECTOR_RUST_USER_AGENT",
            build_user_agent(product),
        );
    });
}

#[cfg(unix)]
pub async fn wait_for_shutdown() {
    use tokio::signal::unix::{SignalKind, signal};

    let mut sigterm = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            eprintln!("received Ctrl+C");
        }
        _ = sigterm.recv() => {
            eprintln!("received SIGTERM");
        }
    }
}

#[cfg(not(unix))]
pub async fn wait_for_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for Ctrl+C");
}
