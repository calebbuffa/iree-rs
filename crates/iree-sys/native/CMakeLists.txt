cmake_minimum_required(VERSION 3.26)
project(iree_rs_native LANGUAGES C CXX)
file(TO_CMAKE_PATH "${IREE_RS_SOURCE}" IREE_RS_SOURCE)
find_package(Python3 REQUIRED COMPONENTS Interpreter)
set(Python_EXECUTABLE "${Python3_EXECUTABLE}" CACHE FILEPATH "" FORCE)
include("${CMAKE_CURRENT_SOURCE_DIR}/windows-genrule.cmake")

foreach(option BUILD_TESTS BUILD_BENCHMARKS BUILD_SAMPLES BUILD_TOOLS
    BUILD_PYTHON_BINDINGS ERROR_ON_MISSING_SUBMODULES HAL_DRIVER_DEFAULTS
    HAL_EXECUTABLE_LOADER_DEFAULTS TARGET_BACKEND_DEFAULTS)
  set(IREE_${option} OFF CACHE BOOL "" FORCE)
endforeach()
set(IREE_BUILD_COMPILER ${IREE_RS_COMPILER} CACHE BOOL "" FORCE)
set(IREE_EMBEDDED_RELEASE_INFO ON CACHE BOOL "" FORCE)
set(IREE_LINK_COMPILER_SHARED_LIBRARY ON CACHE BOOL "" FORCE)
set(IREE_HAL_DRIVER_LOCAL_SYNC ${IREE_RS_LOCAL_SYNC} CACHE BOOL "" FORCE)
set(IREE_HAL_DRIVER_LOCAL_TASK ${IREE_RS_LOCAL_TASK} CACHE BOOL "" FORCE)
set(IREE_HAL_DRIVER_VULKAN ${IREE_RS_VULKAN} CACHE BOOL "" FORCE)
foreach(loader EMBEDDED_ELF SYSTEM_LIBRARY VMVX_MODULE)
  set(IREE_HAL_EXECUTABLE_LOADER_${loader} ${IREE_RS_CPU} CACHE BOOL "" FORCE)
endforeach()
set(IREE_RUNTIME_OPTIMIZATION_PROFILE "" CACHE STRING "" FORCE)
foreach(option ENABLE_ASSERTIONS INCLUDE_TESTS INCLUDE_BENCHMARKS)
  set(LLVM_${option} OFF CACHE BOOL "" FORCE)
endforeach()
set(LLVM_PARALLEL_LINK_JOBS 1 CACHE STRING "" FORCE)
foreach(option ZLIB ZSTD LIBXML2 CURL)
  set(LLVM_ENABLE_${option} OFF CACHE BOOL "" FORCE)
endforeach()
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  string(APPEND CMAKE_SHARED_LINKER_FLAGS " -static-libstdc++ -static-libgcc")
  string(APPEND CMAKE_EXE_LINKER_FLAGS " -static-libstdc++ -static-libgcc")
endif()
if(APPLE)
  set(CMAKE_INSTALL_RPATH "@loader_path")
  set(CMAKE_INSTALL_NAME_DIR "@rpath")
elseif(UNIX)
  set(CMAKE_INSTALL_RPATH "$ORIGIN")
endif()
set(CMAKE_BUILD_WITH_INSTALL_RPATH ON)
set(CMAKE_BUILD_WITH_INSTALL_NAME_DIR ON)
add_subdirectory("${IREE_RS_SOURCE}" iree EXCLUDE_FROM_ALL)
if(IREE_RS_TASK AND NOT IREE_ENABLE_THREADING)
  message(FATAL_ERROR "Task execution requires native threading; use local-sync without default features for a nonthreaded runtime.")
endif()
if(IREE_RS_WEBGPU)
  add_subdirectory("${IREE_RS_SOURCE}/runtime/src/iree/hal/drivers/webgpu"
    iree/runtime/src/iree/hal/drivers/webgpu EXCLUDE_FROM_ALL)
endif()

# Include standalone public API families, not just runtime/session dependencies.
set(runtime_roots iree_runtime_impl iree_base_internal_cpu
  iree_async_util_proactor_pool iree_io_parameter_index iree_io_parameter_index_provider
  iree_io_scope_map iree_io_stream iree_io_formats_parser_registry
  iree_io_formats_irpa_irpa iree_modules_io_parameters_parameters)
if(IREE_RS_TASK)
  list(APPEND runtime_roots iree_task_api)
endif()
if(IREE_RS_CPU)
  list(APPEND runtime_roots iree_hal_local_loaders_static_library_loader iree_hal_local_plugins_static_plugin)
endif()
if(IREE_RS_WEBGPU)
  list(APPEND runtime_roots iree_hal_drivers_webgpu_webgpu)
endif()
set(runtime_objects)
foreach(root IN LISTS runtime_roots)
  target_link_libraries(iree_runtime_unified PUBLIC ${root})
  list(APPEND runtime_objects "$<GENEX_EVAL:$<TARGET_PROPERTY:${root},INTERFACE_IREE_TRANSITIVE_OBJECTS>>")
endforeach()
set_property(TARGET iree_runtime_unified PROPERTY SOURCES "$<REMOVE_DUPLICATES:${runtime_objects}>")
if(MSVC AND IREE_RS_CPU AND CMAKE_SYSTEM_PROCESSOR MATCHES "AMD64|amd64|x86_64|X86_64")
  # Upstream attaches the ELF ABI bridge outside its transitive-object metadata.
  set(thunks "${CMAKE_CURRENT_BINARY_DIR}/iree/runtime/src/iree/hal/local/elf/arch/x86_64_msvc.obj")
  set_source_files_properties("${thunks}" PROPERTIES GENERATED TRUE)
  target_sources(iree_runtime_unified PRIVATE "${thunks}")
  add_dependencies(iree_runtime_unified iree_hal_local_elf_arch)
endif()

add_library(iree_rs_shim STATIC shim.c)
target_link_libraries(iree_rs_shim PRIVATE iree_runtime_unified)
target_compile_features(iree_rs_shim PRIVATE c_std_11)
target_compile_definitions(iree_rs_shim PRIVATE IREE_RS_VULKAN=$<BOOL:${IREE_RS_VULKAN}>)
target_include_directories(iree_rs_shim PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}")
if(MSVC)
  target_compile_options(iree_rs_shim PRIVATE /Zc:preprocessor)
endif()

if(IREE_RS_VULKAN)
  target_link_libraries(iree_rs_shim PRIVATE iree::hal::drivers::vulkan)
endif()

# Bindgen and its C forwarders consume the same transitive include paths and
# definitions as the shim, rather than guessing the native ABI from the host.
file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/iree-bindgen-args.txt" CONTENT
"-I$<JOIN:$<TARGET_PROPERTY:iree_rs_shim,INCLUDE_DIRECTORIES>,\n-I>
-D$<JOIN:$<TARGET_PROPERTY:iree_rs_shim,COMPILE_DEFINITIONS>,\n-D>
-I${IREE_RS_SOURCE}/compiler/bindings/c
-I${IREE_RS_SOURCE}/third_party/llvm-project/mlir/include
-I${IREE_RS_SOURCE}/third_party/llvm-project/llvm/include
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

set(archives iree_rs_shim iree_runtime_unified flatcc_parsing flatcc_runtime)
set(deployment "${CMAKE_INSTALL_PREFIX}/deploy")
set(install_commands
  COMMAND "${CMAKE_COMMAND}" -E make_directory "${CMAKE_INSTALL_PREFIX}/lib"
  COMMAND "${CMAKE_COMMAND}" -E remove_directory "${deployment}"
  COMMAND "${CMAKE_COMMAND}" -E make_directory "${deployment}")
set(required_files)
foreach(archive IN LISTS archives)
  list(APPEND install_commands COMMAND "${CMAKE_COMMAND}" -E copy_if_different
    "$<TARGET_FILE:${archive}>" "${CMAKE_INSTALL_PREFIX}/lib/")
  list(APPEND required_files "${CMAKE_INSTALL_PREFIX}/lib/$<TARGET_FILE_NAME:${archive}>")
endforeach()
set(install_dependencies ${archives})
if(IREE_RS_COMPILER)
  list(APPEND install_dependencies iree_compiler_API_SharedImpl)
  list(APPEND install_commands
    COMMAND "${CMAKE_COMMAND}" -E make_directory "${CMAKE_INSTALL_PREFIX}/bin"
    COMMAND "${CMAKE_COMMAND}" -E copy_if_different
      "$<TARGET_FILE:iree_compiler_API_SharedImpl>" "${CMAKE_INSTALL_PREFIX}/bin/"
    COMMAND "${CMAKE_COMMAND}" -E copy_if_different
      "$<TARGET_LINKER_FILE:iree_compiler_API_SharedImpl>" "${CMAKE_INSTALL_PREFIX}/lib/")
  list(APPEND install_commands
    COMMAND "${CMAKE_COMMAND}" -E copy_if_different "$<TARGET_FILE:iree_compiler_API_SharedImpl>" "${deployment}/")
  list(APPEND required_files
    "${CMAKE_INSTALL_PREFIX}/bin/$<TARGET_FILE_NAME:iree_compiler_API_SharedImpl>"
    "${CMAKE_INSTALL_PREFIX}/lib/$<TARGET_LINKER_FILE_NAME:iree_compiler_API_SharedImpl>"
    "${deployment}/$<TARGET_FILE_NAME:iree_compiler_API_SharedImpl>")
  if(TARGET iree-lld)
    list(APPEND install_dependencies iree-lld)
    list(APPEND install_commands
      COMMAND "${CMAKE_COMMAND}" -E copy_if_different "$<TARGET_FILE:iree-lld>" "${CMAKE_INSTALL_PREFIX}/bin/"
      COMMAND "${CMAKE_COMMAND}" -E copy_if_different "$<TARGET_FILE:iree-lld>" "${deployment}/")
    list(APPEND required_files "${CMAKE_INSTALL_PREFIX}/bin/$<TARGET_FILE_NAME:iree-lld>"
      "${deployment}/$<TARGET_FILE_NAME:iree-lld>")
  endif()
  get_property(resources GLOBAL PROPERTY IREE_COMPILER_DYLIB_RELPATHS)
  foreach(resource IN LISTS resources)
    get_filename_component(directory "${resource}" DIRECTORY)
    list(APPEND install_commands
      COMMAND "${CMAKE_COMMAND}" -E make_directory "${CMAKE_INSTALL_PREFIX}/bin/${directory}"
      COMMAND "${CMAKE_COMMAND}" -E copy_if_different
        "$<TARGET_FILE_DIR:iree_compiler_API_SharedImpl>/${resource}" "${CMAKE_INSTALL_PREFIX}/bin/${resource}"
      COMMAND "${CMAKE_COMMAND}" -E make_directory "${deployment}/${directory}"
      COMMAND "${CMAKE_COMMAND}" -E copy_if_different
        "$<TARGET_FILE_DIR:iree_compiler_API_SharedImpl>/${resource}" "${deployment}/${resource}")
    list(APPEND required_files "${CMAKE_INSTALL_PREFIX}/bin/${resource}" "${deployment}/${resource}")
  endforeach()
endif()
if(MSVC AND CMAKE_MSVC_RUNTIME_LIBRARY MATCHES "DLL")
  set(CMAKE_INSTALL_SYSTEM_RUNTIME_LIBS_SKIP TRUE)
  include(InstallRequiredSystemLibraries)
  if(NOT CMAKE_INSTALL_SYSTEM_RUNTIME_LIBS)
    message(FATAL_ERROR "MSVC redistributable libraries were not found; install the Visual C++ Redistributable SDK component.")
  endif()
  foreach(runtime IN LISTS CMAKE_INSTALL_SYSTEM_RUNTIME_LIBS)
    get_filename_component(name "${runtime}" NAME)
    list(APPEND install_commands
      COMMAND "${CMAKE_COMMAND}" -E copy_if_different "${runtime}" "${deployment}/${name}")
    list(APPEND required_files "${deployment}/${name}")
  endforeach()
endif()

# Installed metadata is the only input to Cargo's link step.
set(system_libraries)
set(frameworks)
if(WIN32)
  list(APPEND system_libraries synchronization ws2_32 advapi32 bcrypt)
elseif(UNIX)
  list(APPEND system_libraries m pthread)
  if(NOT APPLE)
    list(APPEND system_libraries dl)
  endif()
endif()
if(IREE_RS_CPU AND NOT MSVC)
  if(APPLE)
    list(APPEND system_libraries c++)
  else()
    list(APPEND system_libraries "static=stdc++")
  endif()
endif()
if(IREE_HAL_DRIVER_METAL)
  list(APPEND frameworks Foundation Metal QuartzCore)
endif()
function(json_list output)
  set(items)
  foreach(item IN LISTS ARGN)
    string(REPLACE "\\" "/" item "${item}")
    string(REPLACE "\"" "\\\"" item "${item}")
    list(APPEND items "\"${item}\"")
  endforeach()
  string(JOIN "," items ${items})
  set(${output} "[${items}]" PARENT_SCOPE)
endfunction()
json_list(capabilities_json ${IREE_RS_CAPABILITIES})
json_list(archives_json ${archives})
json_list(system_json ${system_libraries})
json_list(frameworks_json ${frameworks})
json_list(files_json ${required_files})
if(IREE_ENABLE_THREADING)
  set(threading true)
else()
  set(threading false)
endif()
file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/iree-native.json" CONTENT
"{
  \"fingerprint\": \"${IREE_RS_FINGERPRINT}\",
  \"target\": \"${IREE_RS_RUST_TARGET}\",
  \"version\": \"${IREE_RELEASE_VERSION}\",
  \"threading\": ${threading},
  \"capabilities\": ${capabilities_json},
  \"source\": \"${IREE_RS_SOURCE}\",
  \"build\": \"${CMAKE_CURRENT_BINARY_DIR}\",
  \"prefix\": \"${CMAKE_INSTALL_PREFIX}\",
  \"deployment\": \"${deployment}\",
  \"static_libraries\": ${archives_json},
  \"system_libraries\": ${system_json},
  \"frameworks\": ${frameworks_json},
  \"required_files\": ${files_json}
}
")
add_custom_target(iree_rs_install ${install_commands}
  COMMAND "${CMAKE_COMMAND}" -E copy_if_different
    "${CMAKE_CURRENT_BINARY_DIR}/iree-native.json" "${CMAKE_INSTALL_PREFIX}/iree-native.json"
  DEPENDS ${install_dependencies} VERBATIM)
