<h1 align="center">mime-db</h1>

<div align="center">
  <p><strong>Media Type Database, looks up `extension` or `media type`</strong>
  </p>
  <p> Synced with
    <a href="https://github.com/jshttp/mime-db">
      <img alt="jshttp/mime-db" src="https://img.shields.io/npm/v/mime-db/latest?style=flat-square&label=jshttp%2Fmime-db"></a>
  </p>
</div>

<div align="center">
  <!-- docs.rs docs -->
  <a href="https://docs.rs/mime-db">
    <img src="https://img.shields.io/badge/mime-db.svg?style=flat-square"
      alt="docs.rs docs" /></a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/mime-db">
    <img src="https://img.shields.io/crates/v/mime-db.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/mime-db">
    <img src="https://img.shields.io/crates/d/mime-db.svg?style=flat-square"
      alt="Download" /></a>
</div>

## Installation

```shell
cargo add mime-db
```

## Usage

```rust
use mime_db::{extension, extensions, lookup};

assert_eq!(lookup("json").unwrap(), "application/json");
assert_eq!(lookup(".md").unwrap(), "text/markdown");
assert_eq!(lookup("folder/file.js").unwrap(), "application/javascript");
assert_eq!(lookup("folder/.htaccess"), None);
assert_eq!(lookup("cats"), None);

assert!(extensions("application/octet-stream").unwrap().eq([
    "bin", "dms", "lrf", "mar", "so", "dist", "distz", "pkg", "bpk", "dump", "elc",
    "deploy", "exe", "dll", "deb", "dmg", "iso", "img", "msi", "msp", "msm", "buffer"
]
.iter()
.cloned()));

assert_eq!(extension("application/octet-stream").unwrap(), "bin");
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
