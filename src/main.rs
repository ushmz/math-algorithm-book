mod exercise311;
mod exercise312;

fn main() {
    exercise();
}

fn exercise() {
    println!("{}", exercise311::is_prime(25));
    println!("{:?}", exercise312::factor(20211225));
}
