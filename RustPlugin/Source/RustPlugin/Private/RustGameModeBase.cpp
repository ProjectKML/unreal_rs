#include "RustGameModeBase.h"

ARustGameModeBase::ARustGameModeBase() {
	PrimaryActorTick.bStartWithTickEnabled = true;
	PrimaryActorTick.bCanEverTick = true;	
}

ARustGameModeBase::~ARustGameModeBase() {
	
}

void ARustGameModeBase::StartPlay() {
	Super::StartPlay();
	
	UE_LOG(LogTemp, Warning, TEXT("ARustGameModeBase::StartPlay"));
}

void ARustGameModeBase::Tick(float Dt) {
	Super::Tick(Dt);
	UE_LOG(LogTemp, Warning, TEXT("ARustGameModeBase::Tick"));
}