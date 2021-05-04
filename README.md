## <p align='center'>checker</p>

<p align='center'>
  a crates.io crate name availability checker

  <br/>
  <br/>

  <a href="https://crates.io/crates/checker" target="_blank">
    <img src="https://shields.io/crates/v/checker.svg">
  </a>

  <a href="https://github.com/terror/checker/blob/main/.github/workflows/ci.yml" target="_blank">
    <img src="https://github.com/terror/checker/actions/workflows/ci.yml/badge.svg">
  </a>
</p>

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
checker 0.0.2
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
use checker::{check, Crate, Status};

fn main() {
  let result: Crate = check("t").unwrap();

  assert_eq!(result.name, "t");
  assert_eq!(result.is_taken(), true);
  assert_eq!(result.is_inactive().unwrap(), true);

  assert!(result.days_since_last_updated().unwrap() >= 1825);
  assert!(result.data.is_some());
  assert!(result.owners.is_some());
}
```
