//! Clears and restores the `Fill` of the taskbar background rectangles. The original brush
//! is read before the first change and set back on restore; a background whose original
//! cannot be read is never cleared. Runs only on the taskbar's UI thread.

use crate::bindings::CoTaskMemFree;
use crate::xamlom::{IVisualTreeService2, InstanceHandle};
use hyaline_core::tap::fill_property_index;
use std::cell::{Cell, RefCell};
use windows_core::BSTR;

struct Target {
    handle: InstanceHandle,
    fill_index: u32,
    /// Brush to put back; `Some(0)` means the property had no value.
    original: Option<InstanceHandle>,
}

thread_local! {
    static SERVICE: RefCell<Option<IVisualTreeService2>> = const { RefCell::new(None) };
    static TARGETS: RefCell<Vec<Target>> = const { RefCell::new(Vec::new()) };
    static CLEARED: Cell<bool> = const { Cell::new(false) };
}

/// Starts managing a background element; clears it right away if the taskbar is cleared.
pub fn track(service: &IVisualTreeService2, handle: InstanceHandle) {
    let Some(fill_index) = fill_index(service, handle) else {
        return;
    };
    SERVICE.with_borrow_mut(|s| {
        s.get_or_insert_with(|| service.clone());
    });
    let mut target = Target {
        handle,
        fill_index,
        original: None,
    };
    if CLEARED.get() {
        clear(service, &mut target);
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

/// Clears or restores every managed element. Returns false if any element failed.
pub fn set_cleared(cleared: bool) -> bool {
    CLEARED.set(cleared);
    SERVICE.with_borrow(|service| {
        let Some(service) = service else {
            return true;
        };
        TARGETS.with_borrow_mut(|targets| {
            targets.iter_mut().fold(true, |ok, target| {
                let done = if cleared {
                    clear(service, target)
                } else {
                    restore(service, target)
                };
                done && ok
            })
        })
    })
}

fn clear(service: &IVisualTreeService2, target: &mut Target) -> bool {
    if target.original.is_none() {
        let mut original = 0;
        // SAFETY: `original` is a valid out pointer.
        let read =
            unsafe { service.GetProperty(target.handle, target.fill_index, &raw mut original) };
        if read.is_err() {
            return false;
        }
        target.original = Some(original);
    }
    let Some(brush) = transparent_brush(service) else {
        return false;
    };
    // SAFETY: plain values; XAML validates the handles.
    unsafe { service.SetProperty(target.handle, brush, target.fill_index) }.is_ok()
}

fn restore(service: &IVisualTreeService2, target: &mut Target) -> bool {
    let restored = match target.original {
        None => return true,
        // SAFETY: plain values; XAML validates the handles.
        Some(0) => unsafe { service.ClearProperty(target.handle, target.fill_index) },
        // SAFETY: as above.
        Some(original) => unsafe {
            service.SetProperty(target.handle, original, target.fill_index)
        },
    };
    // Re-read on the next clear, in case Explorer changed the brush meanwhile.
    target.original = None;
    restored.is_ok()
}

fn transparent_brush(service: &IVisualTreeService2) -> Option<InstanceHandle> {
    let type_name = BSTR::from("Windows.UI.Xaml.Media.SolidColorBrush");
    let value = BSTR::from("Transparent");
    let mut brush = 0;
    // SAFETY: both strings are valid, NUL-terminated BSTRs alive for the call.
    unsafe { service.CreateInstance(type_name.as_ptr(), value.as_ptr(), &raw mut brush) }
        .is_ok()
        .then_some(brush)
}

fn fill_index(service: &IVisualTreeService2, handle: InstanceHandle) -> Option<u32> {
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
    fill_property_index(names.iter().map(|(name, index)| (name.as_str(), *index)))
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
