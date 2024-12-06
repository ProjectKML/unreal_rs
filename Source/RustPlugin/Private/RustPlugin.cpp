#include "RustPlugin.h"

#define LOCTEXT_NAMESPACE "FRustPluginModule"

void FRustPluginModule::StartupModule() {
	UE_LOG(LogTemp, Warning, TEXT("FRustPluginModule::StartupModule"));
}

void FRustPluginModule::ShutdownModule() {
	UE_LOG(LogTemp, Warning, TEXT("FRustPluginModule::ShutdownModule"));
}

#undef LOCTEXT_NAMESPACE

IMPLEMENT_PRIMARY_GAME_MODULE(FRustPluginModule, RustPlugin, "RustPlugin");