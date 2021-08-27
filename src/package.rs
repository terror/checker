use crate::common::*;

#[derive(Debug)]
pub struct Package {
  pub name:   String,
  pub data:   Option<Data>,
  pub owners: Option<Vec<Owner>>,
  pub status: Status,
}

impl Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Name: {}, Taken: {}", self.name, self.is_taken())
  }
}

impl Package {
  pub fn new(name: String, data: Option<Data>, owners: Option<Vec<Owner>>, status: Status) -> Self {
    Package {
      name,
      data,
      owners,
      status,
    }
  }

  pub fn days_since_last_updated(&self) -> Result<i64> {
    if let Some(data) = &self.data {
      return Ok(
        chrono::Utc::now()
          .naive_utc()
          .signed_duration_since(NaiveDateTime::parse_from_str(
            &data.updated_at,
            "%Y-%m-%dT%H:%M:%S%Z",
          )?)
          .num_days(),
      );
    }
    Ok(-1)
  }

  pub fn is_taken(&self) -> bool {
    !matches!(self.status, Status::Free)
  }

  pub fn is_inactive(&self) -> Result<bool> {
    if matches!(self.status, Status::Free) {
      return Ok(true);
    }
    Ok(self.days_since_last_updated()? >= 1825)
  }
}
