<h1 align="center">mime-db</h1>
<div align="center">
  <strong>
    Media Type Database, looks up `extension` or `media type`.
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/mime-db">
    <img src="https://img.shields.io/crates/v/mime-db.svg"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/mime-db">
    <img src="https://img.shields.io/crates/d/mime-db.svg"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/mime-db">
    <img src="https://img.shields.io/badge/mime-db.svg"
      alt="docs.rs docs" />
  </a>
  <!-- CI status -->
  <a href="https://github.com/trek-rs/mime-db/actions">
    <img src="https://github.com/trek-rs/mime-db/workflows/CI/badge.svg"
      alt="CI Status" />
  </a>
</div>

### Installation

```shell
cargo add mime-db
```

### Usage

```rust
use mime_db::{extension, extensions, lookup};

assert_eq!(lookup("json").unwrap(), "application/json");
assert_eq!(lookup(".md").unwrap(), "text/markdown");
assert_eq!(lookup("folder/file.js").unwrap(), "application/javascript");
assert_eq!(lookup("folder/.htaccess"), None);
assert_eq!(lookup("cats"), None);

assert_eq!(
    extensions("application/octet-stream").unwrap(),
    vec![
        "bin", "dms", "lrf", "mar", "so", "dist", "distz", "pkg", "bpk", "dump", "elc",
        "deploy", "exe", "dll", "deb", "dmg", "iso", "img", "msi", "msp", "msm", "buffer"
    ]
);

assert_eq!(extension("application/octet-stream").unwrap(), "bin");

```

### License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
