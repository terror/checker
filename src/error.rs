use crate::common::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Failed to build client: {}", source))]
  Client { source: reqwest::Error },

  #[snafu(display("Failed to issue a GET request to {}: {}", endpoint, source))]
  Get {
    source:   reqwest::Error,
    endpoint: String,
  },

  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },

  #[snafu(context(false), display("Deserialization error: {}", source))]
  Deserialize { source: serde_json::Error },

  #[snafu(context(false), display("Error parsing DateTime: {}", source))]
  DateTimeParse { source: chrono::format::ParseError },
}
