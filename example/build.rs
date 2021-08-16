use bevy_assets_bundler::*;

fn main() {
    let key = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut options = AssetBundlingOptions::default();
    options.set_encryption_key(key);
    options.enabled_on_debug_build = true;
    options.encode_file_names = true;
    AssetBundler::from(options).build().unwrap();
}
