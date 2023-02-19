use proc_macro2::{Literal, TokenStream};
use quote::ToTokens;

pub trait ToFormattedTokens {
    fn to_formatted_tokens(&self) -> TokenStream;
}

impl ToFormattedTokens for &str {
    fn to_formatted_tokens(&self) -> TokenStream {
        return self.to_token_stream();
    }
}

macro_rules! literal_unsuffixed {
    ($($name:ident => $t:ident,)*) => ($(
        impl ToFormattedTokens for $t {
            fn to_formatted_tokens(&self) -> TokenStream {
                return Literal::$name(*self).to_token_stream();
            }
        }
    )*)
}

#[macro_export]
macro_rules! to_tokens {
    ($t:ident) => {
        impl ToFormattedTokens for $t {
            fn to_formatted_tokens(&self) -> TokenStream {
                return self.to_token_stream();
            }
        }
    };
}

literal_unsuffixed! {
    u8_unsuffixed => u8,
    u16_unsuffixed => u16,
    u32_unsuffixed => u32,
    u64_unsuffixed => u64,
    u128_unsuffixed => u128,
    usize_unsuffixed => usize,
    i8_unsuffixed => i8,
    i16_unsuffixed => i16,
    i32_unsuffixed => i32,
    i64_unsuffixed => i64,
    i128_unsuffixed => i128,
    isize_unsuffixed => isize,
    f32_unsuffixed => f32,
    f64_unsuffixed => f64,
}

to_tokens!(String);
to_tokens!(bool);
