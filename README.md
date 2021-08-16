# bevy_assets_bundler

[![main](https://github.com/hanabi1224/bevy_assets_bundler/actions/workflows/main.yml/badge.svg)](https://github.com/hanabi1224/bevy_assets_bundler/actions/workflows/main.yml)
[![travis](https://travis-ci.com/hanabi1224/bevy_assets_bundler.svg?branch=main)](https://travis-ci.com/github/hanabi1224/bevy_assets_bundler)
[![codecov](https://codecov.io/gh/hanabi1224/bevy_assets_bundler/branch/main/graph/badge.svg?token=gOcqVpMmIY)](https://codecov.io/gh/hanabi1224/bevy_assets_bundler)
[![MIT License](https://img.shields.io/github/license/hanabi1224/bevy_assets_bundler.svg)](https://github.com/hanabi1224/bevy_assets_bundler/blob/master/LICENSE)

Assets Bundler for bevy, with encryption support. Current archive format is tar and encryption algorithm is AES

## Features

- Bundle asset folder into a single assets.bin file
- Asset encryption with custom key
- Asset file names encoding (base58 when ecryption is off, AES+base58 otherwise)
- One simple switch to turn off bundling on debug build

## [Installation](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/example/Cargo.toml)
```toml
# Cargo.toml
[dependencies]
bevy = "0.5"
bevy_assets_bundler = {git = "https://github.com/hanabi1224/bevy_assets_bundler"}

[build-dependencies]
bevy_assets_bundler = {git = "https://github.com/hanabi1224/bevy_assets_bundler"}
```

## [Build Script](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/example/build.rs)
```rust
// build.rs
// encryption key: [u8; 16] array
// make sure the key is consistent between build.rs and main.rs
let key = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
let mut options = AssetBundlingOptions::default();
options.set_encryption_key(key);
options.encode_file_names = true;
options.enabled_on_debug_build = true;
AssetBundler::from(options).build().unwrap();
```

## [Bevy Setup](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/example/src/main.rs)
```rust
use bevy_assets_bundler::*;
use bevy::asset::AssetPlugin;

fn main() {
    // encryption key: [u8; 16] array
    // make sure the key is consistent between build.rs and main.rs
    let key = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut options = AssetBundlingOptions::default();
    options.set_encryption_key(key);
    options.encode_file_names = true;
    options.enabled_on_debug_build = true;

    App::build()
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<AssetPlugin, _>(BundledAssetIoPlugin::from(
                options.clone(),
            ))
        })
        .add_startup_system(setup.system())
        .run();
}
```

## [Options](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/src/plugin/bundled_asset_options.rs)
```rust
#[derive(Debug, Clone)]
pub struct AssetBundlingOptions {
    #[cfg(feature = "encryption")]
    pub encryption_on: bool,
    #[cfg(feature = "encryption")]
    pub encryption_key: Option<[u8; 16]>,
    #[cfg(feature = "compression")]
    pub enable_compression: bool,
    pub enabled_on_debug_build: bool,
    pub encode_file_names: bool,
    pub asset_bundle_name: String,
}
```

## TODO

- Compression
- More encryption algorithms
