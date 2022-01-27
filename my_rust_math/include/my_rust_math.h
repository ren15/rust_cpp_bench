#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

uint32_t get_prime_cnt_leq_u32(uint32_t num);

uint64_t get_prime_cnt_leq_u64(uint64_t num);

uint16_t get_prime_cnt_leq_u16(uint16_t num);

} // extern "C"
