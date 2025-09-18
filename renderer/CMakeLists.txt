cmake_minimum_required(VERSION 3.20)
project(renderer LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

include(FetchContent)

# --- Dependencies ---
FetchContent_Declare(sdl GIT_REPOSITORY "https://github.com/libsdl-org/SDL.git" GIT_TAG "release-2.30.3")

FetchContent_MakeAvailable(sdl)

# --- Project Target ---
file(GLOB_RECURSE SOURCES ${PROJECT_SOURCE_DIR}/src/*.cpp)
add_executable(renderer ${SOURCES})

# --- Linking ---
target_link_libraries(renderer PRIVATE SDL2::SDL2)
