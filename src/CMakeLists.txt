cmake_minimum_required(VERSION 3.29.5)
project(W11Boost VERSION 1.1 LANGUAGES CXX C)

function(replace_manifest TARGET_NAME MANIFEST_FILE)
  if(NOT TARGET_NAME)
    message(FATAL_ERROR "You must provide a target")
  endif()

  if(NOT MANIFEST_FILE)
    message(FATAL_ERROR "You must provide a manifest file")
  endif()

  add_custom_command(
    TARGET ${TARGET_NAME}
    POST_BUILD
    COMMAND "mt.exe" -manifest \"${MANIFEST_FILE}\" \"-outputresource:$<TARGET_FILE:${TARGET_NAME}>\" -nologo
  )
endfunction()

file(GLOB SRC_FILES CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/*.c")

add_executable(W11Boost WIN32 ${SRC_FILES} W11Boost.rc)

target_compile_features(W11Boost PUBLIC c_std_23)
set_target_properties(W11Boost PROPERTIES
  CMAKE_C_STANDARD 23
  CMAKE_C_STANDARD_REQUIRED ON
  CMAKE_C_EXTENSIONS OFF
)
replace_manifest(W11Boost ${CMAKE_SOURCE_DIR}/app.manifest)
add_definitions(-DUNICODE -D_UNICODE)
target_link_options(W11Boost PRIVATE -municode -mwindows)
target_compile_options(W11Boost PRIVATE -Wall -Wextra -Werror -Wno-missing-field-initializers)

# Include headers
target_include_directories(W11Boost PRIVATE ${CMAKE_SOURCE_DIR})

target_link_libraries(W11Boost PRIVATE
  gpedit.lib
  userenv.lib
  wininet.lib
  shlwapi.lib
)
