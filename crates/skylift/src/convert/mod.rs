//! Strutures and enums are converted with match patterns. This is to prevent modification of
//! the `target_lexicon` types.
//!
//! If there is a better way to do this let me know. I've experimented with PROST macros but
//! eventually decided to not as it requires tampering with `target_lexicon` types.

pub(crate) mod rpc2internal;
pub(crate) mod internal2rpc;

use prost_types::Any;
use serde::Serialize;

fn to_any_bincode<S: Serialize>(s: &S) -> Option<Any> {
    bincode::serialize(s).ok().map(|value| Any {
        value,
        ..Default::default()
    })
}

