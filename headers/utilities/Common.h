#pragma once
#include "../windissect_forwards.h"

// Reconstructed from coreprivacysettingsstore.dll by Windissect. 2 member(s).
namespace Windows::Internal::CorePrivacySettingsStore::Private {
class Common {
public /*unspecified*/:
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?IsRecoverableRegistryError@Common@Private@CorePrivacySettingsStore@Internal@Windows@@YA_NJ@Z
    bool IsRecoverableRegistryError(long);
    // Category: Method | Source: PDB Internal
    // Symbol: ?RegistryFlagsFromDwordType@Common@Private@CorePrivacySettingsStore@Internal@Windows@@YAKK@Z
    unsigned long RegistryFlagsFromDwordType(unsigned long);
};
} // namespace Windows::Internal::CorePrivacySettingsStore::Private
