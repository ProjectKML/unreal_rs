#pragma once

#include "Bindings.h"

namespace impl {
    void Log(const char* Message, uintptr_t Size);

    bindings::UWorld* AActor_GetWorld(bindings::AActor* This);
    void AActor_GetActorLabel(bindings::AActor* This, bindings::RustString *Name);
    void AActor_SetActorLabel(bindings::AActor* This, const char* NamePtr, uintptr_t NameLen);
    bindings::UClass* AActor_StaticClass();

    bindings::UClass* UActorComponent_StaticClass();

    bindings::UClass* UMeshComponent_StaticClass();

    bindings::UObject* UObject_CreateDefaultSubobject(bindings::UObject* This,
                                                      const uint8_t* SubobjectFNamePtr,
                                                      uintptr_t SubobjectFNameLen,
                                                      bindings::UClass* ReturnType,
                                                      bindings::UClass* ClassToCreateByDefault,
                                                      bool bIsRequired,
                                                      bool bIsTransient);
    bindings::UClass* UObject_StaticClass();

    bindings::UClass* UPrimitiveComponent_StaticClass();

    bindings::UClass* USceneComponent_StaticClass();

    bindings::UClass* UStaticMeshComponent_StaticClass();

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
