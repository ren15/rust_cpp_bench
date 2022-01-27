#include <my_rust_math.h>
#include "cpp_math.h"

#include <benchmark/benchmark.h>

#include <cassert>
#include <iostream>

static void BM_pointer(benchmark::State& state)
{
    for (auto _ : state) {
        int a =1;
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_pointer);


BENCHMARK_MAIN();