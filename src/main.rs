use crate::common::*;

mod check;
mod client;
mod common;
mod data;
mod error;
mod opt;
mod package;
mod status;
mod table;

#[macro_use]
extern crate prettytable;

fn main() {
  match Opt::from_args().run() {
    Ok(()) => {}
    Err(e) => eprintln!("{}", e),
  }
}
