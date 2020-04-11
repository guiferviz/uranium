
################################
#  CMakeLists.txt for Uranium  #
################################


################
# Requirements #
################

# If you are using a lower version of CMake remove this and try it.
# Let me know which was the lowest version you could compile with
# to downgrade the version here.
cmake_minimum_required(VERSION 3.10.2)

# Of course LLVM is required! The bin directory of LLVM should be on your PATH.
# More details about LLVM and CMake for personal projects in:
# http://llvm.org/docs/CMake.html#embedding-llvm-in-your-project
# With LLVM 10.0.0 I get a couple of warnings but everything seems to work
# fine (at least with simple examples).
find_package(LLVM REQUIRED CONFIG)
message(STATUS "Found LLVM version: ${LLVM_PACKAGE_VERSION}")


############
# Compiler #
############

# We will use clang++ as compiler.
# Better to use the same version as the LLVM library.
set(CMAKE_CXX_COMPILER ${LLVM_TOOLS_BINARY_DIR}/clang++)
# I tried to change the linker to the LLVM linker but I didn't get anything.
# Setting CMAKE_LINKER does not make any different and setting
# CMAKE_CXX_LINK_EXECUTABLE gives errors.


######################
# Project definition #
######################

project(uranium CXX)
set(URANIUM_VERSION 0.0.1)
set(EXECUTABLE_NAME uc)
set(INCLUDE_DIR "include")
set(SOURCES_DIR "src")


#####################
# Executable target #
#####################

# Including header files at include dir.
include_directories(${INCLUDE_DIR})
# Include LLVM headers and definitions.
# The LLVM_INSTALL_INCLUDE_DIR is created by the find_package function.
include_directories(${LLVM_INCLUDE_DIR})
add_definitions(${LLVM_DEFINITIONS})

# Adding definitions to use with the preprocessor.
# Using this we do not need to maintain different version numbers in several
# files. The version in this CMakeList is the only one to maintain.
add_definitions(-DURANIUM_VERSION=\"${URANIUM_VERSION}\")  # Version as a string.

# Add executable target.
add_executable(${EXECUTABLE_NAME} ${SOURCES_DIR}/main.cpp)
# Find the libraries that correspond to the LLVM components that we want to use.
# I suppose this is for avoiding linking unused libraries, but not sure.
# The line in the documentation includes support and irreader that are not
# needed in the most basic example (generating IR for an empty module).
#llvm_map_components_to_libnames(llvm_libs support core irreader)
llvm_map_components_to_libnames(llvm_libs core)
target_link_libraries(${EXECUTABLE_NAME} ${llvm_libs})  # llvm_libs must be lower case!

