use regex::Regex;
/*判断是否是邮箱地址*/
pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})$").unwrap();
    re.is_match(email)
}
