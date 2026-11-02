mod checker;
mod  cli;

use std::{error::Error, time::Duration};
use clap::Parser;

use cli::Cli;
use checker::{check, CheckError, CheckConfig};
use reqwest::Method;

use crate::cli::HttpMethod;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();
    let method = match cli.method {
        HttpMethod::Get => Method::GET,
        HttpMethod::Post => Method::POST,
    };

    let headers = cli
        .headers
        .iter()
        .map(|header| {
            let (name, value) = header
                .split_once(':')
                .expect("Invalid header format");

            (name.trim().to_string(), value.trim().to_string())
        })
        .collect();

    let config = CheckConfig {
        url: cli.url.clone(),
        method,
        expected_status: cli.expect,
        timeout: Duration::from_secs(cli.timeout),
        headers,
        body: cli.body.clone(),
    };

    match check(&client, &config).await {
        Ok(result) => {
            println!("Status:  {}", result.actual_status);
            println!(
                "Latency: {:.2} ms",
                result.latency.as_secs_f64() * 1000.0
            );

            println!(
                "Result:  {}",
                if result.passed { "PASS" } else { "FAIL" }
            );
        }
        Err(CheckError::Timeout(_)) => {
            println!("Result: TIMEOUT");
            println!(
                "Error: Request timed out after {} seconds",
                cli.timeout
            );
        }
        Err(CheckError::Request(error)) => {
            println!("Result: ERROR");
            println!("Error: {error}");
        }
        Err(CheckError::InvalidHeader(error)) => {
            println!("Result: ERROR");
            println!("Invalid header: {error}");
        }

        Err(CheckError::InvalidJson(error)) => {
            println!("Result: ERROR");
            println!("Invalid JSON: {error}");
        }
    }
    Ok(())
}
