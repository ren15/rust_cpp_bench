#include "cpp_math.h"
#include <cstdint>
#include <my_rust_math.h>

#include <benchmark/benchmark.h>

#include <cassert>
#include <cstdlib>
#include <iostream>

uint64_t get_num()
{
    static auto num = static_cast<uint64_t>(rand() % 100 + 1000);
    return num;
}

static void BM_cpp_u64(benchmark::State& state)
{
    auto num = get_num();
    for (auto _ : state) {
        auto a = get_prime_cnt_leq_cpp(num);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_cpp_u64);

static void BM_cpp_u64_constexpr(benchmark::State& state)
{
    for (auto _ : state) {
        auto a = get_prime_cnt_leq_cpp(1001UL);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_cpp_u64_constexpr);

static void BM_rust_u64(benchmark::State& state)
{
    auto num = get_num();
    for (auto _ : state) {
        auto a = get_prime_cnt_leq(num);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_rust_u64);

BENCHMARK_MAIN();