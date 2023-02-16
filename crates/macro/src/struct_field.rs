use proc_macro2::Ident;
use quote::format_ident;
use syn::Fields;

use crate::field_type::FieldType;

pub struct StructField {
    pub i: usize,
    pub struct_field: Ident,
    pub named_field: Ident,
    pub field_type: FieldType,
}

impl StructField {
    pub fn from_fields(fields: &Fields) -> Vec<Self> {
        return fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let (struct_field, named_field) = if let Some(ident) = &field.ident {
                    (ident.clone(), ident.clone())
                } else {
                    (format_ident!("{i}"), format_ident!("_{i}"))
                };

                let ty = field.ty.clone();

                let field_type = ty.into();

                return StructField { i, struct_field, named_field, field_type };
            })
            .collect();
    }
}
