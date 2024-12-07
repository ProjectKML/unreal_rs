#include "Bridge.cpp"

extern "C" {
    void ur_register_module();
}

namespace RustModule {
    void Init() {
        UE_LOG(LogTemp, Warning, TEXT("FRustPluginModule::StartupModule"));

        ur_register_module();
    }

    void Shutdown() {
        UE_LOG(LogTemp, Warning, TEXT("FRustPluginModule::ShutdownModule"));
    }
}