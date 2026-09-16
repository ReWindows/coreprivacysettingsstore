// Flat C exports observed in coreprivacysettingsstore.dll. Unknown ABIs are intentionally not declared.
// Validated dialects: ISO C17/C23/C26 draft and Windissect C20 (C ABI under C++20).
#pragma once
#ifndef COREPRIVACYSETTINGSSTORE_C_H
#define COREPRIVACYSETTINGSSTORE_C_H
#if defined(__cplusplus)
#  if __cplusplus < 202002L
#    define WINDISSECT_C20_PROFILE 0
#  else
#    define WINDISSECT_C20_PROFILE 1
#  endif
extern "C" {
#else
#  if defined(__STDC_VERSION__) && __STDC_VERSION__ < 201710L
#    error "Windissect C output requires C17 or newer"
#  endif
#  define WINDISSECT_C20_PROFILE 0
#  define WINDISSECT_C_STANDARD __STDC_VERSION__
#endif

// Export: DllCanUnloadNow (ABI unverified)
// Export: CPSSFreeComponentArray (ABI unverified)
// Export: CPSSFreeKeyArray (ABI unverified)
// Export: CPSSFreeStringSetting (ABI unverified)
// Export: CPSSGetBoolSetting (ABI unverified)
// Export: CPSSGetBoolSettingByKey (ABI unverified)
// Export: CPSSGetDwordSetting (ABI unverified)
// Export: CPSSGetDwordSettingByKey (ABI unverified)
// Export: CPSSGetStringSetting (ABI unverified)
// Export: CPSSGetStringSettingByKey (ABI unverified)
// Export: CPSSPeekBoolSetting (ABI unverified)
// Export: CPSSPeekBoolSettingByKey (ABI unverified)
// Export: CPSSPeekDwordSetting (ABI unverified)
// Export: CPSSPeekDwordSettingByKey (ABI unverified)
// Export: CPSSPeekStringSetting (ABI unverified)
// Export: CPSSPeekStringSettingByKey (ABI unverified)
// Export: CPSSPutBoolSetting (ABI unverified)
// Export: CPSSPutBoolSettingByKey (ABI unverified)
// Export: CPSSPutDwordSetting (ABI unverified)
// Export: CPSSPutDwordSettingByKey (ABI unverified)
// Export: CPSSPutStringSetting (ABI unverified)
// Export: CPSSPutStringSettingByKey (ABI unverified)
// Export: CPSSQueryAllComponents (ABI unverified)
// Export: CPSSQueryKeysForComponent (ABI unverified)
// Export: CPSSRemoveSetting (ABI unverified)
// Export: CPSSRemoveSettingByKey (ABI unverified)

#ifdef __cplusplus
} // extern "C"
#endif
#endif // COREPRIVACYSETTINGSSTORE_C_H
