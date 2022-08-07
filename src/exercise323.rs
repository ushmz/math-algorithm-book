pub fn n_lcm(n: &Vec<i64>) -> i64 {
    let mut n1 = n[0];

    if let Some(nx) = n.get(1..) {
        for ni in nx {
            n1 = lcm(n1, *ni);
        }
    }

    n1
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn gcd(a: i64, b: i64) -> i64 {
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
