use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="netprobe", version, about="A lightweight HTTP endpoint checker")]
pub struct Cli {
  /// Target URL
  pub url: String,

  /// Expected HTTP status code
  #[arg(long, default_value_t = 200)]
  pub expect: u16,
}

