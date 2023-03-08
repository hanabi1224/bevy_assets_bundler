#![doc = include_str!("../README.md")]

#[macro_use]
extern crate tracing;

mod options;
pub use options::AssetBundlingOptions;

mod bundler;
pub use bundler::AssetBundler;

mod plugin;
#[cfg(feature = "encryption")]
use aes::Aes128;
pub use bevy::asset::AssetIo;
#[cfg(feature = "encryption")]
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
pub use plugin::{BundledAssetIo, BundledAssetIoPlugin};

#[cfg(feature = "encryption")]
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const DEFAULT_ASSET_FOLDER: &str = "assets";
const DEFAULT_ASSET_BUNDLE_NAME: &str = "assets.bin";
