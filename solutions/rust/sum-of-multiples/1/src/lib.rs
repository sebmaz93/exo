pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    fn get_multiplies(n: &u32, l: u32) -> Vec<u32> {
        let mut multiples: Vec<u32> = vec![];
        for i in 1..l {
            if n * i >= l {
                break;
            } else {
                multiples.push(n * i);
            }
        }
        multiples
    }

    let mut multiples: Vec<u32> = vec![];
    for x in factors {
        multiples.extend(get_multiplies(x, limit).iter());
    }
    multiples.sort();
    multiples.dedup();
    multiples.iter().sum() 
}
