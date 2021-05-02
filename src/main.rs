use reqwest::{blocking, StatusCode, Url};
use std::error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
  name = "checker",
  about = "a crates.io crate name availability checker"
)]
struct Opt {
  #[structopt(short = "c", long = "check", help = "Check crate name availability")]
  check: String,
}

enum Status {
  Free,
  Taken,
  Unknown,
}

fn check(name: &str) -> Result<Status, Box<dyn error::Error>> {
  let url = Url::parse(&format!("https://crates.io/api/v1/crates/{}", name))?;

  let resp = blocking::get(url)?;

  println!("{}", resp.status());

  let status = match resp.status() {
    StatusCode::OK => Status::Taken,
    StatusCode::NOT_FOUND => Status::Free,
    _ => Status::Unknown,
  };

  Ok(status)
}

fn main() {
  let opt = Opt::from_args();

  match check(&opt.check).unwrap() {
    Status::Free => println!("Free."),
    Status::Taken => println!("Taken."),
    Status::Unknown => println!("An unknown error has occured."),
  };
}
