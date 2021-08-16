#[cfg(test)]
mod tests {
    use bevy_assets_bundler::*;
    #[cfg(feature = "encryption")]
    use rand::prelude::*;
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    const ASSET_PATH: &str = "example/assets";

    #[test]
    fn e2e_all_default() {
        e2e_default_inner().unwrap();
    }

    #[test]
    fn e2e_encryption_off() {
        e2e_inner(false).unwrap();
    }

    #[cfg(feature = "encryption")]
    #[test]
    fn e2e_encryption_on() {
        e2e_inner(true).unwrap();
    }

    fn e2e_inner(_enable_encryption: bool) -> anyhow::Result<()> {
        let mut options = AssetBundlingOptions::default();
        options.enabled_on_debug_build = true;
        #[cfg(feature = "encryption")]
        if _enable_encryption {
            let mut rng = rand::thread_rng();
            let mut key = [0; 16];
            rng.try_fill_bytes(&mut key)?;
            options.set_encryption_key(key);
            options.asset_bundle_name = "assets.encrypted.bin".into();
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

        assert_eq!(asset_io.is_directory(Path::new("fonts")), true);
        assert_eq!(asset_io.is_directory(Path::new("dummy")), false);

        let mut n = 0;
        for _ in asset_io.read_directory(Path::new("fonts"))? {
            n += 1;
        }
        assert!(n > 0);

        assert!(asset_io.read_directory(Path::new("dummy")).is_err());

        // Valid assets
        for asset_path in ["branding/bevy_logo_dark_big.png", "fonts/FiraSans-Bold.ttf"] {
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
