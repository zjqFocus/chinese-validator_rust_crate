//! 身份证验证

/// 验证身份证号码
pub fn validate_id_card(id_card: &str) -> bool {
    id_card.len() == 18
        && id_card
            .chars()
            .all(|c| c.is_ascii_digit() || c == 'X' || c == 'x')
}
