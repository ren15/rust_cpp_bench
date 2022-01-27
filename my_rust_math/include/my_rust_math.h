#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Msg {
  uint8_t msg[4];
  uint8_t data[4];
};

extern "C" {

Msg get_primes_rust(uint64_t num);

uint64_t get_prime_cnt_rust(uint64_t num);

} // extern "C"
