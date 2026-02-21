//! 验证函数模块

mod address;
mod email;
mod id_card;
mod license_plate;
mod name;
mod phone;
mod postal_code;
mod qq;
mod username;

pub use address::validate_address;
pub use email::validate_email;
pub use id_card::validate_id_card;
pub use license_plate::validate_license_plate;
pub use name::validate_chinese_name;
pub use phone::validate_cn_phone;
pub use postal_code::validate_postal_code;
pub use qq::validate_qq;
pub use username::validate_username;
