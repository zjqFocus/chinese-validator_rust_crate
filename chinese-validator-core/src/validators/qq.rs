//! QQ号码验证

/// 验证QQ号码
pub fn validate_qq(qq: &str) -> bool {
    if qq.is_empty() || qq.len() < 5 || qq.len() > 11 {
        return false;
    }

    qq.chars().all(|c| c.is_ascii_digit()) && qq.chars().next().unwrap() != '0'
}
