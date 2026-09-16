#pragma once
#include "../windissect_forwards.h"

// Reconstructed from coreprivacysettingsstore.dll by Windissect. 5 member(s).
namespace Microsoft::WRL::Details {
class ModuleBase {
public:
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetFirstEntryPointer@ModuleBase@Details@WRL@Microsoft@@UEBAPEAPEBUCreatorMap@234@XZ
    virtual WindissectOpaque const * * GetFirstEntryPointer() const;
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetLastEntryPointer@ModuleBase@Details@WRL@Microsoft@@UEBAPEAPEBUCreatorMap@234@XZ
    virtual WindissectOpaque const * * GetLastEntryPointer() const;
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetLock@ModuleBase@Details@WRL@Microsoft@@UEBAPEAU_RTL_SRWLOCK@@XZ
    virtual _RTL_SRWLOCK * GetLock() const;
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetMidEntryPointer@ModuleBase@Details@WRL@Microsoft@@UEBAPEAPEBUCreatorMap@234@XZ
    virtual WindissectOpaque const * * GetMidEntryPointer() const;
    // Category: Accessor | Source: PDB Internal
    // Symbol: ?GetObjectCount@ModuleBase@Details@WRL@Microsoft@@UEBAKXZ
    virtual unsigned long GetObjectCount() const;
};
} // namespace Microsoft::WRL::Details
