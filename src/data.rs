use crate::common::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
  pub id:         String,
  pub updated_at: String,
  pub created_at: String,
  pub repository: Option<String>,
  pub links:      Links,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
  pub owner_user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
  pub login: String,
  pub url:   String,
}

impl Display for Owner {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} - {}", self.login, self.url)
  }
}
