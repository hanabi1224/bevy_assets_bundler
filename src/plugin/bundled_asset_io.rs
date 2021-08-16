use super::path_info::ArchivePathInfo;
use crate::AssetBundlingOptions;
use bevy::{
    asset::{AssetIo, AssetIoError},
    utils::BoxedFuture,
};
use std::{
    borrow::Borrow,
    collections::HashMap,
    env,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use tar::Archive;

type ParentDirToPathInfo = HashMap<String, Vec<ArchivePathInfo>>;

pub struct BundledAssetIo {
    options: AssetBundlingOptions,
    parent_dir_to_path_info: Option<Arc<RwLock<ParentDirToPathInfo>>>,
}

impl Default for BundledAssetIo {
    fn default() -> Self {
        Self {
            options: AssetBundlingOptions::default(),
            parent_dir_to_path_info: None,
        }
    }
}

impl From<AssetBundlingOptions> for BundledAssetIo {
    fn from(options: AssetBundlingOptions) -> Self {
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
            let mut mappings: ParentDirToPathInfo = HashMap::new();
            let mut n_entries = 0;
            for entry in archive.entries()? {
                if entry.is_ok() {
                    n_entries += 1;
                    let entry_file = entry?;
                    let path = entry_file.path()?;
                    let decoded_path = if self.options.encode_file_names {
                        self.options.try_decode_path(path.borrow())?
                        // PathBuf::from(self.options.try_decode_string(path.to_str().unwrap())?)
                    } else {
                        path.to_path_buf()
                    };
                    // let is_dir = path.is_dir();
                    let mut parent_dir = decoded_path.clone();
                    let parent_dir_str = if parent_dir.pop() {
                        parent_dir.to_str().unwrap_or("").replace('\\', "/")
                    } else {
                        "".into()
                    };
                    debug!("Loading asset file {:?}, dir:{:?}", path, parent_dir);
                    let path_info = ArchivePathInfo::new(decoded_path);
                    if let Some(vec) = mappings.get_mut(&parent_dir_str) {
                        vec.push(path_info);
                    } else {
                        mappings.insert(parent_dir_str, vec![path_info]);
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
        let mut bundle_path = env::current_exe().map_err(AssetIoError::Io)?;
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
            let encoded_entry_path = if self.options.encode_file_names {
                self.options.try_encode_path(path).map_err(map_error)?
            } else {
                PathBuf::from(normalize_path(path))
            };
            let mut archive = Archive::new(file);
            for entry in archive.entries()? {
                if entry.is_ok() {
                    let mut entry_file = entry?;
                    let entry_path = entry_file.path()?;
                    if entry_path.eq(&encoded_entry_path) {
                        let mut vec = Vec::new();
                        entry_file.read_to_end(&mut vec)?;
                        #[cfg(feature = "encryption")]
                        if let Some(decrypted) =
                            self.options.try_decrypt(&vec).map_err(map_error)?
                        {
                            return Ok(decrypted);
                        }
                        return Ok(vec);
                    }
                }
            }
            Err(AssetIoError::NotFound(path.to_path_buf()))
        })
    }

    fn read_directory(
        &self,
        path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        info!("[read_directory] {:?}", path);
        if let Some(lock) = self.parent_dir_to_path_info.clone() {
            let mappings = lock.read().unwrap();
            let path_str = normalize_path(path);
            if let Some(entries) = mappings.get(&path_str) {
                let vec: Vec<PathBuf> = entries.iter().map(|e| e.path()).collect();
                return Ok(Box::new(vec.into_iter()));
            }
        }
        Err(AssetIoError::NotFound(path.to_path_buf()))
    }

    fn is_directory(&self, path: &Path) -> bool {
        // TODO: normalize path
        info!("is_directory: {:?}", path);
        if let Some(lock) = self.parent_dir_to_path_info.clone() {
            let mappings = lock.read().unwrap();
            let path_str = normalize_path(path);
            mappings.contains_key(&path_str)
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

fn map_error(err: anyhow::Error) -> AssetIoError {
    AssetIoError::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("{}", err),
    ))
}

fn normalize_path(path: &Path) -> String {
    path.to_str().unwrap_or("").replace('\\', "/")
}
