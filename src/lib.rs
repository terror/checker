//! A crates.io crate name availability checker
//!
//! ## Installation
//! Simply add `checker = "0.0.1"` to your Cargo.toml
//!
//! ## Example
//! ```
//! use checker::{check, Crate, Status};
//!
//! let result: Crate = check("checker-example").unwrap();
//! assert_eq!(result.name, "checker-example");
//! assert_eq!(result.is_taken(), false);
//! assert!(result.data.is_none());
//! assert!(result.owners.is_none());
//! ```
use reqwest::{blocking, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, Value};
use std::{error, fmt};

static APP_USER_AGENT: &str = "checker (https://crates.io/crates/checker)";
static BASE_URL: &str = "https://crates.io";

#[derive(Debug)]
pub enum Status {
  Free,
  Taken,
  Unknown,
}

impl fmt::Display for Status {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct Crate {
  pub name:   String,
  pub data:   Option<Data>,
  pub owners: Option<Vec<Owner>>,
  pub status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
  pub id:            String,
  pub updated_at:    String,
  pub versions:      Vec<i64>,
  pub description:   String,
  pub created_at:    String,
  pub downloads:     i64,
  pub documentation: Option<String>,
  pub homepage:      Option<String>,
  pub repository:    Option<String>,
  pub links:         Links,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
  owner_user: String,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
  pub id:     i64,
  pub login:  String,
  pub name:   Option<String>,
  pub avatar: String,
  pub url:    String,
}

impl Crate {
  pub fn new(
    name: String,
    data: Option<Data>,
    owners: Option<Vec<Owner>>,
    status: Status,
  ) -> Crate {
    Crate {
      name,
      data,
      owners,
      status,
    }
  }

  pub fn is_taken(&self) -> bool {
    !matches!(self.status, Status::Free)
  }
}

impl fmt::Display for Crate {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.name, self.is_taken())
  }
}

pub fn check(name: &str) -> Result<Crate, Box<dyn error::Error>> {
  let client = blocking::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()?;

  let res = client
    .get(&format!("{}/api/v1/crates/{}", BASE_URL, name))
    .send()?;

  let status = match res.status() {
    StatusCode::OK => Status::Taken,
    StatusCode::NOT_FOUND => Status::Free,
    _ => Status::Unknown,
  };

  let json: Value = from_str(&res.text()?)?;

  let data: Option<Data> = from_value(json["crate"].to_owned())?;

  let owners = find_owners(client, &data)?;

  Ok(Crate::new(name.to_string(), data, owners, status))
}

fn find_owners(
  client: blocking::Client,
  data: &Option<Data>,
) -> Result<Option<Vec<Owner>>, Box<dyn error::Error>> {
  if let Some(data) = data {
    let res = client
      .get(&format!("{}{}", BASE_URL, &data.links.owner_user))
      .send()?;

    let json: Value = from_str(&res.text()?)?;

    let owners: Vec<Owner> = from_value(json["users"].to_owned())?;

    Ok(Some(owners))
  } else {
    Ok(None)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_free_crate() {
    let result: Crate = check("freecratenameyeet").unwrap();
    assert!(result.name == String::from("freecratenameyeet"));
    assert!(result.data.is_none());
    assert!(result.owners.is_none());
  }

  #[test]
  fn test_taken_crate() {
    let result = check("syn").unwrap();
    assert!(result.name == String::from("syn"));
    assert!(result.data.is_some());
    assert!(result.owners.is_some());
  }
}
