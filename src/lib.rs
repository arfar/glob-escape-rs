pub fn escape(unescaped: String) -> String {
    let mut num_chars_to_escape = 0;

    for c in unescaped.chars() {
        match c {
            '['|'?'|'*' => num_chars_to_escape += 1,
            _ => continue
        }
    }
    println!("{} contains {} escape-able character(s)", unescaped, num_chars_to_escape);

    let mut escaped = String::with_capacity(num_chars_to_escape + unescaped.len());

    for c in unescaped.chars() {
        match c {
            '['|'?'|'*' => {
                escaped.push('\\');
                escaped.push(c);
            },
            _ => escaped.push(c),
        }
    }
    escaped
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_escaping_1() {
        let s1 = String::from("test");
        assert_eq!("test", escape(s1));
    }

    #[test]
    fn no_escaping_2() {
        let s1 = String::from("test]");
        assert_eq!("test]", escape(s1));
    }

    #[test]
    fn bracket_1() {
        let s1 = String::from("[test");
        assert_eq!("\\[test", escape(s1));
    }

    #[test]
    fn bracket_2() {
        let s1 = String::from("[test]");
        assert_eq!("\\[test]", escape(s1));
    }

    #[test]
    fn bracket_3() {
        let s1 = String::from("test[]");
        assert_eq!("test\\[]", escape(s1));
    }

    #[test]
    fn bracket_4() {
        let s1 = String::from("test[");
        assert_eq!("test\\[", escape(s1));
    }

    #[test]
    fn bracket_5() {
        let s1 = String::from("test[[");
        assert_eq!("test\\[\\[", escape(s1));
    }

    #[test]
    fn bracket_6() {
        let s1 = String::from("[[test[[");
        assert_eq!("\\[\\[test\\[\\[", escape(s1));
    }

    #[test]
    fn asterisk_1() {
        let s1 = String::from("*test");
        assert_eq!("\\*test", escape(s1));
    }

    #[test]
    fn asterisk_2() {
        let s1 = String::from("*test]");
        assert_eq!("\\*test]", escape(s1));
    }

    #[test]
    fn asterisk_3() {
        let s1 = String::from("test*]");
        assert_eq!("test\\*]", escape(s1));
    }

    #[test]
    fn asterisk_4() {
        let s1 = String::from("test*");
        assert_eq!("test\\*", escape(s1));
    }

    #[test]
    fn asterisk_5() {
        let s1 = String::from("test**");
        assert_eq!("test\\*\\*", escape(s1));
    }

    #[test]
    fn asterisk_6() {
        let s1 = String::from("**test**");
        assert_eq!("\\*\\*test\\*\\*", escape(s1));
    }

    #[test]
    fn qmark_1() {
        let s1 = String::from("?test");
        assert_eq!("\\?test", escape(s1));
    }

    #[test]
    fn qmark_2() {
        let s1 = String::from("?test]");
        assert_eq!("\\?test]", escape(s1));
    }

    #[test]
    fn qmark_3() {
        let s1 = String::from("test?]");
        assert_eq!("test\\?]", escape(s1));
    }

    #[test]
    fn qmark_4() {
        let s1 = String::from("test?");
        assert_eq!("test\\?", escape(s1));
    }

    #[test]
    fn qmark_5() {
        let s1 = String::from("test??");
        assert_eq!("test\\?\\?", escape(s1));
    }

    #[test]
    fn qmark_6() {
        let s1 = String::from("??test??");
        assert_eq!("\\?\\?test\\?\\?", escape(s1));
    }

    #[test]
    fn mixed_1() {
        let s1 = String::from("[?]test[*]");
        assert_eq!("\\[\\?]test\\[\\*]", escape(s1));
    }
}
