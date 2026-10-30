cmake_minimum_required(VERSION 3.26)
project(iree_native_native LANGUAGES C CXX)

file(TO_CMAKE_PATH "${IREE_NATIVE_SOURCE}" IREE_NATIVE_SOURCE)
find_package(Python3 REQUIRED COMPONENTS Interpreter)
set(Python_EXECUTABLE "${Python3_EXECUTABLE}" CACHE FILEPATH "" FORCE)

include("${CMAKE_CURRENT_LIST_DIR}/configuration.cmake")
include("${CMAKE_CURRENT_LIST_DIR}/targets.cmake")
include("${CMAKE_CURRENT_LIST_DIR}/installation.cmake")
