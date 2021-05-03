use checker::{check, Status};
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

fn main() {
  let opt = Opt::from_args();

  match check(&opt.check).unwrap() {
    Status::Free => println!("Free."),
    Status::Taken => println!("Taken."),
    Status::Unknown => println!("An unknown error has occured."),
  };
}
