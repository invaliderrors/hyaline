//! Hides and shows the taskbar background rectangles through their `Opacity`.
//!
//! Explorer never sets `Opacity` on them, so hiding sets a local value of 0 and restoring
//! clears it, which returns them exactly to Explorer's state. Their brushes are never
//! touched (XAML Diagnostics handles do not keep objects alive, so a replaced brush could
//! not be put back). Runs only on the taskbar's UI thread.

use crate::bindings::CoTaskMemFree;
use crate::xamlom::{IVisualTreeService2, InstanceHandle};
use hyaline_core::tap::{OPACITY, property_index};
use std::cell::{Cell, RefCell};
use windows_core::BSTR;

struct Target {
    handle: InstanceHandle,
    opacity_index: u32,
}

thread_local! {
    static SERVICE: RefCell<Option<IVisualTreeService2>> = const { RefCell::new(None) };
    static TARGETS: RefCell<Vec<Target>> = const { RefCell::new(Vec::new()) };
    static CLEARED: Cell<bool> = const { Cell::new(false) };
}

/// Starts managing a background element; hides it right away if the taskbar is cleared.
pub fn track(service: &IVisualTreeService2, handle: InstanceHandle) {
    let Some(opacity_index) = opacity_index(service, handle) else {
        return;
    };
    SERVICE.with_borrow_mut(|s| {
        s.get_or_insert_with(|| service.clone());
    });
    let target = Target {
        handle,
        opacity_index,
    };
    if CLEARED.get() {
        hide(service, &target);
    }
    TARGETS.with_borrow_mut(|targets| {
        targets.retain(|t| t.handle != handle);
        targets.push(target);
    });
}

/// Stops managing an element XAML removed.
pub fn forget(handle: InstanceHandle) {
    TARGETS.with_borrow_mut(|targets| targets.retain(|t| t.handle != handle));
}

/// Hides or restores every managed element. Returns false if any element failed.
pub fn set_cleared(cleared: bool) -> bool {
    CLEARED.set(cleared);
    SERVICE.with_borrow(|service| {
        let Some(service) = service else {
            return true;
        };
        TARGETS.with_borrow(|targets| {
            targets.iter().fold(true, |ok, target| {
                let done = if cleared {
                    hide(service, target)
                } else {
                    restore(service, target)
                };
                done && ok
            })
        })
    })
}

fn hide(service: &IVisualTreeService2, target: &Target) -> bool {
    let type_name = BSTR::from("Windows.Foundation.Double");
    let zero = BSTR::from("0");
    let mut value = 0;
    // SAFETY: both strings are valid, NUL-terminated BSTRs alive for the call.
    let created =
        unsafe { service.CreateInstance(type_name.as_ptr(), zero.as_ptr(), &raw mut value) };
    // SAFETY: plain values; XAML validates the handles.
    created.is_ok()
        && unsafe { service.SetProperty(target.handle, value, target.opacity_index) }.is_ok()
}

fn restore(service: &IVisualTreeService2, target: &Target) -> bool {
    // SAFETY: plain values; XAML validates the handle.
    unsafe { service.ClearProperty(target.handle, target.opacity_index) }.is_ok()
}

fn opacity_index(service: &IVisualTreeService2, handle: InstanceHandle) -> Option<u32> {
    let (mut source_count, mut sources) = (0, std::ptr::null_mut());
    let (mut value_count, mut values) = (0, std::ptr::null_mut());
    // SAFETY: all out pointers are valid.
    let result = unsafe {
        service.GetPropertyValuesChain(
            handle,
            &raw mut source_count,
            &raw mut sources,
            &raw mut value_count,
            &raw mut values,
        )
    };
    // SAFETY: on return the arrays (or nulls) are ours to free, with the reported lengths.
    let (_sources, values) = unsafe {
        (
            take_array(sources, source_count),
            take_array(values, value_count),
        )
    };
    if result.is_err() {
        return None;
    }
    let names: Vec<(String, u32)> = values
        .iter()
        .map(|v| (v.property_name.display().to_string(), v.index))
        .collect();
    property_index(
        names.iter().map(|(name, index)| (name.as_str(), *index)),
        OPACITY,
    )
}

/// Moves `len` items out of a `CoTaskMemAlloc`ed array and frees the array.
///
/// # Safety
/// `ptr` must be null or point to `len` initialized items in a block the caller owns.
unsafe fn take_array<T>(ptr: *mut T, len: u32) -> Vec<T> {
    if ptr.is_null() {
        return Vec::new();
    }
    // SAFETY: guaranteed by the caller; each item is read exactly once.
    let items = (0..len as usize)
        .map(|i| unsafe { ptr.add(i).read() })
        .collect();
    // SAFETY: the items were moved out above; the block is freed exactly once.
    unsafe { CoTaskMemFree(ptr.cast()) };
    items
}
