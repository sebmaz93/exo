pub fn reverse(input: &str) -> String {
   let mut temp = input.chars().collect::<Vec<_>>();
    temp.reverse();
    temp.iter().collect::<String>()
}
