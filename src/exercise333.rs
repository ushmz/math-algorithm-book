fn fuctorial(n: u64) -> u64 {
    if n < 1 {
        return 1;
    }
    return n * fuctorial(n - 1);
}

fn permutation(n: u64, r: u64) -> u64 {
    if n < r {
        return 0;
    }
    return fuctorial(n) / fuctorial(n - r);
}

pub fn combination(n: u64, r: u64) -> u64 {
    return permutation(n, r) / fuctorial(r);
}
