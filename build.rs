use async_std::fs::File;
use async_std::io::BufWriter;
use async_std::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Source: https://raw.githubusercontent.com/jshttp/mime-db/master/db.json
const DB_URL: &str = "https://unpkg.com/mime-db@1.42.0/db.json";

const TPL_E: &str = r#"pub const {{name}}: [(&str, usize); {{len}}] = [{{items}}];"#;
const TPL_T: &str = r#"pub const {{name}}: [(&str, usize, usize); {{len}}] = [{{items}}];"#;

#[derive(Debug, Deserialize, Serialize)]
struct Kind {
    source: Option<String>,
    compressible: Option<bool>,
    extensions: Option<Vec<String>>,
}

type Kinds = BTreeMap<String, Kind>;

#[async_std::main]
async fn main() -> Result<(), surf::Exception> {
    let body: Kinds = surf::get(DB_URL).await?.body_json().await?;
    let db: Kinds = body
        .into_iter()
        .filter(|(_k, v)| v.extensions.is_some())
        .collect();

    let types_file = File::create("src/types.rs").await?;
    let exts_file = File::create("src/extensions.rs").await?;
    {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        let mut n: usize = 0;
        for (key, value) in db.iter() {
            let extensions: Vec<(String, usize)> = value
                .extensions
                .as_ref()
                .unwrap()
                .iter()
                .map(|e| (e.to_owned(), n))
                .collect();

            if key == "application/octet-stream" {
                dbg!(&extensions);
            }

            // extensions.sort_by_key(|e| e.0.to_owned());

            keys.push((key, values.len(), extensions.len()));
            values.append(&mut extensions.clone());
            n += 1;
        }

        let mut exts_writer = BufWriter::new(exts_file);
        let exts: Vec<String> = values
            .iter()
            .map(|e| {
                r#"("{{ext}}", {{pos}})"#
                    .replace("{{ext}}", &e.0)
                    .replace("{{pos}}", &e.1.to_string())
            })
            .collect();
        exts_writer
            .write(
                TPL_E
                    .replace("{{name}}", "EXTENSIONS")
                    .replace("{{len}}", &exts.len().to_string())
                    .replace("{{items}}", &exts.join(r#", "#))
                    .as_bytes(),
            )
            .await?;
        exts_writer.flush().await?;

        let mut types_writer = BufWriter::new(types_file);
        let types: Vec<String> = keys
            .iter()
            .map(|e| {
                r#"("{{ext}}", {{start}}, {{end}})"#
                    .replace("{{ext}}", &e.0)
                    .replace("{{start}}", &e.1.to_string())
                    .replace("{{end}}", &e.2.to_string())
            })
            .collect();
        types_writer
            .write(
                TPL_T
                    .replace("{{name}}", "TYPES")
                    .replace("{{len}}", &types.len().to_string())
                    .replace("{{items}}", &types.join(r#", "#))
                    .as_bytes(),
            )
            .await?;
        types_writer.flush().await?;
    }

    Ok(())
}
