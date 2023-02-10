# namewise-rs
Derive trivial transformations between fields that are mostly unpacking and converting
similarly-nemd fields from a source into a target.

Example usage:

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
