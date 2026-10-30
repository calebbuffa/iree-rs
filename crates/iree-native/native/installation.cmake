set(static_targets iree_runtime_unified flatcc_parsing flatcc_runtime)
foreach(target IN LISTS static_targets)
    if(NOT TARGET "${target}")
        message(FATAL_ERROR
      "IREE ${IREE_RELEASE_VERSION} does not define required install target '${target}'. "
      "Review native/installation.cmake against this release before building.")
    endif()
endforeach()
install(TARGETS ${static_targets} ARCHIVE DESTINATION lib)
set(static_libraries ${static_targets})
set(required_files)
foreach(target IN LISTS static_targets)
    list(APPEND required_files
    "${CMAKE_INSTALL_PREFIX}/lib/$<TARGET_FILE_NAME:${target}>")
endforeach()

set(deployment "${CMAKE_INSTALL_PREFIX}/deploy")
set(runtime_deployment_files)
install(CODE "file(MAKE_DIRECTORY \"${deployment}\")")
set(install_dependencies ${static_targets})

if(IREE_NATIVE_COMPILER)
    install(TARGETS iree_compiler_API_SharedImpl
    RUNTIME DESTINATION bin
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib)
    if(NOT WIN32)
        install(FILES "$<TARGET_FILE:iree_compiler_API_SharedImpl>"
      DESTINATION bin)
    endif()
    install(FILES "$<TARGET_FILE:iree_compiler_API_SharedImpl>"
    DESTINATION deploy)
    list(APPEND install_dependencies iree_compiler_API_SharedImpl)
    list(APPEND required_files
    "${CMAKE_INSTALL_PREFIX}/bin/$<TARGET_FILE_NAME:iree_compiler_API_SharedImpl>"
    "${CMAKE_INSTALL_PREFIX}/lib/$<TARGET_LINKER_FILE_NAME:iree_compiler_API_SharedImpl>"
    "${deployment}/$<TARGET_FILE_NAME:iree_compiler_API_SharedImpl>")

    if(TARGET iree-lld)
        install(TARGETS iree-lld RUNTIME DESTINATION bin)
        install(FILES "$<TARGET_FILE:iree-lld>" DESTINATION deploy)
        list(APPEND install_dependencies iree-lld)
        list(APPEND required_files
      "${CMAKE_INSTALL_PREFIX}/bin/$<TARGET_FILE_NAME:iree-lld>"
      "${deployment}/$<TARGET_FILE_NAME:iree-lld>")
    endif()

    get_property(compiler_resources GLOBAL PROPERTY IREE_COMPILER_DYLIB_RELPATHS)
    foreach(resource IN LISTS compiler_resources)
        get_filename_component(directory "${resource}" DIRECTORY)
        install(FILES "$<TARGET_FILE_DIR:iree_compiler_API_SharedImpl>/${resource}"
      DESTINATION "bin/${directory}")
        install(FILES "$<TARGET_FILE_DIR:iree_compiler_API_SharedImpl>/${resource}"
      DESTINATION "deploy/${directory}")
        list(APPEND required_files
      "${CMAKE_INSTALL_PREFIX}/bin/${resource}"
      "${deployment}/${resource}")
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
        install(FILES "${runtime}" DESTINATION deploy)
        list(APPEND required_files "${deployment}/${name}")
        list(APPEND runtime_deployment_files "${deployment}/${name}")
    endforeach()
endif()

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
if(IREE_NATIVE_CPU AND NOT MSVC AND NOT IREE_NATIVE_BARE_METAL)
    if(APPLE)
        list(APPEND system_libraries c++)
    else()
        include("${CMAKE_CURRENT_LIST_DIR}/cxx-runtime.cmake")
    endif()
endif()
if(IREE_HAL_DRIVER_METAL)
    list(APPEND frameworks Foundation Metal QuartzCore)
endif()

function(json_escape output value)
    string(REPLACE "\\" "\\\\" escaped "${value}")
    string(REPLACE "\"" "\\\"" escaped "${escaped}")
    string(REPLACE "\n" "\\n" escaped "${escaped}")
    set(${output} "\"${escaped}\"" PARENT_SCOPE)
endfunction()

function(json_list output)
    set(items)
    foreach(item IN LISTS ARGN)
        json_escape(quoted "${item}")
        list(APPEND items "${quoted}")
    endforeach()
    string(JOIN "," items ${items})
    set(${output} "[${items}]" PARENT_SCOPE)
endfunction()

json_list(capabilities_json ${IREE_NATIVE_CAPABILITIES})
json_list(static_libraries_json ${static_libraries})
json_list(system_libraries_json ${system_libraries})
json_list(frameworks_json ${frameworks})
json_list(required_files_json ${required_files})
json_list(runtime_deployment_files_json ${runtime_deployment_files})
json_escape(fingerprint_json "${IREE_NATIVE_FINGERPRINT}")
json_escape(target_json "${IREE_NATIVE_RUST_TARGET}")
json_escape(version_json "${IREE_RELEASE_VERSION}")
json_escape(source_json "${IREE_NATIVE_SOURCE}")
json_escape(build_json "${CMAKE_CURRENT_BINARY_DIR}")
json_escape(prefix_json "${CMAKE_INSTALL_PREFIX}")
json_escape(deployment_json "${deployment}")
if(IREE_ENABLE_THREADING)
    set(threading_json true)
else()
    set(threading_json false)
endif()

file(GENERATE OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/iree-native.json" CONTENT
"{
  \"fingerprint\": ${fingerprint_json},
  \"target\": ${target_json},
  \"version\": ${version_json},
  \"threading\": ${threading_json},
  \"capabilities\": ${capabilities_json},
  \"source\": ${source_json},
  \"build\": ${build_json},
  \"prefix\": ${prefix_json},
  \"deployment\": ${deployment_json},
  \"runtime_deployment_files\": ${runtime_deployment_files_json},
  \"static_libraries\": ${static_libraries_json},
  \"system_libraries\": ${system_libraries_json},
  \"frameworks\": ${frameworks_json},
  \"required_files\": ${required_files_json}
}
")
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/iree-native.json" DESTINATION .)

add_custom_target(iree_native_install
  COMMAND "${CMAKE_COMMAND}" --install "${CMAKE_BINARY_DIR}" --config "$<CONFIG>"
  DEPENDS ${install_dependencies}
  VERBATIM)
