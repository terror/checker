//! A crates.io crate name availability checker
//!
//! ## Installation
//! Simply add `checker = "0.0.3"` to your Cargo.toml
//!
//! ## Example
//! ```
//! use checker::{check, Package, Status};
//!
//! let result: Package = check("t").unwrap();
//!
//! assert_eq!(result.name, "t");
//! assert_eq!(result.is_taken(), true);
//! assert_eq!(result.is_inactive().unwrap(), true);
//!
//! assert!(result.days_since_last_updated().unwrap() >= 1825);
//! assert!(result.data.is_some());
//! assert!(result.owners.is_some());
//! ```

#![allow(dead_code, unused_variables)]

#[macro_use]
extern crate prettytable;

mod check;
mod client;
mod common;
mod data;
mod error;
mod opt;
mod package;
mod status;
mod table;

pub use crate::{check::check, error::Error, package::Package, status::Status};
