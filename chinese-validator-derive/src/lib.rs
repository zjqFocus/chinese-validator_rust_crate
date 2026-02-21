use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Path}; // 删除 Attribute

// ... 其余代码不变
#[proc_macro_derive(ChineseValidate, attributes(chinese))]
pub fn chinese_validate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let validate_fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .filter_map(|field| {
                    let field_name = field.ident.as_ref().unwrap();
                    for attr in &field.attrs {
                        if attr.path().is_ident("chinese") {
                            if let Ok(meta) = attr.parse_args::<Path>() {
                                if let Some(seg) = meta.segments.last() {
                                    return Some((field_name, seg.ident.to_string()));
                                }
                            }
                        }
                    }
                    None
                })
                .collect::<Vec<_>>(),
            _ => vec![],
        },
        _ => vec![],
    };

    let validate_impls = validate_fields.iter().map(|(field_name, rule)| {
        let field_str = field_name.to_string();
        match rule.as_str() {
            "phone" => {
                quote! {
                    if !::chinese_validator_core::validators::validate_cn_phone(&self.#field_name) {
                        return Err(::chinese_validator_core::ValidationError::InvalidField(#field_str.to_string()));
                    }
                }
            }
            _ => quote! {}
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn validate(&self) -> Result<(), ::chinese_validator_core::ValidationError> {
                #(#validate_impls)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
