pub fn factor(num: i64) -> Vec<i64> {
    let mut prime_factors = Vec::new();
    let mut r = num;
    loop {
        let pf = get_min_pf(r);
        prime_factors.push(pf);
        if r == pf {
            break;
        }
        r = r / pf;
    }

    return prime_factors;
}

fn get_min_pf(num: i64) -> i64 {
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return i;
        }
        i += 1;
    }
    return num;
}
