use bevy::{
    app::{App, Plugin},
    asset::AssetServer,
};

use super::BundledAssetIo;
use crate::AssetBundlingOptions;

#[derive(Default)]
pub struct BundledAssetIoPlugin {
    options: AssetBundlingOptions,
}

impl From<AssetBundlingOptions> for BundledAssetIoPlugin {
    fn from(options: AssetBundlingOptions) -> Self {
        Self { options }
    }
}

impl Plugin for BundledAssetIoPlugin {
    fn build(&self, app: &mut App) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)]{
                if !self.options.enabled_on_debug_build {
                    warn!("disabled on debug build");
                    return;
                }
            }
        }

        let mut io = BundledAssetIo::from(self.options.clone());
        match io.ensure_loaded() {
            Err(err) => {
                error!("Fail to load bundled asset: {:?}", err);
            }
            _ => {
                app.insert_resource(AssetServer::new(io));
            }
        }
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
