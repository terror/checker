use crate::common::*;

#[derive(Debug)]
pub enum Status {
  Free,
  Taken,
  Unknown,
}

impl Display for Status {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Status::Free => write!(f, "Free"),
      Status::Taken => write!(f, "Taken"),
      _ => write!(f, "Unknwon"),
    }
  }
}

impl From<StatusCode> for Status {
  fn from(status_code: StatusCode) -> Self {
    match status_code {
      StatusCode::OK => Self::Taken,
      StatusCode::NOT_FOUND => Self::Free,
      _ => Self::Unknown,
    }
  }
}
