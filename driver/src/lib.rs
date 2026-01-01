#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate wdk_panic;

use wdk_sys::*;
use wdk_sys::ntddk::DbgPrint;

#[allow(unused_variables)]
#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    _driver_object: PDRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    unsafe { DbgPrint(b"[Rust] Loaded Successfully\n\0".as_ptr() as *const i8); }

    STATUS_SUCCESS
}
