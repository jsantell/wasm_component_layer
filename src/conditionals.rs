//! Cross-target [`Send`]/[`Sync`] compatability helpers.
//!
//! The traits herein are intended for use as a compatiblity mechanism
//! for targeting both `wasm32-unknown-unknown` as well as native targets
//! where the implementor may be shared across threads.

#[allow(missing_docs)]
#[cfg(not(target_arch = "wasm32"))]
pub trait ConditionalSync: Send + Sync {}

#[cfg(not(target_arch = "wasm32"))]
impl<S> ConditionalSync for S where S: Send + Sync {}

#[allow(missing_docs)]
#[cfg(target_arch = "wasm32")]
pub trait ConditionalSync {}

#[cfg(target_arch = "wasm32")]
impl<S> ConditionalSync for S {}
