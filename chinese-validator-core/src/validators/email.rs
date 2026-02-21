//! 邮箱验证

/// 验证邮箱
pub fn validate_email(email: &str, _allow_chinese: bool) -> bool {
    if email.is_empty() {
        return false;
    }

    // 简单的邮箱验证：包含@和.，且@不在开头，.不在结尾
    let at_pos = match email.find('@') {
        Some(pos) if pos > 0 && pos < email.len() - 1 => pos,
        _ => return false,
    };

    let domain_part = &email[at_pos + 1..];
    if domain_part.is_empty() || !domain_part.contains('.') {
        return false;
    }

    let dot_pos = domain_part.find('.').unwrap();
    dot_pos > 0 && dot_pos < domain_part.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        assert!(validate_email("user@example.com", true));
        assert!(validate_email("user.name@example.com", true));
        assert!(validate_email("user+tag@example.com", true));
        assert!(validate_email("user@example.co.uk", true));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!validate_email("", true)); // 空字符串
        assert!(!validate_email("invalid", true)); // 没有@
        assert!(!validate_email("@example.com", true)); // @在开头
        assert!(!validate_email("user@", true)); // @在结尾
        assert!(!validate_email("user@.com", true)); // .在开头
        assert!(!validate_email("user@com.", true)); // .在结尾
        assert!(!validate_email("user@com", true)); // 没有点
    }
}
