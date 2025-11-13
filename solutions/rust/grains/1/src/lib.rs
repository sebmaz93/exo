pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 and 64");
    }
    if s == 1 {
        return 1;
    }
    return 2u64.pow(s - 1);
}

pub fn total() -> u64 {
    let n: u32 = 64;
    let a: u64 = 2;
    let r: u64 = 2;

    return 1u64 + (a * (r.pow(n - 1) - 1)) / (r - 1);
}