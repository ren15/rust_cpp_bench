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

fn is_prime(num: u64) -> bool {
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
