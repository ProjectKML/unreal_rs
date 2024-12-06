#include "RustPlugin.h"

void FRustPluginModule::StartupModule() {
	UE_LOG(LogTemp, Warning, TEXT("FRustPluginModule::StartupModule"));
}

void FRustPluginModule::ShutdownModule() {
	UE_LOG(LogTemp, Warning, TEXT("FRustPluginModule::ShutdownModule"));
}