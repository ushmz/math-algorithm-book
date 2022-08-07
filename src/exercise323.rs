use super::exercise322::gcd;

pub fn n_lcm(n: &Vec<i64>) -> i64 {
    let mut _lcm = n[0];

    if let Some(nx) = n.get(1..) {
        for ni in nx {
            _lcm = lcm(_lcm, *ni);
        }
    }

    _lcm
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
