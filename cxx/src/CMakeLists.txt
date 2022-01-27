
add_executable(main
main.cpp
)


target_include_directories(main
BEFORE PRIVATE "${CMAKE_SOURCE_DIR}/../my_rust_math/include"
)

find_library(MY_RUST_MATH_LIB my_rust_math
HINTS "${CMAKE_SOURCE_DIR}/../my_rust_math/target/release")

target_link_libraries(main
${MY_RUST_MATH_LIB}
${CONAN_LIBS}
${BENCHMARK_LIBRARY}
pthread
)
