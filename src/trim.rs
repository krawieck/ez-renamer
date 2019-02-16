use regex::Regex;

pub fn trim_right_from(string: &str, from: &str) -> String {
    if from == "" {
        return string.to_owned();
    }
    let from: String = regex::escape(from);
    let reg = Regex::new(&format!("(?P<result>.*){}.*", from)).unwrap();
    reg.captures(&string)
        .unwrap()
        .name("result")
        .unwrap()
        .as_str()
        .to_owned()
}

pub fn trim_right_after(string: &str, after: &str) -> String {
    if after == "" {
        return string.to_owned();
    }
    let after: String = regex::escape(after);
    println!("after {}", after);
    let reg = Regex::new(&format!("(?P<result>.*{}).*", after)).unwrap();
    reg.captures(&string)
        .unwrap()
        .name("result")
        .unwrap()
        .as_str()
        .to_owned()
}

pub fn trim_left_from(string: &str, from: &str) -> String {
    if from == "" {
        return string.to_owned();
    }
    let from: String = regex::escape(from);
    let reg = Regex::new(&format!(".*{}(?P<result>.*)", from)).unwrap();
    reg.captures(&string)
        .unwrap()
        .name("result")
        .unwrap()
        .as_str()
        .to_owned()
}

pub fn trim_left_after(string: &str, after: &str) -> String {
    if after == "" {
        return string.to_owned();
    }
    let after: String = regex::escape(after);
    println!("after {}", after);
    let reg = Regex::new(&format!(".*(?P<result>{}.*)", after)).unwrap();
    reg.captures(&string)
        .unwrap()
        .name("result")
        .unwrap()
        .as_str()
        .to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trim_right_after() {
        assert_eq!(
            super::trim_right_after("black mirror bandersnatch [x265] [1080p]", "snatch"),
            String::from("black mirror bandersnatch")
        );

        assert_eq!(
            super::trim_right_after("black mirror bandersnatch", ""),
            String::from("black mirror bandersnatch")
        );
    }

    #[test]
    fn test_trim_right_from() {
        assert_eq!(
            super::trim_right_from("black mirror bandersnatch [x265] [1080p]", "[x26"),
            String::from("black mirror bandersnatch ")
        );

        assert_eq!(
            super::trim_right_from("black mirror bandersnatch", ""),
            String::from("black mirror bandersnatch")
        );
    }

    #[test]
    fn trim_left_after() {
        assert_eq!(
            super::trim_left_after("black mirror bandersnatch", ""),
            String::from("black mirror bandersnatch")
        );
    }

    #[test]
    fn trim_left_from() {
        assert_eq!(
            super::trim_left_from("black mirror bandersnatch", ""),
            String::from("black mirror bandersnatch")
        );
    }
}
