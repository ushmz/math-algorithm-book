pub fn n_gcd(n: &Vec<i64>) -> i64 {
    let mut _gcd = n[0];

    if let Some(nx) = n.get(1..) {
        for ni in nx {
            _gcd = gcd(_gcd, *ni);
        }
    }

    _gcd
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    match a > b {
        true => gcd(a % b, b),
        false => gcd(a, b % a),
    }
}
