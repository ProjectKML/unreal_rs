#pragma once

#include "CoreMinimal.h"
#include "BindingsImpl.h"
#include "Modules/ModuleManager.h"

class FRustPluginModule : public IModuleInterface
{
private:
    bindings::RustBindings RustFunctions;
	
	bool TryLoadDynamic();
public:
  	static FRustPluginModule& Get();
    static FRustPluginModule* TryGet();

	virtual void StartupModule() override;
	virtual void ShutdownModule() override;

    [[nodiscard]] const bindings::RustBindings& GetRustFunctions() const noexcept {
    	return RustFunctions;
    }
};
