use crate::args_parser::Args;

pub fn trim(string: &str, args: &Args) -> String {
    use regex::Regex;
    // TODO: set verbose as env variable and use it here
    let mut string = string;
    let mut regs: Vec<Regex> = vec![];

    if args.trim_right_with != "" {
        regs.push(
            Regex::new(&format!(
                "(?P<result>.*){}.*",
                regex::escape(&args.trim_right_with)
            ))
            .unwrap(),
        )
    }

    if args.trim_right_after != "" {
        regs.push(
            Regex::new(&format!(
                "(?P<result>.*{}).*",
                regex::escape(&args.trim_right_after)
            ))
            .unwrap(),
        )
    }

    if args.trim_left_with != "" {
        regs.push(
            Regex::new(&format!(
                ".*{}(?P<result>.*)",
                regex::escape(&args.trim_left_with)
            ))
            .unwrap(),
        )
    }

    if args.trim_left_after != "" {
        regs.push(
            Regex::new(&format!(
                ".*(?P<result>{}.*)",
                regex::escape(&args.trim_left_after)
            ))
            .unwrap(),
        )
    }
    for reg in regs {
        string = match reg.captures(&string) {
            Some(ref capture) => capture.name("result").unwrap().as_str(),
            None => string,
        }
    }

    string.to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trim() {
        use crate::args_parser::Args;

        let mut args = Args::new();

        assert_eq!(
            super::trim("black mirror bandersnatch [x265] [1080p]", &args),
            String::from("black mirror bandersnatch [x265] [1080p]")
        );

        args.trim_right_after = String::from("snatch");
        assert_eq!(
            super::trim("black mirror bandersnatch [x265] [1080p]", &args),
            String::from("black mirror bandersnatch")
        );

        args.trim_right_after = String::new();
        args.trim_right_with = String::from("[x26");
        assert_eq!(
            super::trim("black mirror bandersnatch [x265] [1080p]", &args),
            String::from("black mirror bandersnatch ")
        );

        args.trim_right_with = String::new();
        args.trim_left_after = String::from("black");

        assert_eq!(
            super::trim("[HoriibelSubs] black mirror bandersnatch", &args),
            String::from("black mirror bandersnatch")
        );

        args.trim_left_after = String::new();
        args.trim_left_with = String::from("ubs]");
        assert_eq!(
            super::trim("[HoriibelSubs] black mirror bandersnatch", &args),
            String::from(" black mirror bandersnatch")
        );

        assert_eq!(
            super::trim("fnaewuofajdsnfawkjenfkjandskjfawhebfouiabsnjdnfai", &args),
            String::from("fnaewuofajdsnfawkjenfkjandskjfawhebfouiabsnjdnfai")
        );

        assert_eq!(super::trim("", &args), String::from(""));
    }
}
