//! Olm types.

use napi_derive::*;

/// Struct representing the state of our private cross signing keys,
/// it shows which private cross signing keys we have locally stored.
#[napi]
#[derive(Debug)]
pub struct CrossSigningStatus {
    inner: matrix_sdk_crypto::olm::CrossSigningStatus,
}

impl From<matrix_sdk_crypto::olm::CrossSigningStatus> for CrossSigningStatus {
    fn from(inner: matrix_sdk_crypto::olm::CrossSigningStatus) -> Self {
        Self { inner }
    }
}

#[napi]
impl CrossSigningStatus {
    /// Do we have the master key.
    #[napi(getter)]
    pub fn has_master(&self) -> bool {
        self.inner.has_master
    }

    /// Do we have the self signing key, this one is necessary to sign
    /// our own devices.
    #[napi(getter)]
    pub fn has_self_signing(&self) -> bool {
        self.inner.has_self_signing
    }

    /// Do we have the user signing key, this one is necessary to sign
    /// other users.
    #[napi(getter)]
    pub fn has_user_signing(&self) -> bool {
        self.inner.has_user_signing
    }
}
