//! 中国车牌号验证

/// 验证中国大陆车牌号
///
/// # 规则
/// - 传统车牌：1位汉字 + 1位字母 + 5位字母/数字 = 7位
/// - 新能源车牌：1位汉字 + 1位字母 + 6位字母/数字 = 8位
pub fn validate_license_plate(plate: &str) -> bool {
    if plate.is_empty() {
        return false;
    }

    let chars: Vec<char> = plate.chars().collect();
    let char_count = chars.len();

    // 第1位必须是汉字
    if !is_chinese_char(chars[0]) {
        return false;
    }

    // 第2位必须是字母
    if !chars[1].is_ascii_uppercase() {
        return false;
    }

    match char_count {
        7 => {
            // 传统车牌：后面5位必须是字母或数字
            chars[2..]
                .iter()
                .all(|&c| c.is_ascii_uppercase() || c.is_ascii_digit())
        }
        8 => {
            // 新能源车牌：后面6位必须是字母或数字
            chars[2..]
                .iter()
                .all(|&c| c.is_ascii_uppercase() || c.is_ascii_digit())
        }
        _ => false,
    }
}

/// 判断是否是中文字符
fn is_chinese_char(c: char) -> bool {
    ('\u{4e00}'..='\u{9fa5}').contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_traditional_plates() {
        println!("\n=== 测试传统车牌（7位）===");
        let test_cases = vec!["京A12345", "沪B88888", "粤C1234E", "苏D5678F"];

        for plate in test_cases {
            let result = validate_license_plate(plate);
            println!(
                "车牌 '{}' ({}位) 验证结果: {}",
                plate,
                plate.chars().count(),
                result
            );
            assert!(result, "传统车牌 '{}' 应该验证通过", plate);
        }
    }

    #[test]
    fn test_valid_new_energy_plates() {
        println!("\n=== 测试新能源车牌（8位）===");
        let test_cases = vec![
            "京AD12345", // 8位：京 + A + D12345
            "沪AF88888", // 8位：沪 + A + F88888
            "粤BA12345", // 8位：粤 + B + A12345
            "苏CF12345", // 8位：苏 + C + F12345
        ];

        for plate in test_cases {
            let char_count = plate.chars().count();
            let result = validate_license_plate(plate);
            println!("车牌 '{}' ({}位) 验证结果: {}", plate, char_count, result);
            assert!(result, "新能源车牌 '{}' 应该验证通过", plate);
        }
    }

    #[test]
    fn test_invalid_plates() {
        println!("\n=== 测试无效车牌 ===");
        let test_cases = vec![
            ("", false),           // 空字符串
            ("京A1234", false),    // 6位，太短
            ("京A1234567", false), // 9位，太长
            ("京12345", false),    // 第2位不是字母
            ("京A12#45", false),   // 包含特殊字符
            ("京Aa1234", false),   // 小写字母
            ("京A1234@", false),   // 特殊字符结尾
            ("X京A12345", false),  // 第一个字符不是汉字
            ("京A 1234", false),   // 包含空格
            ("京A123456", true),   // 8位，应该是新能源车牌
            ("粤BA123456", false), // 9位，太长（修正：应为false）
        ];

        for (plate, expected) in test_cases {
            let char_count = plate.chars().count();
            let result = validate_license_plate(plate);
            println!(
                "车牌 '{}' ({}位) 验证结果: {}, 期望: {}",
                plate, char_count, result, expected
            );
            assert_eq!(result, expected, "车牌 '{}' 验证失败", plate);
        }
    }
}
