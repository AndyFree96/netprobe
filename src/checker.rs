use std::time::{Duration, Instant};

use reqwest::StatusCode;
use tokio::time::{error::Elapsed, timeout};

#[derive(Debug)]
pub struct CheckResult {
  pub url: String,
  pub expected_status: u16,
  pub actual_status: StatusCode,
  pub latency: Duration,
  pub passed: bool
}

#[derive(Debug)]
pub enum CheckError {
    Request(reqwest::Error),
    Timeout(Elapsed)
}

pub async fn check(url: &str, 
  expected_status: u16, 
  timeout_duration: Duration
) -> Result<CheckResult, CheckError> {
  let start = Instant::now();

  let response = timeout(timeout_duration, reqwest::get(url)).await.map_err(CheckError::Timeout)?.map_err(CheckError::Request)?;

  let latency = start.elapsed();

  let actual_status = response.status();

  let passed = actual_status.as_u16() == expected_status;

  Ok(CheckResult {
    url: url.to_string(),
    expected_status,
    actual_status,
    latency,
    passed,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_status_code_match() {
    let actual = StatusCode::OK;
    let expected = 200;

    assert_eq!(actual.as_u16(), expected);
  }

  #[test]
  fn test_status_code_mismatch() {
    let actual = StatusCode::OK;
    let expected = 201;

   assert_ne!(actual.as_u16(), expected); 
  }

  #[test]
  fn test_duration_to_millis() {
    let duration = Duration::from_millis(1500);

    assert_eq!(duration.as_millis(), 1500);
  }

  #[test]
  fn test_duration_to_micros() {
    let duration = Duration::from_millis(1);

    assert_eq!(duration.as_micros(), 1000);
  }

  #[tokio::test]
  async fn test_timeout() {
    let result = timeout(Duration::from_millis(100),
      tokio::time::sleep(Duration::from_secs(1))
    ).await;

    assert!(result.is_err());
  }

  #[tokio::test]
  async fn test_no_timeout() {
      let result = timeout(
          Duration::from_secs(2),
          tokio::time::sleep(Duration::from_millis(100)),
      )
      .await;

      assert!(result.is_ok());
  }
}