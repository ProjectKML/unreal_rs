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

using UObject = void;

using UClass = void;

using PFN_UObject_CreateDefaultSubobject = UObject*(*)(UObject *This,
                                                       const uint8_t *SubobjectFNamePtr,
                                                       uintptr_t SubobjectFNameLen,
                                                       UClass *ReturnType,
                                                       UClass *ClassToCreateByDefault,
                                                       bool bIsRequired,
                                                       bool bIsTransient);

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

using PFN_UWorld_SpawnActor = AActor*(*)(UWorld *This,
                                         UClass *InClass,
                                         const FVector *Location,
                                         const FRotator *Rotation);

struct UnrealBindings {
  PFN_Log Log;
  PFN_AActor_GetWorld AActor_GetWorld;
  PFN_UObject_CreateDefaultSubobject UObject_CreateDefaultSubobject;
  PFN_UWorld_SpawnActor UWorld_SpawnActor;
};

using PFN_BeginPlayECS = void(*)(UWorld *world);

using PFN_TickECS = void(*)(UWorld *world, float dt);

struct RustBindings {
  PFN_BeginPlayECS begin_play_ecs;
  PFN_TickECS tick_ecs;
};

using PFN_RegisterModule = uint32_t(*)(const UnrealBindings*, RustBindings*);

}  // namespace bindings
