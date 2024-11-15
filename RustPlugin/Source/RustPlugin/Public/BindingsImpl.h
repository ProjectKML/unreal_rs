#pragma once

#include "Bindings.h"

namespace impl {
    void Log(const char* Message, uintptr_t Size);

    bindings::UWorld* AActor_GetWorld(bindings::AActor* This);

    bindings::UObject* UObject_CreateDefaultSubobject(bindings::UObject* This,
                                                      const uint8_t* SubobjectFNamePtr,
                                                      uintptr_t SubobjectFNameLen,
                                                      bindings::UClass* ReturnType,
                                                      bindings::UClass* ClassToCreateByDefault,
                                                      bool bIsRequired,
                                                      bool bIsTransient);

    bindings::AActor* UWorld_SpawnActor(bindings::UWorld* This,
                                        bindings::UClass* InClass,
                                        const bindings::FVector* Location,
                                        const bindings::FRotator* Rotation,
                                        const bindings::FActorSpawnParameters* SpawnParameters);

    bindings::AActor* UWorld_SpawnECSActor(bindings::UWorld* This,
                                        uint64_t Entity,
                                        const bindings::FVector* Location,
                                        const bindings::FRotator* Rotation,
                                        const bindings::FActorSpawnParameters* SpawnParameters);
}
