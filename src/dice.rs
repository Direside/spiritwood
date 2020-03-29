use rand::thread_rng;
use rand::Rng;

pub fn roll(count: u32, sides: u16) -> Vec<u16> {
    let mut rng = thread_rng();
    (0..count).map(|_| rng.gen_range(1, sides + 1)).collect()
}

#[test]
fn test_roll() {
    assert_eq!(roll(15, 10).len(), 15);
    assert!(roll(500, 6).iter().max().expect("empty vec") <= &6);
    assert!(roll(500, 6).iter().min().expect("empty vec") >= &1);
}
