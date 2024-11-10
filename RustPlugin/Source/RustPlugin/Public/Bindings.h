#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using PFN_Log = void(*)(const char*, uintptr_t);

struct UnrealBindings {
  PFN_Log log;
};

using PFN_BeginPlayECS = void(*)();

using PFN_TickECS = void(*)(float dt);

struct RustBindings {
  PFN_BeginPlayECS begin_play_ecs;
  PFN_TickECS tick_ecs;
};

using PFN_RegisterModule = uint32_t(*)(const UnrealBindings*, RustBindings*);
