#include "cpp_math.h"
#include <my_rust_math.h>

#include <iostream>
using std::cout;

int main()
{
    cout << "Rust Prime cnt <= 10: " << get_prime_cnt_leq(10) << "\n";
    cout << "Rust Prime cnt <= 100: " << get_prime_cnt_leq(100) << "\n";

    cout << "Cpp Prime cnt <= 10: " << get_prime_cnt_leq_cpp(10UL) << "\n";
    cout << "Cpp Prime cnt <= 100: " << get_prime_cnt_leq_cpp(100UL) << "\n";
}