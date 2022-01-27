#include <concepts>
#include <vector>

template <std::unsigned_integral T>
constexpr auto is_prime(T num) noexcept -> bool
{
    if (num == T { 0 }) {
        return true;
    }
    if (num % T { 2 } == T { 0 }) {
        return false;
    }
    for (auto i = T { 3 }; i * i <= num; i += T { 2 }) {
        if (num % i == T { 0 }) {
            return false;
        }
    }
    return true;
}

template <std::unsigned_integral T>
constexpr auto get_prime_cnt_cpp(T num) noexcept -> T
{
    auto cnt = T { 0 };
    for (auto i = T { 2 }; i <= num; i++) {
        if (is_prime(i)) {
            cnt++;
        }
    }
    return cnt;
}

template <std::unsigned_integral T>
auto get_primes_cpp(T num) noexcept -> std::vector<T>
{
    std::vector<T> primes;
    for (auto i = T { 2 }; i <= num; i++) {
        if (is_prime(i)) {
            primes.push_back(i);
        }
    }
    return primes;
}
