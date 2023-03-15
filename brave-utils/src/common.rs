use regex::Regex;
/*判断是否是邮箱地址*/
pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})$").unwrap();
    re.is_match(email)
}

/*判断是否是outlook邮箱*/
pub fn is_outlook_email(email: &str) -> bool {
    let email_parts: Vec<&str> = email.split('@').collect();
    if email_parts.len() != 2 {
        return false;
    }
    let domain = email_parts[1].to_lowercase();
    domain == "outlook.com"
}
