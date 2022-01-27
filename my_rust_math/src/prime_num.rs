#[no_mangle]
pub extern "C" fn get_prime_cnt_leq_u32(num: u32) -> u32 {
    let mut cnt = 0;
    for i in 2..=num {
        if is_prime(i) {
            cnt += 1;
        }
    }
    cnt
}

#[no_mangle]
pub extern "C" fn get_prime_cnt_leq_u16(num: u16) -> u16 {
    let mut cnt = 0;
    for i in 2..=num {
        if is_prime(i) {
            cnt += 1;
        }
    }
    cnt
}

#[no_mangle]
pub extern "C" fn get_prime_cnt_leq(num: u64) -> u64 {
    let mut cnt = 0;
    for i in 2..=num {
        if is_prime(i) {
            cnt += 1;
        }
    }
    cnt
}

use num_traits::{FromPrimitive, PrimInt, ToPrimitive, Unsigned};

fn is_prime<T>(num: T) -> bool
where
    T: PrimInt + Unsigned + FromPrimitive + ToPrimitive,
{
    let num = num.to_u64().unwrap();
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
