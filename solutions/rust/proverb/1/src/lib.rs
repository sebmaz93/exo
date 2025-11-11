pub fn build_proverb(list: &[&str]) -> String {
    let len = list.len();

    if len == 0 {
        return String::from("");
    }

    let mut res = String::new();

    fn phrase(x: &str, y: Option<&str>) -> String {
        if let Some(y_str) = y {
            return format!("For want of a {} the {} was lost.", x, y_str);
        }
        format!("And all for the want of a {}.", x)
    }

    for (idx, i) in list.iter().enumerate() {
        if idx + 1 == len {
            res.push_str(&phrase(list[0], None));
        } else {
        res.push_str(&phrase(i, Some(list[idx + 1])));
        res.push('\n');
            
        }
    }

    res
}
