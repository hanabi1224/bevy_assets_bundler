use std::path::PathBuf;

pub(crate) struct ArchivePathInfo {
    path: PathBuf,
}

impl ArchivePathInfo {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}
