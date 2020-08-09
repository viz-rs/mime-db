#![no_std]

mod extensions;
mod types;

pub use extensions::EXTENSIONS;
pub use types::TYPES;

pub fn lookup(extension: impl AsRef<str>) -> Option<&'static str> {
    let extension = extension.as_ref();
    let extension = extension
        .rfind('.')
        .map_or(extension, |i| &extension[i + 1..]);

    EXTENSIONS
        .iter()
        .find(|(ext, _)| *ext == extension)
        .and_then(|(_, i)| TYPES.get(*i).map(|(kind, _, _)| *kind))
}

pub fn extensions(mime_type: impl AsRef<str>) -> Option<impl Iterator<Item = &'static str>> {
    let mime_type = mime_type.as_ref();

    TYPES
        .iter()
        .find(|(kind, _, _)| *kind == mime_type)
        .map(|(_, start, len)| (&EXTENSIONS[*start..][..*len]).iter().map(|(ext, _)| *ext))
}

pub fn extension(mime_type: impl AsRef<str>) -> Option<&'static str> {
    extensions(mime_type).and_then(|mut exts| exts.next())
}

#[cfg(test)]
#[test]
fn search() {
    assert_eq!(lookup("json").unwrap(), "application/json");
    assert_eq!(lookup(".md").unwrap(), "text/markdown");
    assert_eq!(lookup("folder/file.js").unwrap(), "application/javascript");
    assert_eq!(lookup("folder/.htaccess"), None);
    assert_eq!(lookup("cats"), None);

    assert!(extensions("application/octet-stream").unwrap().eq([
        "bin", "dms", "lrf", "mar", "so", "dist", "distz", "pkg", "bpk", "dump", "elc", "deploy",
        "exe", "dll", "deb", "dmg", "iso", "img", "msi", "msp", "msm", "buffer"
    ]
    .iter()
    .cloned()));

    assert_eq!(extension("application/octet-stream").unwrap(), "bin");
}
