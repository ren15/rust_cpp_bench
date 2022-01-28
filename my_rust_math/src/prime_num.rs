struct DataStruct {
    primes: Vec<u64>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Msg {
    msg: [u8; 4],
    data: [u8; 4],
}

#[no_mangle]
pub extern "C" fn get_primes_rust(num: u64) {
    let _data = get_primes(num);
}

#[no_mangle]
pub extern "C" fn get_prime_cnt_rust(num: u64) -> u64 {
    get_prime_cnt_rust1(num)
}

fn get_prime_cnt_rust1(num: u64) -> u64 {
    let mut cnt = 0;
    for i in 2..=num {
        if is_prime(i) {
            cnt += 1;
        }
    }
    cnt
}

fn get_prime_cnt_rust2(num: u64) -> u64 {
    (2..=num)
        .into_iter()
        .filter(|i| is_prime(*i))
        .fold(0, |acc, _| acc + 1)
}

fn get_primes(num: u64) -> Vec<u64> {
    (2..=num)
        .into_iter()
        .filter(|i| is_prime(*i))
        .collect::<Vec<u64>>()
}

#[inline]
fn is_prime(num: u64) -> bool {
    let mut j = 2;
    while j * j <= num {
        if num % j == 0 {
            return false;
        }
        j += 1;
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
