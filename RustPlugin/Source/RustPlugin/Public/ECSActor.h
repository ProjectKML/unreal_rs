#pragma once

#include "GameFramework/Actor.h"
#include "ECSActor.generated.h"

UCLASS()
class AECSActor : public AActor {
    GENERATED_BODY()
protected:
    uint64 Entity;

    virtual void BeginPlay() override;
public:
    AECSActor();

    virtual void Tick(float DeltaTime) override;

    uint64 GetEntity() const {
        return Entity;
    }
};