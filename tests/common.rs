#[allow(dead_code)]
#[cfg(test)]
pub mod prelude {
    pub use bevy_assets_bundler::*;
    use rand::{prelude::*, rngs::OsRng};

    pub const ASSET_PATH: &str = "example/assets";

    pub fn create_default_options_with_random_bundle_name() -> AssetBundlingOptions {
        AssetBundlingOptions {
            asset_bundle_name: format!("{}.bin", uuid::Uuid::new_v4()),
            ..Default::default()
        }
    }

    pub fn create_random_key() -> [u8; 16] {
        let mut key = [0; 16];
        OsRng.fill_bytes(&mut key);
        key
    }
}
