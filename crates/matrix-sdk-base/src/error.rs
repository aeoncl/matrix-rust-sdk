// Copyright 2020 Damir Jelić
// Copyright 2020 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Error conditions.

use matrix_sdk_common::store_locks::LockStoreError;
#[cfg(feature = "e2e-encryption")]
use matrix_sdk_crypto::{CryptoStoreError, MegolmError, OlmError};
use ruma::OwnedRoomId;
use thiserror::Error;

use crate::event_cache::store::EventCacheStoreError;

/// Result type of the rust-sdk.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Internal representation of errors.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// Attempting to restore a session after the olm-machine has already been
    /// set up fails
    #[cfg(feature = "e2e-encryption")]
    #[error("The olm machine has already been initialized")]
    BadCryptoStoreState,

    /// The room where a group session should be shared is not encrypted.
    #[cfg(feature = "e2e-encryption")]
    #[error("The room where a group session should be shared is not encrypted")]
    EncryptionNotEnabled,

    /// A generic error returned when the state store fails not due to
    /// IO or (de)serialization.
    #[error(transparent)]
    StateStore(#[from] crate::store::StoreError),

    /// An error happened while manipulating the event cache store.
    #[error(transparent)]
    EventCacheStore(#[from] EventCacheStoreError),

    /// An error happened while attempting to lock the event cache store.
    #[error(transparent)]
    EventCacheLock(#[from] LockStoreError),

    /// An error occurred in the crypto store.
    #[cfg(feature = "e2e-encryption")]
    #[error(transparent)]
    CryptoStore(#[from] CryptoStoreError),

    /// An error occurred during a E2EE operation.
    #[cfg(feature = "e2e-encryption")]
    #[error(transparent)]
    OlmError(#[from] OlmError),

    /// An error occurred during a group E2EE operation.
    #[cfg(feature = "e2e-encryption")]
    #[error(transparent)]
    MegolmError(#[from] MegolmError),

    /// An error caused by calling the `BaseClient::receive_all_members`
    /// function with invalid parameters
    #[error("receive_all_members function was called with invalid parameters")]
    InvalidReceiveMembersParameters,

    /// This request failed because the local data wasn't sufficient.
    #[error("Local cache doesn't contain all necessary data to perform the action.")]
    InsufficientData,

    /// There was a [`serde_json`] deserialization error.
    #[error(transparent)]
    DeserializationError(#[from] serde_json::error::Error),

    /// Tombstoned rooms are creating a loop, or a merger.
    ///
    /// The shortest loop is a room upgrading/replacing itself:
    ///
    /// ```text
    /// m.room.tombstone
    /// replaced by room A
    /// ┌──────────────┐
    /// │              │
    /// │  ┌────────┐  │
    /// └──┤ room A ◄──┘
    ///    └────────┘
    /// ```
    ///
    /// But a more common case can involve more rooms:
    ///
    /// ```text
    ///      m.room.tombstone
    ///      replaced by room B
    ///     ┌───────────────────┐
    ///     │                   │
    /// ┌───┴────┐         ┌────▼───┐
    /// │ room A │         │ room B │
    /// └───▲────┘         └────┬───┘
    ///     │                   │
    ///     └───────────────────┘
    ///      m.room.tombstone
    ///      replaced by room A
    /// ```
    ///
    /// A merger is when two rooms are upgrading to the same room:
    ///
    /// ```text
    ///      m.room.tombstone
    ///      replaced by room C
    ///      ┌──────────────┐
    ///      │              │
    /// ┌────┴───┐          │
    /// │ room A │          │
    /// └────────┘     ┌────▼───┐
    ///                │ room C │
    /// ┌────────┐     └────▲───┘
    /// │ room B │          │
    /// └────┬───┘          │
    ///      │              │
    ///      └──────────────┘
    ///      m.room.tombstone
    ///      replaced by room C
    /// ```
    #[error("inconsistent tombstone room state: a loop or a merger is detected, it includes `{room_in_path:?}`")]
    InconsistentTombstonedRooms {
        /// One of the room that is part of the loop, or a merger.
        room_in_path: OwnedRoomId,
    },
}
