#include <concepts>

constexpr auto is_prime(std::unsigned_integral auto num) -> bool
{
    if (num == 2) {
        return true;
    }
    if (num % 2 == 0) {
        return false;
    }
    for (auto i = decltype(num) { 3 }; i * i <= num; i += 2) {
        if (num % i == 0) {
            return false;
        }
    }
    return true;
}

constexpr auto get_prime_cnt_leq_cpp(std::unsigned_integral auto num) -> decltype(num)
{
    auto cnt = decltype(num) { 0 };
    for (auto i = decltype(num) { 2 }; i <= num; i++) {
        if (is_prime(i)) {
            cnt++;
        }
    }
    return cnt;
}
