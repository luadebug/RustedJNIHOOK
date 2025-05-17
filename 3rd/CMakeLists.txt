cmake_minimum_required(VERSION 3.21)

project(jnihook LANGUAGES C CXX)

set (CMAKE_CXX_STANDARD 23) # Only necessary for <jnihook.hpp>. If you will use <jnihook.h> only, C++ 11 is fine.

option(JNIHOOK_BUILD_TESTS "Enable building of tests" OFF)

if(MSVC)
	option(JNIHOOK_MT_RUNTIME "Build with MT runtime" OFF)
endif()

include(FetchContent)

FetchContent_Declare(OpenJDK
  URL "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.7%2B6/OpenJDK21U-jdk_x64_windows_hotspot_21.0.7_6.zip"
  URL_HASH SHA256=38f4b9fa0b36def9812f6576fd45f6224630477db8c4e669ee78eaa35abb9195
  DOWNLOAD_EXTRACT_TIMESTAMP TRUE
)

FetchContent_MakeAvailable(OpenJDK)
# Access the populated source/binary directories
message(STATUS "OpenJDK source dir: ${openjdk_SOURCE_DIR}")
message(STATUS "OpenJDK binary dir: ${openjdk_BINARY_DIR}")

# Set a custom variable pointing to the extracted JDK
set(JAVA_HOME "${openjdk_SOURCE_DIR}")  # Adjust path if needed (see note below)
message(STATUS "JAVA_HOME = ${JAVA_HOME}")

set(JNIHOOK_DIR "${PROJECT_SOURCE_DIR}/src")
set(JNIHOOK_INC "${PROJECT_SOURCE_DIR}/include")
set(JNIHOOK_SRC "${JNIHOOK_DIR}/jnihook.cpp" "${JNIHOOK_DIR}/classfile.cpp" "${JNIHOOK_DIR}/uuid.cpp")

set(JAVA_INCLUDES "${JAVA_HOME}/include")
if(${CMAKE_SYSTEM_NAME} STREQUAL "Windows")
	set(JAVA_INCLUDES ${JAVA_INCLUDES} "${JAVA_HOME}/include/win32")
else()
	message(ERROR "Windows OS ONLY. Operating system not supported: " ${CMAKE_SYSTEM_NAME})
endif()

add_library(jnihook STATIC ${JNIHOOK_SRC})
target_include_directories(jnihook PUBLIC ${JNIHOOK_INC} ${JAVA_INCLUDES})
set_target_properties(jnihook PROPERTIES POSITION_INDEPENDENT_CODE True)
# TODO: Add other architectures for OpenJDK8 lookup path
#       since it uses a non-standard path across platforms ($JAVA_HOME/jre/lib/<ARCH>/server)
target_link_directories(jnihook PUBLIC "${JAVA_HOME}/lib" "${JAVA_HOME}/lib/server" "${JAVA_HOME}/jre/lib/amd64/server/")
target_link_libraries(jnihook PUBLIC jvm)

# Runtime library configuration (for MSVC)
if(MSVC)
	if(JNIHOOK_MT_RUNTIME)
		message(STATUS "Building for MT/MTd")
        target_compile_options(jnihook PRIVATE
            $<$<CONFIG:>:/MT>
            $<$<CONFIG:Debug>:/MTd>
            $<$<CONFIG:Release>:/MT>
        )
    else()
		message(STATUS "Building for MD/MDd")
        target_compile_options(jnihook PRIVATE
            $<$<CONFIG:>:/MD>
            $<$<CONFIG:Debug>:/MDd>
            $<$<CONFIG:Release>:/MD>
        )
	endif()
endif()

if(JNIHOOK_BUILD_TESTS)
	# Build Java classes
	file(COPY "${PROJECT_SOURCE_DIR}/tests/dummy" DESTINATION .)
	execute_process(COMMAND "${JAVA_HOME}/bin/javac${CMAKE_EXECUTABLE_SUFFIX}" "dummy/Dummy.java")
	
	# Build library to inject
	set(TESTS_DIR "${PROJECT_SOURCE_DIR}/tests")
	set(TESTS_SRC "${TESTS_DIR}/test.cpp")
	add_library(test SHARED ${TESTS_SRC})
	target_link_libraries(test PRIVATE jnihook)
	set_target_properties(test PROPERTIES POSITION_INDEPENDENT_CODE True)
endif()
