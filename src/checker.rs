use std::time::{Duration, Instant};

use reqwest::{Client, Method, StatusCode, header::{HeaderName, HeaderValue}};
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
    Timeout(Elapsed),
    InvalidHeader(String),
    InvalidJson(serde_json::Error)
}


#[derive(Debug)]
pub struct CheckConfig {
  pub url: String,
  pub method: Method,
  pub expected_status: u16,
  pub timeout: Duration,
  pub headers: Vec<(String, String)>,
  pub body: Option<String>,
}

pub async fn check(
    client: &Client,
    config: &CheckConfig,
) -> Result<CheckResult, CheckError> {
    let start = Instant::now();

    let mut request = client.request(
        config.method.clone(),
        &config.url,
    );

    // 添加自定义请求头
    for (name, value) in &config.headers {
        let name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|error| {
                CheckError::InvalidHeader(error.to_string())
            })?;

        let value = HeaderValue::from_str(value)
            .map_err(|error| {
                CheckError::InvalidHeader(error.to_string())
            })?;

        request = request.header(name, value);
    }

    // 添加 JSON 请求体
    if let Some(body) = &config.body {
        let json: serde_json::Value =
            serde_json::from_str(body)
                .map_err(CheckError::InvalidJson)?;

        request = request.json(&json);
    }

    // 发送请求，并执行超时控制
    let response = timeout(
        config.timeout,
        request.send(),
    )
    .await
    .map_err(CheckError::Timeout)?
    .map_err(CheckError::Request)?;

    let latency = start.elapsed();

    let actual_status = response.status();

    let passed = actual_status.as_u16()
        == config.expected_status;

    Ok(CheckResult {
        url: config.url.clone(),
        expected_status: config.expected_status,
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

  #[test]
  fn test_parse_valid_json() {
    let body = r#"{"name":"Rust","version":"1.0"}"#;

    let result: Result<serde_json::Value, _> =
        serde_json::from_str(body);

    assert!(result.is_ok());
  }

  #[test]
  fn test_parse_invalid_json() {
      let body = r#"{"name":"Rust""#;

      let result: Result<serde_json::Value, _> =
          serde_json::from_str(body);

      assert!(result.is_err());
  }
}