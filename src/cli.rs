use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum HttpMethod {
  Get,
  Post
}

#[derive(Debug, Parser)]
#[command(name="netprobe", version, about="A lightweight HTTP endpoint checker")]
pub struct Cli {
  /// Target URL
  pub url: String,

  /// Expected HTTP status code
  #[arg(long, default_value_t = 200)]
  pub expect: u16,

  /// Request timeout in seconds
  #[arg(long, default_value_t = 10)]
  pub timeout: u64,

  /// HTTP request method
  #[arg(long, value_enum, default_value = "get")]
  pub method: HttpMethod,

  /// HTTP request headers, can be specified multiple times
  #[arg(long = "header", action = clap::ArgAction::Append)]
  pub headers: Vec<String>,

  /// JSON request body
  #[arg(long)]
  pub body: Option<String>,
}

