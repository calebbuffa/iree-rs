# Rust resolves archives outside the compiler driver's implicit search paths.
find_file(IREE_NATIVE_CXX_RUNTIME_ARCHIVE
  NAMES libstdc++.a
  PATHS ${CMAKE_CXX_IMPLICIT_LINK_DIRECTORIES}
  NO_DEFAULT_PATH REQUIRED)
install(FILES "${IREE_NATIVE_CXX_RUNTIME_ARCHIVE}"
  DESTINATION lib RENAME libstdc++.a)
list(APPEND required_files "${CMAKE_INSTALL_PREFIX}/lib/libstdc++.a")
list(APPEND system_libraries "static=stdc++")
