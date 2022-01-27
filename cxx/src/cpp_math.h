#include <concepts>

constexpr auto is_prime(std::unsigned_integral auto num) noexcept -> bool
{
    constexpr auto literal_0 = decltype(num) { 0 };
    constexpr auto literal_2 = decltype(num) { 2 };
    constexpr auto literal_3 = decltype(num) { 3 };

    if (num == literal_0) {
        return true;
    }
    if (num % literal_2 == literal_0) {
        return false;
    }
    for (auto i = literal_3; i * i <= num; i += literal_2) {
        if (num % i == literal_0) {
            return false;
        }
    }
    return true;
}

constexpr auto get_prime_cnt_leq_cpp(std::unsigned_integral auto num) noexcept -> decltype(num)
{
    constexpr auto literal_0 = decltype(num) { 0 };
    constexpr auto literal_2 = decltype(num) { 2 };

    auto cnt = literal_0;
    for (auto i = literal_2; i <= num; i++) {
        if (is_prime(i)) {
            cnt++;
        }
    }
    return cnt;
}
