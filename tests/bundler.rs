mod common;

#[cfg(test)]
mod tests {
    use super::common::prelude::*;

    #[test]
    fn invalid_asset_folder() {
        let options = create_default_options_with_random_bundle_name();
        match AssetBundler::from(options)
            .with_asset_folder("i/am/an/invalid/path")
            .build()
        {
            Ok(_) => {
                assert!(false)
            }
            _ => {}
        }
    }

    #[cfg(feature = "encryption")]
    #[test]
    fn no_encryption_key() {
        let mut options = create_default_options_with_random_bundle_name();
        options.encryption_on = true;
        match AssetBundler::from(options)
            .with_asset_folder(ASSET_PATH)
            .build()
        {
            Ok(_) => {
                assert!(false)
            }
            _ => {}
        }
    }
}
