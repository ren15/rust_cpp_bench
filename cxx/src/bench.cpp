#include "cpp_math.hpp"
#include <my_rust_math.h>

#include <benchmark/benchmark.h>

#include <cassert>
#include <cstdint>
#include <cstdlib>
#include <iostream>

uint64_t get_num()
{
    static auto num = static_cast<uint64_t>(rand() % 10 + 100);
    return num;
}

static void BM_cpp_u64(benchmark::State& state)
{
    auto num = static_cast<uint64_t>(state.range(0));
    for (auto _ : state) {
        auto a = get_prime_cnt_cpp(num);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_cpp_u64)->RangeMultiplier(4)->Range(64, 1 << 12);

static void BM_cpp_u64_constexpr(benchmark::State& state)
{
    for (auto _ : state) {
        auto a = get_prime_cnt_cpp(1000UL);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_cpp_u64_constexpr);

static void BM_rust_u64(benchmark::State& state)
{
    auto num = static_cast<uint64_t>(state.range(0));
    for (auto _ : state) {
        auto a = get_prime_cnt_rust(num);
        benchmark::DoNotOptimize(a);
    }
}

BENCHMARK(BM_rust_u64)->RangeMultiplier(4)->Range(64, 1 << 12);

BENCHMARK_MAIN();