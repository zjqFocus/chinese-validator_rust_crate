//! 手机号验证

/// 验证中国大陆手机号
pub fn validate_cn_phone(phone: &str) -> bool {
    phone.len() == 11 && phone.chars().all(|c| c.is_ascii_digit())
}
