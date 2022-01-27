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
// BENCHMARK(BM_cpp_u64_constexpr);

static void BM_cpp_u32(benchmark::State& state)
{
    auto num = static_cast<uint32_t>(get_num());
    for (auto _ : state) {
        auto a = get_prime_cnt_leq_cpp(num);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_cpp_u32);

static void BM_cpp_u16(benchmark::State& state)
{
    auto num = static_cast<uint16_t>(get_num());
    for (auto _ : state) {
        auto a = get_prime_cnt_leq_cpp(num);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_cpp_u16);

static void BM_rust_u64(benchmark::State& state)
{
    auto num = get_num();
    for (auto _ : state) {
        auto a = get_prime_cnt_leq_u64(num);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_rust_u64);

static void BM_rust_u32(benchmark::State& state)
{
    auto num = static_cast<uint32_t>(get_num());
    for (auto _ : state) {
        auto a = get_prime_cnt_leq_u32(num);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_rust_u32);

static void BM_rust_u16(benchmark::State& state)
{
    auto num = static_cast<uint16_t>(get_num());
    for (auto _ : state) {
        auto a = get_prime_cnt_leq_u16(num);
        benchmark::DoNotOptimize(a);
    }
}
BENCHMARK(BM_rust_u16);

BENCHMARK_MAIN();