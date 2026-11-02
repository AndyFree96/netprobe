mod checker;
mod  cli;

use std::error::Error;
use clap::Parser;

use cli::Cli;
use checker::check;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    print_usage(&cli.url);

    let result = check(&cli.url, cli.expect).await?;

    println!();
    println!("URL:             {}", result.url);
    println!("Expected Status: {}", result.expected_status);
    println!("Actual Status:   {}", result.actual_status);
    println!("Latency:         {:.2}ms", result.latency.as_secs_f64() * 1000.0);

    let status = if result.passed { "PASS" } else { "FAIL" };

    println!("Result:          {status}");

    Ok(())
}

fn print_usage(url: &str) {
    println!("netprobe v0.1.0");
    println!();
    println!("Target : {url}");
}
