# IREE v3.13.0's Bazel-derived VM ISA genrules hardcode "python3", which on
# Windows is commonly a nonfunctional Store alias even when Python is installed.
# Rewrite only that command token, using CMake's validated interpreter path.
if(WIN32)
  set(_genrule "${IREE_RS_SOURCE}/build_tools/cmake/iree_genrule.cmake")
  file(READ "${_genrule}" _contents)
  if(NOT _contents MATCHES "IREE_RS_PYTHON_NORMALIZED")
    set(_replacement [=[
  # IREE_RS_PYTHON_NORMALIZED
  list(GET _CMD 0 _program)
  if(_program STREQUAL "python3")
    list(REMOVE_AT _CMD 0)
    list(PREPEND _CMD "${Python3_EXECUTABLE}")
  endif()
  add_custom_command(]=])
    string(REPLACE "  add_custom_command(" "${_replacement}" _contents "${_contents}")
    file(WRITE "${_genrule}" "${_contents}")
  endif()
endif()
