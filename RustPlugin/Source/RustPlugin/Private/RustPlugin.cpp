#include "RustPlugin.h"

#define LOCTEXT_NAMESPACE "FRustPluginModule"

static FRustPluginModule* _SINGLETON = nullptr;

FRustPluginModule& FRustPluginModule::Get() {
	return *_SINGLETON;
}

FRustPluginModule* FRustPluginModule::TryGet() {
	return _SINGLETON;
}

bool FRustPluginModule::TryLoadDynamic() {
	const auto LocalTargetPath = FPaths::Combine("C:/Users/Lorenz Klaus/Documents/Projects/ue5_rs/target/debug/unreal_example.dll");

	auto* LocalHandle = FPlatformProcess::GetDllHandle(*LocalTargetPath);
	if (LocalHandle == nullptr) {
		UE_LOG(LogTemp, Warning, TEXT("Failed to load dynamic library"));
		return false;
	}

	auto* RegisterModule = static_cast<PFN_RegisterModule>(FPlatformProcess::GetDllExport(LocalHandle, TEXT("unreal_register_module")));
	if (RegisterModule == nullptr) {
		UE_LOG(LogTemp, Warning, TEXT("Failed to load entry point"));
		return false;
	}

	UnrealBindings UnrealBindings = {};
	UnrealBindings.log = BindingsImpl::Log;

	UE_LOG(LogTemp, Warning, TEXT("Starting register"));

	const auto Result = RegisterModule(&UnrealBindings, &RustFunctions);
	if (Result != 0) {
		UE_LOG(LogTemp, Warning, TEXT("Register rust module failed: %d"), Result);
		return false;
	}

	return true;
}

void FRustPluginModule::StartupModule() {
    TryLoadDynamic();

    _SINGLETON = this;
}

void FRustPluginModule::ShutdownModule() {
	_SINGLETON = nullptr;
}

#undef LOCTEXT_NAMESPACE

IMPLEMENT_PRIMARY_GAME_MODULE(FRustPluginModule, RustPlugin, "RustPlugin");