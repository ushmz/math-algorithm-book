use super::exercise333::combination;

fn shortest_sistance(h: u64, w: u64) -> u64 {
    return combination(h + w, h);
}
