// 集成测试

//! 集成测试文件
//! 
//! 这个文件测试 chinese-validator crate 的公共 API 是否正确工作。
//! 运行方式: `cargo test --test integration`

use chinese_validator::ChineseValidate;

/// 测试完整的用户注册表单
#[derive(Debug, ChineseValidate, PartialEq)]
struct RegisterForm {
    #[chinese(name)]
    username: String,
    
    #[chinese(phone)]
    phone: String,
    
    #[chinese(id_card)]
    id_card: String,
    
    #[chinese(email)]
    email: String,
    
    #[chinese(nickname)]
    nickname: String,
}

/// 测试匿名模式
#[derive(Debug, ChineseValidate, PartialEq)]
struct AnonymousForm {
    #[chinese(anonymous_name)]
    name: String,
    
    #[chinese(phone)]
    phone: String,
}

/// 测试标准邮箱模式
#[derive(Debug, ChineseValidate, PartialEq)]
struct StandardEmailForm {
    #[chinese(standard_email)]
    email: String,
}

/// 测试商品订单表单
#[derive(Debug, ChineseValidate, PartialEq)]
struct OrderForm {
    #[chinese(name)]
    receiver_name: String,
    
    #[chinese(phone)]
    contact_phone: String,
    
    #[chinese(address)]
    shipping_address: String,
    
    #[chinese(postal_code)]
    postal_code: String,
}

/// 测试车辆信息表单
#[derive(Debug, ChineseValidate, PartialEq)]
struct VehicleForm {
    #[chinese(license_plate)]
    plate: String,
    
    #[chinese(qq)]
    owner_qq: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // ==================== 成功验证测试 ====================
    
    #[test]
    fn test_register_form_valid() {
        let form = RegisterForm {
            username: "张三".to_string(),
            phone: "13800138000".to_string(),
            id_card: "110101199001011234".to_string(),
            email: "张三@example.com".to_string(),
            nickname: "老张_007".to_string(),
        };
        
        let result = form.validate();
        assert!(result.is_ok(), "验证应该成功，但失败: {:?}", result.err());
    }
    
    #[test]
    fn test_anonymous_form_valid() {
        let form = AnonymousForm {
            name: "张*".to_string(),
            phone: "13912345678".to_string(),
        };
        
        assert!(form.validate().is_ok());
    }
    
    #[test]
    fn test_standard_email_form_valid() {
        let form = StandardEmailForm {
            email: "user@example.com".to_string(),
        };
        
        assert!(form.validate().is_ok());
    }
    
    #[test]
    fn test_order_form_valid() {
        let form = OrderForm {
            receiver_name: "李四".to_string(),
            contact_phone: "15712345678".to_string(),
            shipping_address: "北京市朝阳区建国门外大街1号".to_string(),
            postal_code: "100020".to_string(),
        };
        
        assert!(form.validate().is_ok());
    }
    
    #[test]
    fn test_vehicle_form_valid() {
        let form = VehicleForm {
            plate: "京A12345".to_string(),
            owner_qq: "12345678".to_string(),
        };
        
        assert!(form.validate().is_ok());
    }
    
    // ==================== 失败验证测试 ====================
    
    #[test]
    fn test_register_form_invalid_name() {
        let form = RegisterForm {
            username: "张".to_string(),  // 太短
            phone: "13800138000".to_string(),
            id_card: "110101199001011234".to_string(),
            email: "张三@example.com".to_string(),
            nickname: "老张_007".to_string(),
        };
        
        assert!(form.validate().is_err());
    }
    
    #[test]
    fn test_register_form_invalid_phone() {
        let form = RegisterForm {
            username: "张三".to_string(),
            phone: "12345678901".to_string(),  // 无效手机号
            id_card: "110101199001011234".to_string(),
            email: "张三@example.com".to_string(),
            nickname: "老张_007".to_string(),
        };
        
        assert!(form.validate().is_err());
    }
    
    #[test]
    fn test_register_form_invalid_id_card() {
        let form = RegisterForm {
            username: "张三".to_string(),
            phone: "13800138000".to_string(),
            id_card: "123456789012345678".to_string(),  // 无效身份证
            email: "张三@example.com".to_string(),
            nickname: "老张_007".to_string(),
        };
        
        assert!(form.validate().is_err());
    }
    
    #[test]
    fn test_register_form_invalid_email() {
        let form = RegisterForm {
            username: "张三".to_string(),
            phone: "13800138000".to_string(),
            id_card: "110101199001011234".to_string(),
            email: "not-an-email".to_string(),  // 无效邮箱
            nickname: "老张_007".to_string(),
        };
        
        assert!(form.validate().is_err());
    }
    
    #[test]
    fn test_register_form_invalid_nickname() {
        let form = RegisterForm {
            username: "张三".to_string(),
            phone: "13800138000".to_string(),
            id_card: "110101199001011234".to_string(),
            email: "张三@example.com".to_string(),
            nickname: "@#$%".to_string(),  // 包含非法字符
        };
        
        assert!(form.validate().is_err());
    }
    
    #[test]
    fn test_standard_email_form_invalid() {
        let form = StandardEmailForm {
            email: "张三@example.com".to_string(),  // 标准模式不支持中文
        };
        
        assert!(form.validate().is_err());
    }
    
    #[test]
    fn test_order_form_invalid_postal() {
        let form = OrderForm {
            receiver_name: "李四".to_string(),
            contact_phone: "15712345678".to_string(),
            shipping_address: "北京市朝阳区建国门外大街1号".to_string(),
            postal_code: "1234".to_string(),  // 无效邮编
        };
        
        assert!(form.validate().is_err());
    }
    
    #[test]
    fn test_vehicle_form_invalid_plate() {
        let form = VehicleForm {
            plate: "京A1234".to_string(),  // 无效车牌
            owner_qq: "12345678".to_string(),
        };
        
        assert!(form.validate().is_err());
    }
    
    // ==================== 多个字段验证测试 ====================
    
    #[test]
    fn test_validate_all_returns_all_errors() {
        let form = RegisterForm {
            username: "张".to_string(),              // 错误1
            phone: "123".to_string(),                 // 错误2
            id_card: "123".to_string(),               // 错误3
            email: "not-email".to_string(),           // 错误4
            nickname: "@#$%".to_string(),             // 错误5
        };
        
        let result = form.validate_all();
        assert!(result.is_err());
        
        match result {
            Err(chinese_validator::ValidationError::MultipleFields(fields)) => {
                assert_eq!(fields.len(), 5, "应该返回5个错误");
                assert!(fields.contains(&"username".to_string()));
                assert!(fields.contains(&"phone".to_string()));
                assert!(fields.contains(&"id_card".to_string()));
                assert!(fields.contains(&"email".to_string()));
                assert!(fields.contains(&"nickname".to_string()));
            }
            _ => panic!("应该返回 MultipleFields 错误"),
        }
    }
    
    // ==================== 边界条件测试 ====================
    
    #[test]
    fn test_empty_strings() {
        let form = RegisterForm {
            username: "".to_string(),
            phone: "".to_string(),
            id_card: "".to_string(),
            email: "".to_string(),
            nickname: "".to_string(),
        };
        
        assert!(form.validate().is_err());
    }
    
    #[test]
    fn test_very_long_strings() {
        let long_name = "张".repeat(20);
        let long_phone = "1".repeat(20);
        
        let form = RegisterForm {
            username: long_name,
            phone: long_phone,
            id_card: "110101199001011234".to_string(),
            email: "张三@example.com".to_string(),
            nickname: "nick".to_string(),
        };
        
        assert!(form.validate().is_err()); // 应该失败，因为太长
    }
}
