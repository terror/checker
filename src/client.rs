use crate::common::*;

const USER_AGENT: &str = "checker (https://crates.io/crates/checker)";
const BASE_URL: &str = "https://crates.io";

#[derive(Debug)]
pub struct Client<'a> {
  base_url:       &'a str,
  request_client: blocking::Client,
}

impl<'a> Client<'a> {
  pub fn new() -> Result<Self> {
    Ok(Self {
      base_url:       BASE_URL,
      request_client: blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .context(error::Client)?,
    })
  }

  pub fn get(&self, endpoint: &str) -> Result<blocking::Response> {
    let endpoint = format!("{}{}", self.base_url, endpoint);

    self
      .request_client
      .get(endpoint.clone())
      .send()
      .context(error::Get { endpoint })
  }
}
