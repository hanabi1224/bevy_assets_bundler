mod common;

#[cfg(test)]
mod tests {
    use super::common::prelude::*;

    #[test]
    fn invalid_asset_folder() -> anyhow::Result<()> {
        let options = create_default_options_with_random_bundle_name();
        anyhow::ensure!(AssetBundler::from(options)
            .with_asset_folder("i/am/an/invalid/path")
            .build()
            .is_err());
        Ok(())
    }

    #[cfg(feature = "encryption")]
    #[test]
    fn no_encryption_key() -> anyhow::Result<()> {
        let mut options = create_default_options_with_random_bundle_name();
        options.encryption_on = true;
        anyhow::ensure!(AssetBundler::from(options)
            .with_asset_folder(ASSET_PATH)
            .build()
            .is_err());
        Ok(())
    }
}
