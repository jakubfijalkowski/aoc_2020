// We could use some more advanced algo for discrete logarithm here (I'm really tempted to use the
// quantum one on Q#...), but maybe next year ;)
fn turn(v: u64, subject: u64) -> u64 {
    (v * subject) % 20201227
}

fn reverse_loop_size(v: u64, subject: u64) -> u64 {
    let mut it = 0..;
    it.try_fold(1, |agg, _| {
        if agg == v {
            None
        } else {
            Some(turn(agg, subject))
        }
    });
    it.next().unwrap() - 1
}

fn transform(size: u64, subject: u64) -> u64 {
    (0..size).fold(1, |x, _| turn(x, subject))
}

fn part1() {
    const A: u64 = 18356117;
    const B: u64 = 5909654;
    let size_a = reverse_loop_size(A, 7);
    let size_b = reverse_loop_size(B, 7);
    let enc_a = transform(size_a, B);
    let enc_b = transform(size_b, A);
    assert_eq!(enc_a, enc_b);

    println!("Part 1: {}", enc_a);
}

fn main() {
    part1();
    // There is no part2 :)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_loop_size_simple() {
        assert_eq!(reverse_loop_size(5764801, 7), 8);
        assert_eq!(reverse_loop_size(17807724, 7), 11);
    }

    #[test]
    fn transform_simple() {
        assert_eq!(transform(8, 17807724), 14897079);
        assert_eq!(transform(11, 5764801), 14897079);
    }
}
