# [recfiles](https://crates.io/crates/recfiles) & [serde_rec](https://crates.io/crates/serde_rec)

Manage GNU Recfiles from rust.

[![recfiles](https://img.shields.io/crates/v/recfiles.svg)](https://crates.io/crates/recfiles) [![serde_rec](https://img.shields.io/crates/v/serde_rec.svg)](https://crates.io/crates/serde_rec)

## Installation

`recfiles` (in-memory recfile handling) & `serde_rec` (data format for serde) on [crates.io](https://crates.io).

## Usage

```rust
use recfiles::Record;
use serde_rec::to_string;

#[derive(Default, Serialize)]
#[serde(rename_all="PascalCase")]
struct Book {
    author: Vec<String>,
    title: String,
    publisher: Option<String>,
}

impl Record for Book {}

let book = Book {
    author: vec![String::from("A.E.J. Eliott, OBE")],
    title: String::from("Thirty Days in the Samarkind Desert with the Duchess of Kent"),
    ..Default::default()
}

let serialised = to_string(&book).unwrap();

assert_eq!(serialised, textwrap::dedent("
    Author: A.E.J. Eliott, OBE
    Title: Thirty Days in the Samarking Desert with the Duchess of Kent

");
```

We can also specify the record descriptor for 'Book':
```rust
let book_descriptor = recfiles::Descriptor {
    name: String::from("Book"),
    key: vec![String::from("Title")],
    allowed: vec![String::from("Publisher"),
}
```

And serialise a full record set:
```rust
let rs = recfiles::RecordSet {
    descriptor: Some(book_descriptor),
    records: vec![book],
}

serde_rec::to_string(&rs);
```

## Work in progress

This is a work in progress, and currently incomplete & poorly documented.

(As of writing, the above's all you're getting :wink:.)

### Coming...

* Deserialisation
* Deriving descriptors
* File (rather than just to/from string) helpers
* In-memory (not shelling out to `recutils`) querying helpers
* Docs!
