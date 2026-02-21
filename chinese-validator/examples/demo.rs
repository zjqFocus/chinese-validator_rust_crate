
//! 使用示例
//! 
//! 运行方式：
//! ```bash
//! cargo run --example demo
//! ```

use chinese_validator::ChineseValidate;

#[derive(ChineseValidate, Debug)]
#[allow(dead_code)]
struct RegisterForm {
    #[chinese(phone)]
    phone: String,
    
    #[chinese(name)]
    name: String,
}

fn main() {
    let valid_form = RegisterForm {
        phone: "13800138000".to_string(),
        name: "张三".to_string(),
    };
    
    println!("验证有效表单: {:?}", valid_form);
    match valid_form.validate() {
        Ok(_) => println!("✅ 验证通过！"),
        Err(e) => println!("❌ 验证失败: {}", e),
    }
    
    let invalid_form = RegisterForm {
        phone: "123".to_string(),
        name: "张".to_string(),
    };
    
    println!("\n验证无效表单: {:?}", invalid_form);
    match invalid_form.validate() {
        Ok(_) => println!("✅ 验证通过！"),
        Err(e) => println!("❌ 验证失败: {}", e),
    }
}