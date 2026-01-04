pub fn factors(n: u64) -> Vec<u64> {
    let mut fac: Vec<u64> = vec![];
    let mut num = n;
    let mut divisor = 2;

    while num != 1 {
        if num % divisor == 0 {
            num /= divisor;
            fac.push(divisor)
        } else {
            divisor += 1;
        }
    }

    fac
}
