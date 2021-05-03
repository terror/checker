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

You can use checker as a command line utility or a rust crate.

### CLI

```
checker 0.0.1
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
use checker::{check, Crate};

fn main() {
  let result: Crate = check("checker-example").unwrap();
  assert_eq!(result.name, "checker-example");
  assert_eq!(result.is_taken(), false);
  assert!(result.data.is_none());
  assert!(result.owners.is_none());
}
```
