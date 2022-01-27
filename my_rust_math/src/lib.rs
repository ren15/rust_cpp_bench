pub mod prime_num;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_prime_cnt() {
        assert_eq!(prime_num::get_prime_cnt_leq(1), 0);
        assert_eq!(prime_num::get_prime_cnt_leq(2), 1);
        assert_eq!(prime_num::get_prime_cnt_leq(3), 2);
        assert_eq!(prime_num::get_prime_cnt_leq(10), 4);
    }
}
