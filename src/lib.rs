#![no_std]

use core::slice;

mod extensions;
mod types;

pub use extensions::EXTENSIONS;
pub use types::TYPES;

pub fn lookup(extension: impl AsRef<str>) -> Option<&'static str> {
    let extension = extension.as_ref();
    if extension.is_empty() {
        return None;
    }

    let extension = extension
        .rfind('.')
        .map_or(extension, |i| &extension[i + 1..]);

    EXTENSIONS
        .iter()
        .find(|(ext, _)| *ext == extension)
        .and_then(|(_, i)| TYPES.get(*i).map(|(kind, _, _)| *kind))
}

pub fn extensions(mime_type: impl AsRef<str>) -> Option<impl Iterator<Item = &'static str>> {
    let iter = extensions2(mime_type);
    if iter.size_hint().0 == 0 {
        None
    } else {
        Some(iter)
    }
}

pub fn extensions2(mime_type: impl AsRef<str>) -> ExtensionsIter {
    let mime_type = mime_type.as_ref();

    if mime_type.is_empty() {
        return ExtensionsIter::default();
    }

    TYPES
        .iter()
        .find(|(kind, _, _)| *kind == mime_type)
        .map_or_else(
            || ExtensionsIter::default(),
            |(_, start, len)| ExtensionsIter {
                inner: EXTENSIONS[*start..][..*len].iter(),
            },
        )
}

#[inline]
pub fn extension(mime_type: impl AsRef<str>) -> Option<&'static str> {
    extensions2(mime_type).next()
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug, Clone)]
pub struct ExtensionsIter {
    // Uses std's implementation of slice Iterator, since it's much more optimized
    // and probably uses unsafe internally to avoid bounds checks.
    inner: slice::Iter<'static, (&'static str, usize)>,
}

impl Default for ExtensionsIter {
    fn default() -> Self {
        // easy way to get an empty &'static slice
        const EMPTY: &[(&str, usize)] = &[];
        Self {
            inner: EMPTY.iter(),
        }
    }
}

impl Iterator for ExtensionsIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(ext, _)| *ext)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.inner.count()
    }
}

#[cfg(test)]
#[test]
fn search() {
    assert_eq!(lookup("json").unwrap(), "application/json");
    assert_eq!(lookup(".md").unwrap(), "text/markdown");
    assert_eq!(lookup("folder/file.js").unwrap(), "application/javascript");
    assert_eq!(lookup("folder/.htaccess"), None);
    assert_eq!(lookup("cats"), None);
    assert_eq!(lookup(""), None);

    assert!(extensions("application/octet-stream").unwrap().eq([
        "bin", "dms", "lrf", "mar", "so", "dist", "distz", "pkg", "bpk", "dump", "elc", "deploy",
        "exe", "dll", "deb", "dmg", "iso", "img", "msi", "msp", "msm", "buffer"
    ]
    .iter()
    .cloned()));
    assert!(extensions("application/cat").is_none());

    assert!(extensions2("application/octet-stream").eq([
        "bin", "dms", "lrf", "mar", "so", "dist", "distz", "pkg", "bpk", "dump", "elc", "deploy",
        "exe", "dll", "deb", "dmg", "iso", "img", "msi", "msp", "msm", "buffer"
    ]
    .iter()
    .cloned()));
    assert!(extensions2("application/cat").next().is_none());
    assert_eq!(extensions2("application/cat").size_hint(), (0, Some(0)));
    assert_eq!(extensions2("application/cat").count(), 0);
    assert_eq!(extensions2("").count(), 0);

    assert_eq!(extension("application/octet-stream").unwrap(), "bin");
}
