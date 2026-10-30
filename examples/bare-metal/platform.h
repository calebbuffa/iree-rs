#ifndef IREE_RUST_BARE_METAL_PLATFORM_H_
#define IREE_RUST_BARE_METAL_PLATFORM_H_
#include <stdint.h>
#include <stdbool.h>
#ifdef __cplusplus
extern "C" {
#endif
int64_t iree_bare_metal_time_now_ns(void);
bool iree_bare_metal_wait_until_ns(int64_t deadline_ns);
#ifdef __cplusplus
}
#endif
#define IREE_TIME_NOW_FN { return iree_bare_metal_time_now_ns(); }
#define IREE_WAIT_UNTIL_FN iree_bare_metal_wait_until_ns
#endif
