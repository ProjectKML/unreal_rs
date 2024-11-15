#include "RustGameModeBase.h"
#include "RustPlugin.h"
#include "Modules/ModuleManager.h"

ARustGameModeBase::ARustGameModeBase() {
	PrimaryActorTick.bStartWithTickEnabled = true;
	PrimaryActorTick.bCanEverTick = true;	
}

ARustGameModeBase::~ARustGameModeBase() {}

void ARustGameModeBase::StartPlay() {
	Super::StartPlay();

    const auto& Module = FRustPluginModule::Get();
    const auto& Functions = Module.GetRustFunctions();
	Functions.begin_play_ecs(static_cast<bindings::UWorld*>(GetWorld()));
}

void ARustGameModeBase::Tick(float Dt) {
	Super::Tick(Dt);

	const auto& Module = FRustPluginModule::Get();
	const auto& Functions = Module.GetRustFunctions();
	Functions.tick_ecs(static_cast<bindings::UWorld*>(GetWorld()), Dt);
}