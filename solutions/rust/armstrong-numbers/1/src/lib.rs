pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|n| n.to_string().parse().expect("provide a valid number!"))
        .collect();

    let length = digits.len();
    let sum : u64 = digits.iter().fold(0, |acc, &x| acc + u64::from(x.pow(length as u32)));
    sum == u64::from(num)
}
