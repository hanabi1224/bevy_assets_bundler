#[cfg(feature = "encryption")]
use std::io::Read;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::AssetBundlingOptions;

pub struct AssetBundler {
    pub options: AssetBundlingOptions,
    pub asset_folder: String,
}

impl Default for AssetBundler {
    fn default() -> Self {
        Self {
            options: AssetBundlingOptions::default(),
            asset_folder: crate::DEFAULT_ASSET_FOLDER.to_owned(),
        }
    }
}

impl From<AssetBundlingOptions> for AssetBundler {
    fn from(options: AssetBundlingOptions) -> Self {
        Self {
            options,
            asset_folder: crate::DEFAULT_ASSET_FOLDER.to_owned(),
        }
    }
}

impl AssetBundler {
    pub fn with_asset_folder(&mut self, path: impl Into<String>) -> &mut Self {
        self.asset_folder = path.into();
        self
    }

    pub fn build(&self) -> anyhow::Result<()> {
        if !self.options.enabled_on_debug_build
            && env::var("PROFILE").unwrap_or_else(|_| "".into()) == "debug"
        {
            warn!("disabled on debug build");
            return Ok(());
        }
        info!("Start bundling assets, cwd: {:?}", env::current_dir());

        #[cfg(feature = "encryption")]
        if self.options.encryption_on && self.options.encryption_key.is_none() {
            // Default key?
            return Err(anyhow::Error::msg(
                "Asset encryption is enabled but encryption key is not provided.",
            ));
        }

        let asset_dir = PathBuf::from(&self.asset_folder);
        if asset_dir.is_dir() {
            let exe_dir = get_exe_dir()?;
            let bundle_file_path = exe_dir.join(self.options.asset_bundle_name.clone());
            // Create bundle_path parent dir if not exist.
            if let Some(bundle_file_dir) = bundle_file_path.parent() {
                if !bundle_file_dir.exists() {
                    fs::create_dir_all(bundle_file_dir)?;
                }
            }

            let tar_file = fs::File::create(bundle_file_path)?;
            let mut tar_builder = tar::Builder::new(tar_file);
            archive_dir(&mut tar_builder, &asset_dir, &self.options)?;
            Ok(())
        } else {
            Err(anyhow::Error::msg(format!(
                "Asset folder not found: {}, cwd: {:?}",
                self.asset_folder,
                env::current_dir()?
            )))
        }
    }
}

fn archive_dir(
    builder: &mut tar::Builder<fs::File>,
    asset_dir: &Path,
    options: &AssetBundlingOptions,
) -> anyhow::Result<()> {
    archive_dir_recursive(builder, asset_dir, asset_dir, options)?;
    Ok(())
}

fn archive_dir_recursive(
    builder: &mut tar::Builder<fs::File>,
    dir: &Path,
    prefix: &Path,
    options: &AssetBundlingOptions,
) -> anyhow::Result<()> {
    for entry_result in fs::read_dir(dir)? {
        let entry_path = entry_result?.path();
        if entry_path.is_dir() {
            archive_dir_recursive(builder, &entry_path, prefix, options)?;
        } else {
            let mut name_in_archive = entry_path.strip_prefix(prefix)?.to_owned();
            if options.encode_file_names {
                name_in_archive = options.try_encode_path(&name_in_archive)?;
            }
            let mut file = fs::File::open(entry_path.clone())?;
            #[cfg(feature = "encryption")]
            if options.is_encryption_ready() {
                let mut plain = Vec::new();
                file.read_to_end(&mut plain)?;
                if let Some(encrypted) = options.try_encrypt(&plain)? {
                    let mut header = tar::Header::new_gnu();
                    header.set_path(name_in_archive)?;
                    let metadata = fs::metadata(&entry_path)?;
                    header.set_metadata(&metadata);
                    header.set_size(encrypted.len() as u64);
                    // header.set_mode(0o400);
                    header.set_cksum();
                    builder.append(&header, encrypted.as_slice())?;
                    continue;
                }
            }

            builder.append_file(name_in_archive, &mut file)?;
        }
    }
    Ok(())
}

fn get_exe_dir() -> anyhow::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    if !env::var("OUT_DIR").unwrap_or_else(|_| "".into()).is_empty() {
        dir.pop();
        dir.pop();
    }
    Ok(dir)
}
