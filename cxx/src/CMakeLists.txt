
find_library(MY_RUST_MATH_LIB my_rust_math
HINTS "${CMAKE_SOURCE_DIR}/../my_rust_math/target/release")

## ==============================================================

add_executable(main
main.cpp
)

target_include_directories(main
BEFORE PRIVATE "${CMAKE_SOURCE_DIR}/../my_rust_math/include"
)

target_link_libraries(main
${MY_RUST_MATH_LIB}
${CONAN_LIBS}
)

## ===============================================================

set(target_name bench)

add_executable(${target_name}
bench.cpp
)

target_include_directories(${target_name}
BEFORE PRIVATE "${BENCHMARK_FOLDER}/include"
)
target_include_directories(${target_name}
BEFORE PRIVATE "${CMAKE_SOURCE_DIR}/../my_rust_math/include"
)

target_link_libraries(${target_name}
${MY_RUST_MATH_LIB}
${CONAN_LIBS}
${BENCHMARK_LIBRARY}
pthread
)
