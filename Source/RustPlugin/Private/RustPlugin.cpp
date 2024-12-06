#include "RustPlugin.h"

#define LOCTEXT_NAMESPACE "FRustPluginModule"

extern "C" {
	void ur_register_module();
}

void FRustPluginModule::StartupModule() {
	UE_LOG(LogTemp, Warning, TEXT("FRustPluginModule::StartupModule"));

    ur_register_module();
}

void FRustPluginModule::ShutdownModule() {
	UE_LOG(LogTemp, Warning, TEXT("FRustPluginModule::ShutdownModule"));
}

#undef LOCTEXT_NAMESPACE

IMPLEMENT_PRIMARY_GAME_MODULE(FRustPluginModule, RustPlugin, "RustPlugin");