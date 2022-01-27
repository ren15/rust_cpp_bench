#[derive(Debug, Clone)]
#[repr(C)]
pub struct Msg {
    msg: [u8; 4],
    data: [u8; 4],
}

#[no_mangle]
pub extern "C" fn gen_prime_vec(num: u64) -> Msg {
    let mut data = Vec::new();
    for i in 2..=num {
        if is_prime_u64(i) {
            data.push(i);
        }
    }
    Msg {
        msg: [0; 4],
        data: [1; 4],
    }
}

#[no_mangle]
pub extern "C" fn get_prime_cnt_rust(num: u64) -> u64 {
    let mut cnt = 0;
    for i in 2..=num {
        if is_prime_u64(i) {
            cnt += 1;
        }
    }
    cnt
}

fn is_prime_u64(num: u64) -> bool {
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
