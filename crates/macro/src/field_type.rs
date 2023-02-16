use proc_macro2::Ident;
use quote::format_ident;
use syn::Type;

pub enum FieldType {
    Bool,
    LiteralOrd(Ident),
    Eq(Ident),
}

impl From<Type> for FieldType {
    fn from(value: Type) -> Self {
        let ident = FieldType::type_to_ident(&value);

        return if ident == format_ident!("bool") {
            FieldType::Bool
        } else if FieldType::is_type_ord(&ident) {
            FieldType::LiteralOrd(ident)
        } else {
            FieldType::Eq(ident)
        };
    }
}

impl FieldType {
    pub fn get_ident(&self) -> Ident {
        return match self {
            FieldType::Bool => format_ident!("bool"),
            FieldType::LiteralOrd(ty) => ty.clone(),
            FieldType::Eq(ty) => ty.clone(),
        };
    }

    pub fn gain_ratio_calculator_function(&self) -> Ident {
        return match self {
            FieldType::Bool => format_ident!("calculate_gain_ratio_bool"),
            FieldType::LiteralOrd(_) => format_ident!("calculate_gain_ratio_ord"),
            FieldType::Eq(_) => format_ident!("calculate_gain_ratio_eq"),
        };
    }

    fn type_to_ident(ty: &Type) -> Ident {
        if let Type::Path(type_path) = ty {
            if let Some(last_segment) = type_path.path.segments.last() {
                return last_segment.ident.clone();
            }
        }

        panic!("Cant handle given type");
    }

    fn is_type_ord(ident: &Ident) -> bool {
        return [
            format_ident!("usize"),
            format_ident!("f64"),
            format_ident!("f32"),
            format_ident!("u64"),
            format_ident!("u32"),
            format_ident!("i64"),
            format_ident!("i32"),
        ]
        .contains(ident);
    }
}
