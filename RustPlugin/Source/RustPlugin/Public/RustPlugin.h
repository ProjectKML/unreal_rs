#pragma once

#include "CoreMinimal.h"
#include "Modules/ModuleManager.h"

struct FPlugin final
{
	FPlugin();

	bool TryLoad();
};

class RustPluginModule : public IModuleInterface
{
private:
	FPlugin Plugin;
public:

	virtual void StartupModule() override;
	virtual void ShutdownModule() override;
};
