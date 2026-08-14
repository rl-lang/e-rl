//! Runtime-agnostic core for the embedded RL standard library.
//!
//! This crate holds everything the shared stdlib (`rl-std`) and the runtime
//! (`rl-vm`) need in common, without referencing the runtime's value type - so
//! it can sit below it in the dependency graph:
//!
//! - [`Runtime`] / [`HandleStore`] - the abstraction the runtime implements,
//! - [`NativeHandle`] / [`Arity`] - the thin-`fn`-pointer native descriptor,
//! - [`ValueType`] / [`FromValueR`] / [`IntoValueR`] - value <-> Rust type
//!   conversions,
//! - [`StdFn`] / [`ModuleNames`] - the checker signature types,
//! - [`Xoshiro256`] - the shared PRNG.
#![no_std]

#[macro_use]
extern crate alloc;

pub mod convert;
pub mod handle;
#[macro_use]
pub mod module;
pub mod rng;
pub mod runtime;
pub mod signatures;

pub use convert::{FromValueR, IntoValueR, ValueType};
pub use handle::{Arity, NativeHandle, NativeThunk};
pub use rng::Xoshiro256;
pub use runtime::{HandleStore, Runtime};
pub use signatures::{ModuleNames, StdFn};
