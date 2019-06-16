//! Types for the *m.room.aliases* event.

use js_int::UInt;
use ruma_identifiers::RoomAliasId;
use serde::{Deserialize, Serialize};

state_event! {
    /// Informs the room about what room aliases it has been given.
    pub struct AliasesEvent(AliasesEventContent) {}
}

/// The payload of an `AliasesEvent`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AliasesEventContent {
    /// A list of room aliases.
    pub aliases: Vec<RoomAliasId>,
}
