# CMakeList.txt: projeto de CMake para commons, incluir origem e definir
# lógica específica a um projeto aqui.
#

# Adicione a origem ao executável deste projeto.
add_executable (commons "commons.cpp" "commons.h"
        EncodeH264.cpp
        EncodeH264.h
        Encoders.cpp
        Encoders.h)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET commons PROPERTY CXX_STANDARD 20)
endif()

# TODO: Adicione testes e instale destinos, se necessário.

find_package(FFMPEG REQUIRED)
target_include_directories(commons PRIVATE ${FFMPEG_INCLUDE_DIRS})
target_link_directories(commons PRIVATE ${FFMPEG_LIBRARY_DIRS})
target_link_libraries(commons PRIVATE ${FFMPEG_LIBRARIES})