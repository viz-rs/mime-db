## mime-db

Media Type Database, looks up `extension` or `media type`.

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
