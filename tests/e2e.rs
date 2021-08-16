mod common;

#[cfg(test)]
mod tests {
    use super::common::prelude::*;

    use std::{
        fs,
        path::{Path, PathBuf},
    };

    #[test]
    fn e2e_all_default() {
        e2e_default_inner().unwrap();
    }

    #[test]
    fn e2e_encryption_off_filename_encoding_on() {
        e2e_inner(false, true).unwrap();
    }

    #[test]
    fn e2e_encryption_off_filename_encoding_off() {
        e2e_inner(false, false).unwrap();
    }

    #[cfg(feature = "encryption")]
    #[test]
    fn e2e_encryption_on_filename_encoding_on() {
        e2e_inner(true, true).unwrap();
    }

    #[cfg(feature = "encryption")]
    #[test]
    fn e2e_encryption_on_filename_encoding_off() {
        e2e_inner(true, false).unwrap();
    }

    fn e2e_inner(enable_encryption: bool, encode_file_names: bool) -> anyhow::Result<()> {
        let mut options = create_default_options_with_random_bundle_name();
        options.enabled_on_debug_build = true;
        options.encode_file_names = encode_file_names;
        if enable_encryption {
            #[cfg(feature = "encryption")]
            options.set_encryption_key(create_random_key());
        }

        // build bundle
        let mut bundler = AssetBundler::from(options.clone());
        bundler.asset_folder = ASSET_PATH.into();
        bundler.build()?;

        // load bundle
        let mut asset_io = BundledAssetIo::from(options.clone());
        verify_asset_io(&mut asset_io)?;
        Ok(())
    }

    fn e2e_default_inner() -> anyhow::Result<()> {
        // build bundle
        AssetBundler::default()
            .with_asset_folder(ASSET_PATH)
            .build()?;

        // load bundle
        let mut asset_io = BundledAssetIo::default();
        verify_asset_io(&mut asset_io)?;
        Ok(())
    }

    fn verify_asset_io(asset_io: &mut BundledAssetIo) -> anyhow::Result<()> {
        asset_io.ensure_loaded()?;

        asset_io.watch_for_changes()?;
        asset_io.watch_path_for_changes(Path::new("any"))?;

        // Valid directories
        for dir in ["fonts", "nonascii/图", "nonascii\\图"] {
            assert_eq!(asset_io.is_directory(Path::new(dir)), true);
            let mut n = 0;
            for _ in asset_io.read_directory(Path::new(dir))? {
                n += 1;
            }
            assert!(n > 0);
        }

        // Invalid directories
        for dir in ["dummy", "fonts/dummy", "fonts\\dummy"] {
            assert_eq!(asset_io.is_directory(Path::new(dir)), false);
            assert!(asset_io.read_directory(Path::new("dummy")).is_err());
        }

        // Valid assets
        for asset_path in [
            "branding/bevy_logo_dark_big.png",
            "fonts/FiraSans-Bold.ttf",
            "nonascii/图/图.png",
        ] {
            let future = asset_io.load_path(Path::new(asset_path));
            futures_lite::future::block_on(async {
                match future.await {
                    Ok(v) => {
                        assert!(v.len() > 0);
                        let mut file_path = PathBuf::from(ASSET_PATH);
                        file_path.push(asset_path);
                        let file_data = fs::read(file_path).unwrap();
                        assert_eq!(v.len(), file_data.len());
                        assert_eq!(&v, &file_data);
                    }
                    _ => {
                        assert!(false);
                    }
                };
            });
        }

        // Valid assets windows path seperator
        for asset_path in [
            "branding\\bevy_logo_dark_big.png",
            "fonts\\FiraSans-Bold.ttf",
            "nonascii\\图\\图.png",
        ] {
            let future = asset_io.load_path(Path::new(asset_path));
            futures_lite::future::block_on(async {
                match future.await {
                    Ok(v) => {
                        assert!(v.len() > 0);
                        let mut file_path = PathBuf::from(ASSET_PATH);
                        file_path.push(asset_path.replace('\\', "/"));
                        let file_data = fs::read(file_path).unwrap();
                        assert_eq!(v.len(), file_data.len());
                        assert_eq!(&v, &file_data);
                    }
                    _ => {
                        assert!(false);
                    }
                };
            });
        }

        // Invalid assets
        for asset_path in ["branding/dummy.png", "dummy.svg"] {
            let future = asset_io.load_path(Path::new(asset_path));
            futures_lite::future::block_on(async {
                match future.await {
                    Ok(_) => {
                        assert!(false);
                    }
                    _ => {}
                }
            });
        }
        Ok(())
    }
}
