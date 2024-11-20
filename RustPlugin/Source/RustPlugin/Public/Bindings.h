#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace bindings {

using PFN_Log = void(*)(const char*, uintptr_t);

using UWorld = void;

using AActor = void;

using PFN_AActor_GetWorld = UWorld*(*)(AActor *This);

using RustString = void;

using PFN_AActor_GetActorLabel = void(*)(AActor *This, RustString *Name);

using PFN_AActor_SetActorLabel = void(*)(AActor *This, const char *NamePtr, uintptr_t NameLen);

using UClass = void;

using PFN_AActor_StaticClass = UClass*(*)();

using UObject = void;

using PFN_UObject_CreateDefaultSubobject = UObject*(*)(UObject *This,
                                                       const uint8_t *SubobjectFNamePtr,
                                                       uintptr_t SubobjectFNameLen,
                                                       UClass *ReturnType,
                                                       UClass *ClassToCreateByDefault,
                                                       bool bIsRequired,
                                                       bool bIsTransient);

using PFN_UObject_StaticClass = UClass*(*)();

template<typename T>
struct TVector {
  T X;
  T Y;
  T Z;
};

using FVector = TVector<double>;

template<typename T>
struct TRotator {
  T Pitch;
  T Yaw;
  T Roll;
};

using FRotator = TRotator<double>;

struct FActorSpawnParameters {
  const char *NamePtr;
  uintptr_t NameLen;
  AActor *Template;
  AActor *Owner;
};

using PFN_UWorld_SpawnActor = AActor*(*)(UWorld *This,
                                         UClass *InClass,
                                         const FVector *Location,
                                         const FRotator *Rotation,
                                         const FActorSpawnParameters *SpawnParameters);

using PFN_UWorld_SpawnECSActor = AActor*(*)(UWorld *This,
                                            uint64_t Entity,
                                            const FVector *Location,
                                            const FRotator *Rotation,
                                            const FActorSpawnParameters *SpawnParameters);

struct UnrealBindings {
  PFN_Log Log;
  PFN_AActor_GetWorld AActor_GetWorld;
  PFN_AActor_GetActorLabel AActor_GetActorLabel;
  PFN_AActor_SetActorLabel AActor_SetActorLabel;
  PFN_AActor_StaticClass AActor_StaticClass;
  PFN_UObject_CreateDefaultSubobject UObject_CreateDefaultSubobject;
  PFN_UObject_StaticClass UObject_StaticClass;
  PFN_UWorld_SpawnActor UWorld_SpawnActor;
  PFN_UWorld_SpawnECSActor UWorld_SpawnECSActor;
};

using PFN_String_PushStr = void(*)(RustString *s, const char *ptr, uintptr_t len);

using PFN_BeginPlayECS = void(*)(UWorld *world);

using PFN_TickECS = void(*)(UWorld *world, float dt);

struct RustBindings {
  PFN_String_PushStr string_push_str;
  PFN_BeginPlayECS begin_play_ecs;
  PFN_TickECS tick_ecs;
};

using PFN_RegisterModule = uint32_t(*)(const UnrealBindings*, RustBindings*);

}  // namespace bindings
