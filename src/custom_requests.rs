use anyhow::{Context, Result, anyhow};
use binance_sdk::common::utils::send_request;
use binance_sdk::config::{ConfigurationRestApi, ConfigurationRestApiBuilder, PrivateKey};
use binance_sdk::models::{RestApiResponse, TimeUnit};
use clap::{Args, Parser};
use reqwest::Method;
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::{self, IsTerminal, Read};
use url::Url;

use crate::utils::{CliConfiguration, get_configuration_rest_api};

#[derive(Args, Debug)]
pub struct CustomRequestCommand {
    /// HTTP Method: GET | POST | PUT | DELETE
    pub method: String,

    /// Request URL, e.g. https://api.binance.com/api/v3/exchangeInfo
    pub url: String,

    /// For signed endpoints
    #[arg(long)]
    pub signed: bool,

    /// Time Unit: MILLISECOND or MICROSECOND
    #[arg(long = "time-unit", default_value = "MILLISECOND", value_parser = parse_time_unit)]
    pub time_unit: TimeUnit,

    /// Optional profile name
    #[arg(long)]
    pub profile: Option<String>,

    /// Arbitrary request params.
    ///
    /// Supports:
    /// --symbol BTCUSDT
    /// --limit 10
    /// --recvWindow=5000
    /// --test
    ///
    /// Important:
    /// Put known CLI options like --signed, --profile, --time-unit before
    /// arbitrary request params.
    #[arg(
        num_args = 0..,
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    pub request_params: Vec<String>,
}

impl TryFrom<CliConfiguration> for ConfigurationRestApi {
    type Error = anyhow::Error;

    fn try_from(cli: CliConfiguration) -> Result<Self> {
        let base_path = match cli.base_path {
            Some(base_path) => Some(base_path),
            None => cli
                .env
                .as_deref()
                .and_then(env_to_base_path)
                .map(String::from),
        };

        let private_key = match cli.private_key {
            Some(private_key_string) => Some(parse_private_key(private_key_string)?),
            None => None,
        };

        let mut builder = ConfigurationRestApiBuilder::default();

        if !cli.api_key.is_empty() {
            builder = builder.api_key(cli.api_key);
        }

        if !cli.api_secret.is_empty() {
            builder = builder.api_secret(cli.api_secret);
        }

        if let Some(base_path) = base_path {
            builder = builder.base_path(base_path);
        }

        if let Some(private_key) = private_key {
            builder = builder.private_key(private_key);
        }

        let configuration = builder
            .build()
            .map_err(|err| anyhow!("Failed to build ConfigurationRestApi: {err}"))?;

        Ok(configuration)
    }
}

fn env_to_base_path(env: &str) -> Option<&'static str> {
    match env.to_lowercase().as_str() {
        "prod" | "production" => Some("https://api.binance.com"),
        "testnet" | "spot_testnet" | "spot-testnet" => Some("https://testnet.binance.vision"),
        "us" | "binance_us" | "binance-us" => Some("https://api.binance.us"),
        _ => None,
    }
}

fn parse_private_key(private_key_string: String) -> Result<PrivateKey> {
    Ok(PrivateKey::File(private_key_string))
}

pub async fn handle_custom_request(cmd: CustomRequestCommand) -> Result<()> {
    let method = parse_method(&cmd.method)?;

    let url = Url::parse(&cmd.url).with_context(|| format!("Invalid URL: {}", cmd.url))?;

    let endpoint = url.path();

    let base_path = build_base_path(&url)?;

    let stdin_obj = read_stdin_json_object()?;

    let mut request_params = BTreeMap::<String, Value>::new();

    if !stdin_obj.is_empty() {
        // Same behavior as the TS version:
        //
        // if stdin has a non-empty JSON object, use it as request params.
        request_params.extend(stdin_obj);
    } else {
        // Include query params already present in the URL:
        //
        // https://api.binance.com/api/v3/depth?symbol=BTCUSDT&limit=10
        for (key, value) in url.query_pairs() {
            request_params.insert(key.to_string(), parse_jsonish_value(&value));
        }

        // Include arbitrary CLI params:
        //
        // --symbol BTCUSDT --limit 10 --recvWindow=5000
        let cli_params = parse_request_params(&cmd.request_params)?;
        request_params.extend(cli_params);
    }

    let mut configuration = get_configuration_rest_api(cmd.profile.as_deref(), "custom")
        .ok_or("Invalid Configuration")
        .unwrap();

    configuration.base_path = Some(base_path);

    let body_params = BTreeMap::<String, Value>::new();

    let response: RestApiResponse<Value> = send_request::<Value>(
        &ConfigurationRestApi::try_from(configuration)?,
        endpoint,
        method,
        request_params,
        body_params,
        Some(cmd.time_unit),
        cmd.signed,
    )
    .await?;

    let data = response.data().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    Ok(())
}

fn parse_method(method: &str) -> Result<Method> {
    match method.to_uppercase().as_str() {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PUT" => Ok(Method::PUT),
        "DELETE" => Ok(Method::DELETE),
        other => Err(anyhow!("{other} is not a valid HTTP method.")),
    }
}

fn parse_time_unit(value: &str) -> std::result::Result<TimeUnit, String> {
    match value.to_uppercase().as_str() {
        "MILLISECOND" | "MILLISECONDS" => Ok(TimeUnit::Millisecond),
        "MICROSECOND" | "MICROSECONDS" => Ok(TimeUnit::Microsecond),
        other => Err(format!(
            "{other} is not a valid time unit. Use MILLISECOND or MICROSECOND."
        )),
    }
}

fn build_base_path(url: &Url) -> Result<String> {
    let host = url.host_str().ok_or_else(|| anyhow!("URL missing host"))?;

    let base_path = match url.port() {
        Some(port) => format!("{}://{}:{}", url.scheme(), host, port),
        None => format!("{}://{}", url.scheme(), host),
    };

    Ok(base_path)
}

fn read_stdin_json_object() -> Result<BTreeMap<String, Value>> {
    if io::stdin().is_terminal() {
        return Ok(BTreeMap::new());
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read stdin")?;

    let input = input.trim();

    if input.is_empty() {
        return Ok(BTreeMap::new());
    }

    let value: Value = serde_json::from_str(input).context("Failed to parse stdin as JSON")?;

    match value {
        Value::Object(map) => Ok(map.into_iter().collect()),
        _ => Err(anyhow!("stdin JSON must be an object")),
    }
}

fn parse_request_params(args: &[String]) -> Result<BTreeMap<String, Value>> {
    let mut params = BTreeMap::<String, Value>::new();

    let mut i = 0;

    while i < args.len() {
        let current = &args[i];

        if !current.starts_with("--") {
            return Err(anyhow!(
                "Unexpected argument `{}`. Expected --key value or --key=value.",
                current
            ));
        }

        let without_prefix = &current[2..];

        if without_prefix.is_empty() {
            return Err(anyhow!("Invalid empty parameter name"));
        }

        // Supports --key=value
        if let Some((key, raw_value)) = without_prefix.split_once('=') {
            if key.trim().is_empty() {
                return Err(anyhow!("Invalid empty parameter name"));
            }

            params.insert(key.to_string(), parse_jsonish_value(raw_value));
            i += 1;
            continue;
        }

        let key = without_prefix;

        // Supports boolean flag style:
        //
        // --test
        //
        // If the next item is missing or starts with --, treat this as true.
        if i + 1 >= args.len() || args[i + 1].starts_with("--") {
            params.insert(key.to_string(), Value::Bool(true));
            i += 1;
            continue;
        }

        // Supports --key value
        let raw_value = &args[i + 1];

        params.insert(key.to_string(), parse_jsonish_value(raw_value));

        i += 2;
    }

    Ok(params)
}

fn parse_jsonish_value(raw: &str) -> Value {
    serde_json::from_str::<Value>(raw).unwrap_or_else(|_| Value::String(raw.to_string()))
}
