#[derive(Debug, Clone)]
#[repr(C)]
pub struct Msg {
    msg: [u8; 4],
    data: [u8; 4],
}

#[no_mangle]
pub extern "C" fn get_primes_rust(num: u64) -> Msg {
    let data = get_primes(num);
    
    Msg {
        msg: [0; 4],
        data: [1; 4],
    }
}

#[no_mangle]
pub extern "C" fn get_prime_cnt_rust(num: u64) -> u64 {
    let mut cnt = 0;
    for i in 2..=num {
        if is_prime(i) {
            cnt += 1;
        }
    }
    cnt
}

fn get_primes(num: u64) -> Vec<u64> {
    let mut data = Vec::new();
    for i in 2..=num {
        if is_prime(i) {
            data.push(i);
        }
    }
    data
}

#[inline]
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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_prime_cnt() {
        assert_eq!(get_prime_cnt_rust(2), 1);
        assert_eq!(get_prime_cnt_rust(3), 2);
        assert_eq!(get_prime_cnt_rust(10), 4);
    }
    #[test]
    fn test_get_primes() {
        let primes = get_primes(10);
        let sum = primes.iter().sum::<u64>();
        assert_eq!(sum, 17);
    }
}
