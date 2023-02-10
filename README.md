# namewise-rs
[![docs.rs](https://img.shields.io/docsrs/namewise)](https://docs.rs/namewise/)
[![crates.io](https://img.shields.io/crates/v/namewise.svg)](https://crates.io/crates/namewise)
[![Test](https://github.com/amrhassan/namewise-rs/actions/workflows/main.yaml/badge.svg)](https://github.com/amrhassan/namewise-rs/actions/workflows/main.yaml)


Derive trivial transformations between fields that are mostly unpacking and converting
similarly-nemd fields from a source into a target.

## How to use

1. Add to a cargo project:
```
cargo add namewise
```

2. Use to derive trivial conversion impls for you.

```rust
use namewise;
use std::collections::HashSet;

struct Source {
    a: &'static str,
    text: String,
    numeric: i16,
    truth: bool,
    truths: Vec<bool>,
}

#[derive(namewise::From)]
#[namewise_from(from_type = "Source")]
struct Destination {
    a: String,
    text: String,
    #[namewise_from(from_name = "numeric")]
    number: i64,
    #[namewise_from(collect)]
    truths: HashSet<bool>,
}
```

This should be equivalent to:

```rust
use std::collections::HashSet;

struct Source {
    a: &'static str,
    text: String,
    numeric: i16,
    truth: bool,
    truths: Vec<bool>,
}

struct Destination {
    a: String,
    text: String,
    number: i64,
    truths: HashSet<bool>,
}

impl From<Source> for Destination {
    fn from(value: Source) -> Destination {
        Destination {
            a: value.a.into(),
            text: value.text.into(),
            number: value.numeric.into(),
            truths: value.truths.into_iter().collect(),
        }
    }
}
```
