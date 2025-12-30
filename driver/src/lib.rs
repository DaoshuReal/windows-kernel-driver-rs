#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WDKAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use wdk_sys::*;
use wdk_sys::ntddk::DbgPrint;

#[allow(unused_variables)]
#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    driver_object: PDRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    KdPrint!(b"[Rust] Loaded Successfully\n\0");

    STATUS_SUCCESS
}

#[macro_export]
macro_rules! KdPrint {
    ($msg:expr) => {{
        #[cfg(debug_assertions)]
        unsafe {
            DbgPrint($msg.as_ptr() as *const i8);
        }
    }};
}
