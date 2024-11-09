#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using PFN_Log = void(*)(const char*, uintptr_t);

struct UnrealBindings {
  PFN_Log log;
};

struct RustBindings {

};

using PFN_RegisterModule = uint32_t(*)(const UnrealBindings*, RustBindings*);
