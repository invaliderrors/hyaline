//! Keeps panics from unwinding into Explorer. A panic crossing an `extern "system"`
//! boundary aborts the process, which would take Explorer (and the taskbar) down.

use std::panic::{AssertUnwindSafe, catch_unwind};

/// Runs `f` and returns its result, or `fallback` if it panics.
pub fn guard<T>(fallback: T, f: impl FnOnce() -> T) -> T {
    catch_unwind(AssertUnwindSafe(f)).unwrap_or(fallback)
}
