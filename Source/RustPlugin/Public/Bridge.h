#pragma once

#define DEFINE_HANDLE(name) typedef struct {} name;

#ifdef __cplusplus
extern "C" {
#endif
    void ur_log(const char* s, size_t len);
#ifdef __cplusplus
}
#endif