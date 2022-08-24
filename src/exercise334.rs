fn product_permutation(products: &Vec<u64>) -> u64 {
    let (mut p100, mut p200, mut p300, mut p400) = (0, 0, 0, 0);

    for p in products {
        if p == &100 {
            p100 += 1;
        } else if p == &200 {
            p200 += 1;
        } else if p == &300 {
            p300 += 1;
        } else if p == &400 {
            p400 += 1;
        }
    }
    return p100 * p400 + p200 * p300;
}
