foreach(option BUILD_TESTS BUILD_BENCHMARKS BUILD_SAMPLES BUILD_TOOLS
    BUILD_PYTHON_BINDINGS ERROR_ON_MISSING_SUBMODULES HAL_DRIVER_DEFAULTS
    HAL_EXECUTABLE_LOADER_DEFAULTS TARGET_BACKEND_DEFAULTS)
    set(IREE_${option} OFF CACHE BOOL "" FORCE)
endforeach()
set(IREE_BUILD_COMPILER ${IREE_NATIVE_COMPILER} CACHE BOOL "" FORCE)
set(IREE_EMBEDDED_RELEASE_INFO ON CACHE BOOL "" FORCE)
set(IREE_LINK_COMPILER_SHARED_LIBRARY ON CACHE BOOL "" FORCE)
set(IREE_HAL_DRIVER_LOCAL_SYNC ${IREE_NATIVE_LOCAL_SYNC} CACHE BOOL "" FORCE)
set(IREE_HAL_DRIVER_LOCAL_TASK ${IREE_NATIVE_LOCAL_TASK} CACHE BOOL "" FORCE)
set(IREE_HAL_DRIVER_VULKAN ${IREE_NATIVE_VULKAN} CACHE BOOL "" FORCE)
foreach(loader EMBEDDED_ELF SYSTEM_LIBRARY VMVX_MODULE)
    set(IREE_HAL_EXECUTABLE_LOADER_${loader} ${IREE_NATIVE_CPU} CACHE BOOL "" FORCE)
endforeach()

if(CMAKE_SYSTEM_NAME STREQUAL "Emscripten")
    # WebAssembly deployment target (wasm32-unknown-emscripten). Narrowly gated so
    # desktop and the Generic bare-metal branch below are untouched. Placed after
    # the loader force loop above so these overrides win.
    #
    # Single-threaded: the browser/Node default path has no Workers or
    # SharedArrayBuffer, so build the synchronous local-sync runtime with IREE
    # threading disabled.
    set(IREE_ENABLE_THREADING OFF CACHE BOOL "" FORCE)
    # Only the portable VMVX bytecode loader works on wasm; the native-code
    # loaders (dlopen/ELF) cannot run in the sandbox. VMVX itself stays enabled
    # via IREE_NATIVE_CPU in the loader loop above.
    set(IREE_HAL_EXECUTABLE_LOADER_EMBEDDED_ELF OFF CACHE BOOL "" FORCE)
    set(IREE_HAL_EXECUTABLE_LOADER_SYSTEM_LIBRARY OFF CACHE BOOL "" FORCE)
    # base/internal/synchronization and futex.h reference the wasm atomic
    # wait/notify builtins at compile time even on the single-threaded path;
    # enable the atomics + bulk-memory features so those TUs compile. No shared
    # memory is actually used.
    add_compile_options(-matomics -mbulk-memory)
endif()

set(IREE_RUNTIME_OPTIMIZATION_PROFILE "" CACHE STRING "" FORCE)

if(IREE_NATIVE_BARE_METAL)
    if(NOT CMAKE_SYSTEM_NAME STREQUAL "Generic")
        message(FATAL_ERROR "bare-metal requires a Generic CMake toolchain with a C library and platform clock.")
    endif()
    set(IREE_ENABLE_THREADING OFF CACHE BOOL "" FORCE)
    set(IREE_ENABLE_POSIX OFF CACHE BOOL "" FORCE)
    set(IREE_SYNCHRONIZATION_DISABLE_UNSAFE ON CACHE BOOL "" FORCE)
    set(IREE_HAL_EXECUTABLE_LOADER_SYSTEM_LIBRARY OFF CACHE BOOL "" FORCE)
    set(IREE_FILE_IO_ENABLE OFF CACHE BOOL "" FORCE)
    add_compile_definitions(IREE_PLATFORM_GENERIC=1 IREE_FILE_IO_ENABLE=0)
endif()

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
