#pragma once

#define DEFINE_HANDLE(name) typedef void* UR_##name;

DEFINE_HANDLE(Actor);
DEFINE_HANDLE(Class);
DEFINE_HANDLE(FloatProperty);
DEFINE_HANDLE(IntProperty);
DEFINE_HANDLE(Property);
DEFINE_HANDLE(Struct);

#ifdef __cplusplus
extern "C" {
#else
#include <stdint.h>
#include <string.h>
#endif
    void ur_log(const char* message_ptr, size_t message_len);

    //AActor
    UR_Class* ur_actor_get_class(UR_Actor* self);

    //FFloatProperty
    void ur_float_property_get_value_in_container(UR_FloatProperty* self, const void* in_container, float* out_value);
    void ur_float_property_set_value_in_container(UR_FloatProperty* self, void* out_container, float in_value);

    //FIntProperty
    void ur_int_property_get_value_in_container(UR_IntProperty* self, const void* in_container, int32_t* out_value);
    void ur_int_property_set_value_in_container(UR_IntProperty* self, void* out_container, int32_t in_value);

    //UClass

    //UStruct
    UR_Property* ur_struct_find_property_by_name(UR_Struct* self, const char* name_ptr, const size_t name_len);
#ifdef __cplusplus
}
#endif
