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

    const auto* Module = FRustPluginModule::TryGet();
    if (!Module) {
      	UE_LOG(LogTemp, Warning, TEXT("StartPlay failed"));
    	return;
    }

    const auto& Functions = Module->GetRustFunctions();
    Functions.begin_play_ecs();

    Initialized = true;
}

void ARustGameModeBase::Tick(float Dt) {
	Super::Tick(Dt);

    const auto* Module = FRustPluginModule::TryGet();
    if (!Module) {
        UE_LOG(LogTemp, Warning, TEXT("Tick failed"));
    	return;
    }

    const auto& Functions = Module->GetRustFunctions();
    if (!Initialized) {
    	Initialized = true;
        Functions.begin_play_ecs();
    }

	Functions.tick_ecs(Dt);
}