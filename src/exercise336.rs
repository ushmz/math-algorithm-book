use std::collections::HashMap;

pub fn cards_combination_exhaustive(cards: &Vec<u64>) -> u64 {
    let mut map = HashMap::<u64, u64>::new();
    let mut ans = 0;

    for card in cards {
        *map.entry(*card).or_insert(0) += 1;
    }

    for i in 1..50000 {
        ans += map[&i] * map[&(100000 - i)];
    }
    ans += map[&50000] * (map[&50000] - 1) / 2;

    return ans;
}
