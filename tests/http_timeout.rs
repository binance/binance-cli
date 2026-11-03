//! Verify REST product and custom request timeouts against a slow local server.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::{Command, Output, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

/// Run the CLI against a local server that responds after 1200 milliseconds.
///
/// Args:
/// - product: spot runs the public ping command; request runs a custom public GET.
/// - timeout: The subprocess timeout variable; None leaves it unset.
///
/// Returns:
/// - The CLI exit status, stdout and stderr; no real account credentials are used.
///
/// Examples:
/// - slow_request("spot", None) succeeds and returns JSON {}.
/// - slow_request("request", Some("100")) fails with a request timeout.
fn slow_request(product: &str, timeout: Option<&str>) -> Output {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    listener.set_nonblocking(true).unwrap();
    let finished = Arc::new(AtomicBool::new(false));
    let server_finished = Arc::clone(&finished);
    let server = thread::spawn(move || {
        let mut handlers = Vec::new();
        while !server_finished.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => handlers.push(thread::spawn(move || {
                    stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
                    let mut request = [0; 8192];
                    // A timed-out retry may disconnect before sending its HTTP request.
                    match stream.read(&mut request) {
                        Ok(0) | Err(_) => return,
                        Ok(_) => {}
                    }
                    thread::sleep(Duration::from_millis(1200));
                    // Keep accepting SDK retries; writes to timed-out clients may fail.
                    let _ = stream.write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
                    );
                })),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("Local test server failed to accept a connection: {error}"),
            }
        }
        for handler in handlers {
            handler.join().unwrap();
        }
    });
    let mut command = Command::new(env!("CARGO_BIN_EXE_binance-cli"));
    command
        .stdin(Stdio::null())
        .env("BINANCE_API_KEY", "")
        .env("BINANCE_SECRET_KEY", "")
        .env("BINANCE_API_ENV", "prod")
        .env("NO_PROXY", "127.0.0.1")
        .env("no_proxy", "127.0.0.1")
        .env("BINANCE_spot_BASE_PATH", &base_url)
        .env("BINANCE_SPOT_BASE_PATH", &base_url)
        .env_remove("BINANCE_HTTP_TIMEOUT_MS");
    if let Some(timeout) = timeout {
        command.env("BINANCE_HTTP_TIMEOUT_MS", timeout);
    }
    if product == "spot" {
        command.args(["spot", "ping"]);
    } else {
        command.args(["request", "GET", &base_url]);
    }
    let output = command.output().unwrap();
    finished.store(true, Ordering::Relaxed);
    server.join().unwrap();
    output
}

/// Verify the default allows responses slower than the SDK one-second deadline.
/// Args: None.
/// Returns: Nothing; both commands must successfully return JSON {}.
/// Examples: spot ping and request GET both succeed after a 1200 ms response delay.
#[test]
fn default_timeout_allows_slow_responses() {
    for product in ["spot", "request"] {
        let output = slow_request(product, None);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(),
            serde_json::json!({})
        );
    }
}

/// Verify the configured timeout reaches the HTTP client.
/// Args: None.
/// Returns: Nothing; a 2000 ms timeout succeeds and a 100 ms timeout fails.
/// Examples: The same 1200 ms server succeeds or times out according to the setting.
#[test]
fn configured_timeout_controls_requests() {
    for product in ["spot", "request"] {
        assert!(slow_request(product, Some("2000")).status.success());
        let output = slow_request(product, Some("100"));
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("timed out"),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Verify invalid configuration is rejected before a network request.
/// Args: None.
/// Returns: Nothing; the failure must identify the timeout configuration.
/// Examples: BINANCE_HTTP_TIMEOUT_MS=0 fails before connecting to local port 1.
#[test]
fn invalid_timeout_fails_before_request() {
    for value in ["0", "-1", "", "abc", "18446744073709551616"] {
        let output = Command::new(env!("CARGO_BIN_EXE_binance-cli"))
            .args(["request", "GET", "http://127.0.0.1:1"])
            .env("BINANCE_API_KEY", "")
            .env("BINANCE_SECRET_KEY", "")
            .env("BINANCE_HTTP_TIMEOUT_MS", value)
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("BINANCE_HTTP_TIMEOUT_MS"));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("HTTP request failed"));
    }
}
