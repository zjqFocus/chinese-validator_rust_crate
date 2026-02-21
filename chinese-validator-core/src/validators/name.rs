//! 中文姓名验证

/// 验证中文姓名
pub fn validate_chinese_name(name: &str, allow_anonymous: bool) -> bool {
    if name.is_empty() {
        return false;
    }

    if allow_anonymous {
        name.len() >= 1 && name.len() <= 10
    } else {
        name.len() >= 2 && name.len() <= 10
    }
}
