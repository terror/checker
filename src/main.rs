#[macro_use]
extern crate prettytable;

use checker::{check, Crate, Status};
use std::{fs::write, io, path::PathBuf};
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

fn main() -> io::Result<()> {
  let opt = Opt::from_args();

  let mut table = table!(["Name", "Status", "Owner(s)", "Last Updated"]);

  for name in &opt.names {
    let c: Crate = check(&name).unwrap();

    match c.status {
      Status::Free => {
        table.add_row(row![&name, Fg -> c.status.to_string(), "N/A", "N/A"]);
      },
      Status::Taken => {
        let days = &c.days_since_last_updated().unwrap();

        let mut owners = String::new();

        for owner in &c.owners.unwrap() {
          owners.push_str(&format!("{} - {}\n", &owner.login, &owner.url));
        }

        table.add_row(row![
           &name,
           Fr -> c.status.to_string(),
           owners,
           format!("{} day(s) ago", days)
        ]);
      },
      _ => {},
    };
  }

  if let Some(output) = opt.output {
    write(&output, table.to_string())?;
    println!("Exported results to `{}`", &output.display());
  } else {
    table.printstd();
  }

  Ok(())
}
