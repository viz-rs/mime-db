mod extensions;
mod types;

pub use extensions::EXTENSIONS;
pub use types::TYPES;

const DOT: &str = ".";

pub fn lookup(ext: impl AsRef<str>) -> Option<&'static str> {
    let ext = &(DOT.to_owned() + &ext.as_ref().to_lowercase())
        .rsplitn(2, DOT)
        .collect::<Vec<&str>>()[0]
        .to_owned();

    if let Some((_, i)) = EXTENSIONS.iter().find(|(e, _)| *e == ext) {
        if let Some(kind) = TYPES.get(*i) {
            return Some(kind.0);
        }
    }

    None
}

pub fn extensions(t: impl AsRef<str>) -> Option<Vec<&'static str>> {
    let t = t.as_ref();

    if let Some((_, s, e)) = TYPES.iter().find(|(e, _, _)| *e == t) {
        return Some((&EXTENSIONS[*s..(s + e)]).iter().map(|(e, _)| *e).collect());
    }

    None
}

pub fn extension(t: impl AsRef<str>) -> Option<&'static str> {
    if let Some(exts) = extensions(t) {
        return Some(*exts.first().unwrap());
    }

    None
}

#[cfg(test)]
#[test]
fn search() {
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
}
