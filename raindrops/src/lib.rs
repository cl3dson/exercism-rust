pub fn raindrops(n: u32) -> String {
    let mut result =  String::new();
    if n % 3 == 0 {
        result.push_str("Pling");
    }
    if n % 5 == 0 {
        result.push_str("Plang");
    }
    if n % 7 == 0 {
        result.push_str("Plong");
    }

    if result.is_empty() {
        n.to_string()
    }else {
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::raindrops;

    #[test]
    fn raindrops_test() {
        assert_eq!(raindrops(3), String::from("Pling"));
        assert_eq!(raindrops(5), String::from("Plang"));
        assert_eq!(raindrops(7), String::from("Plong"));
        assert_eq!(raindrops(15), String::from("PlingPlang"));
        assert_eq!(raindrops(17), String::from("17"));

    }
}