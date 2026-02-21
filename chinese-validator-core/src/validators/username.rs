//! 用户名验证

/// 验证用户名
pub fn validate_username(username: &str, strict: bool) -> bool {
    if username.is_empty() {
        return false;
    }

    if strict {
        username.len() >= 4 && username.len() <= 20
    } else {
        username.len() >= 2 && username.len() <= 30
    }
}
