pub fn print_gcd_verbose() {
    println!("| Aの値 | Bの値 |");
    println!("| :-: | :-: |");
    gcd_verbose(372, 506);
}

fn gcd_verbose(a: i64, b: i64) -> i64 {
    println!("| {} | {} |", a, b);

    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    match a > b {
        true => gcd_verbose(a % b, b),
        false => gcd_verbose(a, b % a),
    }
}
