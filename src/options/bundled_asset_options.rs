#[cfg(feature = "encryption")]
use crate::{Aes128Cbc, BlockMode};

#[derive(Debug, Clone)]
pub struct AssetBundlingOptions {
    #[cfg(feature = "encryption")]
    pub encryption_on: bool,
    #[cfg(feature = "encryption")]
    pub encryption_key: Option<[u8; 16]>,
    #[cfg(feature = "compression")]
    pub enable_compression: bool,
    pub enabled_on_debug_build: bool,
    pub asset_bundle_name: String,
}

impl Default for AssetBundlingOptions {
    fn default() -> Self {
        Self {
            #[cfg(feature = "encryption")]
            encryption_on: false,
            #[cfg(feature = "encryption")]
            encryption_key: None,
            enabled_on_debug_build: false,
            #[cfg(feature = "compression")]
            enable_compression: false,
            asset_bundle_name: crate::DEFAULT_ASSET_BUNDLE_NAME.to_owned(),
        }
    }
}

impl AssetBundlingOptions {
    #[cfg(feature = "encryption")]
    pub fn set_encryption_key(&mut self, key: [u8; 16]) -> &mut Self {
        self.encryption_on = true;
        self.encryption_key = Some(key);
        self
    }

    #[cfg(feature = "encryption")]
    pub(crate) fn is_encryption_ready(&self) -> bool {
        self.encryption_on && self.encryption_key.is_some()
    }

    #[cfg(feature = "encryption")]
    pub(crate) fn try_get_cipher_if_needed(&self) -> anyhow::Result<Option<Aes128Cbc>> {
        if self.encryption_on {
            if let Some(aes_key) = self.encryption_key {
                return Ok(Some(Aes128Cbc::new_from_slices(&aes_key, &aes_key)?));
            }
        }
        Ok(None)
    }

    #[cfg(feature = "encryption")]
    pub(crate) fn try_encrypt(&self, plain: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        if let Some(cipher) = self.try_get_cipher_if_needed()? {
            return Ok(Some(cipher.encrypt_vec(plain)));
        }
        Ok(None)
    }

    #[cfg(feature = "encryption")]
    pub(crate) fn try_decrypt(&self, encrypted: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        if let Some(cipher) = self.try_get_cipher_if_needed()? {
            return Ok(Some(cipher.decrypt_vec(encrypted)?));
        }
        Ok(None)
    }
}
