#include "Bridge.h"

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

#ifdef __cplusplus
extern "C" {
#endif
    void ur_log(const char* message_ptr, size_t message_len) {
        FString message(message_len, UTF8_TO_TCHAR(message_ptr));
    }

    //AActor
    UR_Class* ur_actor_get_class(UR_Actor* self) {
        return reinterpret_cast<UR_Class*>(reinterpret_cast<AActor*>(self)->GetClass());
    }

    // FFloatProperty
    void ur_float_property_get_value_in_container(UR_FloatProperty* self, const void* in_container, float* out_value) {
        reinterpret_cast<FFloatProperty*>(self)->GetValue_InContainer(in_container, out_value);
    }

    void ur_float_property_set_value_in_container(UR_FloatProperty* self, void* out_container, float in_value) {
        reinterpret_cast<FFloatProperty*>(self)->SetValue_InContainer(out_container, in_value);
    }

    // FIntProperty
    void ur_int_property_get_value_in_container(UR_IntProperty* self, const void* in_container, int32_t* out_value) {
        reinterpret_cast<FIntProperty*>(self)->GetValue_InContainer(in_container, out_value);
    }

    void ur_int_property_set_value_in_container(UR_IntProperty* self, void* out_container, int32_t in_value) {
        reinterpret_cast<FIntProperty*>(self)->SetValue_InContainer(out_container, in_value);
    }

    //UClass
    UR_Property* ur_class_find_property_by_name(UR_Class* self, const char* name_ptr, const size_t name_len) {
        FName name(name_len, UTF8_TO_TCHAR(name_ptr));

        return reinterpret_cast<UR_Property*>(reinterpret_cast<UClass*>(self)->FindPropertyByName(name));
    }
#ifdef __cplusplus
}
#endif