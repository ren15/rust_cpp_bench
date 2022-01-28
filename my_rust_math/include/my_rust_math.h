#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void get_primes_rust(uint64_t num);

uint64_t get_prime_cnt_rust(uint64_t num);

} // extern "C"
