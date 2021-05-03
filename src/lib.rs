use reqwest::{blocking, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, Value};
use std::{error, fmt};

static APP_USER_AGENT: &str = "checker (https://crates.io/crates/checker)";
static BASE_URL: &str = "https://crates.io";

pub enum Status {
  Free,
  Taken,
  Unknown,
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
  pub keywords:      Vec<String>,
  pub badges:        Vec<String>,
  pub created_at:    String,
  pub downloads:     i64,
  pub documentation: Option<String>,
  pub homepage:      Option<String>,
  pub repository:    Option<String>,
  pub links:         Links,
  pub exact_match:   bool,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
  version_downloads:    String,
  versions:             Option<String>,
  owners:               String,
  owner_team:           String,
  owner_user:           String,
  reverse_dependencies: String,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
  pub id:     i64,
  pub login:  String,
  pub name:   String,
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
    match self.status {
      Status::Free => false,
      _ => true,
    }
  }

  pub fn is_inactive(&self) -> bool {
    if let Some(data) = &self.data {
      println!("{}", data.created_at);
    }
    true
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
