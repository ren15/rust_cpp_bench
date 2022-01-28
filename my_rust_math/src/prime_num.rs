struct DataStruct {
    primes: Vec<u64>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Msg {
    msg: [u8; 4],
    data: [u8; 4],
}

// #[no_mangle]
// pub extern "C" fn get_primes_rust(num: u64) {
//     let _data = get_primes(num);
// }

#[no_mangle]
pub extern "C" fn get_prime_cnt_rust(num: u64) -> u64 {
    get_prime_cnt_rust1(num)
}

#[inline]
pub fn get_prime_cnt_rust1(num: u64) -> u64 {
    if num == 2 {
        return 1;
    }
    assert!(num >= 3);

    (3..=num)
        .step_by(2)
        .into_iter()
        .filter(|i| is_prime(*i))
        .fold(0, |acc, _| acc + 1)
}

// TODO: make is_prime generic

#[inline]
fn is_prime(num: u64) -> bool {
    if num == 2 {
        return true;
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

fn get_primes(num: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut i = 2;
    while i <= num {
        if is_prime(i) {
            primes.push(i);
        }
        if i == 2 {
            i += 1;
        } else {
            i += 2;
        }
    }
    primes
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
