use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, ToTokens};
use syn::{Fields, Type};

pub struct StructField {
    pub struct_field: TokenStream,
    pub named_field: Ident,
    pub field_type: Ident,
}

impl StructField {
    pub fn from_fields(fields: &Fields) -> Vec<Self> {
        return fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let (struct_field, named_field) = if let Some(ident) = &field.ident {
                    (ident.clone().to_token_stream(), format_ident!("F{ident}"))
                } else {
                    (Literal::usize_unsuffixed(i).to_token_stream(), format_ident!("_{i}"))
                };

                let field_type = Self::type_to_ident(&field.ty);

                return StructField { struct_field, named_field, field_type };
            })
            .collect();
    }

    fn type_to_ident(ty: &Type) -> Ident {
        if let Type::Path(type_path) = ty {
            if let Some(last_segment) = type_path.path.segments.last() {
                return last_segment.ident.clone();
            }
        }

        panic!("Cant handle given type");
    }
}
