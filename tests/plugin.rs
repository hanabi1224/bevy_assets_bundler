mod common;

#[cfg(test)]
mod tests {
    use super::common::prelude::*;

    #[test]
    fn bundle_not_found() {
        let options = create_default_options_with_random_bundle_name();
        let mut asset_io = BundledAssetIo::from(options);
        match asset_io.ensure_loaded() {
            Ok(_) => {
                assert!(false)
            }
            _ => {}
        }
    }

    #[cfg(feature = "encryption")]
    #[test]
    // #[should_panic]
    fn encryption_key_unmatched() -> anyhow::Result<()> {
        use std::path::Path;

        let mut options = create_default_options_with_random_bundle_name();
        options.set_encryption_key(create_random_key());
        AssetBundler::from(options.clone())
            .with_asset_folder(ASSET_PATH)
            .build()?;

        // Reset key
        options.set_encryption_key(create_random_key());
        let mut asset_io = BundledAssetIo::from(options.clone());
        asset_io.ensure_loaded()?;
        let future = asset_io.load_path(Path::new("branding/bevy_logo_dark_big.png"));
        futures_lite::future::block_on(async {
            match future.await {
                Ok(_) => {
                    assert!(false);
                }
                _ => {}
            };
        });
        Ok(())
    }

    #[cfg(feature = "encryption")]
    #[test]
    #[should_panic]
    fn encryption_key_unmatched_filename_encoded() {
        let mut options = create_default_options_with_random_bundle_name();
        options.encode_file_names = true;
        options.set_encryption_key(create_random_key());
        AssetBundler::from(options.clone())
            .with_asset_folder(ASSET_PATH)
            .build()
            .unwrap();

        // Reset key
        options.set_encryption_key(create_random_key());
        let mut asset_io = BundledAssetIo::from(options.clone());
        asset_io.ensure_loaded().unwrap();
    }
}
