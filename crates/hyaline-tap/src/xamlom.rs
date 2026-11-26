//! XAML Diagnostics interfaces from `um/xamlom.h` (Windows SDK 10.0.26100.0).
//!
//! They are not part of the Windows metadata, so `windows-bindgen` cannot generate them.
//! Method order must match the header's vtables exactly; unused methods are declared only
//! to keep later slots in place.
#![allow(non_snake_case, clippy::too_many_arguments)]

use std::ffi::c_void;
use windows_core::{BSTR, HRESULT, IUnknown, PCWSTR, interface};

/// Opaque handle XAML Diagnostics uses for objects (`InstanceHandle`).
pub type InstanceHandle = u64;

/// `VisualMutationType::Add`.
pub const MUTATION_ADD: i32 = 0;

/// `SourceInfo` as received in callbacks: borrowed strings, never freed by us.
#[repr(C)]
pub struct SourceInfoRef {
    pub file_name: *const u16,
    pub line_number: u32,
    pub column_number: u32,
    pub char_position: u32,
    pub hash: *const u16,
}

/// `SourceInfo` returned to us: owned strings, freed on drop.
#[repr(C)]
pub struct SourceInfo {
    pub file_name: BSTR,
    pub line_number: u32,
    pub column_number: u32,
    pub char_position: u32,
    pub hash: BSTR,
}

#[repr(C)]
pub struct ParentChildRelation {
    pub parent: InstanceHandle,
    pub child: InstanceHandle,
    pub child_index: u32,
}

/// `VisualElement` as received in callbacks (borrowed strings).
#[repr(C)]
pub struct VisualElement {
    pub handle: InstanceHandle,
    pub src_info: SourceInfoRef,
    pub type_name: *const u16,
    pub name: *const u16,
    pub num_children: u32,
}

/// `PropertyChainSource` (owned strings).
#[repr(C)]
pub struct PropertyChainSource {
    pub handle: InstanceHandle,
    pub target_type: BSTR,
    pub name: BSTR,
    pub source: i32,
    pub src_info: SourceInfo,
}

/// `PropertyChainValue` (owned strings).
#[repr(C)]
pub struct PropertyChainValue {
    pub index: u32,
    pub type_name: BSTR,
    pub declaring_type: BSTR,
    pub value_type: BSTR,
    pub item_type: BSTR,
    pub value: BSTR,
    pub overridden: i32,
    pub metadata_bits: i64,
    pub property_name: BSTR,
    pub property_chain_index: u32,
}

#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[interface("AA7A8931-80E4-4FEC-8F3B-553F87B4966E")]
pub unsafe trait IVisualTreeServiceCallback: IUnknown {
    pub fn OnVisualTreeChange(
        &self,
        relation: ParentChildRelation,
        element: VisualElement,
        mutation_type: i32,
    ) -> HRESULT;
}

#[interface("BAD9EB88-AE77-4397-B948-5FA2DB0A19EA")]
pub unsafe trait IVisualTreeServiceCallback2: IVisualTreeServiceCallback {
    pub fn OnElementStateChanged(
        &self,
        element: InstanceHandle,
        element_state: i32,
        context: PCWSTR,
    ) -> HRESULT;
}

#[interface("A593B11A-D17F-48BB-8F66-83910731C8A5")]
pub unsafe trait IVisualTreeService: IUnknown {
    pub fn AdviseVisualTreeChange(&self, callback: *mut c_void) -> HRESULT;
    pub fn UnadviseVisualTreeChange(&self, callback: *mut c_void) -> HRESULT;
    pub fn GetEnums(&self, count: *mut u32, enums: *mut *mut c_void) -> HRESULT;
    pub fn CreateInstance(
        &self,
        type_name: *const u16,
        value: *const u16,
        instance: *mut InstanceHandle,
    ) -> HRESULT;
    pub fn GetPropertyValuesChain(
        &self,
        instance: InstanceHandle,
        source_count: *mut u32,
        sources: *mut *mut PropertyChainSource,
        property_count: *mut u32,
        values: *mut *mut PropertyChainValue,
    ) -> HRESULT;
    pub fn SetProperty(
        &self,
        instance: InstanceHandle,
        value: InstanceHandle,
        property_index: u32,
    ) -> HRESULT;
    pub fn ClearProperty(&self, instance: InstanceHandle, property_index: u32) -> HRESULT;
    pub fn GetCollectionCount(&self, instance: InstanceHandle, size: *mut u32) -> HRESULT;
    pub fn GetCollectionElements(
        &self,
        instance: InstanceHandle,
        start: u32,
        count: *mut u32,
        values: *mut *mut c_void,
    ) -> HRESULT;
    pub fn AddChild(&self, parent: InstanceHandle, child: InstanceHandle, index: u32) -> HRESULT;
    pub fn RemoveChild(&self, parent: InstanceHandle, index: u32) -> HRESULT;
    pub fn ClearChildren(&self, parent: InstanceHandle) -> HRESULT;
}

#[interface("130F5136-EC43-4F61-89C7-9801A36D2E95")]
pub unsafe trait IVisualTreeService2: IVisualTreeService {
    pub fn GetPropertyIndex(
        &self,
        object: InstanceHandle,
        property_name: PCWSTR,
        index: *mut u32,
    ) -> HRESULT;
    pub fn GetProperty(
        &self,
        object: InstanceHandle,
        property_index: u32,
        value: *mut InstanceHandle,
    ) -> HRESULT;
}

#[interface("18C9E2B6-3F43-4116-9F2B-FF935D7770D2")]
pub unsafe trait IXamlDiagnostics: IUnknown {
    pub fn GetDispatcher(&self, dispatcher: *mut *mut c_void) -> HRESULT;
    pub fn GetUiLayer(&self, layer: *mut *mut c_void) -> HRESULT;
    pub fn GetApplication(&self, application: *mut *mut c_void) -> HRESULT;
    pub fn GetIInspectableFromHandle(
        &self,
        instance: InstanceHandle,
        inspectable: *mut *mut c_void,
    ) -> HRESULT;
    pub fn GetHandleFromIInspectable(
        &self,
        inspectable: *mut c_void,
        instance: *mut InstanceHandle,
    ) -> HRESULT;
    pub fn HitTest(
        &self,
        rect: Rect,
        count: *mut u32,
        handles: *mut *mut InstanceHandle,
    ) -> HRESULT;
    pub fn RegisterInstance(
        &self,
        inspectable: *mut c_void,
        instance: *mut InstanceHandle,
    ) -> HRESULT;
    pub fn GetInitializationData(&self, data: *mut BSTR) -> HRESULT;
}
