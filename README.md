# Bevy Assets Bundler

[![github action](https://github.com/hanabi1224/bevy_assets_bundler/actions/workflows/main.yml/badge.svg)](https://github.com/hanabi1224/bevy_assets_bundler/actions/workflows/main.yml)
[![codecov](https://codecov.io/gh/hanabi1224/bevy_assets_bundler/branch/main/graph/badge.svg?token=gOcqVpMmIY)](https://codecov.io/gh/hanabi1224/bevy_assets_bundler)
[![dependency status](https://deps.rs/repo/github/hanabi1224/bevy_assets_bundler/status.svg?style=flat-square)](https://deps.rs/repo/github/hanabi1224/bevy_assets_bundler)
[![loc](https://tokei.rs/b1/github/hanabi1224/bevy_assets_bundler?category=code)](https://github.com/hanabi1224/bevy_assets_bundler)
[![License](https://img.shields.io/github/license/hanabi1224/bevy_assets_bundler.svg)](https://github.com/hanabi1224/bevy_assets_bundler/blob/master/LICENSE)

[![crates.io](https://img.shields.io/crates/v/bevy_assets_bundler)](https://crates.io/crates/bevy_assets_bundler)
[![docs.rs](https://docs.rs/bevy_assets_bundler/badge.svg)](https://docs.rs/bevy_assets_bundler)

Assets Bundler for bevy, with content encryption support. Current archive format is tar and encryption algorithm is AES

## Features

- Bundle asset folder into a single assets.bin file
- Asset encryption with custom key
- Asset file names encoding (base58 when ecryption is off, AES+base58 otherwise)
- One simple switch to turn off bundling on debug build

## [Installation](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/example/Cargo.toml)
```toml
# Cargo.toml
[dependencies]
bevy = "0.9"
bevy_assets_bundler = "0.5"

[build-dependencies]
bevy_assets_bundler = "0.5"
```

## [Build Script](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/example/build.rs)

You can generate a random key with [this playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2018&gist=cd3cb4ca8b86e67070b94caf366d162e)

```rust
use bevy_assets_bundler::*;

// build.rs
// encryption key: [u8; 16] array
// make sure the key is consistent between build.rs and main.rs
// or follow the example code to share code between build.rs and main.rs
fn main() {
    let key = [30, 168, 132, 180, 250, 203, 124, 96, 221, 206, 64, 239, 102, 20, 139, 79];
    let mut options = AssetBundlingOptions::default();
    options.set_encryption_key(key);
    options.encode_file_names = true;
    options.enabled_on_debug_build = true;
    AssetBundler::from(options).build();//.unwrap();
}
```

## [Bevy Setup](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/example/src/main.rs)
```rust
use bevy_assets_bundler::*;
use bevy::{asset::AssetPlugin, prelude::*};

fn main() {
    // encryption key: [u8; 16] array
    // make sure the key is consistent between build.rs and main.rs
    // or follow the example code to share code between build.rs and main.rs
    let key = [30, 168, 132, 180, 250, 203, 124, 96, 221, 206, 64, 239, 102, 20, 139, 79];
    let mut options = AssetBundlingOptions::default();
    options.set_encryption_key(key);
    options.encode_file_names = true;
    options.enabled_on_debug_build = true;

    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(BundledAssetIoPlugin::from(
                    options,
                )),
        )
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

## Bevy Version Supported

|bevy|bevy_assets_bundler|
|---|---|
|main|bevy_main|
|0.9|0.5|
|0.8|0.4|
|0.7|0.3|
|0.6|0.2|
|0.5|0.1|

## Examples

Check out [example](https://github.com/hanabi1224/bevy_assets_bundler/tree/main/example) and [E2E test](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/tests/e2e.rs)

To run example: ```cargo run -p example```

go to ```target/release``` folder, now you can move example(.exe) and assets.bin to some other place and run, just keep the relative path between them.

## Disclaimer

The encryption mechnism this library provides does not protect your assets from **ALL** kinds of reverse engineering as long as the game executable and the assets bundle are distributed to end users.

## [License](https://github.com/hanabi1224/bevy_assets_bundler/blob/main/LICENSE)

MIT
