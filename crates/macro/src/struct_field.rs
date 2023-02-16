use proc_macro2::Ident;
use quote::format_ident;
use syn::{Field, Fields, Type};

pub struct StructField {
    pub i: usize,
    pub struct_field: Ident,
    pub gain_calculator_field: Ident,
    pub ty: Type,
    pub gain_ratio_calculator_call: Ident,
    pub is_equals: bool,
}

impl StructField {
    pub fn from_fields(fields: &Fields) -> Vec<Self> {
        let mut result = vec![];
        let mut i = 0;
        for field in fields {
            let (struct_field, gain_calculator_field) = if let Some(ident) = &field.ident {
                (ident.clone(), ident.clone())
            } else {
                (format_ident!("{i}"), format_ident!("_{i}"))
            };

            let ty = field.ty.clone();

            let (gain_ratio_calculator_call, is_equals) = Self::determine_gain_calculator_call(&ty);

            result.push(StructField { i, struct_field, gain_calculator_field, ty, gain_ratio_calculator_call, is_equals });
            i += 1;
        }

        return result;
    }

    fn determine_gain_calculator_call(ty: &Type) -> (Ident, bool) {
        if let Type::Path(typePath) = ty {
            if let Some(last_segment) = typePath.path.segments.last() {
                if last_segment.ident == format_ident!("bool") {
                    return (format_ident!("calculate_gain_ratio_bool"), true);
                } else if last_segment.ident == format_ident!("usize") {
                    return (format_ident!("calculate_gain_ratio_ord"), false);
                }
            }
        }

        return (format_ident!("calculate_gain_ratio_eq"), true);
    }
}
