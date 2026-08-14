//! The single, runtime-agnostic standard library for the embedded RL engine.
//!
//! Each function is written once, generic over `rl_std_core::Runtime`, and
//! annotated with `#[native_fn]` so it is available to the VM as a thin
//! function pointer.
//!
//! The embedded build is `#![no_std]` and single-threaded (`alloc::rc`).
//! Only pure-computation modules ship: `array`, `bitwise`, `collections`,
//! `debug`, `io` (print/println into the host output buffer), `math` (float
//! transcendentals via `libm`), `random`, `result`, `string`, and `types`.
//! OS-facing modules (`c`, `audio`, `gui`, `http`, `net`, `path`, `process`,
//! `terminal`, `time`, `fs`) are intentionally absent - there is no filesystem
//! or network on bare metal, and wall-clock time / stdin / stderr are host
//! concerns handled by the embedding application.
#![no_std]

#[macro_use]
extern crate alloc;

pub mod array;
pub mod bitwise;
pub mod collections;
pub mod debug;
pub mod io;
pub mod math;
pub mod random;
pub mod result;
pub mod string;
pub mod types;