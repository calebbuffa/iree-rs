if(IREE_NATIVE_WEBGPU)
    include("${IREE_NATIVE_SOURCE}/build_tools/cmake/iree_plugin_register.cmake")
    iree_register_external_hal_driver(
    NAME webgpu
    SOURCE_DIR "${IREE_NATIVE_SOURCE}/runtime/src/iree/hal/drivers/webgpu"
    BINARY_DIR "${CMAKE_CURRENT_BINARY_DIR}/iree/runtime/src/iree/hal/drivers/webgpu"
    DRIVER_TARGET iree::hal::drivers::webgpu::registration
    REGISTER_FN iree_hal_webgpu_driver_module_register)
    list(APPEND IREE_EXTERNAL_HAL_DRIVERS webgpu)
endif()

if(IREE_NATIVE_COMPILER)
    # Public MLIR IRDL C API, independent of compiler plugin registration.
    list(APPEND IREE_COMPILER_API_ADDL_EXPORT_OBJECTS obj.MLIRCAPIIRDL)
endif()

add_subdirectory("${IREE_NATIVE_SOURCE}" iree EXCLUDE_FROM_ALL)

if(IREE_NATIVE_TASK AND NOT IREE_ENABLE_THREADING)
    message(FATAL_ERROR "Task execution requires native threading; use local-sync without default features for a nonthreaded runtime.")
endif()

set(runtime_roots
  iree_runtime_impl
  iree_async_util_proactor_pool
  iree_io_stream
  iree_modules_hal_inline_inline
  iree_hal_utils_files
  iree_hal_utils_file_transfer
  iree_hal_memory_tlsf_pool
  iree_hal_memory_fixed_block_pool
  iree_vm_dynamic_module)

if(IREE_NATIVE_IO_INDEX)
    list(APPEND runtime_roots
    iree_io_parameter_index
    iree_io_scope_map)
endif()
if(IREE_NATIVE_IO_PARAMETERS)
    list(APPEND runtime_roots
    iree_io_parameter_provider
    iree_io_parameter_index_provider
    iree_modules_io_parameters_parameters)
endif()
if(IREE_NATIVE_IO_FORMAT_IRPA)
    list(APPEND runtime_roots iree_io_formats_irpa_irpa)
endif()
if(IREE_NATIVE_IO_FORMAT_GGUF)
    list(APPEND runtime_roots iree_io_formats_gguf_gguf)
endif()
if(IREE_NATIVE_IO_FORMAT_SAFETENSORS)
    list(APPEND runtime_roots iree_io_formats_safetensors_safetensors)
endif()

if(IREE_NATIVE_TASK)
    list(APPEND runtime_roots iree_task_api)
endif()
if(IREE_NATIVE_TOKENIZER)
    list(APPEND runtime_roots
    iree_tokenizer_tokenizer
    iree_tokenizer_format_huggingface_tokenizer_json
    iree_tokenizer_format_tiktoken_tiktoken)
endif()
if(IREE_NATIVE_CPU)
    list(APPEND runtime_roots
    iree_modules_hal_loader_loader
    iree_hal_local_executable_format
    iree_hal_local_loaders_static_library_loader
    iree_hal_local_plugins_static_plugin)
endif()
if(IREE_NATIVE_WEBGPU)
    list(APPEND runtime_roots iree_hal_drivers_webgpu_webgpu)
endif()

set(runtime_objects)
foreach(root IN LISTS runtime_roots)
    if(NOT TARGET "${root}")
        message(FATAL_ERROR
      "IREE ${IREE_RELEASE_VERSION} does not define required runtime target '${root}'. "
      "Review native/targets.cmake against this release before building.")
    endif()
    get_target_property(transitive_objects "${root}" INTERFACE_IREE_TRANSITIVE_OBJECTS)
    if(transitive_objects STREQUAL "transitive_objects-NOTFOUND")
        message(FATAL_ERROR
      "IREE ${IREE_RELEASE_VERSION} target '${root}' lacks "
      "INTERFACE_IREE_TRANSITIVE_OBJECTS; review the upstream runtime target closure.")
    endif()
    target_link_libraries(iree_runtime_unified PUBLIC "${root}")
    list(APPEND runtime_objects
    "$<GENEX_EVAL:$<TARGET_PROPERTY:${root},INTERFACE_IREE_TRANSITIVE_OBJECTS>>")
endforeach()
set_property(TARGET iree_runtime_unified PROPERTY SOURCES
  "$<REMOVE_DUPLICATES:${runtime_objects}>")

if(MSVC AND CMAKE_SYSTEM_PROCESSOR MATCHES "AMD64|amd64|x86_64|X86_64")
    set(thunks "${CMAKE_CURRENT_BINARY_DIR}/iree/runtime/src/iree/hal/local/elf/arch/x86_64_msvc.obj")
    set_source_files_properties("${thunks}" PROPERTIES GENERATED TRUE)
    target_sources(iree_runtime_unified PRIVATE "${thunks}")
    add_dependencies(iree_runtime_unified iree_hal_local_elf_arch)
endif()

file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/iree-bindgen-args.txt" CONTENT
"-I${CMAKE_CURRENT_SOURCE_DIR}
-I$<JOIN:$<TARGET_PROPERTY:iree_runtime_unified,INTERFACE_INCLUDE_DIRECTORIES>,\n-I>
-D$<JOIN:$<TARGET_PROPERTY:iree_runtime_unified,INTERFACE_COMPILE_DEFINITIONS>,\n-D>
-I${IREE_NATIVE_SOURCE}/compiler/bindings/c
-I${IREE_NATIVE_SOURCE}/third_party/llvm-project/mlir/include
-I${IREE_NATIVE_SOURCE}/third_party/llvm-project/llvm/include
-DNDEBUG
")

set(bindgen_platform_args "")
if(CMAKE_OSX_SYSROOT)
    string(APPEND bindgen_platform_args "--sysroot=${CMAKE_OSX_SYSROOT}\n")
elseif(CMAKE_SYSROOT)
    string(APPEND bindgen_platform_args "--sysroot=${CMAKE_SYSROOT}\n")
endif()
if(CMAKE_OSX_DEPLOYMENT_TARGET AND CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    string(APPEND bindgen_platform_args "-mmacosx-version-min=${CMAKE_OSX_DEPLOYMENT_TARGET}\n")
endif()
file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/iree-bindgen-sysroot.txt"
  CONTENT "${bindgen_platform_args}")
file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/iree-c-compiler.txt"
  CONTENT "${CMAKE_C_COMPILER}")
file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/iree-python-executable.txt"
  CONTENT "${Python3_EXECUTABLE}")
