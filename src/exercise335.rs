use super::exercise333::combination;

fn cards_combination(cards: &Vec<u64>) -> u64 {
    let (mut red, mut yellow, mut blue) = (0, 0, 0);

    for c in cards {
        if c == &1 {
            red += 1;
        } else if c == &2 {
            yellow += 1;
        } else if c == &3 {
            blue += 1;
        }
    }
    return combination(red, 2) + combination(yellow, 2) + combination(blue, 2);
}
