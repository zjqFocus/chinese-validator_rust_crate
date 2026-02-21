//! 中国地址验证

/// 验证中国地址
pub fn validate_address(address: &str) -> bool {
    if address.is_empty() {
        return false;
    }

    // 使用 chars().count() 获取字符数，而不是字节数
    let char_count = address.chars().count();
    if char_count < 5 || char_count > 100 {
        return false;
    }

    // 检查每个字符
    for c in address.chars() {
        // 允许中文字符
        if ('\u{4e00}'..='\u{9fa5}').contains(&c) {
            continue;
        }
        // 允许英文字母（大小写都允许）
        if c.is_ascii_alphabetic() {
            continue;
        }
        // 允许数字
        if c.is_ascii_digit() {
            continue;
        }
        // 允许空格
        if c.is_ascii_whitespace() {
            continue;
        }
        // 允许常见标点符号
        match c {
            ',' | '.' | '-' | '_' | '/' | '\\' | '#' | '@' | '&' | '*' | '+' | '=' | '|' | ':'
            | ';' | '!' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '，' | '。'
            | '、' | '；' | '：' | '！' | '？' | '（' | '）' | '【' | '】' | '《' | '》' | '—'
            | '·' => continue,
            _ => return false,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_addresses() {
        assert!(validate_address("北京市朝阳区xxx街道xxx号"));
        assert!(validate_address("上海市浦东新区世纪大道100号"));
        assert!(validate_address("广东省广州市天河区体育西路123号"));
        assert!(validate_address("Building A, No.123, Street Name, Beijing"));
        assert!(validate_address("XX小区5号楼301室"));
    }

    #[test]
    fn test_invalid_addresses() {
        assert!(!validate_address(""), "空字符串应该返回false");

        // 调试输出
        let test_str = "北京";
        println!("字符串: '{}'", test_str);
        println!("字节数: {}", test_str.len());
        println!("字符数: {}", test_str.chars().count());

        assert!(
            !validate_address(test_str),
            "'北京'字符数是2，应该返回false"
        );
        assert!(!validate_address(&"a".repeat(101)), "长度101应该返回false");
        assert!(
            !validate_address("北京市@#$%朝阳区"),
            "包含特殊字符应该返回false"
        );
    }
}
