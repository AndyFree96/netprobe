mod checker;
mod  cli;

use std::{error::Error, time::Duration};
use clap::Parser;

use cli::Cli;
use checker::{check, CheckError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    print_usage(&cli.url);

    let time_duration = Duration::from_secs(cli.timeout);

    match check(&cli.url, cli.expect, time_duration).await {
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
    }
    Ok(())
}

fn print_usage(url: &str) {
    println!("netprobe v0.1.0");
    println!();
    println!("Target : {url}");
}
