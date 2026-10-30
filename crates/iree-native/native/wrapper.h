#ifndef IREE_NATIVE_WRAPPER_H_
#define IREE_NATIVE_WRAPPER_H_

#if defined(__linux__)
// IREE's inline futex helpers use syscall and timespec.
#ifndef _GNU_SOURCE
#define _GNU_SOURCE 1
#endif
#include <time.h>
#endif

#include "iree/runtime/api.h"
#if IREE_NATIVE_BASE
#include "iree/hal/device.h"
#include "iree/hal/drivers/init.h"

IREE_API_EXPORT iree_status_t iree_native_hal_device_queue_barrier(
    iree_hal_device_t *device, iree_hal_queue_affinity_t queue_affinity,
    const iree_hal_semaphore_list_t *wait_semaphore_list,
    const iree_hal_semaphore_list_t *signal_semaphore_list,
    iree_hal_execute_flags_t flags);
IREE_API_EXPORT iree_status_t iree_native_hal_device_queue_execute(
    iree_hal_device_t *device, iree_hal_queue_affinity_t queue_affinity,
    const iree_hal_semaphore_list_t *wait_semaphore_list,
    const iree_hal_semaphore_list_t *signal_semaphore_list,
    iree_hal_command_buffer_t *command_buffer,
    const iree_hal_buffer_binding_table_t *binding_table,
    iree_hal_execute_flags_t flags);
#if IREE_NATIVE_FLAGS
#include "iree/base/tooling/flags.h"
#endif
#if IREE_NATIVE_MPI
#include "iree/hal/utils/mpi_channel_provider.h"
#endif
#include "iree/hal/profile_sink.h"
#include "iree/hal/utils/file_cache.h"
#include "iree/hal/utils/profile_file.h"
#include "iree/hal/utils/statistics_sink.h"

#endif
#if IREE_NATIVE_TOKENIZER
#include "iree/tokenizer/decoder/byte_fallback.h"
#include "iree/tokenizer/decoder/byte_level.h"
#include "iree/tokenizer/decoder/ctc.h"
#include "iree/tokenizer/decoder/metaspace.h"
#include "iree/tokenizer/decoder/passthrough.h"
#include "iree/tokenizer/decoder/replace.h"
#include "iree/tokenizer/decoder/sequence.h"
#include "iree/tokenizer/decoder/strip.h"
#include "iree/tokenizer/decoder/wordpiece.h"
#include "iree/tokenizer/format/huggingface/added_tokens_json.h"
#include "iree/tokenizer/format/huggingface/decoder_json.h"
#include "iree/tokenizer/format/huggingface/model_json.h"
#include "iree/tokenizer/format/huggingface/normalizer_json.h"
#include "iree/tokenizer/format/huggingface/postprocessor_json.h"
#include "iree/tokenizer/format/huggingface/segmenter_json.h"
#include "iree/tokenizer/format/huggingface/tokenizer_json.h"
#include "iree/tokenizer/format/tiktoken/tiktoken.h"
#include "iree/tokenizer/model/bpe.h"
#include "iree/tokenizer/model/unigram.h"
#include "iree/tokenizer/model/wordpiece.h"
#include "iree/tokenizer/normalizer/bert.h"
#include "iree/tokenizer/normalizer/deferred.h"
#include "iree/tokenizer/normalizer/lowercase.h"
#include "iree/tokenizer/normalizer/nfc.h"
#include "iree/tokenizer/normalizer/nfd.h"
#include "iree/tokenizer/normalizer/nfkd.h"
#include "iree/tokenizer/normalizer/passthrough.h"
#include "iree/tokenizer/normalizer/precompiled.h"
#include "iree/tokenizer/normalizer/prepend.h"
#include "iree/tokenizer/normalizer/regex_replace.h"
#include "iree/tokenizer/normalizer/replace.h"
#include "iree/tokenizer/normalizer/sequence.h"
#include "iree/tokenizer/normalizer/strip.h"
#include "iree/tokenizer/normalizer/strip_accents.h"
#include "iree/tokenizer/regex/compile.h"
#include "iree/tokenizer/segmenter/bert.h"
#include "iree/tokenizer/segmenter/digits.h"
#include "iree/tokenizer/segmenter/metaspace.h"
#include "iree/tokenizer/segmenter/passthrough.h"
#include "iree/tokenizer/segmenter/punctuation.h"
#include "iree/tokenizer/segmenter/sequence.h"
#include "iree/tokenizer/segmenter/split.h"
#include "iree/tokenizer/segmenter/whitespace.h"
#include "iree/tokenizer/tokenizer.h"
#include "iree/tokenizer/vocab/vocab.h"
#include "iree/tokenizer/vocab/vocab_builder.h"
#include "iree/tokenizer/vocab/vocab_hash.h"
#include "iree/tokenizer/vocab/vocab_merge_hash.h"
#include "iree/tokenizer/vocab/vocab_trie.h"

#endif
#include "iree/async/api.h"
#if IREE_NATIVE_TASK
#include "iree/task/api.h"
#endif
#include "iree/io/file_contents.h"
#if IREE_NATIVE_IO_INDEX
#include "iree/io/parameter_index.h"
#include "iree/io/scope_map.h"
#endif
#if IREE_NATIVE_IO_PARAMETERS
#include "iree/io/parameter_index_provider.h"
#include "iree/modules/io/parameters/module.h"
#endif
#include "iree/io/memory_stream.h"
#include "iree/io/stdio_stream.h"
#include "iree/io/vec_stream.h"
#if IREE_NATIVE_IO_FORMAT_GGUF
#include "iree/io/formats/gguf/gguf_parser.h"
#endif
#if IREE_NATIVE_IO_FORMAT_IRPA
#include "iree/io/formats/irpa/irpa_builder.h"
#include "iree/io/formats/irpa/irpa_parser.h"

#endif
#if IREE_NATIVE_IO_FORMAT_SAFETENSORS
#include "iree/io/formats/safetensors/safetensors_parser.h"
#endif
#include "iree/async/frontier_tracker.h"
#include "iree/async/util/proactor_pool.h"
#include "iree/hal/buffer_transfer.h"
#include "iree/hal/executable_cache.h"
#include "iree/hal/memory/fixed_block_pool.h"
#include "iree/hal/memory/tlsf_pool.h"
#include "iree/hal/utils/fd_file.h"
#include "iree/hal/utils/file_registry.h"
#include "iree/hal/utils/file_transfer.h"
#include "iree/hal/utils/memory_file.h"
#include "iree/modules/hal/debugging.h"
#include "iree/modules/hal/inline/module.h"
#include "iree/modules/hal/module.h"
#include "iree/vm/bytecode/module.h"
#include "iree/vm/dynamic/module.h"
#include "iree/vm/loop_inline.h"

#if IREE_NATIVE_COMPILER
#include "iree/compiler/embedding_api.h"
#include "iree/compiler/mlir_interop.h"
#if defined(_WIN32)
uint32_t iree_native_load_compiler(const uint16_t *path);
#endif
#include "mlir-c/AffineExpr.h"
#include "mlir-c/AffineMap.h"
#include "mlir-c/BuiltinAttributes.h"
#include "mlir-c/BuiltinTypes.h"
#include "mlir-c/Dialect/IRDL.h"
#include "mlir-c/IR.h"
#include "mlir-c/IntegerSet.h"
#include "mlir-c/Interfaces.h"
#include "mlir-c/Support.h"

#if IREE_NATIVE_COMPILER_TOOLS
#include "iree/compiler/tool_entry_points_api.h"
#endif
#include "mlir-c/Diagnostics.h"
#include "mlir-c/Dialect/Linalg.h"
#include "mlir-c/Pass.h"

// Some dialect headers use C++ struct-name lookup in their C API.
// Forward aliases make the unchanged upstream declarations valid C as well.
typedef struct ireeCodegenTranslationInfoParameters
    ireeCodegenTranslationInfoParameters;
typedef struct ireeCodegenCompilationInfoParameters
    ireeCodegenCompilationInfoParameters;
typedef struct ireeCodegenAttentionOpDetail ireeCodegenAttentionOpDetail;
typedef struct ireeCodegenIGEMMGenericConvDetails
    ireeCodegenIGEMMGenericConvDetails;
typedef struct ireeCodegenScaledContractionDimensions
    ireeCodegenScaledContractionDimensions;
typedef struct ireeGPUMMAInfo ireeGPUMMAInfo;
typedef struct ireeGPUTileSizes ireeGPUTileSizes;
typedef struct ireeGPUSubgroupBasisInfo ireeGPUSubgroupBasisInfo;
typedef struct ireeGPUMMASingleSubgroupLayout ireeGPUMMASingleSubgroupLayout;
typedef struct ireeGPUTargetInfo ireeGPUTargetInfo;
#include "iree/compiler/dialects/iree_codegen.h"
#include "iree/compiler/dialects/iree_gpu.h"
#endif
#if IREE_NATIVE_CUDA
#include "iree/hal/drivers/cuda/api.h"
#endif
#if IREE_NATIVE_HIP
#include "iree/hal/drivers/hip/api.h"
#endif
#if IREE_NATIVE_AMDGPU
#include "iree/hal/drivers/amdgpu/api.h"
#include "iree/hal/drivers/amdgpu/util/libhsa.h"
#include "iree/hal/drivers/amdgpu/util/topology.h"
#endif
#if IREE_NATIVE_METAL
#include "iree/hal/drivers/metal/api.h"
#endif
#if IREE_NATIVE_NULL
#include "iree/hal/drivers/null/api.h"
#endif
#if IREE_NATIVE_WEBGPU
#include "iree/hal/drivers/webgpu/api.h"
#endif
#if IREE_NATIVE_LOCAL_SYNC
#include "iree/hal/drivers/local_sync/sync_device.h"
#include "iree/hal/drivers/local_sync/sync_driver.h"

#endif
#if IREE_NATIVE_LOCAL_TASK
#include "iree/hal/drivers/local_task/task_device.h"
#include "iree/hal/drivers/local_task/task_driver.h"

#endif
#if IREE_NATIVE_CPU
#include "iree/hal/local/executable_library.h"
#include "iree/hal/local/executable_plugin.h"
#include "iree/hal/local/loaders/embedded_elf_loader.h"
#include "iree/hal/local/loaders/registration/init.h"
#include "iree/hal/local/loaders/static_library_loader.h"
#include "iree/hal/local/loaders/system_library_loader.h"
#include "iree/hal/local/loaders/vmvx_module_loader.h"
#include "iree/hal/local/plugins/embedded_elf_plugin.h"
#include "iree/hal/local/plugins/system_library_plugin.h"
#include "iree/modules/hal/loader/module.h"

#endif
#if IREE_NATIVE_VULKAN
#include "iree/hal/drivers/vulkan/api.h"
#endif

#endif
