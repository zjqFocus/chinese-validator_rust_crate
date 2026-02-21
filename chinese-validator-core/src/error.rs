use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("字段 '{0}' 验证失败")]
    InvalidField(String),

    #[error("多个字段验证失败: {0:?}")]
    MultipleFields(Vec<String>),
}
