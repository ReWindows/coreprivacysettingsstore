#pragma once
#include "../windissect_forwards.h"

// Reconstructed from coreprivacysettingsstore.dll by Windissect. 4 member(s).
namespace Windows::Internal::CorePrivacySettingsStore::Private {
class InternalCPSSKey {
public:
    // Category: Ctor | Source: PDB Internal
    // Symbol: ??0InternalCPSSKey@Private@CorePrivacySettingsStore@Internal@Windows@@QEAA@IHPEAXW4_CPSS_SETTINGS_TYPE@@PEBG222@Z
    InternalCPSSKey(unsigned int, int, void *, int, unsigned short const *, unsigned short const *, unsigned short const *, unsigned short const *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?LogKeyDetails@InternalCPSSKey@Private@CorePrivacySettingsStore@Internal@Windows@@QEBAX_N@Z
    void LogKeyDetails(bool) const;
    // Category: Dtor | Source: PDB Internal
    // Symbol: ??1InternalCPSSKey@Private@CorePrivacySettingsStore@Internal@Windows@@QEAA@XZ
    ~InternalCPSSKey();
private:
    // Category: Method | Source: PDB Internal
    // Symbol: ?Initialize@InternalCPSSKey@Private@CorePrivacySettingsStore@Internal@Windows@@AEAAXIHPEAXAEBW4_CPSS_SETTINGS_TYPE@@PEBG222@Z
    void Initialize(unsigned int, int, void *, int const &, unsigned short const *, unsigned short const *, unsigned short const *, unsigned short const *);
};
} // namespace Windows::Internal::CorePrivacySettingsStore::Private
