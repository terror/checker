## checker

[![Build](https://github.com/terror/checker/actions/workflows/ci.yaml/badge.svg?branch=master)](https://github.com/terror/checker/actions/workflows/ci.yaml)
[![crates.io](https://shields.io/crates/v/checker.svg)](https://crates.io/crates/checker)

`checker` is a simple [crates.io](https://crates.io/) crate name availability checker,
it lets you easily check multiple crate names for availability and activity data.

## Demo

[![asciicast](https://asciinema.org/a/U94NQHUnj1DAyZi2VHWCu5GU0.svg)](https://asciinema.org/a/U94NQHUnj1DAyZi2VHWCu5GU0)

## Installation

Simply use cargo to install the binary

```bash
$ cargo install checker
```

## Usage

You can use checker as a command line utility or a library.

### CLI

```
checker 0.0.3
a crates.io crate name availability checker

USAGE:
checker [OPTIONS]

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-c, --check <names>...    Check crate name availability
-o, --output <output>     Output file
```

### Library

Example:

```rust
use checker::{check, Package, Status};

let result: Package = check("t").unwrap();

assert_eq!(result.name, "t");
assert_eq!(result.is_taken(), true);
assert_eq!(result.is_inactive().unwrap(), true);

assert!(result.days_since_last_updated().unwrap() >= 1825);
assert!(result.data.is_some());
assert!(result.owners.is_some());
```
