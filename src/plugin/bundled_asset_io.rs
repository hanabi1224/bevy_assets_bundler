use super::{path_info::ArchivePathInfo, BundledAssetIoOptions};
use bevy_asset::{AssetIo, AssetIoError};
use bevy_utils::BoxedFuture;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use tar::Archive;

pub struct BundledAssetIo {
    options: BundledAssetIoOptions,
    parent_dir_to_path_info: Option<Arc<RwLock<HashMap<PathBuf, Vec<ArchivePathInfo>>>>>,
}

impl Default for BundledAssetIo {
    fn default() -> Self {
        Self {
            options: BundledAssetIoOptions::default(),
            parent_dir_to_path_info: None,
        }
    }
}

impl From<BundledAssetIoOptions> for BundledAssetIo {
    fn from(options: BundledAssetIoOptions) -> Self {
        Self {
            options,
            parent_dir_to_path_info: None,
        }
    }
}

impl BundledAssetIo {
    pub fn ensure_loaded(&mut self) -> anyhow::Result<()> {
        if self.parent_dir_to_path_info.is_none() {
            let bundle_path = self.get_bundle_path()?;
            info!("Loading asset bundle: {:?}", bundle_path);
            let file = File::open(bundle_path)?;
            let mut archive = Archive::new(file);
            let mut mappings: HashMap<PathBuf, Vec<ArchivePathInfo>> = HashMap::new();
            let mut n_entries = 0;
            for entry in archive.entries()? {
                if entry.is_ok() {
                    n_entries += 1;
                    let entry_file = entry?;
                    let path = entry_file.path()?;
                    // let is_dir = path.is_dir();
                    let mut parent_dir = path.to_path_buf();
                    if parent_dir.pop() {
                        // parent_dir = parent_dir.canonicalize();
                    } else {
                        parent_dir = PathBuf::from("");
                    }
                    debug!("Loading asset file {:?}, dir:{:?}", path, parent_dir);
                    let path_info = ArchivePathInfo::new(path.to_path_buf());
                    if let Some(vec) = mappings.get_mut(&parent_dir) {
                        vec.push(path_info);
                    } else {
                        let mut vec = Vec::new();
                        vec.push(path_info);
                        mappings.insert(parent_dir, vec);
                    }
                }
            }
            info!("{} asset files loaded.", n_entries);
            self.parent_dir_to_path_info = Some(Arc::new(RwLock::new(mappings)));
            Ok(())
        } else {
            Err(anyhow::Error::msg("Entity file is not found"))
        }
    }

    fn get_bundle_path(&self) -> anyhow::Result<PathBuf, AssetIoError> {
        let mut bundle_path = env::current_exe().map_err(|err| AssetIoError::Io(err))?;
        bundle_path.pop();
        bundle_path.push(self.options.asset_bundle_name.clone());
        Ok(bundle_path)
    }
}

impl AssetIo for BundledAssetIo {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        info!("load_path: {:?}", path);
        Box::pin(async move {
            let bundle_path = self.get_bundle_path()?;
            let file = File::open(bundle_path)?;
            let mut archive = Archive::new(file);
            for entry in archive.entries()? {
                if entry.is_ok() {
                    let mut entry_file = entry?;
                    let entry_path = entry_file.path()?;
                    if entry_path.eq(path) {
                        let mut vec = Vec::new();
                        entry_file.read_to_end(&mut vec)?;
                        #[cfg(feature = "encryption")]
                        if let Some(decrypted) = self.options.try_decrypt(&vec).map_err(|err| {
                            AssetIoError::Io(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("{}", err),
                            ))
                        })? {
                            return Ok(decrypted);
                        }
                        return Ok(vec);
                    }
                }
            }
            return Err(AssetIoError::NotFound(path.to_path_buf()));
        })
    }

    fn read_directory(
        &self,
        path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        info!("[read_directory] {:?}", path);
        if let Some(lock) = self.parent_dir_to_path_info.clone() {
            let mappings = lock.read().unwrap();
            // TODO: normalize path
            if let Some(entries) = mappings.get(path) {
                let vec: Vec<PathBuf> = entries.into_iter().map(|e| e.path()).collect();
                return Ok(Box::new(vec.into_iter()));
            }
        }
        return Err(AssetIoError::NotFound(path.to_path_buf()));
    }

    fn is_directory(&self, path: &Path) -> bool {
        // TODO: normalize path
        info!("is_directory: {:?}", path);
        if let Some(lock) = self.parent_dir_to_path_info.clone() {
            let mappings = lock.read().unwrap();
            mappings.contains_key(path)
        } else {
            false
        }
    }

    fn watch_path_for_changes(&self, _path: &Path) -> Result<(), AssetIoError> {
        Ok(())
    }

    fn watch_for_changes(&self) -> Result<(), AssetIoError> {
        Ok(())
    }
}
