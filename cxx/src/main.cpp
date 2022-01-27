#include <my_rust_math.h>

#include <iostream>
using std::cout;

int main()
{
    cout << "Prime cnt <= 10: " << get_prime_cnt_leq(10) << "\n";
    cout << "Prime cnt <= 100: " << get_prime_cnt_leq(100) << "\n";
}