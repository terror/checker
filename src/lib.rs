use reqwest::{blocking, StatusCode, Url};
use std::error;

static APP_USER_AGENT: &str = "checker (https://crates.io/crates/checker)";

pub enum Status {
  Free,
  Taken,
  Unknown,
}

pub fn check(name: &str) -> Result<Status, Box<dyn error::Error>> {
  let client = blocking::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()?;

  let url = Url::parse(&format!("https://crates.io/api/v1/crates/{}", name))?;

  let resp = client.get(url).send()?;

  let status = match resp.status() {
    StatusCode::OK => Status::Taken,
    StatusCode::NOT_FOUND => Status::Free,
    _ => Status::Unknown,
  };

  Ok(status)
}
