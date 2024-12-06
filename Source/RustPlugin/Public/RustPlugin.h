#pragma once

#include "CoreMinimal.h"

class FRustPluginModule : public IModuleInterface {
public:
	virtual void StartupModule() override;
	virtual void ShutdownModule() override;
};
