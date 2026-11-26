//! Helper DLL loaded into `explorer.exe` that makes the taskbar's XAML background transparent.

#[rustfmt::skip]
#[allow(clippy::all, clippy::pedantic, clippy::undocumented_unsafe_blocks, nonstandard_style, dead_code)]
mod bindings;
mod guard;
mod xamlom;
