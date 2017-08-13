pub fn escape(unescaped: &str) -> String {
    let mut escaped = String::with_capacity(unescaped.len());

    for c in unescaped.chars() {
        match c {
            '['|']'|'?'|'*' => {
                escaped.push('[');
                escaped.push(c);
                escaped.push(']');
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
        assert_eq!("test", escape("test"));
    }

    #[test]
    fn no_escaping_2() {
        assert_eq!("test[]]", escape("test]"));
    }

    #[test]
    fn bracket_1() {
        assert_eq!("[[]test", escape("[test"));
    }

    #[test]
    fn bracket_2() {
        assert_eq!("[[]test[]]", escape("[test]"));
    }

    #[test]
    fn bracket_3() {
        assert_eq!("test[[][]]", escape("test[]"));
    }

    #[test]
    fn bracket_4() {
        assert_eq!("test[[]", escape("test["));
    }

    #[test]
    fn bracket_5() {
        assert_eq!("test[[][[]", escape("test[["));
    }

    #[test]
    fn bracket_6() {
        assert_eq!("[[][[]test[[][[]", escape("[[test[["));
    }

    #[test]
    fn asterisk_1() {
        assert_eq!("[*]test", escape("*test"));
    }

    #[test]
    fn asterisk_2() {
        assert_eq!("[*]test[]]", escape("*test]"));
    }

    #[test]
    fn asterisk_3() {
        assert_eq!("test[*][]]", escape("test*]"));
    }

    #[test]
    fn asterisk_4() {
        assert_eq!("test[*]", escape("test*"));
    }

    #[test]
    fn asterisk_5() {
        assert_eq!("test[*][*]", escape("test**"));
    }

    #[test]
    fn asterisk_6() {
        assert_eq!("[*][*]test[*][*]", escape("**test**"));
    }

    #[test]
    fn qmark_1() {
        assert_eq!("[?]test", escape("?test"));
    }

    #[test]
    fn qmark_2() {
        assert_eq!("[?]test[]]", escape("?test]"));
    }

    #[test]
    fn qmark_3() {
        assert_eq!("test[?][]]", escape("test?]"));
    }

    #[test]
    fn qmark_4() {
        assert_eq!("test[?]", escape("test?"));
    }

    #[test]
    fn qmark_5() {
        assert_eq!("test[?][?]", escape("test??"));
    }

    #[test]
    fn qmark_6() {
        assert_eq!("[?][?]test[?][?]", escape("??test??"));
    }

    #[test]
    fn mixed_1() {
        assert_eq!("[[][?][]]test[[][*][]]", escape("[?]test[*]"));
    }
}
