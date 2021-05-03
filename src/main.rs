#[macro_use]
extern crate prettytable;

use checker::{check, Status};
use chrono::NaiveDateTime;
use colored::*;
use std::{error, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
  name = "checker",
  about = "a crates.io crate name availability checker"
)]
struct Opt {
  /// Crate name(s) or input file
  #[structopt(short = "c", long = "check", help = "Check crate name availability")]
  names: Vec<String>,

  /// Output file, defaults to stdout
  #[structopt(short = "o", long = "output", help = "Output file")]
  output: Option<PathBuf>,
}

fn main() {
  let opt = Opt::from_args();

  let mut table = table!(["Name", "Status", "Owner(s)", "Last Updated"]);

  for name in &opt.names {
    let c = check(&name).unwrap();

    match c.status {
      Status::Free => {
        table.add_row(row![&name, "Free".green(), "N/A", "N/A"]);
      },
      Status::Taken => {
        let mut owners = String::new();

        for owner in &c.owners.unwrap() {
          owners.push_str(&format!("{} - {}\n", &owner.login, &owner.url));
        }

        let data = c.data.unwrap();

        let updated_at = updated_duration_since(data.updated_at).unwrap();

        table.add_row(row![
          &name,
          "Taken".red(),
          owners,
          format!("{} day(s) ago", updated_at)
        ]);
      },
      _ => {},
    };
  }

  table.printstd();
}

fn updated_duration_since(updated_at: String) -> Result<i64, Box<dyn error::Error>> {
  let last_updated_at = NaiveDateTime::parse_from_str(&updated_at, "%Y-%m-%dT%H:%M:%S%Z")?;

  let now = chrono::Utc::now().naive_utc();

  let diff = now.signed_duration_since(last_updated_at);

  Ok(diff.num_days())
}
