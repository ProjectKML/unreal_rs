#include "BindingsImpl.h"
#include "GameFramework/Actor.h"

namespace BindingsImpl {
    void Log(const char* S, uintptr_t Len) {
        FString Message = FString(Len, UTF8_TO_TCHAR(S));
        UE_LOG(LogTemp, Warning, TEXT("%s"), *Message);
    }
}