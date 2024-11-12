#pragma once

#include "CoreMinimal.h"
#include "BindingsImpl.h"
#include "Modules/ModuleManager.h"

struct FPlugin final
{
	FPlugin();

	bool TryLoad();
};

class FRustPluginModule : public IModuleInterface
{
private:
	FPlugin Plugin;

    RustBindings RustFunctions;
public:
  	static FRustPluginModule& Get();

	virtual void StartupModule() override;
	virtual void ShutdownModule() override;

    [[nodiscard]] const RustBindings& GetRustFunctions() const noexcept {
    	return RustFunctions;
    }
};
