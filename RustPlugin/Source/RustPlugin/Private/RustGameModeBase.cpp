#include "RustGameModeBase.h"
#include "RustPlugin.h"

ARustGameModeBase::ARustGameModeBase() {
	PrimaryActorTick.bStartWithTickEnabled = true;
	PrimaryActorTick.bCanEverTick = true;	
}

ARustGameModeBase::~ARustGameModeBase() {
	
}

void ARustGameModeBase::StartPlay() {
	Super::StartPlay();

    const auto& Functions = FRustPluginModule::Get().GetRustFunctions();
    Functions.begin_play_ecs();
}

void ARustGameModeBase::Tick(float Dt) {
	Super::Tick(Dt);

    const auto& Functions = FRustPluginModule::Get().GetRustFunctions();
    Functions.tick_ecs(Dt);
}