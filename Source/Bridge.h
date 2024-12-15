#pragma once

#define DEFINE_HANDLE(name) typedef void* name;

#ifdef __cplusplus
extern "C" {
#else
#include <stdint.h>
#include <string.h>
#endif
    void ur_log(const char* s, size_t len);
#ifdef __cplusplus
}
#endif
