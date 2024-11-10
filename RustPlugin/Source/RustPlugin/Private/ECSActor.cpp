#include "ECSActor.h"

AECSActor::AECSActor() {
    PrimaryActorTick.bCanEverTick = true;
}

void AECSActor::BeginPlay() {
    Super::BeginPlay();
}

void AECSActor::Tick(float DeltaTime) {
    Super::Tick(DeltaTime);
}