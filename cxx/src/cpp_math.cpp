#include "cpp_math.h"

auto get_prime_cnt_leq_cpp(uint64_t num) -> uint64_t
{
    auto cnt = uint64_t { 0 };
    for (auto i = uint64_t { 2 }; i <= num; i++) {
        if (is_prime(i)) {
            cnt++;
        }
    }
    return cnt;
}

auto is_prime(uint64_t num) -> bool
{
    if (num == 2) {
        return true;
    }
    if (num % 2 == 0) {
        return false;
    }
    for (auto i = uint64_t { 3 }; i * i <= num; i += 2) {
        if (num % i == 0) {
            return false;
        }
    }
    return true;
}