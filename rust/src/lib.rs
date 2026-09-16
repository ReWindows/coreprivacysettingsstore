//! Windissect-generated Rust FFI bindings for coreprivacysettingsstore.dll. Windows only.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

pub mod handles;
pub mod vtables;

pub use handles::*;

pub const EXPECTED_PDB_GUID: &str = "0F633F5854BC35CF7C6A52B770350077";
pub const EXPECTED_PDB_AGE: u32 = 1;
pub const EXPECTED_PE_TIMESTAMP: u32 = 3058442318;

// --- internal-function resolver (ASLR-safe: live base + fixed RVA) ---
#[cfg(target_os = "windows")]
mod resolver {
    use core::sync::atomic::{AtomicUsize, Ordering};
    static BASE: AtomicUsize = AtomicUsize::new(0);
    #[link(name = "kernel32", kind = "raw-dylib")]
    unsafe extern "system" {
        fn GetModuleHandleW(name: *const u16) -> usize;
    }
    pub fn base() -> usize {
        let b = BASE.load(Ordering::Relaxed);
        if b != 0 {
            return b;
        }
        let name: [u16; 29] = [99, 111, 114, 101, 112, 114, 105, 118, 97, 99, 121, 115, 101, 116, 116, 105, 110, 103, 115, 115, 116, 111, 114, 101, 46, 100, 108, 108, 0];
        let h = unsafe { GetModuleHandleW(name.as_ptr()) };
        if h == 0 { return 0; }
        // IMAGE_DOS_HEADER.e_lfanew -> IMAGE_NT_HEADERS.FileHeader.TimeDateStamp.
        let pe_offset = unsafe { core::ptr::read_unaligned((h + 0x3c) as *const u32) } as usize;
        let live_timestamp = unsafe { core::ptr::read_unaligned((h + pe_offset + 8) as *const u32) };
        if live_timestamp != 3058442318 { return 0; }
        BASE.store(h, Ordering::Relaxed);
        h
    }
}
#[cfg(not(target_os = "windows"))]
mod resolver {
    pub fn base() -> usize { 0 }
}

/// Absolute address of an internal function from its RVA. Requires the target
/// module to be loaded in the current process.
#[inline]
pub fn resolve(rva: usize) -> *const () {
    let base = resolver::base();
    if base == 0 { return ::core::ptr::null(); }
    let pe_offset = unsafe { ::core::ptr::read_unaligned((base + 0x3c) as *const u32) } as usize;
    let image_size = unsafe { ::core::ptr::read_unaligned((base + pe_offset + 80) as *const u32) } as usize;
    if rva >= image_size { return ::core::ptr::null(); }
    base.checked_add(rva).map_or(::core::ptr::null(), |address| address as *const ())
}
