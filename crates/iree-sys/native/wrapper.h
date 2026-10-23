#ifndef IREE_RS_WRAPPER_H_
#define IREE_RS_WRAPPER_H_

#if defined(__linux__)
// IREE's inline futex helpers use syscall and timespec.
#ifndef _GNU_SOURCE
#define _GNU_SOURCE 1
#endif
#include <time.h>
#endif

#include "iree/runtime/api.h"
#include "iree/async/api.h"
#if IREE_RS_TASK
#include "iree/task/api.h"
#endif
#include "iree/io/file_contents.h"
#include "iree/io/parameter_index.h"
#include "iree/io/parameter_index_provider.h"
#include "iree/io/scope_map.h"
#include "iree/io/memory_stream.h"
#include "iree/io/stdio_stream.h"
#include "iree/io/vec_stream.h"
#include "iree/io/formats/parser_registry.h"
#include "iree/io/formats/gguf/gguf_parser.h"
#include "iree/io/formats/irpa/irpa_parser.h"
#include "iree/io/formats/irpa/irpa_builder.h"
#include "iree/io/formats/safetensors/safetensors_parser.h"
#include "iree/modules/io/parameters/module.h"
#include "iree/modules/hal/debugging.h"
#include "iree/async/util/proactor_pool.h"
#include "iree/base/threading/numa.h"
#include "iree/async/frontier_tracker.h"
#include "iree/modules/hal/module.h"
#include "iree/hal/drivers/init.h"
#include "iree/vm/bytecode/module.h"
#include "iree/hal/buffer_transfer.h"
#if IREE_RS_COMPILER
#include "iree/compiler/embedding_api.h"
#include "iree/compiler/mlir_interop.h"
#include "iree/compiler/tool_entry_points_api.h"
// v3.13.0 dialect headers use C++ struct-name lookup in their C API.
// Forward aliases make the unchanged upstream declarations valid C as well.
typedef struct ireeCodegenTranslationInfoParameters ireeCodegenTranslationInfoParameters;
typedef struct ireeCodegenCompilationInfoParameters ireeCodegenCompilationInfoParameters;
typedef struct ireeCodegenAttentionOpDetail ireeCodegenAttentionOpDetail;
typedef struct ireeCodegenIGEMMGenericConvDetails ireeCodegenIGEMMGenericConvDetails;
typedef struct ireeCodegenScaledContractionDimensions ireeCodegenScaledContractionDimensions;
typedef struct ireeGPUMMAInfo ireeGPUMMAInfo;
typedef struct ireeGPUTileSizes ireeGPUTileSizes;
typedef struct ireeGPUSubgroupBasisInfo ireeGPUSubgroupBasisInfo;
typedef struct ireeGPUMMASingleSubgroupLayout ireeGPUMMASingleSubgroupLayout;
typedef struct ireeGPUTargetInfo ireeGPUTargetInfo;
#include "iree/compiler/dialects/iree_codegen.h"
#include "iree/compiler/dialects/iree_gpu.h"
#endif
#if IREE_RS_CUDA
#include "iree/hal/drivers/cuda/api.h"
#endif
#if IREE_RS_HIP
#include "iree/hal/drivers/hip/api.h"
#endif
#if IREE_RS_AMDGPU
#include "iree/hal/drivers/amdgpu/api.h"
#endif
#if IREE_RS_METAL
#include "iree/hal/drivers/metal/api.h"
#endif
#if IREE_RS_NULL
#include "iree/hal/drivers/null/api.h"
#endif
#if IREE_RS_WEBGPU
#include "iree/hal/drivers/webgpu/api.h"
#endif
#if IREE_RS_LOCAL_SYNC
#include "iree/hal/drivers/local_sync/sync_device.h"
#endif
#if IREE_RS_LOCAL_TASK
#include "iree/hal/drivers/local_task/task_device.h"
#endif
#if IREE_RS_CPU
#include "iree/hal/local/executable_library.h"
#include "iree/hal/local/executable_plugin.h"
#include "iree/hal/local/loaders/embedded_elf_loader.h"
#include "iree/hal/local/loaders/system_library_loader.h"
#include "iree/hal/local/loaders/static_library_loader.h"
#include "iree/hal/local/loaders/vmvx_module_loader.h"
#include "iree/hal/local/plugins/embedded_elf_plugin.h"
#include "iree/hal/local/plugins/system_library_plugin.h"
#include "iree/hal/local/plugins/static_plugin.h"
#include "iree/modules/vmvx/module.h"
#endif
#if IREE_RS_VULKAN
#include "iree/hal/drivers/vulkan/api.h"
#include "iree/hal/drivers/vulkan/buffer.h"
#endif

iree_allocator_t iree_rs_allocator_system(void);
iree_allocator_t iree_rs_allocator_null(void);
iree_status_code_t iree_rs_status_code(iree_status_t status);
void iree_rs_status_free(iree_status_t status);
iree_timeout_t iree_rs_infinite_timeout(void);
// Returns default device creation parameters (upstream inline helper).
iree_hal_device_create_params_t iree_rs_device_create_params_default(void);
// Creates a pool with one lazily-created proactor per online NUMA node and the
// platform's default threaded poll runner. The caller owns one reference.
iree_status_t iree_rs_proactor_pool_create_default(
    iree_allocator_t host_allocator, iree_async_proactor_pool_t** out_pool);
iree_async_frontier_tracker_options_t iree_rs_frontier_tracker_options_default(
    void);
// Untyped VM list element type (C bitfield struct; not constructible in Rust).
iree_vm_type_def_t iree_rs_vm_undefined_type_def(void);
// Registered VM ref type of iree_hal_buffer_view_t (inline upstream).
iree_vm_ref_type_t iree_rs_hal_buffer_view_type(void);
// Creates a frontier tracker, a single-device group (assigning the device's
// queue frontiers exactly once), and a HAL module over that group, with
// buffer-view trace output routed to stderr. The caller owns one module
// reference. Create at most once per device and share the module across all
// VM contexts using the device; it retains the group, which retains the device.
iree_status_t iree_rs_device_hal_module_create(iree_vm_instance_t* instance,
                                             iree_hal_device_t* device,
                                             iree_allocator_t host_allocator,
                                             iree_vm_module_t** out_module);
// Formats and consumes status. Returns required bytes excluding NUL; a result
// >= capacity signals truncation. A nonempty destination is always terminated.
iree_host_size_t iree_rs_status_message(iree_status_t status, char* destination,
                                      iree_host_size_t capacity);
iree_hal_buffer_t* iree_rs_buffer_allocated_buffer(iree_hal_buffer_t* buffer);
iree_device_size_t iree_rs_buffer_byte_offset(iree_hal_buffer_t* buffer);
iree_device_size_t iree_rs_buffer_byte_length(iree_hal_buffer_t* buffer);
iree_device_size_t iree_rs_buffer_allocation_size(iree_hal_buffer_t* buffer);
void iree_rs_hal_buffer_retain(iree_hal_buffer_t* buffer);
void iree_rs_hal_buffer_release(iree_hal_buffer_t* buffer);
void iree_rs_hal_buffer_view_retain(iree_hal_buffer_view_t* view);
void iree_rs_hal_buffer_view_release(iree_hal_buffer_view_t* view);
void iree_rs_hal_device_retain(iree_hal_device_t* device);
void iree_rs_hal_device_release(iree_hal_device_t* device);

#if IREE_RS_VULKAN
// Allocates one standalone dense Vulkan buffer with exclusive VkBuffer and
// VkDeviceMemory at backing offset 0, bypassing the device allocator's pools
// (upstream iree_hal_vulkan_allocator_allocate_direct_buffer). |device| must be
// a Vulkan logical device whose allocator was not replaced. Returns
// OUT_OF_RANGE (and allocates nothing) if the size would require a sparse
// buffer. Contents are uninitialized. The caller owns one buffer reference.
iree_status_t iree_rs_vulkan_allocate_dedicated_buffer(
    iree_hal_device_t* device, iree_hal_buffer_params_t params,
    iree_device_size_t allocation_size, iree_hal_buffer_t** out_buffer);
#endif

#endif