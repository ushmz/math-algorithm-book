mod exercise311;
mod exercise312;
mod exercise321;
mod exercise322;
mod exercise323;
mod exercise331;
mod exercise332;
mod exercise333;
mod exercise334;

fn main() {
    exercise();
}

fn print_divider(title: &str) {
    print!("\nex. {} -------------\n", title)
}

fn exercise() {
    print_divider("3.1.1");
    println!("{}", exercise311::is_prime(25));

    print_divider("3.1.2");
    println!("{:?}", exercise312::factor(20211225));

    print_divider("3.2.1");
    exercise321::print_gcd_verbose();

    print_divider("3.2.2");
    let n_322 = vec![56, 88, 96, 168, 204];
    println!("{}", exercise322::n_gcd(&n_322));

    print_divider("3.2.3");
    let n_323 = vec![56, 88, 96];
    println!("{}", exercise323::n_lcm(&n_323));

    print_divider("3.3.1");
    exercise331::print_answer();

    print_divider("3.3.2");
    exercise332::print_answer();
}
