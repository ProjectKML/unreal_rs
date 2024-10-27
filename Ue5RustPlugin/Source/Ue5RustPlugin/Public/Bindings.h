#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct UnrealBindings {

};

struct RustBindings {

};

using PFN_RegisterModule = int32_t(*)(UnrealBindings*, RustBindings*);
