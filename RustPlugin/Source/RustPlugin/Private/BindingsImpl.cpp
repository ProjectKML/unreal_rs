#include "BindingsImpl.h"
#include "ECSActor.h"
#include "GameFramework/Actor.h"
#include "RustPlugin.h"

namespace impl {
    void Log(const char* S, uintptr_t Len) {
        FString Message = FString(Len, UTF8_TO_TCHAR(S));
        UE_LOG(LogTemp, Warning, TEXT("%s"), *Message);
    }

    bindings::UWorld* AActor_GetWorld(bindings::AActor* This) {
        return static_cast<bindings::UWorld*>(static_cast<AActor*>(This)->GetWorld());
    }

    void AActor_GetActorLabel(bindings::AActor *This, bindings::RustString *Name) {
     	const auto& NameStr = static_cast<AActor*>(This)->GetActorLabel();

        const auto& RustFunctions = FRustPluginModule::Get().GetRustFunctions();
        //TODO:
    }

    void AActor_SetActorLabel(bindings::AActor *This, const char *NamePtr, uintptr_t NameLen) {
    	FString Name(NameLen, UTF8_TO_TCHAR(NamePtr));
        static_cast<AActor*>(This)->SetActorLabel(Name);
    }

    bindings::UObject* UObject_CreateDefaultSubobject(bindings::UObject* This,
                                                      const uint8_t* SubobjectFNamePtr,
                                                      uintptr_t SubobjectFNameLen,
                                                      bindings::UClass* ReturnType,
                                                      bindings::UClass* ClassToCreateByDefault,
                                                      bool bIsRequired,
                                                      bool bIsTransient) {
        FName SubobjectFName(SubobjectFNameLen, UTF8_TO_TCHAR(SubobjectFNamePtr));
        return static_cast<bindings::UObject*>(static_cast<UObject*>(This)->CreateDefaultSubobject(
            SubobjectFName,
            static_cast<UClass*>(ReturnType),
            static_cast<UClass*>(ClassToCreateByDefault),
            bIsRequired,
            bIsTransient));
    }

    static inline FActorSpawnParameters GetActorSpawnParameters(const bindings::FActorSpawnParameters* SpawnParameters) {
        FActorSpawnParameters Result;

        if (SpawnParameters) {
            if (SpawnParameters->NamePtr) {
                Result.Name = FName(SpawnParameters->NameLen, UTF8_TO_TCHAR(SpawnParameters->NamePtr));
              	UE_LOG(LogTemp, Warning, TEXT("FName value: %s"), *Result.Name.ToString());
            }
            Result.Template = static_cast<AActor*>(SpawnParameters->Template);
            Result.Owner = static_cast<AActor*>(SpawnParameters->Owner);
        }

        return Result;
    }

    bindings::AActor* UWorld_SpawnActor(bindings::UWorld* This,
                                        bindings::UClass* InClass,
                                        const bindings::FVector* Location,
                                        const bindings::FRotator* Rotation,
                                        const bindings::FActorSpawnParameters* SpawnParameters) {
        return static_cast<bindings::AActor*>(static_cast<UWorld*>(This)->SpawnActor(
            static_cast<UClass*>(InClass),
            reinterpret_cast<const FVector*>(Location),
            reinterpret_cast<const FRotator*>(Rotation),
            GetActorSpawnParameters(SpawnParameters)));
    }

    bindings::AActor* UWorld_SpawnECSActor(bindings::UWorld* This,
                                           uint64_t Entity,
                                           const bindings::FVector* Location,
                                           const bindings::FRotator* Rotation,
                                           const bindings::FActorSpawnParameters* SpawnParameters) {
        auto* Actor = Cast<AECSActor>(static_cast<UWorld*>(This)->SpawnActor(
            AECSActor::StaticClass(),
            reinterpret_cast<const FVector*>(Location),
            reinterpret_cast<const FRotator*>(Rotation),
            GetActorSpawnParameters(SpawnParameters)));

        Actor->Entity = Entity;

        return static_cast<bindings::AActor*>(Actor);
    }
}