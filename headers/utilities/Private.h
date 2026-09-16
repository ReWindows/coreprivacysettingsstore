#pragma once
#include "../windissect_forwards.h"

// Reconstructed from coreprivacysettingsstore.dll by Windissect. 18 member(s).
namespace Windows::Internal::CorePrivacySettingsStore {
class Private {
public:
    class CPSSKeyStringGroup;
    class CPSSPolicyBase;
    class CPSSPolicyWrapper;
    class CPSSPolicy_DefaultValue;
    class CPSSPolicy_InheritsFromDevice;
    class CPSSPolicy_LegacyKeyName;
    class CPSSPolicy_LegacyKeyType;
    class CPSSPolicy_LegacyProjection;
    class CPSSPolicy_ManageLegacyKeySecurity;
    class CPSSPolicy_StaticMigrationPath;
    class CPSSPolicy_StaticMigrationPathSS;
    class Common;
    class InternalCPSSKey;
    class StaticMigrationPathWrapper;
public /*unspecified*/:
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetMostSpecificApplicablePolicy@Private@CorePrivacySettingsStore@Internal@Windows@@YAJAEAUCPSSPolicyWrapper@1234@AEBVInternalCPSSKey@1234@AEBUCPSSPolicyBase@1234@@Z
    long GetMostSpecificApplicablePolicy(WindissectOpaque &, WindissectOpaque const &, WindissectOpaque const &);
    // Category: Method | Source: PDB Internal
    // Symbol: ?InheritsFromDeviceEnabled@Private@CorePrivacySettingsStore@Internal@Windows@@YA_NAEBVInternalCPSSKey@1234@@Z
    bool InheritsFromDeviceEnabled(WindissectOpaque const &);
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?IsProjectionRequired@Private@CorePrivacySettingsStore@Internal@Windows@@YA_NAEBVInternalCPSSKey@1234@@Z
    bool IsProjectionRequired(WindissectOpaque const &);
    // Category: Method | Source: PDB Internal
    // Symbol: ?ManageLegacyKeySecurityEnabled@Private@CorePrivacySettingsStore@Internal@Windows@@YA_NAEBVInternalCPSSKey@1234@@Z
    bool ManageLegacyKeySecurityEnabled(WindissectOpaque const &);
    // Category: Method | Source: PDB Internal
    // Symbol: ?PutStringSettingInternal@Private@CorePrivacySettingsStore@Internal@Windows@@YAXAEBVInternalCPSSKey@1234@PEBG1@Z
    void PutStringSettingInternal(WindissectOpaque const &, unsigned short const *, unsigned short const *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?QueryAllComponentsInternal@Private@CorePrivacySettingsStore@Internal@Windows@@YAXIHPEAXPEAIPEAPEAPEAG@Z
    void QueryAllComponentsInternal(unsigned int, int, void *, unsigned int *, unsigned short * * *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?QueryKeysForComponentInternal@Private@CorePrivacySettingsStore@Internal@Windows@@YAXIHPEAXPEBGPEAIPEAPEAU_CPSS_UNIQUE_KEY@@@Z
    void QueryKeysForComponentInternal(unsigned int, int, void *, unsigned short const *, unsigned int *, _CPSS_UNIQUE_KEY * *);
};
} // namespace Windows::Internal::CorePrivacySettingsStore
