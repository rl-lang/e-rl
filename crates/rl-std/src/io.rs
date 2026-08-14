//! `std::io` - output only: `print` and `println`.
//!
//! The embedded build has no filesystem and no stdin. `print`/`println`
//! write into [`Runtime::output_buffer`] when set; when no buffer is present
//! the host installs one to capture script output (the `rl-embed` facade does
//! this), so bare-metal output is fully host-directed.

use alloc::string::String;
use alloc::vec::Vec;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- printing (variadic, untyped) -----------------------------------------

#[native_fn(module = "io", untyped)]
pub fn print<R: Runtime>(cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(&text);
    }
    R::null()
}

#[native_fn(module = "io", untyped)]
pub fn println<R: Runtime>(cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(&text);
        buffer.push('\n');
    }
    R::null()
}

rl_std_core::native_module!("io";
    funcs: [
        print, println,
    ],
);