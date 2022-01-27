set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_C_FLAGS "-O3 -ffast-math -march=native ${CMAKE_C_FLAGS}")
set(CMAKE_C_FLAGS "-Wall -Wextra -Wconversion -Wshadow -Wpedantic ${CMAKE_C_FLAGS}")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} ${CMAKE_CXX_FLAGS}")

# set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -fsanitize=address,undefined")

# set(CMAKE_CXX_CLANG_TIDY "clang-tidy")

