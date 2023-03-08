mod src;
use bevy_assets_bundler::*;
use src::BUNDLE_OPTIONS;

fn main() {
    AssetBundler::from(BUNDLE_OPTIONS.clone()).build().unwrap();
}
