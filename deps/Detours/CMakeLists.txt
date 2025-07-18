project(Detours)

add_library(
	${PROJECT_NAME} STATIC
		creatwth.cpp
		detours.cpp
		detours.h
		detver.h
		disasm.cpp
		disolarm.cpp
		disolarm64.cpp
		disolia64.cpp
		disolx64.cpp
		disolx86.cpp
		image.cpp
		modules.cpp
		uimports.cpp
)
set_property(SOURCE uimports.cpp APPEND PROPERTY HEADER_FILE_ONLY true)

# Set compiler-specific flags
if (CMAKE_CXX_COMPILER_ID STREQUAL "GNU" AND CMAKE_SYSTEM_NAME STREQUAL "Windows")
	# MinGW cross-compilation - use GCC-style flags
	target_compile_definitions(
		${PROJECT_NAME} PRIVATE
			_X86_
			WIN32_LEAN_AND_MEAN
			_CRT_SECURE_NO_WARNINGS
	)
	# Disable some problematic warnings for MinGW
	target_compile_options(
		${PROJECT_NAME} PRIVATE
			-Wno-multichar
			-Wno-attributes
			-Wno-conversion-null
			-Wno-pointer-arith
	)
else()
	# MSVC/Clang flags
	target_compile_options(
		${PROJECT_NAME} PRIVATE
			/we4777
			/we4800
	)
	target_compile_definitions(
		${PROJECT_NAME} PRIVATE
			_X86_
	)
endif()