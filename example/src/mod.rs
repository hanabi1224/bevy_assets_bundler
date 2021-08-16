use bevy_assets_bundler::AssetBundlingOptions;

lazy_static::lazy_static! {
    pub static ref BUNDLE_OPTIONS: AssetBundlingOptions = {
        let key = [30, 168, 132, 180, 250, 203, 124, 96, 221, 206, 64, 239, 102, 20, 139, 79];
        let mut options = AssetBundlingOptions::default();
        options.set_encryption_key(key);
        options.enabled_on_debug_build = true;
        options.encode_file_names = true;
        // options.asset_bundle_name = "a/assets.bin".into();
        options
    };
}
