pub fn is_prime(num: i64) -> bool {
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}
