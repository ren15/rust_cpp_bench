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

trait GenNum {
    fn new() -> Self;
    fn set_as_2(&mut self);
    fn squared_leq(&self, rhs: &Self) -> bool;
    fn is_mod(&self, rhs: &Self) -> bool;
    fn add_2(&mut self);
}

impl GenNum for u32 {
    fn new() -> Self {
        0u32
    }

    fn set_as_2(&mut self) {
        *self = 2;
    }

    fn squared_leq(&self, rhs: &Self) -> bool {
        (*self) * (*self) <= *rhs
    }
    fn is_mod(&self, rhs: &Self) -> bool {
        (*rhs) % (*self) == 0
    }
    fn add_2(&mut self) {
        *self += 2;
    }
}

impl GenNum for u64 {
    fn new() -> Self {
        0u64
    }

    fn set_as_2(&mut self) {
        *self = 2;
    }

    fn squared_leq(&self, rhs: &Self) -> bool {
        (*self) * (*self) <= *rhs
    }
    fn is_mod(&self, rhs: &Self) -> bool {
        (*rhs) % (*self) == 0
    }
    fn add_2(&mut self) {
        *self += 2;
    }
}

#[inline]
fn is_prime_gen<T>(num: T) -> bool
where
    T: GenNum,
{
    let mut i = T::new();
    i.set_as_2();
    while i.squared_leq(&num) {
        if i.is_mod(&num) {
            return false;
        }
        i.add_2();
    }
    true
}

#[test]
fn test_is_prime_gen() {
    assert_eq!(is_prime_gen(2u64), true);
    assert_eq!(is_prime_gen(3u64), true);
    assert_eq!(is_prime_gen(4u64), false);
    assert_eq!(is_prime_gen(5u64), true);
    assert_eq!(is_prime_gen(6u64), false);
    assert_eq!(is_prime_gen(10u64), false);
    assert_eq!(is_prime_gen(13u64), true);

    assert_eq!(is_prime_gen(2u32), true);
    assert_eq!(is_prime_gen(3u32), true);
    assert_eq!(is_prime_gen(4u32), false);
    assert_eq!(is_prime_gen(5u32), true);
    assert_eq!(is_prime_gen(6u32), false);
    assert_eq!(is_prime_gen(10u32), false);
    assert_eq!(is_prime_gen(13u32), true);
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
