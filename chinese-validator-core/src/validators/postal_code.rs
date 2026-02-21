//! 邮政编码验证

/// 验证中国邮政编码
pub fn validate_postal_code(postal_code: &str) -> bool {
    if postal_code.len() != 6 {
        return false;
    }

    postal_code.chars().all(|c| c.is_ascii_digit()) && postal_code.chars().next().unwrap() != '0'
}
