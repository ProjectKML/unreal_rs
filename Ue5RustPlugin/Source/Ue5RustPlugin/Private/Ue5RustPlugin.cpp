#include "Ue5RustPlugin.h"

#define LOCTEXT_NAMESPACE "FUe5RustPluginModule"

FPlugin::FPlugin()
{

}

bool FPlugin::TryLoad()
{
	const auto LocalTargetPath = FPaths::Combine("C:/Users/Lorenz Klaus/Documents/Projects/ue5_rs/target/debug/ue5_example.dll");

	auto* LocalHandle = FPlatformProcess::GetDllHandle(*LocalTargetPath);
	if (LocalHandle == nullptr)
	{
		UE_LOG(LogTemp, Warning, TEXT("Failed to load dynamic library"));
		return false;
	}

	auto* RegisterModule = static_cast<PFN_register_module>(FPlatformProcess::GetDllExport(LocalHandle, TEXT("ue5_register_module")));
	if (RegisterModule == nullptr)
	{
		UE_LOG(LogTemp, Warning, TEXT("Failed to load entry point"));
		return false;
	}

	/*FUnrealBindings UnrealBindings;
	FRustBindings RustBindings;

	const auto Result = RegisterModule(&UnrealBindings, &RustBindings);
	if (Result != 0)
	{
		UE_LOG(LogTemp, Warning, TEXT("Register rust module failed: %d"), Result);
		return false;
	}*/

	return true;
}

void FUe5RustPluginModule::StartupModule()
{
	Plugin.TryLoad();
}

void FUe5RustPluginModule::ShutdownModule()
{

}

#undef LOCTEXT_NAMESPACE

IMPLEMENT_PRIMARY_GAME_MODULE(FUe5RustPluginModule, Ue5RustPlugin, "Ue5RustPlugin");