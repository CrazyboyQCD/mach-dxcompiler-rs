//! Provides a statically-linked copy of `DXC` and includes the ability to call the main exported function ([`DxcCreateInstance`]).
//! Normal COM APIs can be used to interact with DXC objects once they have been created.

use std::ffi::*;
use std::sync::*;
use windows_core::*;

/// Invokes the native [`DxcCreateInstance`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-dxccreateinstance)
/// funcion. Creates a new uninitialized object of the specified class.
/// 
/// # Safety
/// 
/// `rclsid` and `riid` must point to valid Windows [`GUID`]s. `ppv` must point to a writable
/// `*mut c_void` instance.
#[allow(non_snake_case)]
pub unsafe extern "system" fn DxcCreateInstance(rclsid: *const GUID, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
    /// Lock which ensures that the dxcompiler library has been initialized before use.
    static MAIN_INITIALIZER: Once = Once::new();
    MAIN_INITIALIZER.call_once(|| unsafe { bindings::MachDxcompilerInvokeDllMain() });
    bindings::DxcCreateInstance(rclsid, riid, ppv)
}

/// Provides bindings to the native `dxcompiler` library.
mod bindings {
    use super::*;

    extern "system" {
        /// Calls the [`DxcCreateInstance`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-dxccreateinstance) function.
        pub fn DxcCreateInstance(rclsid: *const GUID, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT;

        /// Invokes the [`DllMain`](https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain) entrypoint for the dxcompiler library.
        pub fn MachDxcompilerInvokeDllMain();
    }
}