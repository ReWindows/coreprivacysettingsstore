//! Evidence-labelled vftable metadata and raw slot access.
//! Slot calls remain the consumer's responsibility until a signature is unique.

use core::ffi::c_void;

#[derive(Clone, Copy, Debug)]
pub struct VTableInfo { pub name: &'static str, pub rva: u32, pub first_slot: usize, pub slot_count: usize, pub confidence: &'static str }

#[derive(Clone, Copy, Debug)]
pub struct VTableSlot { pub table_rva: u32, pub slot: u32, pub byte_offset: u32, pub target_rva: u32, pub target_id: Option<&'static str>, pub target_name: Option<&'static str>, pub ambiguous: bool, pub this_adjustment: Option<i32> }

pub static VTABLES: &[VTableInfo] = &[
    VTableInfo { name: "const type_info::`vftable'", rva: 0x1F008, first_slot: 0, slot_count: 1, confidence: "likely" },
    VTableInfo { name: "const std::exception::`vftable'", rva: 0x1F028, first_slot: 1, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const std::bad_alloc::`vftable'", rva: 0x1F040, first_slot: 3, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const std::bad_array_new_length::`vftable'", rva: 0x1F058, first_slot: 5, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const Microsoft::WRL::Details::DefaultModule<1>::`vftable'", rva: 0x1F068, first_slot: 7, slot_count: 12, confidence: "likely" },
    VTableInfo { name: "const wil::ResultException::`vftable'", rva: 0x1F0C8, first_slot: 19, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const Windows::Internal::CorePrivacySettingsStore::Private::CPSSPolicyBase::`vftable'", rva: 0x1F0D8, first_slot: 21, slot_count: 1, confidence: "likely" },
    VTableInfo { name: "const wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::`vftable'", rva: 0x1F0E0, first_slot: 22, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CorePrivacySettingsStoreTelemetry::`vftable'", rva: 0x1F108, first_slot: 27, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const wil::TraceLoggingProvider::`vftable'", rva: 0x1F128, first_slot: 31, slot_count: 4, confidence: "likely" },
    VTableInfo { name: "const wistd::__function::__base<bool __cdecl (void *, uint64_t, void *, uint64_t, unsigned int)>::`vftable'", rva: 0x1F148, first_slot: 35, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const wil::details::ThreadFailureCallbackFn<class <lambda_187d33510e6d72dbc662e4da4aa3681d> >::`vftable'", rva: 0x1F170, first_slot: 40, slot_count: 1, confidence: "likely" },
];

pub static VTABLE_SLOTS: &[VTableSlot] = &[
    VTableSlot { table_rva: 0x1F008, slot: 0, byte_offset: 0, target_rva: 0x3080, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F028, slot: 0, byte_offset: 0, target_rva: 0x45F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F028, slot: 1, byte_offset: 8, target_rva: 0x7260, target_id: Some("?what@exception@std@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl std::exception::what(void) const"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F040, slot: 0, byte_offset: 0, target_rva: 0x45F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F040, slot: 1, byte_offset: 8, target_rva: 0x7260, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F058, slot: 0, byte_offset: 0, target_rva: 0x45F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F058, slot: 1, byte_offset: 8, target_rva: 0x7260, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 0, byte_offset: 0, target_rva: 0x4570, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 1, byte_offset: 8, target_rva: 0x57A0, target_id: Some("?IncrementObjectCount@?$Module@$00V?$DefaultModule@$00@Details@WRL@Microsoft@@@WRL@Microsoft@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl Microsoft::WRL::Module<1, class Microsoft::WRL::Details::DefaultModule<1> >::IncrementObjectCount(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 2, byte_offset: 16, target_rva: 0x4A10, target_id: Some("?DecrementObjectCount@?$Module@$00V?$DefaultModule@$00@Details@WRL@Microsoft@@@WRL@Microsoft@@UEAAKXZ"), target_name: Some("public: virtual unsigned long __cdecl Microsoft::WRL::Module<1, class Microsoft::WRL::Details::DefaultModule<1> >::DecrementObjectCount(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 3, byte_offset: 24, target_rva: 0x5270, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 4, byte_offset: 32, target_rva: 0x4F80, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 5, byte_offset: 40, target_rva: 0x5110, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 6, byte_offset: 48, target_rva: 0x4F90, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 7, byte_offset: 56, target_rva: 0x5100, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 8, byte_offset: 64, target_rva: 0x5F80, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 9, byte_offset: 72, target_rva: 0x5F80, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 10, byte_offset: 80, target_rva: 0x5F80, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F068, slot: 11, byte_offset: 88, target_rva: 0x5F80, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0C8, slot: 0, byte_offset: 0, target_rva: 0x45B0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0C8, slot: 1, byte_offset: 8, target_rva: 0x7100, target_id: Some("?what@ResultException@wil@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl wil::ResultException::what(void) const"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0D8, slot: 0, byte_offset: 0, target_rva: 0x8710, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0E0, slot: 0, byte_offset: 0, target_rva: 0x86D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0E0, slot: 1, byte_offset: 8, target_rva: 0xAE40, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0E0, slot: 2, byte_offset: 16, target_rva: 0xAE40, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0E0, slot: 3, byte_offset: 24, target_rva: 0x81F0, target_id: Some("?destroy@?$__func@V<lambda_8db0ce862824541f40dfb767113f1e28>@@$$A6A_NPEAX_K01I@Z@__function@wistd@@UEAAXXZ"), target_name: Some("public: virtual void __cdecl wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::destroy(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F0E0, slot: 4, byte_offset: 32, target_rva: 0x8620, target_id: Some("??R?$__func@V<lambda_8db0ce862824541f40dfb767113f1e28>@@$$A6A_NPEAX_K01I@Z@__function@wistd@@UEAA_N$$QEAPEAX$$QEA_K01$$QEAI@Z"), target_name: Some("public: virtual bool __cdecl wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::operator()(void * &&, uint64_t &&, void * &&, uint64_t &&, unsigned int &&)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F108, slot: 0, byte_offset: 0, target_rva: 0x9480, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F108, slot: 1, byte_offset: 8, target_rva: 0x81F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F108, slot: 2, byte_offset: 16, target_rva: 0x9520, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F108, slot: 3, byte_offset: 24, target_rva: 0x8760, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F128, slot: 0, byte_offset: 0, target_rva: 0x9480, target_id: Some("?NotifyFailure@TraceLoggingProvider@wil@@EEAA_NAEBUFailureInfo@2@@Z"), target_name: Some("private: virtual bool __cdecl wil::TraceLoggingProvider::NotifyFailure(struct wil::FailureInfo const &)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F128, slot: 1, byte_offset: 8, target_rva: 0x81F0, target_id: Some("?Initialize@TraceLoggingProvider@wil@@MEAAXXZ"), target_name: Some("protected: virtual void __cdecl wil::TraceLoggingProvider::Initialize(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F128, slot: 2, byte_offset: 16, target_rva: 0x9520, target_id: Some("?OnErrorReported@TraceLoggingProvider@wil@@MEAAX_NAEBUFailureInfo@2@@Z"), target_name: Some("protected: virtual void __cdecl wil::TraceLoggingProvider::OnErrorReported(bool, struct wil::FailureInfo const &)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x1F128, slot: 3, byte_offset: 24, target_rva: 0x87A0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F148, slot: 0, byte_offset: 0, target_rva: 0x8690, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F148, slot: 1, byte_offset: 8, target_rva: 0x3840, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F148, slot: 2, byte_offset: 16, target_rva: 0x3840, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F148, slot: 3, byte_offset: 24, target_rva: 0x3840, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F148, slot: 4, byte_offset: 32, target_rva: 0x3840, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x1F170, slot: 0, byte_offset: 0, target_rva: 0x15420, target_id: Some("?NotifyFailure@?$ThreadFailureCallbackFn@V<lambda_187d33510e6d72dbc662e4da4aa3681d>@@@details@wil@@UEAA_NAEBUFailureInfo@3@@Z"), target_name: Some("public: virtual bool __cdecl wil::details::ThreadFailureCallbackFn<class <lambda_187d33510e6d72dbc662e4da4aa3681d> >::NotifyFailure(struct wil::FailureInfo const &)"), ambiguous: false, this_adjustment: None },
];

/// Reads a raw function pointer from an object's primary vftable.
///
/// # Safety
/// `object` must point to a live object with a readable primary vftable,
/// and `slot` must be valid for that concrete object. This function does
/// not invent or transmute a callable signature.
pub unsafe fn raw_object_slot(object: *const c_void, slot: usize) -> Option<*const ()> {
if object.is_null() { return None; }
let table = unsafe { *(object.cast::<*const *const ()>()) };
if table.is_null() { return None; }
let target = unsafe { *table.add(slot) };
(!target.is_null()).then_some(target)
}
