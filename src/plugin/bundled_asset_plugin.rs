use super::BundledAssetIo;
use crate::AssetBundlingOptions;
use bevy::{
    app::{App, Plugin},
    asset::AssetServer,
    tasks::IoTaskPool,
};

pub struct BundledAssetIoPlugin {
    options: AssetBundlingOptions,
}

impl Default for BundledAssetIoPlugin {
    fn default() -> Self {
        Self {
            options: AssetBundlingOptions::default(),
        }
    }
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
        let task_pool = app
            .world
            .get_resource::<IoTaskPool>()
            .expect("`IoTaskPool` resource not found.")
            .0
            .clone();
        let mut io = BundledAssetIo::from(self.options.clone());
        match io.ensure_loaded() {
            Err(err) => {
                error!("Fail to load bundled asset: {:?}", err);
            }
            _ => {
                app.insert_resource(AssetServer::new(io, task_pool));
            }
        }
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
