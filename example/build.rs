mod src;
use src::BUNDLE_OPTIONS;

use bevy_assets_bundler::*;

fn main() {
    AssetBundler::from(BUNDLE_OPTIONS.clone()).build().unwrap();
}
