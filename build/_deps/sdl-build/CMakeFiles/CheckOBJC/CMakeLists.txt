cmake_minimum_required(VERSION 3.31.4)
set(CMAKE_MODULE_PATH "/Users/jacquesthurling/personal-projects/render/renderer/build/_deps/sdl-src/cmake")
project(CheckOBJC OBJC)
file(WRITE "${CMAKE_CURRENT_BINARY_DIR}/result.cmake"
  "set(CMAKE_OBJC_COMPILER \"${CMAKE_OBJC_COMPILER}\")\n"
  
  )