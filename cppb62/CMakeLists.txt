cmake_minimum_required(VERSION 3.15)
project(cppb62)

set(CMAKE_CXX_STANDARD 17)

SET(SOURCE_FILES base62_conversion.cpp base62_conversion.h)

add_library(cppb62 ${SOURCE_FILES})

add_subdirectory(tests)