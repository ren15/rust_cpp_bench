#include "cpp_math.h"
#include <my_rust_math.h>

#include <iostream>
using std::cout;

int main()
{
    cout << "Rust Prime cnt <= 10: " << get_prime_cnt_rust(10) << "\n";
    cout << "Rust Prime cnt <= 100: " << get_prime_cnt_rust(100) << "\n";

    cout << "Cpp Prime cnt <= 10: " << get_prime_cnt_cpp(10UL) << "\n";
    cout << "Cpp Prime cnt <= 100: " << get_prime_cnt_cpp(100UL) << "\n";
}